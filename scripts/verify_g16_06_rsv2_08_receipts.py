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


def verify_all(root=ROOT):
    result = verify(root)
    result['markdown_reports'] = verify_markdown(root)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true', help='Generate the frozen inventory using read-only Git discovery')
    parser.add_argument('--write-markdown', action='store_true', help='Generate the Markdown index at its pinned baseline')
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
    print(json.dumps(verify_all(), sort_keys=True, allow_nan=False))


if __name__ == '__main__':
    main()
