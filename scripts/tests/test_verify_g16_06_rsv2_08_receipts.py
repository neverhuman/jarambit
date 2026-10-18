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
            for relative in [*files, audit.INVENTORY.as_posix(), audit.MARKDOWN_INDEX.as_posix(), audit.CROSS_CHECK.as_posix(),
                             audit.SCRIPT_INVENTORY.as_posix(), audit.SCRIPT_CROSS_CHECK.as_posix(), *audit.SCRIPT_FILES,
                             audit.SUMMARY_MANIFEST.as_posix(), audit.MANIFEST_CROSS_CHECK.as_posix(), audit.CUMULATIVE_RECEIPT_INDEX.as_posix(),
                             audit.ASSERTION_INVENTORY.as_posix(),
                             *[path.as_posix() for _,_,path in audit.summary_paths()]]:
                target = root / relative; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, target)
            self.assertFalse((root / '.git').exists()); self.assertFalse((root / 'target').exists())
            real = Path.open
            def guarded(path, mode='r', *args, **kwargs):
                if not path.is_relative_to(root) or any(c in mode for c in 'wax+'):
                    raise AssertionError('Nonlocal read or write')
                return real(path, mode, *args, **kwargs)
            with patch.object(Path, 'open', new=guarded), patch.dict('os.environ',{},clear=True), patch('subprocess.run', side_effect=AssertionError('Process launch')):
                result = audit.verify_all(root)
                self.assertEqual(result['markdown_reports']['status'],'pass')
                self.assertEqual(result['inventory_cross_check']['unique_members'],30)
                self.assertEqual(result['script_test_inventory']['files'],2)
                self.assertEqual(result['script_and_inventory_cross_check']['unique_members'],32)
                self.assertEqual(result['phase21_to_25_summary']['files'],10)
                self.assertEqual(result['manifest_cross_check']['receipt_references'],4)
                self.assertEqual(result['cumulative_receipt_index']['files'],13)
                self.assertEqual(result['verifier_assertion_inventory']['test_methods'],audit.ast_test_method_count((root / audit.SCRIPT_FILES[1]).read_bytes()))
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


class InventoryCrossCheckTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.CROSS_CHECK).read_bytes())

    def sources(self):
        return [audit.strict_json((ROOT / path).read_bytes()) for path in [audit.INVENTORY, audit.MARKDOWN_INDEX]]

    def test_exact_source_and_member_digest_parity(self):
        d = self.document(); result = audit.verify_cross_check()
        self.assertEqual(result['unique_members'],30)
        self.assertEqual((d['json_file_count'],d['markdown_file_count'],d['total_member_bytes']),(29,1,18550633))
        self.assertEqual(d['overlap_count'],0); self.assertEqual(d['overlap_paths'],[])
        self.assertTrue(d['exact_digest_parity']); self.assertEqual(d['digest_mismatch_count'],0)
        for row in [*d['source_indexes'], *d['members']]:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['bytes'],len(data))
            self.assertEqual(row['expected_sha256'],audit.digest(data))
            self.assertEqual(row['actual_sha256'],row['expected_sha256'])
            self.assertTrue(row['digest_parity'])

    def test_overlap_rejected_even_when_digests_differ(self):
        left, right = self.sources()
        right['files'][0]['path'] = left['files'][0]['path']
        right['files'][0]['sha256'] = '0'*64
        with self.assertRaisesRegex(ValueError,'Cross-index path overlap'):
            audit.disjoint_index_entries(left,right)

    def test_duplicate_paths_in_either_index_rejected(self):
        for which in [0,1]:
            sources = self.sources(); sources[which]['files'].append(copy.deepcopy(sources[which]['files'][0]))
            with self.subTest(index=which), self.assertRaisesRegex(ValueError,'Duplicate path'):
                audit.disjoint_index_entries(*sources)

    def test_forged_overlap_counts_and_parity_rejected(self):
        for key,value in [('overlap_count',1),('exact_digest_parity',False),('unique_member_count',29),('digest_mismatch_count',1)]:
            d = self.document(); d[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'Cross-check audit metadata'):
                audit.verify_cross_check(document=d)
        d = self.document(); d['members'][0]['actual_sha256'] = '0'*64
        with self.assertRaisesRegex(ValueError,'digest parity mismatch'):
            audit.verify_cross_check(document=d)

    def test_cross_audit_and_both_source_index_pins_enforced(self):
        real = Path.read_bytes
        for relative in [audit.CROSS_CHECK,audit.INVENTORY,audit.MARKDOWN_INDEX]:
            victim = ROOT / relative
            def changed(path):
                data = real(path)
                return data + b' ' if path == victim else data
            with self.subTest(path=relative), patch.object(Path,'read_bytes',new=changed), self.assertRaisesRegex(ValueError,'digest mismatch'):
                audit.verify_cross_check()

    def test_static_scope_and_no_literal_host_paths_in_outputs(self):
        d = self.document(); self.assertFalse(d['engineering_qualification_assessed'])
        self.assertEqual(d['qualified_count'],0); self.assertFalse(d['full_qualification'])
        for path in [ROOT / audit.CROSS_CHECK, ROOT / audit.INVENTORY.parent / 'PHASE23.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)


class ScriptTestInventoryTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.SCRIPT_INVENTORY).read_bytes())

    def test_exact_script_hashes_sizes_and_ast_counts(self):
        d = self.document()
        self.assertEqual([row['path'] for row in d['files']],list(audit.SCRIPT_FILES))
        self.assertEqual(d['file_count'],2)
        for row in d['files']:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['sha256'],audit.digest(data)); self.assertEqual(row['bytes'],len(data))
            self.assertEqual(row['ast_test_method_count'],audit.ast_test_method_count(data))
        self.assertEqual(d['files'][0]['ast_test_method_count'],0)
        discovered = unittest.defaultTestLoader.loadTestsFromModule(sys.modules[__name__]).countTestCases()
        self.assertEqual(d['files'][1]['ast_test_method_count'],discovered)
        self.assertEqual(audit.verify_script_inventory()['status'],'pass')

    def test_ast_counts_direct_sync_async_and_decorated_methods_without_execution(self):
        source = b'''raise RuntimeError("Must never execute")
def test_top_level(): pass
class Example:
    def test_sync(self): pass
    async def test_async(self): pass
    @unknown_decorator
    def test_decorated(self): pass
    def helper(self):
        def test_nested_function(): pass
'''
        self.assertEqual(audit.ast_test_method_count(source),3)

    def test_invalid_python_syntax_is_rejected(self):
        with self.assertRaises(SyntaxError):
            audit.ast_test_method_count(b'def broken(')

    def test_forged_bytes_hash_count_and_coverage_rejected(self):
        for key,value in [('bytes',0),('sha256','0'*64),('ast_test_method_count',0),('valid_python_ast',False)]:
            d = self.document(); d['files'][1][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'Script inventory'):
                audit.verify_script_inventory(document=d)
        d = self.document(); d['files'].pop()
        with self.assertRaisesRegex(ValueError,'Script inventory'):
            audit.verify_script_inventory(document=d)

    def test_relocated_read_only_missing_and_changed_script(self):
        base = ROOT / 'target/G16-06-RSV2-08/phase25'; base.mkdir(parents=True,exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            for relative in [*audit.SCRIPT_FILES,audit.SCRIPT_INVENTORY.as_posix()]:
                target = root / relative; target.parent.mkdir(parents=True,exist_ok=True)
                shutil.copyfile(ROOT / relative,target)
            self.assertFalse((root / '.git').exists()); self.assertFalse((root / 'target').exists())
            real = Path.open
            def guarded(path,mode='r',*args,**kwargs):
                if not path.is_relative_to(root) or any(c in mode for c in 'wax+'):
                    raise AssertionError('Nonlocal read or write')
                return real(path,mode,*args,**kwargs)
            with patch.object(Path,'open',new=guarded), patch.dict('os.environ',{},clear=True), patch('subprocess.run',side_effect=AssertionError('Process launch')):
                self.assertEqual(audit.verify_script_inventory(root)['status'],'pass')
            victim = root / audit.SCRIPT_FILES[0]
            def absent(path,*args,**kwargs):
                if path == victim: raise FileNotFoundError(path)
                return real(path,*args,**kwargs)
            with patch.object(Path,'open',new=absent), self.assertRaises(FileNotFoundError):
                audit.verify_script_inventory(root)
            victim.write_bytes(victim.read_bytes()+b'\n# Byte change without AST method change\n')
            with self.assertRaisesRegex(ValueError,'Script inventory'):
                audit.verify_script_inventory(root)
            victim.write_bytes(b'def broken(')
            with self.assertRaises(SyntaxError):
                audit.verify_script_inventory(root)

    def test_static_scope_and_output_host_path_scan(self):
        d = self.document(); self.assertFalse(d['engineering_qualification_assessed'])
        self.assertEqual(d['qualified_count'],0); self.assertFalse(d['full_qualification'])
        for path in [ROOT / audit.SCRIPT_INVENTORY,ROOT / audit.INVENTORY.parent / 'PHASE24.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)


class ScriptInventoryCrossCheckTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.SCRIPT_CROSS_CHECK).read_bytes())

    def sources(self):
        previous = audit.strict_json((ROOT / audit.CROSS_CHECK).read_bytes())
        left = dict(files=[dict(path=row['path'],bytes=row['bytes'],sha256=row['expected_sha256']) for row in previous['members']])
        right = audit.strict_json((ROOT / audit.SCRIPT_INVENTORY).read_bytes())
        return left,right

    def test_exact_source_and_all_member_parity(self):
        d = self.document(); self.assertEqual(audit.verify_script_cross_check()['unique_members'],32)
        self.assertEqual((d['phase23_member_count'],d['phase24_member_count']),(30,2))
        self.assertEqual(d['overlap_paths'],[]); self.assertEqual(d['overlap_count'],0)
        self.assertTrue(d['exact_digest_parity']); self.assertEqual(d['digest_mismatch_count'],0)
        for row in d['source_receipts']:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['bytes'],len(data)); self.assertEqual(row['sha256'],audit.digest(data))
        for row in d['members']:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['bytes'],len(data))
            self.assertEqual(row['expected_sha256'],audit.digest(data))
            self.assertEqual(row['actual_sha256'],row['expected_sha256']); self.assertTrue(row['digest_parity'])

    def test_cross_domain_overlap_with_different_digest_rejected(self):
        left,right = self.sources(); right['files'][0]['path'] = left['files'][0]['path']
        right['files'][0]['sha256'] = '0'*64
        with self.assertRaisesRegex(ValueError,'Cross-index path overlap'):
            audit.disjoint_index_entries(left,right,labels=('phase23','phase24'))

    def test_duplicates_within_either_source_rejected(self):
        for which in [0,1]:
            sources = self.sources(); sources[which]['files'].append(copy.deepcopy(sources[which]['files'][0]))
            with self.subTest(source=which), self.assertRaisesRegex(ValueError,'Duplicate path'):
                audit.disjoint_index_entries(*sources,labels=('phase23','phase24'))

    def test_forged_parity_sources_and_counts_rejected(self):
        for key,value in [('overlap_count',1),('unique_member_count',31),('exact_digest_parity',False)]:
            d = self.document(); d[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'Phase23/24 cross-check'):
                audit.verify_script_cross_check(document=d)
        for group,key in [('members','actual_sha256'),('source_receipts','sha256')]:
            d = self.document(); d[group][0][key] = '0'*64
            with self.subTest(group=group), self.assertRaisesRegex(ValueError,'digest parity mismatch'):
                audit.verify_script_cross_check(document=d)

    def test_stale_missing_and_changed_source_receipts_rejected(self):
        real = Path.read_bytes
        def stale(path):
            data = real(path)
            if path == ROOT / audit.SCRIPT_INVENTORY:
                document = audit.strict_json(data); document['files'][0]['sha256'] = '0'*64
                return audit.json.dumps(document).encode()
            return data
        with patch.object(Path,'read_bytes',new=stale), self.assertRaisesRegex(ValueError,'Script inventory'):
            audit.verify_script_cross_check()
        def changed(path):
            data = real(path)
            return data+b' ' if path == ROOT / audit.CROSS_CHECK else data
        with patch.object(Path,'read_bytes',new=changed), self.assertRaisesRegex(ValueError,'Frozen cross-check'):
            audit.verify_script_cross_check()
        def missing(path):
            if path == ROOT / audit.SCRIPT_INVENTORY: raise FileNotFoundError(path)
            return real(path)
        with patch.object(Path,'read_bytes',new=missing), self.assertRaises(FileNotFoundError):
            audit.verify_script_cross_check()

    def test_static_scope_and_output_host_paths(self):
        d = self.document(); self.assertFalse(d['engineering_qualification_assessed'])
        self.assertEqual(d['qualified_count'],0); self.assertFalse(d['full_qualification'])
        for path in [ROOT / audit.SCRIPT_CROSS_CHECK,ROOT / audit.INVENTORY.parent / 'PHASE25.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)


class PhaseSummaryManifestTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.SUMMARY_MANIFEST).read_bytes())

    def test_complete_five_phase_ten_file_metrics(self):
        d = self.document(); self.assertEqual(audit.verify_summary_manifest()['files'],10)
        self.assertEqual(d['phase_count'],5)
        self.assertEqual([(r['phase'],r['kind'],r['path']) for r in d['files']],
                         [(p,k,path.as_posix()) for p,k,path in audit.summary_paths()])
        for row in d['files']:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['sha256'],audit.digest(data)); self.assertEqual(row['bytes'],len(data))
            self.assertEqual(row['line_count'],len(data.decode('utf-8').splitlines()))
            self.assertEqual(row['literal_host_path_count'],0)

    def test_forged_metrics_and_omitted_file_rejected(self):
        for key,value in [('sha256','0'*64),('bytes',0),('line_count',0),('literal_host_path_count',1)]:
            d = self.document(); d['files'][0][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'summary coverage'):
                audit.verify_summary_manifest(document=d)
        d = self.document(); d['files'].pop()
        with self.assertRaisesRegex(ValueError,'summary coverage'):
            audit.verify_summary_manifest(document=d)

    def test_forged_paths_are_never_used_as_inputs(self):
        for value in ['/tmp/PHASE21.md','../PHASE21.md','docs/PHASE999.md']:
            d = self.document(); d['files'][0]['path'] = value
            with self.subTest(path=value), self.assertRaisesRegex(ValueError,'summary coverage'):
                audit.verify_summary_manifest(document=d)

    def test_missing_report_rejected(self):
        real = Path.read_bytes; victim = ROOT / audit.INVENTORY.parent / 'PHASE25.md'
        def missing(path):
            if path == victim: raise FileNotFoundError(path)
            return real(path)
        with patch.object(Path,'read_bytes',new=missing), self.assertRaises(FileNotFoundError):
            audit.verify_summary_manifest()

    def test_changed_and_host_path_injected_report_rejected(self):
        real = Path.read_bytes; victim = ROOT / audit.INVENTORY.parent / 'PHASE25.md'
        for extra,message in [(b'\nchanged','summary coverage'),(('/'+'home/alice/report').encode(),'Literal host path')]:
            def changed(path):
                data = real(path)
                return data+extra if path == victim else data
            with self.subTest(extra=extra), patch.object(Path,'read_bytes',new=changed), self.assertRaisesRegex(ValueError,message):
                audit.verify_summary_manifest()

    def test_static_scope_qualification_and_new_output_host_paths(self):
        d = self.document(); self.assertFalse(d['engineering_qualification_assessed'])
        self.assertEqual(d['qualified_count'],0); self.assertFalse(d['full_qualification'])
        for path in [ROOT / audit.SUMMARY_MANIFEST,ROOT / audit.INVENTORY.parent / 'PHASE26.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)


class ManifestLiveCrossCheckTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.MANIFEST_CROSS_CHECK).read_bytes())

    def test_all_live_and_receipt_reference_parity(self):
        d = self.document(); self.assertEqual(audit.verify_manifest_cross_check()['summary_files'],10)
        self.assertEqual(d['checked_json_receipt_phases'],[23,24,25])
        self.assertEqual((len(d['file_checks']),len(d['receipt_reference_checks']),len(d['phase24_script_checks'])),(10,4,2))
        for row in [*d['file_checks'],*d['receipt_reference_checks'],*d['phase24_script_checks']]:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['expected_sha256'],audit.digest(data))
            self.assertEqual(row['actual_sha256'],row['expected_sha256'])
            self.assertEqual(row['expected_bytes'],len(data)); self.assertEqual(row['actual_bytes'],len(data))
            self.assertTrue(row['digest_parity']); self.assertTrue(row['byte_parity'])
        self.assertTrue(all(r['summary_reference_parity'] for r in d['receipt_reference_checks']))
        self.assertTrue(all(not r['in_summary'] for r in d['phase24_script_checks']))

    def test_bad_live_digest_or_size_rejected(self):
        row = audit.strict_json((ROOT / audit.SUMMARY_MANIFEST).read_bytes())['files'][0]
        for key,value in [('sha256','0'*64),('bytes',0)]:
            changed = dict(row); changed[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'Live manifest/reference'):
                audit.manifest_parity_record(ROOT,changed,{row['path']:row},True)

    def test_receipt_reference_missing_or_disagrees_with_summary_rejected(self):
        row = audit.strict_json((ROOT / audit.SUMMARY_MANIFEST).read_bytes())['files'][0]
        with self.assertRaisesRegex(ValueError,'reference missing from summary'):
            audit.manifest_parity_record(ROOT,row,{},True)
        for key,value in [('sha256','0'*64),('bytes',0)]:
            changed = dict(row); changed[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'Manifest reference'):
                audit.manifest_parity_record(ROOT,row,{row['path']:changed},True)

    def test_forged_audit_parity_coverage_and_paths_rejected(self):
        for key,value in [('exact_digest_and_byte_parity',False),('summary_file_count',9),('byte_mismatch_count',1)]:
            d = self.document(); d[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'Manifest cross-check'):
                audit.verify_manifest_cross_check(document=d)
        d = self.document(); d['receipt_reference_checks'][0]['expected_sha256'] = '0'*64
        with self.assertRaisesRegex(ValueError,'Manifest cross-check'):
            audit.verify_manifest_cross_check(document=d)
        for path in ['/tmp/input.json','../input.json','docs\\input.json']:
            with self.subTest(path=path), self.assertRaisesRegex(ValueError,'Nonportable'):
                audit.manifest_parity_record(ROOT,dict(path=path),{})

    def test_missing_live_file_rejected(self):
        real = Path.read_bytes; victim = ROOT / audit.INVENTORY.parent / 'PHASE25.md'
        def missing(path):
            if path == victim: raise FileNotFoundError(path)
            return real(path)
        with patch.object(Path,'read_bytes',new=missing), self.assertRaises(FileNotFoundError):
            audit.verify_manifest_cross_check()

    def test_static_scope_and_new_output_host_paths(self):
        d = self.document(); self.assertFalse(d['engineering_qualification_assessed'])
        self.assertEqual(d['qualified_count'],0); self.assertFalse(d['full_qualification'])
        for path in [ROOT / audit.MANIFEST_CROSS_CHECK,ROOT / audit.INVENTORY.parent / 'PHASE27.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)


class CumulativeReceiptIndexTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.CUMULATIVE_RECEIPT_INDEX).read_bytes())

    def test_exact_thirteen_receipt_coverage_digests_and_bytes(self):
        d = self.document(); self.assertEqual(audit.verify_cumulative_receipt_index()['files'],13)
        self.assertEqual([row['phase'] for row in d['files']],list(range(15,28)))
        self.assertEqual([(row['phase'],row['path']) for row in d['files']],
                         [(phase,path.as_posix()) for phase,path in audit.cumulative_receipt_paths()])
        for row in d['files']:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['sha256'],audit.digest(data)); self.assertEqual(row['bytes'],len(data))
            self.assertTrue(row['valid_json']); audit.strict_json(data)
        self.assertEqual(d['total_bytes'],sum(row['bytes'] for row in d['files']))

    def test_omission_duplicate_wrong_phase_and_path_rejected(self):
        original = self.document()
        for mutate in [lambda d:d['files'].pop(), lambda d:d['files'].append(copy.deepcopy(d['files'][0])),
                       lambda d:d['files'][0].__setitem__('phase',14), lambda d:d['files'][0].__setitem__('path','../unexpected.json')]:
            d = copy.deepcopy(original); mutate(d)
            with self.assertRaisesRegex(ValueError,'Cumulative receipt phase coverage'):
                audit.verify_cumulative_receipt_index(document=d)

    def test_forged_hash_size_and_qualification_rejected(self):
        for key,value in [('sha256','0'*64),('bytes',0),('qualified_count',1),('full_qualification',True)]:
            d = self.document(); d['files'][0][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'Cumulative receipt phase coverage'):
                audit.verify_cumulative_receipt_index(document=d)

    def test_source_qualification_claims_rejected(self):
        for phase,document in [(15,dict(all_gate_qualification=True)),
                               (16,dict(full_qualification=True,qualified_count=0)),
                               (16,dict(full_qualification=False,qualified_count=1))]:
            with self.subTest(phase=phase,document=document), self.assertRaisesRegex(ValueError,'qualification changed'):
                audit.cumulative_receipt_record(audit.json.dumps(document).encode(),phase,Path('receipt.json'))

    def test_missing_and_byte_changed_receipt_rejected(self):
        real = Path.read_bytes; victim = ROOT / audit.MANIFEST_CROSS_CHECK
        def missing(path):
            if path == victim: raise FileNotFoundError(path)
            return real(path)
        with patch.object(Path,'read_bytes',new=missing), self.assertRaises(FileNotFoundError):
            audit.verify_cumulative_receipt_index()
        def changed(path):
            data = real(path)
            return data+b' ' if path == victim else data
        with patch.object(Path,'read_bytes',new=changed), self.assertRaisesRegex(ValueError,'Cumulative receipt phase coverage'):
            audit.verify_cumulative_receipt_index()

    def test_static_scope_outputs_and_comms_unchanged(self):
        d = self.document(); self.assertFalse(d['engineering_qualification_assessed'])
        for row in [d,*d['files']]:
            self.assertEqual(row['qualified_count'],0); self.assertFalse(row['full_qualification'])
        for path in [ROOT / audit.CUMULATIVE_RECEIPT_INDEX,ROOT / audit.INVENTORY.parent / 'PHASE28.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)
        before = (ROOT / 'COMMS.md').read_bytes()
        audit.verify_cumulative_receipt_index()
        self.assertEqual((ROOT / 'COMMS.md').read_bytes(),before)


class VerifierAssertionInventoryTests(unittest.TestCase):
    def document(self):
        return audit.strict_json((ROOT / audit.ASSERTION_INVENTORY).read_bytes())

    def test_live_check_functions_sources_and_discovered_test_count(self):
        d = self.document(); self.assertEqual(audit.verify_assertion_inventory()['status'],'pass')
        expected = audit.assertion_function_records((ROOT / audit.SCRIPT_FILES[0]).read_bytes())
        self.assertEqual(d['check_functions'],expected)
        self.assertIn('verify_assertion_inventory',[row['name'] for row in expected])
        self.assertEqual(d['check_function_count'],len(expected))
        discovered = unittest.defaultTestLoader.loadTestsFromModule(sys.modules[__name__]).countTestCases()
        self.assertEqual(d['test_method_count'],discovered)
        self.assertEqual(sum(row['test_method_count'] for row in d['test_classes']),discovered)
        for row in d['sources']:
            data = (ROOT / row['path']).read_bytes()
            self.assertEqual(row['sha256'],audit.digest(data)); self.assertEqual(row['bytes'],len(data))

    def test_nested_guard_ownership_and_named_dispatcher_selection(self):
        source = b'''raise RuntimeError("Never execute")
def verify_parent():
    if True: raise ValueError("parent")
    def child():
        assert False
        raise RuntimeError("child")
    class Nested:
        def verify_member(self): raise ValueError("member")
def verify_dispatcher(): pass
def helper(): return "raise and assert in text"
'''
        rows = {row['name']:row for row in audit.assertion_function_records(source)}
        self.assertEqual(set(rows),{'verify_parent','verify_parent.child','verify_parent.Nested.verify_member','verify_dispatcher'})
        self.assertEqual((rows['verify_parent']['raise_statement_count'],rows['verify_parent']['assert_statement_count']),(1,0))
        self.assertEqual((rows['verify_parent.child']['raise_statement_count'],rows['verify_parent.child']['assert_statement_count']),(1,1))
        self.assertEqual(rows['verify_dispatcher']['raise_statement_count'],0)

    def test_invalid_source_syntax_rejected(self):
        with self.assertRaises(SyntaxError):
            audit.assertion_function_records(b'def broken(')

    def test_forged_guard_count_source_digest_and_test_coverage_rejected(self):
        for key,value in [('check_function_count',0),('test_method_count',0),('raise_statement_count',0)]:
            d = self.document(); d[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError,'assertion function inventory'):
                audit.verify_assertion_inventory(document=d)
        for field in ['check_functions','test_classes']:
            d = self.document(); d[field].pop()
            with self.subTest(field=field), self.assertRaisesRegex(ValueError,'assertion function inventory'):
                audit.verify_assertion_inventory(document=d)
        d = self.document(); d['sources'][0]['sha256'] = '0'*64
        with self.assertRaisesRegex(ValueError,'source digest'):
            audit.verify_assertion_inventory(document=d)

    def test_missing_inventory_source_rejected(self):
        real = Path.read_bytes; victim = ROOT / audit.SCRIPT_FILES[0]
        def absent(path):
            if path == victim: raise FileNotFoundError(path)
            return real(path)
        with patch.object(Path,'read_bytes',new=absent), self.assertRaises(FileNotFoundError):
            audit.verify_assertion_inventory()

    def test_static_outputs_and_comms_unchanged(self):
        d = self.document(); self.assertFalse(d['engineering_qualification_assessed'])
        self.assertEqual(d['qualified_count'],0); self.assertFalse(d['full_qualification'])
        for path in [ROOT / audit.ASSERTION_INVENTORY,ROOT / audit.INVENTORY.parent / 'PHASE29.md']:
            self.assertEqual(audit.markdown_record(path.read_bytes())['literal_host_path_count'],0)
        before = (ROOT / 'COMMS.md').read_bytes()
        audit.verify_assertion_inventory()
        self.assertEqual((ROOT / 'COMMS.md').read_bytes(),before)


if __name__ == '__main__':
    unittest.main(verbosity=2)
