#!/usr/bin/env python3
"""Launch the claim checker with an independently approved operator context.

Approval bytes/hash/registration enter through the trusted application's API,
never a command-line, environment or document-selected authority. The standalone
CLI remains a typed refusal until that application supplies a real context.

A checkout admission is a separate, narrower gate. It compares a recorded
commit id with the commit object and the blobs the job actually checked out.
A match runs the public-document scope check only. It does not build a
TrustedCaller, select a candidate, or turn a missing qualification into a pass.
"""
import json
import re
import subprocess
from pathlib import Path
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from assurance.reporting import claims
from assurance.testing.contracts import ContractError
from ops.ci import check_claims, rs_verdicts


def context_from_operator_approval(raw, *, approved_sha256, approved_registration,
                                   request_path, authority_path):
    """Expected approvals belong to the operator, independently of submitted inputs."""
    caller = claims.TrustedCaller.from_approved_bytes(raw, approved_sha256, approved_registration)
    caller.verify_current()
    return rs_verdicts.ReportingContext(caller, request_path, authority_path)


_SHA = re.compile(r'^[0-9a-f]{40}$')
_RS = re.compile(r'RS-karambit-v\d+')
_FENCE = re.compile(r'^```')
_RULE = re.compile(r'^\|[\s:|-]+\|$')
# Blobs the public-document check reads. A recorded sha is not authority
# until each of these worktree files is the blob in that commit.
_CHECKOUT_BLOBS = (
    'README.md',
    'docs/PROVISIONAL.md',
    'docs/WHITEPAPER.md',
    'crates/cli/src/main.rs',
    'docs/wave27_diversity/plates/wave27_D1_index.json',
    'requirements/RS-karambit-v1.toml',
)


def _git(root, *args):
    try:
        return subprocess.check_output(
            ['git', '-C', str(root), *args], text=True, stderr=subprocess.DEVNULL).strip()
    except (subprocess.CalledProcessError, OSError):
        return None


def admit_checkout(recorded_sha, root=ROOT):
    """Return the commit id when the recorded sha is this checkout.

    ``recorded_sha`` is data. This function does not read the environment,
    a document, or a trusted-string. A missing, malformed, mismatched, or
    dirty blob is a refusal (``None``).
    """
    if type(recorded_sha) is not str or _SHA.fullmatch(recorded_sha) is None:
        return None
    root = Path(root)
    head = _git(root, 'rev-parse', 'HEAD')
    kind = _git(root, 'cat-file', '-t', recorded_sha)
    if head != recorded_sha or kind != 'commit':
        return None
    for rel in _CHECKOUT_BLOBS:
        committed = _git(root, 'rev-parse', f'{recorded_sha}:{rel}')
        path = root / rel
        if committed is None or not path.is_file():
            return None
        if _git(root, 'hash-object', str(path)) != committed:
            return None
    return recorded_sha


def _strip_noise(line):
    line = re.sub(r'`[^`]*`', '', line)
    line = re.sub(r'\]\([^)]*\)', ']', line)
    line = re.sub(r'\S+\.(png|svg|json|csv|md|stl|step)\b', '', line)
    return line


def _read(root, rel):
    path = (Path(root) / rel).resolve()
    if not path.is_relative_to(Path(root).resolve()):
        raise claims.Refusal('document_path_escape')
    return path.read_text(encoding='utf-8')


def _scan_pass(rel, text, stop=None):
    """Unscoped PASS is a violation. A table inherits the header row's RS name."""
    if stop and stop in text:
        text = text[:text.index(stop)]
    errors, header_scoped, in_code, prev_table = [], False, False, False
    for number, raw in enumerate(text.splitlines(), 1):
        if _FENCE.match(raw.strip()):
            in_code = not in_code
            continue
        if in_code:
            continue
        stripped = raw.strip()
        if stripped.startswith('|') and _RULE.match(stripped):
            prev_table = True
            continue
        if stripped.startswith('|') and not prev_table:
            header_scoped = _RS.search(stripped) is not None
        prev_table = stripped.startswith('|')
        if not prev_table:
            header_scoped = False
        line = _strip_noise(raw)
        scoped = _RS.search(raw) is not None or (stripped.startswith('|') and header_scoped)
        if re.search(r'\bPASS\b', line) and not scoped:
            errors.append(f'{rel}:{number}: unscoped PASS')
    return errors


