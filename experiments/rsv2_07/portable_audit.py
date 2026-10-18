"""File integrity and Git snapshot checks shared by the read-only audit commands."""

import hashlib
import json
from pathlib import PurePosixPath
import re
import subprocess

STUDY = "experiments/rsv2_07/"
RAW = "target/rsv2-07/"
CONTROL_RECEIPTS = {
    STUDY + "results/phase7_saved_triplet_gci_audit.json",
    STUDY + "results/phase8_synthetic_gci_audit.json",
    STUDY + "phase23_complete_receipt_manifest_audit.json",
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args], stderr=subprocess.PIPE)


def study_manifest(root):
    paths = set(git(root, "ls-files", "-z", "--", STUDY).decode().split("\0")) - {""}
    paths.update(STUDY + name for name in ("portable_audit.py", "test_portable_audits.py"))
    return {path: digest(root / path) for path in sorted(paths - CONTROL_RECEIPTS)}


def historical_blob(root, name, expected, snapshot):
    revisions = [snapshot] if snapshot else []
    revisions += git(root, "log", "--all", "--format=%H", "--", name).decode().splitlines()
    for revision in dict.fromkeys(revisions):
        try:
            blob = git(root, "cat-file", "blob", f"{revision}:{name}")
        except subprocess.CalledProcessError:
            continue
        if hashlib.sha256(blob).hexdigest() == expected:
            return revision
    raise ValueError(f"Snapshot SHA-256 not found in Git history: {name}")


def check_file(root, name, expected, size=None, snapshot=None):
    path = PurePosixPath(name)
    require(not path.is_absolute() and ".." not in path.parts and str(path) == name,
            f"Invalid manifest path: {name}")
    require(isinstance(expected, str) and re.fullmatch(r"[0-9a-f]{64}", expected) is not None,
            f"Invalid SHA-256: {name}")
    raw = name.startswith(RAW)
    if raw:
        require(type(size) is int and size >= 0, f"Invalid or missing byte size: {name}")
    disk = root / name
    try:
        stat = disk.stat()
    except FileNotFoundError:
        if raw:
            return {"path": name, "mode": "manifest_only", "sha256": expected, "size_bytes": size}
        require(not name.startswith(STUDY), f"Missing tracked study file: {name}")
        stat = None
    if stat is not None:
        actual = digest(disk)
        if actual == expected:
            if raw:
                require(stat.st_size == size, f"Byte size mismatch: {name}")
            return {"path": name, "mode": "on_disk", "sha256": actual}
        require(not raw and not name.startswith(STUDY), f"SHA-256 mismatch: {name}")
    revision = historical_blob(root, name, expected, snapshot)
    return {"path": name, "mode": "historical_snapshot", "sha256": expected, "commit": revision}


def verify_manifest(root, hashes, sizes, study_hashes, snapshot):
    require(isinstance(hashes, dict) and bool(hashes), "Empty or invalid file manifest")
    require(isinstance(study_hashes, dict) and bool(study_hashes), "Missing study manifest")
    target_names = {name for name in hashes if name.startswith(RAW)}
    require(set(sizes) == target_names, "Raw artifact size manifest must cover exactly the raw hash entries")
    tracked = set(git(root, "ls-files", "-z", "--", STUDY).decode().split("\0")) - {""}
    require(tracked - CONTROL_RECEIPTS <= study_hashes.keys(), "Tracked study file missing from hash manifest")
    combined = dict(hashes)
    for name, expected in study_hashes.items():
        require(name.startswith(STUDY) and name not in CONTROL_RECEIPTS,
                f"Invalid study manifest member: {name}")
        require(name not in combined or combined[name] == expected, f"Conflicting manifests: {name}")
        combined[name] = expected
    # Phase-7/8 omit each other and Phase-23, which binds their final bytes.
    # Each controlling receipt is checked by its own verifier without self hashing.
    for name in sorted(CONTROL_RECEIPTS):
        control = json.loads((root / name).read_text())
        require(isinstance(control, dict) and isinstance(control.get("schema"), str),
                f"Invalid control receipt: {name}")
    return [check_file(root, name, expected, sizes.get(name), snapshot)
            for name, expected in sorted(combined.items())]


def summary(records):
    raw = [entry for entry in records if entry["path"].startswith(RAW)]
    return (f"{len(records)} files; target/: "
            f"{sum(x['mode'] == 'on_disk' for x in raw)} on-disk, "
            f"{sum(x['mode'] == 'manifest_only' for x in raw)} manifest-only; "
            f"{sum(x['mode'] == 'historical_snapshot' for x in records)} historical snapshots")
