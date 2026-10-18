#!/usr/bin/env python3
"""Verify a sealed Phase-4 checkpoint without building or changing the repository."""
from pathlib import Path
import hashlib,json

ROOT=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()

def verify():
    package=(ROOT/'SHA256SUMS').read_text().splitlines()
    for row in package:
        expected,name=row.split('  ',1)
        assert sha(ROOT/name)==expected,name
    meta=json.loads((ROOT/'ARCHIVE_PROVENANCE.json').read_text())
    original=ROOT/'unified_study/ORIGINAL_SHA256SUMS'
    assert sha(original)==meta['original_seal_sha256']
    archived=0
    for row in original.read_text().splitlines():
        expected,name=row.split('  ',1)
        if name in meta['omitted_generated_executables']:
            assert name=='study' and expected==meta['omitted_generated_executables'][name]['sha256']
            continue
        assert sha(original.parent/name)==expected,name
        archived+=1
    assert archived==meta['archived_original_artifact_count']==94
    assert sha(ROOT/'UNIFIED_PROTOTYPE_REPORT.md')==meta['report_sha256']
    print(f'PASS: {len(package)} package digests; {archived}/95 original artifacts preserved; generated binary explicitly omitted.')

if __name__=='__main__':
    verify()
