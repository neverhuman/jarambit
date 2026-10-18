"""Inventory immutable legacy path strings and their portable evidence bindings."""
import hashlib
import json
from pathlib import Path, PurePosixPath

import audit_phase16 as phase16
import portable_study_audit as portable

ROOT = Path(__file__).resolve().parents[2]
HERE = Path('experiments/G16-06-RSV2-08')
INVENTORY = HERE / 'phase17_legacy_path_inventory.json'
HOST = '/home/' + 'ubuntu'


def strings(value, keypath=()):
    """Visit every JSON string, including object keys, with typed keypaths."""
    if isinstance(value, dict):
        for key, child in value.items():
            if HOST in key:
                yield keypath + (key,), key, 'key'
            yield from strings(child, keypath + (key,))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            yield from strings(child, keypath + (index,))
    elif isinstance(value, str) and HOST in value:
        yield keypath, value, 'value'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def legacy_entries(root, receipt):
    document = json.loads((root / receipt).read_text())
    entries = []
    for keypath, value, kind in strings(document):
        entry = dict(keypath=list(keypath), string_kind=kind,
                     original_string_sha256=hashlib.sha256(value.encode()).hexdigest())
        if keypath[-2:] == ('gci', 'genome'):
            row = document[keypath[0]][keypath[1]]
            relative = row['genome']
            parts = PurePosixPath(value).parts
            marker = ('jarambit', '.agent', 'worktrees', 'lane-G16-06-RSV2-08')
            if tuple(parts[3:7]) != marker or '/'.join(parts[7:]) != relative:
                raise ValueError('Legacy genome does not match portable binding')
            portable.safe_path(root, relative)
            optional = relative.startswith('target/')
            resolved = portable.raw_path(root, relative) if optional else portable.safe_path(root, relative)
            if not optional and not resolved.is_file():
                raise ValueError('Missing tracked genome')
            if resolved is not None and sha(resolved) != row['genome_sha256']:
                raise ValueError('Mapped genome digest mismatch')
            entry.update(classification='optional_raw_genome' if optional else 'tracked_genome',
                         canonical_repository_relative_path=relative,
                         portable_binding_keypath=list(keypath[:-2]) + ['genome'],
                         expected_sha256=row['genome_sha256'],
                         resolution='optional raw evidence' if optional else 'required tracked evidence')
        elif keypath in [('delivery', 'external_archive_destination'), ('requested_delivery', 'external_path')]:
            entry.update(classification='external_delivery_destination',
                         canonical_repository_relative_path=None,
                         resolution='Historical external directory; not repository evidence or a verifier input')
        elif keypath == ('requested_delivery', 'commit'):
            entry.update(classification='historical_git_error',
                         canonical_repository_relative_path=None,
                         resolution='Diagnostic text; Git administrative lock is not scientific evidence or a verifier input')
        else:
            raise ValueError('Unclassified legacy host string')
        entries.append(entry)
    return entries


def generate(root=ROOT):
    root = Path(root)
    phases = []
    for phase in range(7, 17):
        receipt = phase16.paths(phase)[1] if phase < 16 else phase16.INDEX
        rows = list(strings(json.loads((root / receipt).read_text())))
        record = dict(phase=phase, receipt=receipt.as_posix(), sha256=sha(root / receipt),
                      legacy_host_string_count=len(rows), qualified_count=0, full_qualification=False)
        if phase in (7, 8):
            record['entries'] = legacy_entries(root, receipt)
        elif rows:
            raise ValueError('New host path in portable receipt')
        phases.append(record)
    historical = [rel for n in range(7, 16) for rel in phase16.paths(n)]
    historical += [HERE / name for name in ['PHASE16.md', 'audit_phase16.py',
                   'test_phase16_provenance.py', 'phase16_cross_phase_provenance_index.json',
                   'test_candidate_receipt.py', 'audit_phase7.py', 'audit_phase8.py', 'portable_study_audit.py']]
    return dict(task='G16-06-RSV2-08-p17', qualified_count=0, full_qualification=False,
                filename_note='Assignment Phase7/8 aliases absent; actual immutable receipt filenames used.',
                scientific_reference_count=161, unmappable_non_evidence_strings=3,
                historical_files={p.as_posix(): sha(root / p) for p in historical}, phases=phases)


def verify(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = json.loads((root / INVENTORY).read_text())
    if document != generate(root):
        raise ValueError('Legacy inventory or historical digest mismatch')
    if [p['legacy_host_string_count'] for p in document['phases']] != [148, 16] + [0] * 8:
        raise ValueError('Legacy string coverage mismatch')
    return dict(status='pass', legacy_strings=164, scientific_references=161,
                unmappable_non_evidence_strings=3, qualified_count=0, full_qualification=False)


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        (ROOT / INVENTORY).write_text(json.dumps(generate(), indent=2) + '\n')
    print(json.dumps(verify(), indent=2))
