#!/usr/bin/env python3
"""Read-only verification of tracked JSON and Markdown snapshots; no numerical work."""
import argparse
import ast
import fnmatch
import hashlib
import json
import math
import re
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
INVENTORY = Path('docs/audits/G16-06_RSV2-08/phase21_measurement_json_inventory_audit.json')
BASELINE = '2374e553599b51da4caea6aba9972c30961bea0c'
TRACKED_PATHS_SHA256 = 'ca23fb3bcd3935bef0c17647c1106d600a96fc883507b1da1e059f938256b325'
INVENTORY_SHA256 = '6de32c8ba3c3a90c535b9eb5caff8766f8f00eb7511c584c114cf6a4b1cd63fa'
MARKDOWN_INDEX = INVENTORY.parent / 'phase22_markdown_report_digest_index.json'
MARKDOWN_BASELINE = '7856869b68099329c11980db845391d2f4fa3a35'
MARKDOWN_PATHS_SHA256 = '1ab223e05331f03aec309f5d1d281262f82d143734a0d66593a7a05e881e61d9'
MARKDOWN_INDEX_SHA256 = '7c81cebd99494529cbc1978df11ffc3f0136b2f837f42232d816ef7239df5f5a'
CROSS_CHECK = INVENTORY.parent / 'phase23_inventory_cross_check_audit.json'
CROSS_CHECK_SHA256 = '5d6c1e1b4046a537ccbecd7f64a9e887b2aa384dd8eed5fdb0b15acbc76e1784'
SCRIPT_INVENTORY = INVENTORY.parent / 'phase24_script_test_inventory_audit.json'
SCRIPT_FILES = ('scripts/verify_g16_06_rsv2_08_receipts.py',
                'scripts/tests/test_verify_g16_06_rsv2_08_receipts.py')
SCRIPT_CROSS_CHECK = INVENTORY.parent / 'phase25_script_and_inventory_cross_check_audit.json'
SUMMARY_MANIFEST = INVENTORY.parent / 'phase26_summary_manifest_audit.json'
MANIFEST_CROSS_CHECK = INVENTORY.parent / 'phase27_manifest_cross_check_audit.json'
CUMULATIVE_RECEIPT_INDEX = INVENTORY.parent / 'phase28_cumulative_receipt_index_audit.json'
ASSERTION_INVENTORY = INVENTORY.parent / 'phase29_verifier_assertion_inventory_audit.json'
JSON_SCHEMA_KEYSET_AUDIT = INVENTORY.parent / 'phase30_json_schema_keyset_audit.json'
MARKDOWN_HEADING_AUDIT = INVENTORY.parent / 'phase31_markdown_heading_consistency_audit.json'
HEX64_DIGEST_AUDIT = INVENTORY.parent / 'phase32_hex64_digest_format_audit.json'
HOST_PATH_PATTERN = re.compile(r'/(?:home|Users|tmp|root|var/tmp|private/tmp)/|[A-Za-z]:[\\/](?:Users|Temp)[\\/]')


def digest(data):
    return hashlib.sha256(data).hexdigest()


def matches(relative):
    path = PurePosixPath(relative)
    return fnmatch.fnmatchcase(path.name, 'phase*.json') or (
        len(path.parts) > 1 and fnmatch.fnmatchcase(path.parts[-2], 'phase*_measurements')
        and path.suffix == '.json')


def safe_path(root, relative):
    path = PurePosixPath(relative)
    if path.is_absolute() or '..' in path.parts or '\\' in relative or path.as_posix() != relative:
        raise ValueError('Nonportable inventory path')
    if not matches(relative):
        raise ValueError('Path outside inventory patterns')
    return root / relative


def strict_json(data):
    def reject_constant(value):
        raise ValueError('Nonfinite JSON constant: ' + value)
    def finite_float(value):
        number = float(value)
        if not math.isfinite(number):
            raise ValueError('Nonfinite JSON number: ' + value)
        return number
    return json.loads(data, parse_constant=reject_constant, parse_float=finite_float)


def finite_number_counts(document):
    # strict_json rejects nonfinite numbers at every depth before this traversal.
    counts = dict(integer_values=0, float_values=0)
    def visit(value):
        if isinstance(value, dict):
            for child in value.values(): visit(child)
        elif isinstance(value, list):
            for child in value: visit(child)
        elif type(value) is int:
            counts['integer_values'] += 1
        elif type(value) is float:
            if not math.isfinite(value): raise ValueError('Nonfinite parsed number')
            counts['float_values'] += 1
    visit(document)
    return counts


