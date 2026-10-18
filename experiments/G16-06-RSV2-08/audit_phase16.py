"""Read-only cross-phase index verification; no Git, target or FEM required."""
import hashlib
import json
from pathlib import Path, PurePosixPath

HOST_PREFIX = ("/" + "home/" + "ubuntu").encode()
ROOT = Path(__file__).resolve().parents[2]
STUDY = Path('experiments/G16-06-RSV2-08')
INDEX = STUDY / 'phase16_cross_phase_provenance_index.json'


def paths(phase):
    directory = Path('experiments') / ('g16-06-rsv2-08' if 9 <= phase <= 14 else 'G16-06-RSV2-08')
    receipt = f'phase{phase}_receipt.json' if phase != 15 else 'phase15_gate_s_compliant_joint_audit.json'
    return directory / f'PHASE{phase}.md', directory / receipt


def file_record(root, relative):
    data = (root / relative).read_bytes()
    return dict(path=relative.as_posix(), sha256=hashlib.sha256(data).hexdigest(),
                host_absolute_path_occurrences=data.count(HOST_PREFIX))


def phase_record(root, phase):
    report, receipt = paths(phase)
    document = json.loads((root / receipt).read_text())
    results = document['results']
    count = document.get('completed_cases', document.get('candidate_count', len(results)))
    if type(count) is not int or count != len(results):
        raise ValueError('Candidate coverage mismatch')
    if document['all_gate_qualification'] is not False:
        raise ValueError('Historical full qualification changed')
    if any(row['all_gate_qualification'] is not False for row in results):
        raise ValueError('Historical candidate qualification changed')
    return dict(phase=phase, candidate_count=count, qualified_count=0,
                full_qualification=False, report=file_record(root, report),
                receipt=file_record(root, receipt))


def generate(root=ROOT):
    return dict(task='G16-06-RSV2-08-p16', schema_version=1,
                candidate_count_scope='Per-phase main results; reused candidates are not deduplicated; ancillary controls excluded.',
                qualified_count=0, full_qualification=False,
                phases=[phase_record(Path(root), phase) for phase in range(7, 16)])


def verify(root=ROOT, document=None, require_portable=False):
    root = Path(root)
    if document is None:
        document = json.loads((root / INDEX).read_text())
    # Check canonical paths before reading any supplied path.
    for entry in document['phases']:
        for kind in ('report', 'receipt'):
            value = entry[kind]['path']
            path = PurePosixPath(value)
            if path.is_absolute() or '..' in path.parts or '\\' in value:
                raise ValueError('Nonportable indexed path')
    if document != generate(root):
        raise ValueError('Index differs from historical files, counts or qualification')
    failures = [entry[kind]['path'] for entry in document['phases']
                for kind in ('report', 'receipt')
                if entry[kind]['host_absolute_path_occurrences']]
    if require_portable and failures:
        raise ValueError('Historical host-absolute paths remain: ' + ', '.join(failures))
    return dict(integrity='pass', indexed_files=18, phases=9,
                portable=not failures, nonportable_files=failures,
                qualified_count=0, full_qualification=False)


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true', help='Generate the index; never modify historical files')
    parser.add_argument('--require-portable', action='store_true')
    args = parser.parse_args()
    if args.write:
        (ROOT / INDEX).write_text(json.dumps(generate(), indent=2) + '\n')
    print(json.dumps(verify(require_portable=args.require_portable), indent=2))