def _verdict_block(root, rel):
    text = _read(root, rel)
    match = re.search(r'<!-- RS:BEGIN (\S+) (\S+) -->\s*\n(.*?)\n\s*<!-- RS:END -->', text, re.S)
    if match is None:
        return [f'{rel}: missing RS verdict block']
    index_rel, label, body = match.group(1), match.group(2), match.group(3).strip()
    index_path = (Path(root) / index_rel).resolve()
    if not index_path.is_relative_to(Path(root).resolve()) or not index_path.is_file():
        return [f'{rel}: RS index is not in this checkout']
    rows = json.loads(index_path.read_text(encoding='utf-8'))
    record = next((row for row in rows if row.get('label') == label), None)
    if record is None:
        return [f'{rel}: label {label} not in {index_rel}']
    requirement = Path(root) / 'requirements' / 'RS-karambit-v1.toml'
    with requirement.open('rb') as handle:
        rs = tomllib.load(handle)
    want = rs_verdicts.verdict(record, rs)
    if want not in body:
        return [f'{rel}: RS verdict block is stale']
    return []


def _evaluator_errors(root):
    main = _read(root, 'crates/cli/src/main.rs')
    readme = _read(root, 'README.md')
    opt_in = 'var("JARAMBIT_FEM").ok().as_deref() == Some("1")' in main
    errors = []
    if opt_in and re.search(r'plate level by default', readme):
        errors.append('README.md: plate level is opt-in (JARAMBIT_FEM=1)')
    if opt_in and 'JARAMBIT_FEM=1' not in readme:
        errors.append('README.md: must document the JARAMBIT_FEM=1 opt-in')
    return errors


def public_document_check(root=ROOT):
    """Scoped PASS and the published RS verdict line. Not a provider resolution."""
    errors = []
    try:
        for rel, stop in (('README.md', None), ('docs/PROVISIONAL.md', None), ('docs/WHITEPAPER.md', '## Appendix A')):
            errors.extend(_scan_pass(rel, _read(root, rel), stop))
        # v3.0.0 (owner, 2026-10-26: "the README is 100 % accurate with the best individual (no older data)"): the README
        # no longer reports the v1 provisional design, so its RS-karambit-v1 verdict block lives only in PROVISIONAL.md.
        # The README is still scanned for unscoped PASS claims (above) and the JARAMBIT_FEM opt-in (below).
        for rel in ('docs/PROVISIONAL.md',):
            errors.extend(_verdict_block(root, rel))
        errors.extend(_evaluator_errors(root))
    except (OSError, UnicodeError, json.JSONDecodeError, KeyError, claims.Refusal) as error:
        errors.append(str(error))
    return errors


def checkout_main(recorded_sha, root=ROOT):
    """Admit only a matching checkout, then check public documents.

    Exit 2 is a refusal. Exit 1 is a document violation. Exit 0 means the
    recorded commit is this checkout and the scoped verdict text matches.
    It is not a qualification and it does not install a caller.
    """
    if admit_checkout(recorded_sha, root) is None:
        print('ERROR: checkout_sha_refused')
        print('check_claims: 1 violation(s)')
        return 2
    errors = public_document_check(root)
    for error in errors:
        print('ERROR:', error)
    print(f'check_claims: {len(errors)} violation(s)')
    return 1 if errors else 0


def main(argv=None, *, context=None):
    try:
        claims.require(context is not None, 'independent_caller_admission_unavailable')
        claims.require(type(context) is rs_verdicts.ReportingContext,
                       'independent_caller_admission_required')
        # A previous result must not survive an altered caller/source/binary.
        context.last_resolution = None
        context.last_report = None
        context.caller.verify_current()
    except (claims.Refusal, ContractError, OSError, UnicodeError, TypeError, KeyError, ValueError) as error:
        print('ERROR:', error)
        print('check_claims: 1 violation(s)')
        return 2
    return check_claims.main(argv, context=context)


if __name__ == '__main__':
    raise SystemExit(main())