def generate(root, paths):
    root = Path(root)
    if paths != sorted(set(paths)) or not paths:
        raise ValueError('Inventory paths must be nonempty, unique and sorted')
    path_digest = digest(('\n'.join(paths) + '\n').encode())
    if path_digest != TRACKED_PATHS_SHA256:
        raise ValueError('Tracked snapshot coverage mismatch')
    records = []
    for relative in paths:
        data = safe_path(root, relative).read_bytes()
        try:
            parsed = strict_json(data)
        except (ValueError, UnicodeError) as error:
            raise ValueError('Invalid or nonfinite JSON in ' + relative + ': ' + str(error)) from error
        records.append(dict(path=relative, bytes=len(data), sha256=digest(data), valid_json=True,
                            nonfinite_numeric_values=0, **finite_number_counts(parsed)))
    return dict(task='G16-06-RSV2-08-p22', study_phase=21, baseline_commit=BASELINE,
                scope='All tracked repository files matching phase*_measurements/*.json or phase*.json at the baseline; newly created untracked outputs excluded.',
                tracked_paths_sha256=path_digest, file_count=len(records),
                total_bytes=sum(row['bytes'] for row in records),
                all_valid_json=True, nonfinite_numeric_values=0,
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                files=records)


def verify(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        data = (root / INVENTORY).read_bytes()
        if digest(data) != INVENTORY_SHA256:
            raise ValueError('Frozen JSON inventory digest mismatch')
        document = strict_json(data)
    actual = generate(root, [record['path'] for record in document['files']])
    if document != actual:
        raise ValueError('JSON inventory metadata, size or SHA-256 mismatch')
    return dict(status='pass', files=actual['file_count'], bytes=actual['total_bytes'],
                nonfinite_numeric_values=0, engineering_qualification_assessed=False,
                qualified_count=0, full_qualification=False)


def markdown_record(data):
    text = data.decode('utf-8')
    host_paths = len(HOST_PATH_PATTERN.findall(text))
    if host_paths:
        raise ValueError('Literal host path found in Markdown report')
    return dict(bytes=len(data), sha256=digest(data), line_count=len(text.splitlines()),
                literal_host_path_count=host_paths)


def generate_markdown(root, paths):
    root = Path(root)
    if not paths or paths != sorted(set(paths)):
        raise ValueError('Markdown paths must be nonempty, unique and sorted')
    path_digest = digest(('\n'.join(paths) + '\n').encode())
    if path_digest != MARKDOWN_PATHS_SHA256:
        raise ValueError('Tracked Markdown coverage mismatch')
    files = []
    for value in paths:
        path = PurePosixPath(value)
        if path.parent != PurePosixPath(INVENTORY.parent.as_posix()) or not fnmatch.fnmatchcase(path.name, 'PHASE*.md'):
            raise ValueError('Markdown path outside assigned report directory')
        if path.as_posix() != value or '\\' in value:
            raise ValueError('Nonportable Markdown path')
        files.append(dict(path=value, **markdown_record((root / value).read_bytes())))
    return dict(task='G16-06-RSV2-08-p23', study_phase=22, baseline_commit=MARKDOWN_BASELINE,
                scope='All tracked PHASE*.md files directly in docs/audits/G16-06_RSV2-08 at the baseline; new untracked report excluded.',
                tracked_paths_sha256=path_digest, file_count=len(files),
                total_bytes=sum(row['bytes'] for row in files),
                total_lines=sum(row['line_count'] for row in files), literal_host_path_count=0,
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                files=files)


def verify_markdown(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        data = (root / MARKDOWN_INDEX).read_bytes()
        if digest(data) != MARKDOWN_INDEX_SHA256:
            raise ValueError('Frozen Markdown index digest mismatch')
        document = strict_json(data)
    actual = generate_markdown(root, [row['path'] for row in document['files']])
    if actual != document:
        raise ValueError('Markdown metadata, size, SHA-256 or line count mismatch')
    return dict(status='pass', files=actual['file_count'], bytes=actual['total_bytes'],
                lines=actual['total_lines'], literal_host_path_count=0)


def disjoint_index_entries(json_document, markdown_document, labels=('json', 'markdown')):
    entries = []
    seen = set()
    for label, document in zip(labels, (json_document, markdown_document)):
        local = [row['path'] for row in document['files']]
        if len(local) != len(set(local)):
            raise ValueError('Duplicate path within ' + label + ' index')
        overlap = seen.intersection(local)
        if overlap:
            raise ValueError('Cross-index path overlap: ' + ', '.join(sorted(overlap)))
        seen.update(local)
        entries.extend((label, row) for row in document['files'])
    return entries


def generate_cross_check(root):
    root = Path(root)
    documents = []
    inputs = []
    for relative, expected in [(INVENTORY, INVENTORY_SHA256), (MARKDOWN_INDEX, MARKDOWN_INDEX_SHA256)]:
        data = (root / relative).read_bytes()
        actual = digest(data)
        if actual != expected:
            raise ValueError('Cross-check source index digest mismatch: ' + relative.as_posix())
        documents.append(strict_json(data))
        inputs.append(dict(path=relative.as_posix(), bytes=len(data), expected_sha256=expected,
                           actual_sha256=actual, digest_parity=True))
    entries = disjoint_index_entries(*documents)
    verify(root, documents[0])
    verify_markdown(root, documents[1])
    members = []
    for label, row in entries:
        data = (root / row['path']).read_bytes()
        actual = digest(data)
        if actual != row['sha256'] or len(data) != row['bytes']:
            raise ValueError('Cross-check member digest/size mismatch: ' + row['path'])
        members.append(dict(path=row['path'], source_index=label, bytes=len(data),
                            expected_sha256=row['sha256'], actual_sha256=actual, digest_parity=True))
    return dict(task='G16-06-RSV2-08-p24', study_phase=23,
                scope='Cross-check unchanged Phase21 JSON and Phase22 Markdown frozen indexes; no resnapshotting or numerical work.',
                source_indexes=inputs, json_file_count=len(documents[0]['files']),
                markdown_file_count=len(documents[1]['files']), unique_member_count=len(members),
                overlap_paths=[], overlap_count=0, exact_digest_parity=True,
                digest_mismatch_count=0, total_member_bytes=sum(row['bytes'] for row in members),
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                members=members)


def verify_cross_check(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        data = (root / CROSS_CHECK).read_bytes()
        if digest(data) != CROSS_CHECK_SHA256:
            raise ValueError('Frozen cross-check audit digest mismatch')
        document = strict_json(data)
    actual = generate_cross_check(root)
    if document != actual:
        raise ValueError('Cross-check audit metadata or digest parity mismatch')
    return dict(status='pass', source_indexes=2, unique_members=actual['unique_member_count'],
                overlap_count=0, digest_mismatch_count=0, exact_digest_parity=True)


def ast_test_method_count(data):
    tree = ast.parse(data)
    return sum(isinstance(method, (ast.FunctionDef, ast.AsyncFunctionDef))
               and method.name.startswith('test_')
               for node in ast.walk(tree) if isinstance(node, ast.ClassDef)
               for method in node.body)


def generate_script_inventory(root):
    root = Path(root)
    files = []
    for relative in SCRIPT_FILES:
        data = (root / relative).read_bytes()
        files.append(dict(path=relative, bytes=len(data), sha256=digest(data),
                          valid_python_ast=True, ast_test_method_count=ast_test_method_count(data)))
    return dict(task='G16-06-RSV2-08-p25', study_phase=24,
                scope='Exact final verifier and test source bytes; AST only, no source execution.',
                test_method_definition='Direct class FunctionDef/AsyncFunctionDef declarations whose names start with test_; standalone and nested helper functions excluded.',
                file_count=len(files), total_bytes=sum(row['bytes'] for row in files),
                total_ast_test_method_count=sum(row['ast_test_method_count'] for row in files),
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                files=files)


def verify_script_inventory(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / SCRIPT_INVENTORY).read_bytes())
    actual = generate_script_inventory(root)
    if document != actual:
        raise ValueError('Script inventory size, SHA-256, AST method count or metadata mismatch')
    return dict(status='pass', files=actual['file_count'], bytes=actual['total_bytes'],
                ast_test_method_count=actual['total_ast_test_method_count'])


def generate_script_cross_check(root):
    root = Path(root)
    verify_cross_check(root)
    verify_script_inventory(root)
    inputs = []
    documents = []
    for path in (CROSS_CHECK, SCRIPT_INVENTORY):
        data = (root / path).read_bytes()
        inputs.append(dict(path=path.as_posix(), bytes=len(data), sha256=digest(data)))
        documents.append(strict_json(data))
    phase23 = dict(files=[dict(path=row['path'], bytes=row['bytes'], sha256=row['expected_sha256'])
                          for row in documents[0]['members']])
    entries = disjoint_index_entries(phase23, documents[1], labels=('phase23', 'phase24'))
    members = []
    for label, row in entries:
        data = (root / row['path']).read_bytes(); actual = digest(data)
        if actual != row['sha256'] or len(data) != row['bytes']:
            raise ValueError('Script/inventory member digest or size mismatch: ' + row['path'])
        members.append(dict(path=row['path'], source_receipt=label, bytes=len(data),
                            expected_sha256=row['sha256'], actual_sha256=actual, digest_parity=True))
    return dict(task='G16-06-RSV2-08-p26', study_phase=25,
                scope='Phase23 member inventory and Phase24 current-script inventory; Phase24 metadata refreshed for authorized source updates.',
                source_receipts=inputs, phase23_member_count=len(phase23['files']),
                phase24_member_count=len(documents[1]['files']), unique_member_count=len(members),
                overlap_paths=[], overlap_count=0, exact_digest_parity=True, digest_mismatch_count=0,
                total_member_bytes=sum(row['bytes'] for row in members),
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                members=members)


def verify_script_cross_check(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / SCRIPT_CROSS_CHECK).read_bytes())
    actual = generate_script_cross_check(root)
    if document != actual:
        raise ValueError('Phase23/24 cross-check metadata or digest parity mismatch')
    return dict(status='pass', source_receipts=2, unique_members=actual['unique_member_count'],
                overlap_count=0, digest_mismatch_count=0, exact_digest_parity=True)


def summary_paths():
    receipts = (INVENTORY, MARKDOWN_INDEX, CROSS_CHECK, SCRIPT_INVENTORY, SCRIPT_CROSS_CHECK)
    return [(phase, kind, path) for phase, receipt in zip(range(21,26), receipts)
            for kind, path in [('markdown', INVENTORY.parent / f'PHASE{phase}.md'), ('json', receipt)]]


def generate_summary_manifest(root):
    root = Path(root)
    verify_script_cross_check(root)
    files = []
    for phase, kind, path in summary_paths():
        data = (root / path).read_bytes()
        if kind == 'json':
            strict_json(data)
        files.append(dict(phase=phase, kind=kind, path=path.as_posix(), **markdown_record(data)))
    return dict(task='G16-06-RSV2-08-p27', study_phase=26,
                scope='Exact current Phase21..25 report and JSON receipt bytes; Phase24/25 source-dependent metadata refreshed.',
                phase_count=5, file_count=len(files), total_bytes=sum(row['bytes'] for row in files),
                total_lines=sum(row['line_count'] for row in files), literal_host_path_count=0,
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                files=files)


def verify_summary_manifest(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / SUMMARY_MANIFEST).read_bytes())
    if document != generate_summary_manifest(root):
        raise ValueError('Phase21..25 summary coverage, size, digest, lines or metadata mismatch')
    return dict(status='pass', phases=5, files=10, literal_host_path_count=0)


