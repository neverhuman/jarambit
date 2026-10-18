#!/usr/bin/env python3
"""Read-only verification of tracked JSON and Markdown snapshots; no numerical work."""
import argparse
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


def disjoint_index_entries(json_document, markdown_document):
    entries = []
    seen = set()
    for label, document in [('json', json_document), ('markdown', markdown_document)]:
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


def verify_all(root=ROOT):
    result = verify(root)
    result['markdown_reports'] = verify_markdown(root)
    result['inventory_cross_check'] = verify_cross_check(root)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true', help='Generate the frozen inventory using read-only Git discovery')
    parser.add_argument('--write-markdown', action='store_true', help='Generate the Markdown index at its pinned baseline')
    parser.add_argument('--write-cross-check', action='store_true', help='Generate the cross-check from the two unchanged frozen indexes')
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
    print(json.dumps(verify_all(), sort_keys=True, allow_nan=False))


if __name__ == '__main__':
    main()
