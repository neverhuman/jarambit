"""Read-only verification of the Phase18 manifest and its immutable inputs."""
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
STUDY = Path('experiments/G16-06-RSV2-08')
MANIFEST = STUDY / 'phase18_complete_study_manifest.json'
HOST = ('/' + 'home/' + 'ubuntu').encode()
MANIFEST_SHA256 = '1b5c2d6deb189b419d330d8166e0f20a0fb9f92c12f73b01e0bf3d369e3b7a52'


def phase_paths(phase):
    directory = Path('experiments') / ('g16-06-rsv2-08' if 9 <= phase <= 14 else 'G16-06-RSV2-08')
    names = {1: 'candidate_receipt.json', 15: 'phase15_gate_s_compliant_joint_audit.json',
             16: 'phase16_cross_phase_provenance_index.json', 17: 'phase17_legacy_path_inventory.json'}
    return directory / f'PHASE{phase}.md', directory / names.get(phase, f'phase{phase}_receipt.json')


def verify(root=ROOT, document=None, require_complete=False):
    root = Path(root)
    if document is None:
        document = json.loads((root / MANIFEST).read_text())
    if document['qualified_count'] != 0 or document['full_qualification'] is not False:
        raise ValueError('Manifest qualification changed')
    if [entry['phase'] for entry in document['phases']] != list(range(1, 18)):
        raise ValueError('Phase coverage mismatch')
    checked = 0
    missing = []
    for entry in document['phases']:
        if entry['qualified_count'] != 0 or entry['full_qualification'] is not False:
            raise ValueError('Phase qualification changed')
        report, receipt = phase_paths(entry['phase'])
        if entry['expected_report_path'] != report.as_posix():
            raise ValueError('Report identity mismatch')
        for kind, expected in [('report', report), ('receipt', receipt)]:
            record = entry[kind]
            if record is None:
                if kind != 'report' or entry['phase'] != 1 or (root / expected).exists():
                    raise ValueError('Unexpected missing report')
                missing.append(expected.as_posix())
                continue
            value = record['path']; path = PurePosixPath(value)
            if path.is_absolute() or '..' in path.parts or '\\' in value:
                raise ValueError('Nonportable path')
            if value != expected.as_posix():
                raise ValueError('Artifact identity mismatch')
            data = (root / expected).read_bytes()
            if len(data) != record['bytes'] or hashlib.sha256(data).hexdigest() != record['sha256']:
                raise ValueError('Artifact size or SHA-256 mismatch')
            if kind == 'receipt':
                source = json.loads(data)
                flag = source['all_gate_qualification'] if entry['phase'] <= 15 else source['full_qualification']
                if flag is not False:
                    raise ValueError('Historical qualification changed')
            checked += 1
    if document['indexed_file_count'] != checked or document['missing_reports'] != missing:
        raise ValueError('Manifest totals mismatch')
    if document['all_requested_reports_present'] is not (not missing):
        raise ValueError('Completeness claim mismatch')
    if require_complete and missing:
        raise ValueError('Historical phase report missing: ' + ', '.join(missing))
    return dict(integrity='pass', files=checked, missing_reports=missing,
                all_requested_reports_present=not missing, qualified_count=0, full_qualification=False)


class CompleteStudyManifestTests(unittest.TestCase):
    def document(self):
        return json.loads((ROOT / MANIFEST).read_text())

    def test_manifest_digest_pins_historical_hashes_and_sizes(self):
        self.assertEqual(hashlib.sha256((ROOT / MANIFEST).read_bytes()).hexdigest(), MANIFEST_SHA256)
        self.assertEqual(verify()['files'], 33)

    def test_all_seventeen_phases_and_qualification(self):
        d = self.document()
        self.assertEqual([entry['phase'] for entry in d['phases']], list(range(1, 18)))
        for entry in [d, *d['phases']]:
            self.assertEqual(entry['qualified_count'], 0)
            self.assertIs(entry['full_qualification'], False)

    def test_missing_phase1_report_is_explicit_and_strictly_rejected(self):
        d = self.document()
        self.assertIsNone(d['phases'][0]['report'])
        self.assertFalse(d['all_requested_reports_present'])
        self.assertEqual(d['missing_reports'], [(STUDY / 'PHASE1.md').as_posix()])
        with self.assertRaisesRegex(ValueError, 'Historical phase report missing'):
            verify(require_complete=True)

    def test_nonportable_or_wrong_paths_rejected(self):
        for value in ['/tmp/input.json', '../input.json', 'experiments\\input.json', 'experiments/wrong.json']:
            d = self.document(); d['phases'][0]['receipt']['path'] = value
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, 'path|identity'):
                verify(document=d)

    def test_forged_digest_size_or_qualification_rejected(self):
        for field, value in [('sha256', '0'*64), ('bytes', 0)]:
            d = self.document(); d['phases'][0]['receipt'][field] = value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, 'size or SHA-256'):
                verify(document=d)
        for field, value in [('qualified_count', 1), ('full_qualification', True)]:
            d = self.document(); d['phases'][0][field] = value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, 'qualification'):
                verify(document=d)

    def test_coverage_and_false_completeness_rejected(self):
        d = self.document(); d['phases'].pop()
        with self.assertRaisesRegex(ValueError, 'coverage'):
            verify(document=d)
        d = self.document(); d['all_requested_reports_present'] = True
        with self.assertRaisesRegex(ValueError, 'Completeness'):
            verify(document=d)

    def test_new_phase18_files_have_zero_literal_host_prefixes(self):
        for path in [HERE / 'PHASE18.md', ROOT / MANIFEST, HERE / 'test_phase18_manifest.py']:
            self.assertEqual(path.read_bytes().count(HOST), 0)

    def test_relocated_read_only_missing_and_corrupt_evidence(self):
        base = ROOT / 'target/G16-06-RSV2-08/phase19'
        base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            files = [entry[k]['path'] for entry in self.document()['phases'] for k in ['report', 'receipt'] if entry[k]]
            for relative in [*files, MANIFEST.as_posix()]:
                destination = root / relative; destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, destination)
            self.assertFalse((root / '.git').exists()); self.assertFalse((root / 'target').exists())
            original = Path.open
            def guarded(path, mode='r', *args, **kwargs):
                if not path.is_relative_to(root) or any(c in mode for c in 'wax+'):
                    raise AssertionError('Nonlocal read or write')
                return original(path, mode, *args, **kwargs)
            with patch.object(Path, 'open', new=guarded), patch('subprocess.run', side_effect=AssertionError('Process launch')), patch.dict('os.environ', {}, clear=True):
                self.assertEqual(verify(root)['integrity'], 'pass')
            victim = root / files[0]; original_bytes = victim.read_bytes()
            def missing(path, *args, **kwargs):
                if path == victim:
                    raise FileNotFoundError(path)
                return original(path, *args, **kwargs)
            with patch.object(Path, 'open', new=missing), self.assertRaises(FileNotFoundError):
                verify(root)
            victim.write_bytes(original_bytes + b'corrupt')
            with self.assertRaisesRegex(ValueError, 'size or SHA-256'):
                verify(root)


if __name__ == '__main__':
    unittest.main(verbosity=2)