def manifest_parity_record(root, row, summary_records, require_summary=False):
    relative = PurePosixPath(row['path'])
    if relative.is_absolute() or '..' in relative.parts or '\\' in row['path'] or relative.as_posix() != row['path']:
        raise ValueError('Nonportable manifest reference path')
    data = (Path(root) / row['path']).read_bytes()
    actual = digest(data)
    if row['sha256'] != actual or row['bytes'] != len(data):
        raise ValueError('Live manifest/reference SHA-256 or byte count mismatch')
    indexed = summary_records.get(row['path'])
    if require_summary and indexed is None:
        raise ValueError('Receipt reference missing from summary')
    if indexed is not None and (indexed['sha256'] != actual or indexed['bytes'] != len(data)):
        raise ValueError('Manifest reference SHA-256 or byte count mismatch')
    return dict(path=row['path'], expected_sha256=row['sha256'], actual_sha256=actual,
                expected_bytes=row['bytes'], actual_bytes=len(data), digest_parity=True, byte_parity=True,
                in_summary=indexed is not None, summary_reference_parity=True if indexed is not None else None)


def generate_manifest_cross_check(root):
    root = Path(root)
    verify_summary_manifest(root)
    raw = (root / SUMMARY_MANIFEST).read_bytes(); summary = strict_json(raw)
    indexed = {row['path']:row for row in summary['files']}
    files = [dict(phase=row['phase'], kind=row['kind'], **manifest_parity_record(root,row,indexed,True))
             for row in summary['files']]
    phase23 = strict_json((root / CROSS_CHECK).read_bytes())
    phase24 = strict_json((root / SCRIPT_INVENTORY).read_bytes())
    phase25 = strict_json((root / SCRIPT_CROSS_CHECK).read_bytes())
    references = []
    for owner, rows, sha_key in [(CROSS_CHECK, phase23['source_indexes'], 'expected_sha256'),
                                 (SCRIPT_CROSS_CHECK, phase25['source_receipts'], 'sha256')]:
        for row in rows:
            normalized = dict(path=row['path'], bytes=row['bytes'], sha256=row[sha_key])
            references.append(dict(owner_receipt=owner.as_posix(), **manifest_parity_record(root,normalized,indexed,True)))
    scripts = [manifest_parity_record(root,row,indexed) for row in phase24['files']]
    return dict(task='G16-06-RSV2-08-p28', study_phase=27,
                scope='Phase26 summary/live byte parity and Phase23/24/25 receipt relationships; current source-dependent metadata.',
                summary_manifest=dict(path=SUMMARY_MANIFEST.as_posix(), bytes=len(raw), sha256=digest(raw)),
                summary_file_count=len(files), checked_json_receipt_phases=[23,24,25],
                direct_receipt_reference_count=len(references), phase24_script_member_count=len(scripts),
                digest_mismatch_count=0, byte_mismatch_count=0, exact_digest_and_byte_parity=True,
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                file_checks=files, receipt_reference_checks=references, phase24_script_checks=scripts)


