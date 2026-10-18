"""Exact legacy keypath coverage and relocated scientific evidence resolution."""
import copy
import hashlib
import json
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
import audit_phase7
import audit_phase8
import audit_phase16
import audit_phase17 as phase17


class LegacyPathTests(unittest.TestCase):
    def document(self):
        return json.loads((ROOT / phase17.INVENTORY).read_text())

    def test_exact_keypath_inventory_and_portable_receipts(self):
        d = self.document()
        self.assertEqual([p['legacy_host_string_count'] for p in d['phases']], [148,16] + [0]*8)
        for phase in d['phases']:
            source = json.loads((ROOT / phase['receipt']).read_text())
            found = {(tuple(k), kind): v for k, v, kind in phase17.strings(source)}
            entries = phase.get('entries', [])
            self.assertEqual(set(found), {(tuple(e['keypath']), e['string_kind']) for e in entries})
            for entry in entries:
                value = found[(tuple(entry['keypath']), entry['string_kind'])]
                self.assertEqual(hashlib.sha256(value.encode()).hexdigest(), entry['original_string_sha256'])
        self.assertEqual(phase17.verify()['legacy_strings'], 164)

    def test_scientific_bindings_and_non_evidence_exceptions(self):
        entries = [e for p in self.document()['phases'] for e in p.get('entries', [])]
        categories = {c: sum(e['classification'] == c for e in entries) for c in {e['classification'] for e in entries}}
        self.assertEqual(categories, dict(tracked_genome=158, optional_raw_genome=3,
                                         external_delivery_destination=2, historical_git_error=1))
        for entry in entries:
            relative = entry['canonical_repository_relative_path']
            if relative is None:
                self.assertNotIn('genome', entry['classification'])
            else:
                path = phase17.portable.safe_path(ROOT, relative)
                if path.is_file():
                    self.assertEqual(phase17.sha(path), entry['expected_sha256'])
                else:
                    self.assertEqual(entry['classification'], 'optional_raw_genome')

    def test_historical_files_unchanged(self):
        self.assertEqual(phase17.sha(ROOT / phase17.INVENTORY), '9d2d21d0d3c89be4ef84089b1548ca6dd9d21f65af43b5930820437c96c6ad15')
        for relative, expected in self.document()['historical_files'].items():
            with self.subTest(path=relative):
                self.assertEqual(phase17.sha(ROOT / relative), expected)

    def test_three_deliverables_contain_no_literal_host_prefix(self):
        for name in ['PHASE17.md', 'phase17_legacy_path_inventory.json', 'test_phase17_legacy_paths.py']:
            self.assertNotIn(phase17.HOST.encode(), (HERE / name).read_bytes())

    def test_phase16_and_phase17_files_contain_no_literal_host_prefix(self):
        paths = set(HERE.glob('PHASE16*')) | set(HERE.glob('PHASE17*'))
        paths.update(HERE / name for name in [
            'audit_phase16.py', 'audit_phase17.py',
            'test_phase16_provenance.py', 'test_phase17_legacy_paths.py',
            'phase16_cross_phase_provenance_index.json', 'phase17_legacy_path_inventory.json'])
        self.assertGreaterEqual(len(paths), 8)
        for path in sorted(paths):
            with self.subTest(path=path.name):
                self.assertEqual(path.read_bytes().count(phase17.HOST.encode()), 0)

    def test_qualification_stays_false(self):
        d = self.document()
        for value in [d, *d['phases']]:
            self.assertEqual(value['qualified_count'], 0)
            self.assertIs(value['full_qualification'], False)

    def test_tampering_is_rejected(self):
        for field, value in [('canonical_repository_relative_path', '../unsafe.csv'),
                             ('expected_sha256', '0'*64), ('keypath', ['wrong'])]:
            d = copy.deepcopy(self.document()); d['phases'][0]['entries'][0][field] = value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, 'inventory or historical'):
                phase17.verify(document=d)

    def test_relocated_verifiers_never_read_original_host_paths(self):
        d = self.document()
        files = set(d['historical_files']) | {phase17.INVENTORY.as_posix()}
        for phase in d['phases'][:2]:
            receipt = json.loads((ROOT / phase['receipt']).read_text())
            files.update(p for p in receipt['sha256'] if not p.startswith('target/'))
            files.update(e['canonical_repository_relative_path'] for e in phase['entries']
                         if e['canonical_repository_relative_path'] is not None)
        base = ROOT / 'target/G16-06-RSV2-08/phase17'
        base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            for relative in files:
                source = ROOT / relative
                if not source.is_file():
                    self.assertTrue(relative.startswith('target/'))
                    continue
                destination = root / relative
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source, destination)
            self.assertFalse((root / '.git').exists())
            original_open, original_stat = Path.open, Path.stat
            def inside(path):
                if not path.is_relative_to(root):
                    raise AssertionError('Read outside relocated repository: ' + str(path))
            def guarded_open(path, mode='r', *args, **kwargs):
                inside(path)
                if any(c in mode for c in 'wax+'):
                    raise AssertionError('Verifier write attempted')
                return original_open(path, mode, *args, **kwargs)
            def guarded_stat(path, *args, **kwargs):
                inside(path)
                return original_stat(path, *args, **kwargs)
            def no_raw(path, *args, **kwargs):
                inside(path)
                if path.is_relative_to(root / 'target'):
                    raise FileNotFoundError(path)
                return original_stat(path, *args, **kwargs)
            with patch.object(Path, 'open', new=guarded_open), patch.object(Path, 'stat', new=guarded_stat), patch('subprocess.run', side_effect=AssertionError('Process launch')):
                self.assertEqual(audit_phase7.audit(root)['status'], 'pass')
                self.assertEqual(audit_phase8.audit(root)['status'], 'pass')
                self.assertEqual(audit_phase16.verify(root)['integrity'], 'pass')
                self.assertEqual(phase17.verify(root)['status'], 'pass')
                with self.assertRaisesRegex(ValueError, 'Historical host-absolute'):
                    audit_phase16.verify(root, require_portable=True)
                with patch.object(Path, 'stat', new=no_raw):
                    self.assertEqual(audit_phase7.audit(root)['raw_evidence']['checked'], 0)
                    self.assertEqual(audit_phase8.audit(root)['raw_evidence']['checked'], 0)
                    self.assertEqual(phase17.verify(root)['status'], 'pass')


if __name__ == '__main__':
    unittest.main(verbosity=2)
