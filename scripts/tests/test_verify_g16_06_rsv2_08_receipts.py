"""Static JSON parsing, exact snapshot integrity and portable receipt checks."""
import copy
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts'))
import verify_g16_06_rsv2_08_receipts as audit


class JsonInventoryTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.INVENTORY).read_bytes())

    def test_complete_snapshot_sizes_hashes_and_finite_values(self):
        result = audit.verify()
        self.assertEqual((result['files'], result['bytes']), (29, 18547344))
        self.assertEqual(result['nonfinite_numeric_values'], 0)
        for record in self.document()['files']:
            data = (ROOT / record['path']).read_bytes()
            self.assertEqual(len(data), record['bytes'])
            self.assertEqual(audit.digest(data), record['sha256'])
            self.assertTrue(record['valid_json'])
            self.assertEqual(record['nonfinite_numeric_values'], 0)

    def test_valid_nested_json_and_strings_are_not_nonfinite_numbers(self):
        value = audit.strict_json(b'{"a":[1,2.5,true,null,{"text":"NaN Infinity"}]}')
        self.assertEqual(audit.finite_number_counts(value), dict(integer_values=1, float_values=1))

    def test_malformed_json_is_rejected(self):
        for data in [b'{', b'{"a":}', b'{"a":1,}', b'{} {}', b'{"a":\xff}']:
            with self.subTest(data=data), self.assertRaises((ValueError, UnicodeError)):
                audit.strict_json(data)

    def test_nonstandard_nonfinite_constants_are_rejected_at_any_depth(self):
        for token in [b'NaN', b'Infinity', b'-Infinity']:
            with self.subTest(token=token), self.assertRaisesRegex(ValueError, 'Nonfinite'):
                audit.strict_json(b'{"nested":[{"value":' + token + b'}]}')

    def test_json_float_overflow_is_rejected(self):
        for token in [b'1e999', b'-1e999']:
            with self.subTest(token=token), self.assertRaisesRegex(ValueError, 'Nonfinite'):
                audit.strict_json(b'[' + token + b']')

    def test_forged_digest_size_validity_and_qualification_rejected(self):
        for key, value in [('sha256','0'*64), ('bytes',0), ('valid_json',False), ('nonfinite_numeric_values',1)]:
            d = copy.deepcopy(self.document()); d['files'][0][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, 'inventory metadata'):
                audit.verify(document=d)
        d = self.document(); d['qualified_count'] = 1
        with self.assertRaisesRegex(ValueError, 'inventory metadata'):
            audit.verify(document=d)

    def test_inventory_digest_is_independently_pinned(self):
        real = Path.read_bytes
        def changed(path):
            data = real(path)
            return data + b' ' if path == ROOT / audit.INVENTORY else data
        with patch.object(Path, 'read_bytes', new=changed), self.assertRaisesRegex(ValueError, 'Frozen JSON inventory digest'):
            audit.verify()

    def test_scope_coverage_and_nonportable_paths_rejected(self):
        paths = [row['path'] for row in self.document()['files']]
        with self.assertRaisesRegex(ValueError, 'coverage'):
            audit.generate(ROOT, paths[:-1])
        with self.assertRaisesRegex(ValueError, 'unique and sorted'):
            audit.generate(ROOT, paths + [paths[0]])
        for relative in ['/tmp/phase1.json', '../phase1.json', 'experiments\\phase1.json']:
            with self.subTest(path=relative), self.assertRaisesRegex(ValueError, 'Nonportable'):
                audit.safe_path(ROOT, relative)
        self.assertTrue(audit.matches('x/phase12_measurements/result.json'))
        self.assertTrue(audit.matches('x/phase12_receipt.json'))
        self.assertFalse(audit.matches('x/candidate_config.json'))

    def test_static_scope_qualification_and_new_output_paths(self):
        d = self.document()
        self.assertFalse(d['engineering_qualification_assessed'])
        self.assertFalse(d['full_qualification']); self.assertEqual(d['qualified_count'],0)
        files = [ROOT / audit.INVENTORY, ROOT / audit.INVENTORY.parent / 'PHASE21.md',
                 ROOT / 'scripts/verify_g16_06_rsv2_08_receipts.py', Path(__file__)]
        for path in files:
            self.assertNotIn(('/'+'home/'+'ubuntu').encode(), path.read_bytes())

    def test_relocated_read_only_missing_corrupt_and_nonfinite_inputs(self):
        base = ROOT / 'target/G16-06-RSV2-08/phase22'; base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            for relative in [*[r['path'] for r in self.document()['files']], audit.INVENTORY.as_posix()]:
                target = root / relative; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, target)
            self.assertFalse((root / '.git').exists()); self.assertFalse((root / 'target').exists())
            real = Path.open
            def guarded(path, mode='r', *args, **kwargs):
                if not path.is_relative_to(root) or any(c in mode for c in 'wax+'):
                    raise AssertionError('Nonlocal read or write')
                return real(path, mode, *args, **kwargs)
            with patch.object(Path, 'open', new=guarded), patch.dict('os.environ',{},clear=True), patch('subprocess.run', side_effect=AssertionError('Process launch')):
                self.assertEqual(audit.verify(root)['status'], 'pass')
            victim = root / self.document()['files'][0]['path']; original_bytes = victim.read_bytes()
            def absent(path, *args, **kwargs):
                if path == victim: raise FileNotFoundError(path)
                return real(path, *args, **kwargs)
            with patch.object(Path, 'open', new=absent), self.assertRaises(FileNotFoundError):
                audit.verify(root)
            victim.write_bytes(original_bytes + b' ')
            with self.assertRaisesRegex(ValueError, 'size or SHA-256'):
                audit.verify(root)
            victim.write_bytes(b'{"value":NaN}')
            with self.assertRaisesRegex(ValueError, 'Invalid or nonfinite JSON'):
                audit.verify(root)


class MarkdownReportIndexTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.MARKDOWN_INDEX).read_bytes())

    def test_index_metrics_and_combined_default_verification(self):
        d = self.document(); self.assertEqual(d['file_count'], 1)
        self.assertEqual(d['files'][0]['path'], (audit.INVENTORY.parent / 'PHASE21.md').as_posix())
        for row in d['files']:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row, dict(path=row['path'], **audit.markdown_record(data)))
        result = audit.verify_all()
        self.assertEqual(result['status'], 'pass')
        self.assertEqual(result['markdown_reports']['files'], 1)
        self.assertEqual(result['markdown_reports']['literal_host_path_count'], 0)

    def test_line_count_empty_unterminated_crlf_and_unicode(self):
        for data, count in [(b'',0), (b'a',1), (b'a\n',1), (b'a\n\n',2), (b'a\r\nb',2), ('é\n'.encode(),1)]:
            with self.subTest(data=data):
                record = audit.markdown_record(data)
                self.assertEqual(record['line_count'], count)
                self.assertEqual(record['bytes'], len(data))
        with self.assertRaises(UnicodeError):
            audit.markdown_record(b'\xff')

    def test_literal_host_paths_rejected(self):
        for path in ['/'+'home/alice/report', '/'+'Users/alice/report', '/'+'tmp/report',
                     '/'+'root/report', '/'+'var/tmp/report', '/'+'private/tmp/report', 'C:'+'\\Users\\alice\\report']:
            with self.subTest(path=path), self.assertRaisesRegex(ValueError, 'Literal host path'):
                audit.markdown_record(path.encode())
        self.assertEqual(audit.markdown_record(b'docs/audits/report.md')['literal_host_path_count'],0)

    def test_forged_size_hash_lines_or_host_count_rejected(self):
        for key, value in [('bytes',0), ('sha256','0'*64), ('line_count',0), ('literal_host_path_count',1)]:
            d = self.document(); d['files'][0][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, 'Markdown metadata'):
                audit.verify_markdown(document=d)

    def test_coverage_and_outside_paths_rejected(self):
        paths = [row['path'] for row in self.document()['files']]
        with self.assertRaisesRegex(ValueError, 'nonempty'):
            audit.generate_markdown(ROOT, [])
        with self.assertRaisesRegex(ValueError, 'unique'):
            audit.generate_markdown(ROOT, paths + paths)
        for path in ['/tmp/PHASE21.md','../PHASE21.md','docs/PHASE21.md']:
            with self.subTest(path=path), self.assertRaises(ValueError):
                audit.generate_markdown(ROOT, [path])

    def test_index_digest_pin_and_new_files_have_no_literal_host_paths(self):
        real = Path.read_bytes
        def changed(path):
            data = real(path)
            return data + b' ' if path == ROOT / audit.MARKDOWN_INDEX else data
        with patch.object(Path, 'read_bytes', new=changed), self.assertRaisesRegex(ValueError, 'Frozen Markdown index'):
            audit.verify_markdown()
        for path in [ROOT / audit.MARKDOWN_INDEX, ROOT / audit.INVENTORY.parent / 'PHASE22.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)

    def test_relocated_combined_read_only_and_missing_corrupt_host_path_reports(self):
        base = ROOT / 'target/G16-06-RSV2-08/phase23'; base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            files = [r['path'] for r in audit.strict_json((ROOT / audit.INVENTORY).read_bytes())['files']]
            files += [r['path'] for r in self.document()['files']]
            for relative in [*files, audit.INVENTORY.as_posix(), audit.MARKDOWN_INDEX.as_posix()]:
                target = root / relative; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, target)
            self.assertFalse((root / '.git').exists()); self.assertFalse((root / 'target').exists())
            real = Path.open
            def guarded(path, mode='r', *args, **kwargs):
                if not path.is_relative_to(root) or any(c in mode for c in 'wax+'):
                    raise AssertionError('Nonlocal read or write')
                return real(path, mode, *args, **kwargs)
            with patch.object(Path, 'open', new=guarded), patch.dict('os.environ',{},clear=True), patch('subprocess.run', side_effect=AssertionError('Process launch')):
                self.assertEqual(audit.verify_all(root)['markdown_reports']['status'],'pass')
            victim = root / self.document()['files'][0]['path']; original = victim.read_bytes()
            def absent(path, *args, **kwargs):
                if path == victim: raise FileNotFoundError(path)
                return real(path, *args, **kwargs)
            with patch.object(Path, 'open', new=absent), self.assertRaises(FileNotFoundError):
                audit.verify_markdown(root)
            victim.write_bytes(original + b'changed')
            with self.assertRaisesRegex(ValueError, 'Markdown metadata'):
                audit.verify_markdown(root)
            victim.write_bytes(original + ('/'+'home/alice/secret').encode())
            with self.assertRaisesRegex(ValueError, 'Literal host path'):
                audit.verify_markdown(root)


if __name__ == '__main__':
    unittest.main(verbosity=2)