def verify_manifest_cross_check(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / MANIFEST_CROSS_CHECK).read_bytes())
    if document != generate_manifest_cross_check(root):
        raise ValueError('Manifest cross-check coverage, digest/byte parity or metadata mismatch')
    return dict(status='pass', summary_files=10, receipt_references=4, phase24_script_members=2,
                digest_mismatch_count=0, byte_mismatch_count=0)


def cumulative_receipt_paths():
    older = ('phase15_gate_s_compliant_joint_audit.json', 'phase16_cross_phase_provenance_index.json',
             'phase17_legacy_path_inventory.json', 'phase18_complete_study_manifest.json',
             'phase19_solver_environment_audit.json', 'phase20_candidate_csv_schema_audit.json')
    paths = tuple(Path('experiments/G16-06-RSV2-08') / name for name in older)
    paths += (INVENTORY, MARKDOWN_INDEX, CROSS_CHECK, SCRIPT_INVENTORY, SCRIPT_CROSS_CHECK,
              SUMMARY_MANIFEST, MANIFEST_CROSS_CHECK)
    return list(zip(range(15,28), paths))


def cumulative_receipt_record(data, phase, path):
    document = strict_json(data)
    flag = document['all_gate_qualification'] if phase == 15 else document['full_qualification']
    if flag is not False or (phase > 15 and document['qualified_count'] != 0):
        raise ValueError('Cumulative receipt qualification changed')
    return dict(phase=phase, path=path.as_posix(), bytes=len(data), sha256=digest(data),
                valid_json=True, qualified_count=0, full_qualification=False)


