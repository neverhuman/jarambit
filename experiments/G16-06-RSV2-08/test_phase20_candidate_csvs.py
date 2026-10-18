"""Read-only CSV integrity audit of the frozen tracked candidate inventory."""
import ast
import copy
import csv
import hashlib
import io
import json
import math
from pathlib import Path, PurePosixPath
import re
import shutil
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
STUDY = Path('experiments/G16-06-RSV2-08')
AUDIT = STUDY / 'phase20_candidate_csv_schema_audit.json'
SOURCE = Path('crates/crescent/src/genome.rs')
AUDIT_SHA256 = '59548ae6e1a65c98bd6fb69535756ddb05511b1858a6d36b1073c6a73ab3f594'
COUNTS = {3:9, 4:8, 5:163, 6:72, 7:144, 8:12, 9:0, 10:0, 11:0, 12:0, 13:0, 14:0, 15:3}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def native_header(root):
    text = (root / SOURCE).read_text()
    n_cp = int(re.search(r'pub const N_CP: usize = (\d+);', text)[1])
    ng = int(re.search(r'pub const NG: usize = (\d+);', text)[1])
    names = ast.literal_eval(re.search(r'pub const NAMES:.*?= (\[.*?\]);', text, re.S)[1])
    header = [f'cp{i:02}' for i in range(n_cp)] + names + ['material']
    if (n_cp, ng, len(header)) != (40,128,129) or len(set(header)) != 129:
        raise ValueError('Native schema changed')
    return header


def inspect_csv(data, header):
    rows = list(csv.reader(io.StringIO(data.decode('utf-8'), newline=''), strict=True))
    if len(rows) != 2 or rows[0] != header or len(rows[1]) != len(header):
        raise ValueError('CSV header/row shape mismatch')
    try:
        genes = [float(value) for value in rows[1][:-1]]
    except ValueError as error:
        raise ValueError('Nonnumeric gene') from error
    if len(genes) != 128 or not all(math.isfinite(value) for value in genes):
        raise ValueError('Nonfinite gene or wrong gene count')
    if rows[1][-1] != 'Ti-6Al-4V':
        raise ValueError('Recorded material changed')
    return dict(sha256=digest(data), bytes=len(data), header_column_count=len(header),
                gene_column_count=len(genes), data_row_count=1, finite_gene_count=len(genes),
                all_genes_finite=True, material=rows[1][-1], qualified_count=0, full_qualification=False)


def generate(root, paths):
    root = Path(root); header = native_header(root); rows = []
    if paths != sorted(set(paths)):
        raise ValueError('Tracked path inventory must be unique and sorted')
    for value in paths:
        path = PurePosixPath(value)
        if path.is_absolute() or '..' in path.parts or '\\' in value:
            raise ValueError('Nonportable candidate path')
        match = re.fullmatch(r'experiments/G16-06-RSV2-08/phase(\d+)_candidates/[^/]+\.csv', value)
        if not match or int(match[1]) not in COUNTS:
            raise ValueError('Candidate outside assigned scope')
        rows.append(dict(path=value, phase=int(match[1]), **inspect_csv((root / value).read_bytes(), header)))
    counts = {phase:sum(row['phase'] == phase for row in rows) for phase in COUNTS}
    if counts != COUNTS:
        raise ValueError('Tracked candidate coverage mismatch')
    return dict(task='G16-06-RSV2-08-p21', study_phase=20,
                scope='Tracked CSV files in uppercase phase3..15 candidate directories at baseline ee41703; lowercase study directories excluded.',
                qualified_count=0, full_qualification=False, candidate_file_count=len(rows),
                tracked_path_list_sha256=digest(('\n'.join(paths)+'\n').encode()),
                schema_source=dict(path=SOURCE.as_posix(), sha256=digest((root / SOURCE).read_bytes())),
                schema=dict(header=header, gene_columns=128, material_columns=1, control_point_columns=40),
                phase_counts=[dict(phase=n, tracked_csv_count=counts[n], qualified_count=0, full_qualification=False) for n in COUNTS],
                files=rows)


