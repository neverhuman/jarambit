"""Phase16 regressions kept separate to preserve Phase15's hash-bound test file."""
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
import audit_phase16 as phase16

# Frozen before Phase16: independent of the regenerable index.
FROZEN = {'experiments/G16-06-RSV2-08/PHASE7.md': '18c3ef60e9edf901174300a800e98128e2ce4fefa3398669d148bfa1864c828c', 'experiments/G16-06-RSV2-08/phase7_receipt.json': '71e751199151a99c61058625d55ad2358485e69e308b101d383b16b7ec8cee4a', 'experiments/G16-06-RSV2-08/PHASE8.md': '44c3876584adf55ecff5266e4852742faaa6ee74e2d32277e585205c1d98e55e', 'experiments/G16-06-RSV2-08/phase8_receipt.json': 'fecbe9932bfb8d905b38ca87bba137dfd39fcd8c585519e3eabb006c89167b44', 'experiments/g16-06-rsv2-08/PHASE9.md': '351f9fe7f6f0d107494e590c93c8478781e3c0a7a41b9267aa49759526ebfc1e', 'experiments/g16-06-rsv2-08/phase9_receipt.json': 'cc111ae884ad1f800cbda88491d57d420ead06bb75f12bcbee318ef80287d516', 'experiments/g16-06-rsv2-08/PHASE10.md': 'dab2897315cde615db39bbed66c4b95e53fe5bc52e13534fe69570f69e014219', 'experiments/g16-06-rsv2-08/phase10_receipt.json': '42ddc1663327794b71578d05fdaed678d0daac97e86dfbb8d4b0c4033fb92615', 'experiments/g16-06-rsv2-08/PHASE11.md': 'b4a0dcd19bda7719587b7880b76f67e637d6e5a752590afd2612f56490ec9c83', 'experiments/g16-06-rsv2-08/phase11_receipt.json': '26809af6faab94b759bdabad0b3dafe68bf8aaae5068b1276ee35bdd0ffe5d48', 'experiments/g16-06-rsv2-08/PHASE12.md': '4250477a92ab0fe51b32d2acdae7621a04b88a1b8930631cca25a61c940b34aa', 'experiments/g16-06-rsv2-08/phase12_receipt.json': '43457e945f9d4071ad02dbf48003755af889481309c06e5f0828895b3639639a', 'experiments/g16-06-rsv2-08/PHASE13.md': '7253eaf8ce0737ead6c517ef6db9af904c247d13efbb7bd570ebc63defa9b712', 'experiments/g16-06-rsv2-08/phase13_receipt.json': '26591423f6dad3332509295a7c97e2aa2ff3d140f274e52751e108fcb675288d', 'experiments/g16-06-rsv2-08/PHASE14.md': '7f7d60ec3df4d0f88c872b39525ba998f9971c9e3d0b091e45277e7f4c833132', 'experiments/g16-06-rsv2-08/phase14_receipt.json': '50773824742898a463ebb46954ed8d3ebd456a50900f1761378fb3483d833a81', 'experiments/G16-06-RSV2-08/PHASE15.md': 'c69286dc0dcc6fe324b8f5f00ad5f16e6bb9aee2492ed52e20c6c675a0a63ef8', 'experiments/G16-06-RSV2-08/phase15_gate_s_compliant_joint_audit.json': '205a359a7e9f1cd7065b5a35da7b1491fa6a37356a6e20473c4fff9451bfd51d'}


class CrossPhaseProvenanceTests(unittest.TestCase):
    def document(self):
        return json.loads((ROOT / phase16.INDEX).read_text())

    def test_all_eighteen_historical_files_unchanged(self):
        document = self.document()
        self.assertEqual(len(document['phases']), 9)
        for entry in document['phases']:
            for kind in ('report', 'receipt'):
                record = entry[kind]
                with self.subTest(path=record['path']):
                    path = ROOT / record['path']
                    self.assertTrue(path.is_file())
                    actual = hashlib.sha256(path.read_bytes()).hexdigest()
                    self.assertEqual(actual, FROZEN[record['path']])
                    self.assertEqual(actual, record['sha256'])
        self.assertEqual({e[k]['path'] for e in document['phases'] for k in ('report', 'receipt')}, set(FROZEN))

    def test_index_has_no_host_absolute_paths(self):
        self.assertNotIn(b'/home/ubuntu', (ROOT / phase16.INDEX).read_bytes())

    def test_historical_portability_blockers_are_not_hidden(self):
        failures = {}
        for entry in self.document()['phases']:
            for kind in ('report', 'receipt'):
                r = entry[kind]
                count = (ROOT / r['path']).read_bytes().count(b'/home/ubuntu')
                self.assertEqual(count, r['host_absolute_path_occurrences'])
                if count:
                    failures[entry['phase']] = count
        self.assertEqual(failures, {7: 148, 8: 16})
        with self.assertRaisesRegex(ValueError, 'Historical host-absolute paths remain'):
            phase16.verify(require_portable=True)

    def test_phase_coverage_counts_and_qualification(self):
        d = self.document()
        self.assertEqual([e['phase'] for e in d['phases']], list(range(7, 16)))
        self.assertEqual([e['candidate_count'] for e in d['phases']], [144,12,36,36,36,36,24,16,3])
        for entry in d['phases']:
            self.assertEqual(entry['qualified_count'], 0)
            self.assertIs(entry['full_qualification'], False)
        self.assertEqual(phase16.verify()['integrity'], 'pass')

    def test_read_only_without_process_or_environment(self):
        original = Path.open
        def read_only(path, mode='r', *args, **kwargs):
            if any(c in mode for c in 'wax+'):
                raise AssertionError('write attempted')
            return original(path, mode, *args, **kwargs)
        with patch.object(Path, 'open', new=read_only), patch.dict('os.environ', {}, clear=True), patch('subprocess.run', side_effect=AssertionError('Process launch')):
            self.assertEqual(phase16.verify()['indexed_files'], 18)

    def test_forged_hash_count_or_qualification_rejected(self):
        for key, value in [('candidate_count', 145), ('qualified_count', 1), ('full_qualification', True)]:
            d = self.document(); d['phases'][0][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, 'Index differs'):
                phase16.verify(document=d)
        d = self.document(); d['phases'][0]['receipt']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'Index differs'):
            phase16.verify(document=d)

    def test_nonportable_path_rejected(self):
        for value in ['/tmp/report.md', '../report.md', 'experiments/../report.md', 'experiments\\report.md']:
            d = self.document(); d['phases'][0]['report']['path'] = value
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, 'Nonportable'):
                phase16.verify(document=d)

    def test_relocation_missing_and_corrupt_files(self):
        # Only reports, receipts and index: no Git, native sources, target evidence or solver.
        base = ROOT / 'target/G16-06-RSV2-08/phase16'
        base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            for rel in [*FROZEN, str(phase16.INDEX)]:
                dest = root / rel; dest.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / rel, dest)
            self.assertFalse((root / '.git').exists())
            self.assertFalse((root / 'target').exists())
            self.assertEqual(phase16.verify(root)['integrity'], 'pass')
            victim = root / next(iter(FROZEN))
            with patch.object(Path, 'read_bytes', side_effect=FileNotFoundError('missing')):
                with self.assertRaises(FileNotFoundError):
                    phase16.verify(root)
            victim.write_bytes(victim.read_bytes() + b'corrupt')
            with self.assertRaisesRegex(ValueError, 'Index differs'):
                phase16.verify(root)


if __name__ == '__main__':
    unittest.main(verbosity=2)