def generate_cumulative_receipt_index(root):
    root = Path(root)
    verify_manifest_cross_check(root)
    files = [cumulative_receipt_record((root / path).read_bytes(),phase,path)
             for phase,path in cumulative_receipt_paths()]
    return dict(task='G16-06-RSV2-08-p29', study_phase=28,
                scope='Exact current Phase15..27 JSON receipts; source-dependent metadata refreshed through Phase27.',
                phase_count=13, file_count=len(files), total_bytes=sum(row['bytes'] for row in files),
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                files=files)


def verify_cumulative_receipt_index(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / CUMULATIVE_RECEIPT_INDEX).read_bytes())
    if document != generate_cumulative_receipt_index(root):
        raise ValueError('Cumulative receipt phase coverage, SHA-256, bytes or metadata mismatch')
    return dict(status='pass', phases=13, files=13, qualified_count=0, full_qualification=False)


def assertion_function_records(data):
    tree = ast.parse(data)
    records = []
    def owned_nodes(node):
        for child in ast.iter_child_nodes(node):
            if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef, ast.Lambda)):
                continue
            yield child
            yield from owned_nodes(child)
    def functions(node, prefix=''):
        for child in ast.iter_child_nodes(node):
            if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef)):
                qualified = prefix + child.name
                nodes = list(owned_nodes(child))
                raises = sum(isinstance(n, ast.Raise) for n in nodes)
                asserts = sum(isinstance(n, ast.Assert) for n in nodes)
                entrypoint = child.name == 'verify' or child.name.startswith('verify_')
                if raises or asserts or entrypoint:
                    records.append(dict(name=qualified, line=child.lineno, end_line=child.end_lineno,
                                        raise_statement_count=raises, assert_statement_count=asserts,
                                        if_statement_count=sum(isinstance(n,ast.If) for n in nodes),
                                        verification_entrypoint=entrypoint))
                functions(child, qualified + '.')
            elif isinstance(child, ast.ClassDef):
                functions(child, prefix + child.name + '.')
            else:
                functions(child, prefix)
    functions(tree)
    return records


def generate_assertion_inventory(root):
    root = Path(root)
    verify_cumulative_receipt_index(root)
    verifier = (root / SCRIPT_FILES[0]).read_bytes(); tests = (root / SCRIPT_FILES[1]).read_bytes()
    functions = assertion_function_records(verifier)
    classes = []
    for node in ast.parse(tests).body:
        if isinstance(node, ast.ClassDef):
            methods = [method.name for method in node.body
                       if isinstance(method,(ast.FunctionDef,ast.AsyncFunctionDef)) and method.name.startswith('test_')]
            if methods:
                classes.append(dict(name=node.name, test_method_count=len(methods), test_methods=methods))
    return dict(task='G16-06-RSV2-08-p30', study_phase=29,
                scope='AST-only verifier guard/verification functions and test declarations; no runtime coverage claim.',
                function_selection='Own-scope explicit Raise/Assert statements or verify/verify_* name; nested function/class/lambda bodies excluded from parent counts.',
                sources=[dict(path=path,bytes=len(data),sha256=digest(data)) for path,data in zip(SCRIPT_FILES,(verifier,tests))],
                check_function_count=len(functions), check_functions=functions,
                verification_entrypoint_count=sum(row['verification_entrypoint'] for row in functions),
                raise_statement_count=sum(row['raise_statement_count'] for row in functions),
                assert_statement_count=sum(row['assert_statement_count'] for row in functions),
                test_class_count=len(classes), test_method_count=ast_test_method_count(tests), test_classes=classes,
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False)