def verify(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = json.loads((root / AUDIT).read_text())
    actual = generate(root, [row['path'] for row in document['files']])
    if actual != document:
        raise ValueError('CSV schema audit mismatch')
    return dict(integrity='pass', files=len(actual['files']), finite_genes=128*len(actual['files']),
                qualified_count=0, full_qualification=False)


class CandidateCsvTests(unittest.TestCase):
    def document(self):
        return json.loads((ROOT / AUDIT).read_text())

    def example(self):
        return (ROOT / self.document()['files'][0]['path']).read_bytes()

    def changed(self, mutate):
        rows = list(csv.reader(io.StringIO(self.example().decode()))); mutate(rows)
        output = io.StringIO(newline=''); csv.writer(output, lineterminator='\n').writerows(rows)
        return output.getvalue().encode()

    def test_digest_pin_every_file_and_finite_gene_count(self):
        self.assertEqual(digest((ROOT / AUDIT).read_bytes()), AUDIT_SHA256)
        self.assertEqual(verify()['files'], 411)
        self.assertEqual(verify()['finite_genes'], 52608)

    def test_phase_coverage_and_native_schema(self):
        d = self.document()
        self.assertEqual({p['phase']:p['tracked_csv_count'] for p in d['phase_counts']}, COUNTS)
        self.assertEqual(d['schema']['header'], native_header(ROOT))
        self.assertEqual(len(d['schema']['header']), 129)
        for row in d['files']:
            self.assertEqual(row['gene_column_count'], 128); self.assertTrue(row['all_genes_finite'])

    def test_nonfinite_and_nonnumeric_genes_rejected(self):
        for value in ['nan','inf','-inf','1e999','', 'not-a-float']:
            data = self.changed(lambda rows: rows[1].__setitem__(0, value))
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, 'Nonfinite|Nonnumeric'):
                inspect_csv(data, native_header(ROOT))

    def test_wrong_header_short_rows_and_extra_records_rejected(self):
        for mutate in [lambda r:r[0].__setitem__(0,'wrong'), lambda r:r[1].pop(), lambda r:r[1].append('0'), lambda r:r.append(list(r[1]))]:
            with self.assertRaisesRegex(ValueError, 'shape'):
                inspect_csv(self.changed(mutate), native_header(ROOT))
        data = self.changed(lambda r:r[1].__setitem__(-1, 'unknown-material'))
        with self.assertRaisesRegex(ValueError, 'material'):
            inspect_csv(data, native_header(ROOT))

    def test_omission_duplicate_and_unsafe_path_rejected(self):
        d = self.document(); paths = [r['path'] for r in d['files']]
        with self.assertRaisesRegex(ValueError, 'coverage'):
            generate(ROOT, paths[:-1])
        with self.assertRaisesRegex(ValueError, 'unique and sorted'):
            generate(ROOT, paths + [paths[0]])
        for value in ['/tmp/input.csv', '../input.csv', 'experiments\\input.csv']:
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, 'Nonportable'):
                generate(ROOT, [value])

    def test_forged_hash_count_or_qualification_rejected(self):
        for key, value in [('sha256','0'*64), ('gene_column_count',127), ('all_genes_finite',False), ('qualified_count',1), ('full_qualification',True)]:
            d = copy.deepcopy(self.document()); d['files'][0][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, 'audit mismatch'):
                verify(document=d)

    def test_no_engineering_qualification_inferred(self):
        d = self.document()
        for row in [d, *d['phase_counts'], *d['files']]:
            self.assertEqual(row['qualified_count'], 0); self.assertIs(row['full_qualification'], False)

    def test_new_phase20_files_contain_no_literal_host_prefix(self):
        for name in ['PHASE20.md','phase20_candidate_csv_schema_audit.json','test_phase20_candidate_csvs.py']:
            self.assertNotIn(('/'+'home/'+'ubuntu').encode(), (HERE / name).read_bytes())

    def test_relocated_read_only_missing_and_changed_candidate(self):
        base = ROOT / 'target/G16-06-RSV2-08/phase21'; base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            for relative in [*[r['path'] for r in self.document()['files']], SOURCE.as_posix(), AUDIT.as_posix()]:
                target = root / relative; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, target)
            self.assertFalse((root / '.git').exists()); self.assertFalse((root / 'target').exists())
            original = Path.open
            def guarded(path, mode='r', *args, **kwargs):
                if not path.is_relative_to(root) or any(c in mode for c in 'wax+'):
                    raise AssertionError('Nonlocal read or write')
                return original(path, mode, *args, **kwargs)
            with patch.object(Path, 'open', new=guarded), patch.dict('os.environ', {}, clear=True), patch('subprocess.run', side_effect=AssertionError('Process launch')):
                self.assertEqual(verify(root)['integrity'], 'pass')
            victim = root / self.document()['files'][0]['path']
            def missing(path, *args, **kwargs):
                if path == victim: raise FileNotFoundError(path)
                return original(path, *args, **kwargs)
            with patch.object(Path, 'open', new=missing), self.assertRaises(FileNotFoundError):
                verify(root)
            victim.write_bytes(victim.read_bytes().replace(b'\n',b'\r\n'))
            with self.assertRaisesRegex(ValueError, 'audit mismatch'):
                verify(root)


if __name__ == '__main__':
    unittest.main(verbosity=2)