def verify_assertion_inventory(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / ASSERTION_INVENTORY).read_bytes())
    actual = generate_assertion_inventory(root)
    if document != actual:
        raise ValueError('Verifier assertion function inventory, source digest or test count mismatch')
    return dict(status='pass', check_functions=actual['check_function_count'], test_methods=actual['test_method_count'])


def json_schema_key_record(data, phase, path):
    document = strict_json(data)
    if not isinstance(document, dict):
        raise ValueError('Receipt must be a top-level JSON object')
    keys = sorted(document)
    return dict(phase=phase, path=path.as_posix(), bytes=len(data), sha256=digest(data),
                top_level_key_count=len(keys), top_level_keys=keys,
                keyset_sha256=digest(json.dumps(keys,separators=(',',':'),ensure_ascii=True).encode()))


def generate_json_schema_keyset_audit(root):
    root = Path(root)
    verify_assertion_inventory(root)
    paths = cumulative_receipt_paths() + [(28,CUMULATIVE_RECEIPT_INDEX),(29,ASSERTION_INVENTORY)]
    files = [json_schema_key_record((root / path).read_bytes(),phase,path) for phase,path in paths]
    groups = {}
    for row in files:
        key = row['keyset_sha256']
        groups.setdefault(key,dict(keyset_sha256=key,top_level_keys=row['top_level_keys'],phases=[]))['phases'].append(row['phase'])
    keysets = [set(row['top_level_keys']) for row in files]
    return dict(task='G16-06-RSV2-08-p31', study_phase=30,
                scope='Parsed top-level keysets of all Phase15..29 receipts; no nested-key/type/schema-equivalence claim.',
                phase_count=15, file_count=len(files), total_bytes=sum(row['bytes'] for row in files),
                distinct_keyset_count=len(groups), common_top_level_keys=sorted(set.intersection(*keysets)),
                union_top_level_keys=sorted(set.union(*keysets)), keyset_groups=list(groups.values()),
                engineering_qualification_assessed=False, qualified_count=0, full_qualification=False,
                files=files)


def verify_json_schema_keyset_audit(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / JSON_SCHEMA_KEYSET_AUDIT).read_bytes())
    actual = generate_json_schema_keyset_audit(root)
    if document != actual:
        raise ValueError('JSON receipt keyset coverage, keys, source digest or metadata mismatch')
    return dict(status='pass', receipts=15, distinct_keysets=actual['distinct_keyset_count'])


def markdown_heading_paths():
    return [(phase, (Path('experiments/G16-06-RSV2-08') if phase < 21 else INVENTORY.parent) / f'PHASE{phase}.md')
            for phase in range(15,31)]


def markdown_heading_record(data, phase, path):
    text = data.decode('utf-8')
    headings = []; fence = None
    for number,line in enumerate(text.splitlines(),1):
        marker = re.match(r'^ {0,3}(`{3,}|~{3,})(.*)$',line)
        if fence is not None:
            if marker and marker[1][0] == fence[0] and len(marker[1]) >= len(fence) and not marker[2].strip():
                fence = None
            continue
        if marker and (marker[1][0] != '`' or '`' not in marker[2]):
            fence = marker[1]; continue
        heading = re.match(r'^ {0,3}(#{1,6})(?:[ \t]+(.*)|$)',line)
        if heading:
            title = re.sub(r'(?:^[ \t]*|[ \t]+)#+[ \t]*$','',heading[2] or '').strip()
            headings.append(dict(line=number,level=len(heading[1]),title=title))
    violations = []
    h1 = [row for row in headings if row['level'] == 1]
    if len(h1) != 1: violations.append('Expected exactly one H1')
    if not headings or headings[0]['level'] != 1: violations.append('First heading must be H1')
    if not h1 or not h1[0]['title'].startswith(f'G16-06-RSV2-08 Phase {phase} — '):
        violations.append('Phase title mismatch')
    if any(not row['title'] for row in headings): violations.append('Empty heading')
    if any(b['level'] > a['level']+1 for a,b in zip(headings,headings[1:])):
        violations.append('Skipped heading level')
    normalized = [' '.join(row['title'].casefold().split()) for row in headings]
    if len(set(normalized)) != len(normalized): violations.append('Duplicate heading text')
    if fence is not None: violations.append('Unclosed code fence')
    return dict(phase=phase,path=path.as_posix(),bytes=len(data),sha256=digest(data),line_count=len(text.splitlines()),
                heading_count=len(headings),h1_count=len(h1),heading_levels=[r['level'] for r in headings],
                headings=headings,violation_count=len(violations),violations=violations)


def generate_markdown_heading_audit(root):
    root = Path(root)
    verify_json_schema_keyset_audit(root)
    files = [markdown_heading_record((root / path).read_bytes(),phase,path) for phase,path in markdown_heading_paths()]
    if any(row['violation_count'] for row in files):
        raise ValueError('Markdown heading structure is inconsistent')
    return dict(task='G16-06-RSV2-08-p32',study_phase=31,
                scope='ATX headings outside fenced code in PHASE15..30; no full CommonMark or rendered-anchor claim.',
                phase_count=16,file_count=len(files),total_bytes=sum(r['bytes'] for r in files),
                total_lines=sum(r['line_count'] for r in files),total_headings=sum(r['heading_count'] for r in files),
                violation_count=0,heading_structure_consistent=True,
                engineering_qualification_assessed=False,qualified_count=0,full_qualification=False,files=files)


def verify_markdown_heading_audit(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / MARKDOWN_HEADING_AUDIT).read_bytes())
    actual = generate_markdown_heading_audit(root)
    if document != actual:
        raise ValueError('Markdown heading coverage, structure, source digest or metadata mismatch')
    return dict(status='pass',reports=16,headings=actual['total_headings'],violation_count=0)


def hex64_receipt_paths():
    return cumulative_receipt_paths() + [(28,CUMULATIVE_RECEIPT_INDEX),(29,ASSERTION_INVENTORY),
                                         (30,JSON_SCHEMA_KEYSET_AUDIT),(31,MARKDOWN_HEADING_AUDIT)]


def hex64_digest_fields(document):
    fields = []
    def visit(value, pointer='', selected=False):
        if isinstance(value,(dict,list)):
            if selected and not value:
                raise ValueError('Empty SHA256 container at ' + pointer)
            items = sorted(value.items()) if isinstance(value,dict) else enumerate(value)
            for key,child in items:
                token = str(key).replace('~','~0').replace('/','~1')
                visit(child,pointer+'/'+token,selected or isinstance(key,str) and 'sha256' in key.casefold())
        elif selected:
            if not isinstance(value,str) or re.fullmatch(r'[0-9a-f]{64}',value) is None:
                raise ValueError('Invalid lowercase hex64 SHA256 at ' + pointer)
            fields.append(dict(json_pointer=pointer,value=value))
    visit(document)
    return fields


def hex64_receipt_record(data, phase, path):
    document = strict_json(data)
    if not isinstance(document,dict):
        raise ValueError('SHA256 audit receipt must be a top-level JSON object')
    fields = hex64_digest_fields(document)
    return dict(phase=phase,path=path.as_posix(),bytes=len(data),sha256=digest(data),
                sha256_field_count=len(fields),invalid_field_count=0,fields=fields)


def generate_hex64_digest_audit(root):
    root = Path(root)
    verify_markdown_heading_audit(root)
    files = [hex64_receipt_record((root / path).read_bytes(),phase,path) for phase,path in hex64_receipt_paths()]
    return dict(task='G16-06-RSV2-08-p33',study_phase=32,
                scope='All leaves under keys containing sha256 case-insensitively in Phase15..31 JSON receipts; format only.',
                required_format='[0-9a-f]{64}',phase_count=17,file_count=len(files),
                total_bytes=sum(r['bytes'] for r in files),sha256_field_count=sum(r['sha256_field_count'] for r in files),
                invalid_field_count=0,all_sha256_fields_lowercase_hex64=True,
                engineering_qualification_assessed=False,qualified_count=0,full_qualification=False,files=files)


def verify_hex64_digest_audit(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = strict_json((root / HEX64_DIGEST_AUDIT).read_bytes())
    actual = generate_hex64_digest_audit(root)
    if document != actual:
        raise ValueError('SHA256 format inventory coverage, fields, source digest or metadata mismatch')
    return dict(status='pass',receipts=17,sha256_fields=actual['sha256_field_count'],invalid_fields=0)


def verify_all(root=ROOT):
    result = verify(root)
    result['markdown_reports'] = verify_markdown(root)
    result['inventory_cross_check'] = verify_cross_check(root)
    result['script_test_inventory'] = verify_script_inventory(root)
    result['script_and_inventory_cross_check'] = verify_script_cross_check(root)
    result['phase21_to_25_summary'] = verify_summary_manifest(root)
    result['manifest_cross_check'] = verify_manifest_cross_check(root)
    result['cumulative_receipt_index'] = verify_cumulative_receipt_index(root)
    result['verifier_assertion_inventory'] = verify_assertion_inventory(root)
    result['json_schema_keysets'] = verify_json_schema_keyset_audit(root)
    result['markdown_heading_consistency'] = verify_markdown_heading_audit(root)
    result['hex64_digest_format'] = verify_hex64_digest_audit(root)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true', help='Generate the frozen inventory using read-only Git discovery')
    parser.add_argument('--write-markdown', action='store_true', help='Generate the Markdown index at its pinned baseline')
    parser.add_argument('--write-cross-check', action='store_true', help='Generate the cross-check from the two unchanged frozen indexes')
    parser.add_argument('--write-script-inventory', action='store_true', help='Inventory final script bytes and AST test declarations')
    parser.add_argument('--write-script-cross-check', action='store_true', help='Cross-check the Phase23 and current Phase24 receipts')
    parser.add_argument('--write-summary', action='store_true', help='Summarize the exact Phase21..25 reports and receipts')
    parser.add_argument('--write-manifest-cross-check', action='store_true', help='Cross-check the Phase26 summary against live files and receipt references')
    parser.add_argument('--write-cumulative-index', action='store_true', help='Index the exact Phase15..27 receipt digests and byte sizes')
    parser.add_argument('--write-assertion-inventory', action='store_true', help='Inventory verifier guard functions and test declarations by AST')
    parser.add_argument('--write-json-keysets', action='store_true', help='Audit top-level JSON keys across Phase15..29 receipts')
    parser.add_argument('--write-markdown-headings', action='store_true', help='Audit Phase15..30 report heading structure')
    parser.add_argument('--write-hex64-digests', action='store_true', help='Audit SHA256 field format across Phase15..31 receipts')
    args = parser.parse_args()
    if args.write:
        import subprocess
        tracked = subprocess.run(['git', 'ls-tree', '-r', '--name-only', '-z', BASELINE], cwd=ROOT, check=True, capture_output=True).stdout.decode().split('\0')
        paths = sorted(path for path in tracked if path and matches(path))
        document = generate(ROOT, paths)
        (ROOT / INVENTORY).parent.mkdir(parents=True, exist_ok=True)
        (ROOT / INVENTORY).write_text(json.dumps(document, indent=2, allow_nan=False) + '\n')
    if args.write_markdown:
        import subprocess
        tracked = subprocess.run(['git', 'ls-tree', '-r', '--name-only', '-z', MARKDOWN_BASELINE,
                                  '--', INVENTORY.parent.as_posix()], cwd=ROOT, check=True, capture_output=True).stdout.decode().split('\0')
        paths = sorted(path for path in tracked if path and PurePosixPath(path).parent == PurePosixPath(INVENTORY.parent.as_posix())
                       and fnmatch.fnmatchcase(PurePosixPath(path).name, 'PHASE*.md'))
        (ROOT / MARKDOWN_INDEX).write_text(json.dumps(generate_markdown(ROOT, paths), indent=2, allow_nan=False) + '\n')
    if args.write_cross_check:
        (ROOT / CROSS_CHECK).write_text(json.dumps(generate_cross_check(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_script_inventory:
        (ROOT / SCRIPT_INVENTORY).write_text(json.dumps(generate_script_inventory(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_script_cross_check:
        (ROOT / SCRIPT_CROSS_CHECK).write_text(json.dumps(generate_script_cross_check(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_summary:
        (ROOT / SUMMARY_MANIFEST).write_text(json.dumps(generate_summary_manifest(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_manifest_cross_check:
        (ROOT / MANIFEST_CROSS_CHECK).write_text(json.dumps(generate_manifest_cross_check(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_cumulative_index:
        (ROOT / CUMULATIVE_RECEIPT_INDEX).write_text(json.dumps(generate_cumulative_receipt_index(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_assertion_inventory:
        (ROOT / ASSERTION_INVENTORY).write_text(json.dumps(generate_assertion_inventory(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_json_keysets:
        (ROOT / JSON_SCHEMA_KEYSET_AUDIT).write_text(json.dumps(generate_json_schema_keyset_audit(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_markdown_headings:
        (ROOT / MARKDOWN_HEADING_AUDIT).write_text(json.dumps(generate_markdown_heading_audit(ROOT), indent=2, allow_nan=False) + '\n')
    if args.write_hex64_digests:
        (ROOT / HEX64_DIGEST_AUDIT).write_text(json.dumps(generate_hex64_digest_audit(ROOT), indent=2, allow_nan=False) + '\n')
    print(json.dumps(verify_all(), sort_keys=True, allow_nan=False))


if __name__ == '__main__':
    main()
