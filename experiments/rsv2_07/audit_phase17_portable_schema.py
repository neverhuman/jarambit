#!/usr/bin/env python3
"""Audit Phase-10 JSON metadata and Phase-11–16 pin registries; no evaluations."""

import argparse
import hashlib
import importlib
import json
from pathlib import Path, PurePosixPath
import re

from audit_phase6 import load_json
from portable_audit import require

HERE = "experiments/rsv2_07/"
MANIFEST = HERE + "results/phase10_portable_receipt_manifest.json"
RECEIPT = HERE + "phase17_portable_schema_audit.json"
REPORT = HERE + "PHASE17_PORTABLE_SCHEMA_AUDIT.md"
SCHEMA = "jarambit.rsv2_07.phase17_portable_schema.v1"
MODULES = {
    11: ("audit_phase11_wave_receipts", "EXPECTED"),
    12: ("audit_phase12_cross_wave", "PINS"),
    13: ("audit_phase13_screen_rounds", "PINS"),
    14: ("audit_phase14_w319_receipts", "PINS"),
    15: ("audit_phase15_gate_s_and_margins", "PINS"),
    16: ("audit_phase16_xbabe3_waves", "PINS"),
}


def portable_path(value):
    require(isinstance(value, str) and bool(value), "Invalid portable path")
    path = PurePosixPath(value)
    require(not path.is_absolute() and ".." not in path.parts and str(path) == value
            and value != "." and "\\" not in value and ":" not in value
            and not any(ord(char) < 32 for char in value), f"Invalid portable path: {value!r}")
    return value


def digest_metadata(sha, size):
    require(isinstance(sha, str) and re.fullmatch(r"[0-9a-f]{64}", sha), "Invalid SHA-256")
    require(size is None or type(size) is int and size >= 0, "Invalid byte size")


def manifest_entries(document):
    require(isinstance(document, dict) and set(document) == {"schema", "task", "files", "raw_artifacts", "digest_reference_counts"},
            "Invalid Phase-10 manifest fields")
    require(document["schema"] == "jarambit.rsv2_07.portable_receipt_manifest.v1"
            and document["task"] == "RSV2-07-p10", "Invalid Phase-10 manifest identity")
    for key in ("files", "raw_artifacts", "digest_reference_counts"):
        require(isinstance(document[key], dict) and bool(document[key]), f"Empty/invalid manifest map: {key}")
    require(all(type(n) is int and n >= 0 for n in document["digest_reference_counts"].values()), "Invalid digest reference count")
    for name in document["digest_reference_counts"]:
        portable_path(name)
    entries = [(name, sha, None) for name, sha in document["files"].items()]
    for name, record in document["raw_artifacts"].items():
        require(isinstance(record, dict) and set(record) == {"sha256", "size_bytes"}
                and type(record["size_bytes"]) is int and record["size_bytes"] >= 0, "Invalid raw artifact record")
        require(name.startswith("target/rsv2-07/"), "Invalid raw artifact prefix")
        entries.append((name, record["sha256"], record["size_bytes"]))
    return entries


def registry_sources(root):
    sources = {}
    for phase, (module, attribute) in MODULES.items():
        imported = importlib.import_module(module)
        require(Path(imported.__file__).read_bytes() == (root / HERE / (module + ".py")).read_bytes(),
                "Registry module source differs from the supplied checkout")
        sources[phase] = getattr(imported, attribute)
    return sources


def pin_entries(phase, registry):
    require(isinstance(registry, dict) and bool(registry), "Empty/invalid pin registry")
    waves = {"w318": registry} if phase == 11 else {"w319": registry} if phase == 14 else registry
    entries = []
    for wave, files in waves.items():
        require(isinstance(wave, str) and re.fullmatch(r"w[0-9]+(?:[ct]3)?", wave), "Invalid wave key")
        require(isinstance(files, dict) and bool(files), "Empty/invalid wave pin map")
        prefix = "out/"
        if phase == 12 and wave in ("w315", "w316", "w317"):
            prefix = "evidence/evolution_waves/2026-10-17/xbabe2/out/"
        elif phase == 16:
            prefix = "evidence/evolution_waves/2026-10-17/xbabe3/out/"
        for name, pin in files.items():
            portable_path(name)
            require(isinstance(pin, (list, tuple)) and len(pin) == 2 and type(pin[1]) is int and pin[1] >= 0,
                    "Invalid digest/size pin pair")
            entries.append((prefix + wave + "/" + name, pin[0], pin[1]))
    return entries


def merge_entries(groups):
    require(set(groups) == set(range(10, 17)), "Exactly phases 10 through 16 are required")
    registry = {}
    for phase, entries in groups.items():
        require(bool(entries), "Empty phase registry")
        seen = set()
        for name, sha, size in entries:
            portable_path(name)
            digest_metadata(sha, size)
            require(name not in seen, "Duplicate path within phase registry")
            seen.add(name)
            if name in registry:
                previous = registry[name]
                require(previous["sha256"] == sha, f"Conflicting SHA-256 references: {name}")
                require(size is None or previous["size_bytes"] is None or previous["size_bytes"] == size,
                        f"Conflicting byte sizes: {name}")
                if size is not None:
                    previous["size_bytes"] = size
                previous["phases"].append(phase)
            else:
                registry[name] = {"path": name, "sha256": sha, "size_bytes": size, "phases": [phase]}
    return [registry[name] for name in sorted(registry)]


def check_files(root, artifact_root, registry):
    observations = []
    for entry in registry:
        name = entry["path"]
        strict = name.startswith(HERE)
        candidates = [("checkout", root / name)]
        if not strict and artifact_root != root:
            candidates.append(("artifact_root", artifact_root / name))
        copies = []
        for label, path in candidates:
            try:
                data = path.read_bytes()
            except FileNotFoundError:
                continue
            require(hashlib.sha256(data).hexdigest() == entry["sha256"], f"SHA-256 mismatch: {name} ({label})")
            require(entry["size_bytes"] is None or len(data) == entry["size_bytes"], f"Byte size mismatch: {name}")
            copies.append(label)
        require(not strict or bool(copies), f"Missing study input: {name}")
        observations.append({"path": name, "mode": "on_disk" if copies else "manifest_only", "copies": copies})
    return observations


def audit(root, artifact_root=None, sources=None):
    root = Path(root)
    artifact_root = root if artifact_root is None else Path(artifact_root)
    sources = registry_sources(root) if sources is None else sources
    require(set(sources) == set(MODULES), "Exactly six verifier registry sources are required")
    manifest_bytes = (root / MANIFEST).read_bytes()
    groups = {10: manifest_entries(load_json(manifest_bytes))}
    groups.update({phase: pin_entries(phase, registry) for phase, registry in sources.items()})
    registry = merge_entries(groups)
    source_hashes = {MANIFEST: hashlib.sha256(manifest_bytes).hexdigest()}
    for module, _ in MODULES.values():
        name = HERE + module + ".py"
        source_hashes[name] = hashlib.sha256((root / name).read_bytes()).hexdigest()
    profiles = [{"phase": phase, "kind": "json_manifest" if phase == 10 else "python_pin_registry",
                 "source": MANIFEST if phase == 10 else HERE + MODULES[phase][0] + ".py",
                 "attribute": None if phase == 10 else MODULES[phase][1], "references": len(groups[phase])}
                for phase in range(10, 17)]
    observations = check_files(root, artifact_root, registry)
    return {"schema": SCHEMA, "task": "RSV2-07-p17", "scope": "schema_and_sha256_registry_only",
            "source_sha256": source_hashes, "phase_profiles": profiles, "registry": registry,
            "verification": {"unique_paths": len(registry), "references": sum(map(len, groups.values())),
                             "on_disk": sum(row["mode"] == "on_disk" for row in observations),
                             "manifest_only": sum(row["mode"] == "manifest_only" for row in observations),
                             "observations": observations}}


def check_receipt(saved, current):
    require(isinstance(saved, dict) and set(saved) == set(current), "Invalid Phase-17 receipt fields")
    for key in ("schema", "task", "scope", "source_sha256", "phase_profiles", "registry"):
        require(json.dumps(saved[key], sort_keys=True) == json.dumps(current[key], sort_keys=True),
                f"Phase-17 receipt mismatch: {key}")
    verification = saved["verification"]
    require(isinstance(verification, dict) and set(verification) == set(current["verification"]), "Invalid verification fields")
    for key in ("unique_paths", "references", "on_disk", "manifest_only"):
        require(type(verification[key]) is int and verification[key] >= 0, "Invalid verification count")
    require(verification["unique_paths"] == len(saved["registry"])
            and verification["references"] == sum(p["references"] for p in saved["phase_profiles"])
            and verification["on_disk"] + verification["manifest_only"] == verification["unique_paths"], "Inconsistent verification totals")
    rows = verification["observations"]
    require(isinstance(rows, list) and len(rows) == len(saved["registry"]), "Invalid observation list")
    for entry, row in zip(saved["registry"], rows):
        require(isinstance(row, dict) and set(row) == {"path", "mode", "copies"}
                and row["path"] == entry["path"] and isinstance(row["copies"], list)
                and all(copy in ("checkout", "artifact_root") for copy in row["copies"])
                and len(row["copies"]) == len(set(row["copies"]))
                and row["mode"] == ("on_disk" if row["copies"] else "manifest_only"), "Invalid verification observation")
    require(sum(bool(row["copies"]) for row in rows) == verification["on_disk"], "Observation count mismatch")


def report(document):
    counts = document["verification"]
    rows = [f"| {p['phase']} | {p['kind']} | {p['source']} | {p['references']} |" for p in document["phase_profiles"]]
    return "\n".join([
        "# Phase-17 portable schema and SHA-256 registry audit", "",
        "**PASS for schema and registry integrity.** The confirmed inputs are the existing Phase-10 JSON manifest and Phase-11–16 Python pin registries. No seven historical JSON receipts are invented or renamed.", "",
        "| Phase | Profile | Source | References |", "| --- | --- | --- | ---: |", *rows, "",
        f"The {counts['references']} references resolve to {counts['unique_paths']} unique repository-relative paths, with no digest or known-size conflicts. This run verified {counts['on_disk']} paths on disk and recorded {counts['manifest_only']} as manifest-only. Every available checkout/supplemental copy must match; an absent copy never overrides a mismatching present file.", "",
        "The JSON profile requires the existing manifest identity, map fields, nonnegative integer counts and raw-artifact digest/size records. Registry profiles require nonempty wave maps and digest/size pairs. SHA-256 values must be 64 lowercase hexadecimal characters. Paths must be canonical relative POSIX paths without traversal, drive prefixes, backslashes or control characters. Boolean sizes/counts, duplicate paths within one phase, conflicting hashes/sizes, duplicate JSON keys and nonfinite JSON numbers fail closed.", "",
        "Phase-12 w315–w317 registry paths retain their xbabe2 archive locations; Phase-16 paths retain the xbabe3 archive location. Other wave pins use their saved out paths. Source hashes bind the seven manifest/module files. Referenced study inputs require current checkout bytes; wave and target artifacts may be manifest-only when absent. Supplemental roots are supplied at execution and never persisted as absolute paths.", "",
        "The JSON receipt records the original availability observations. Read-only verification requires its source hashes, profiles and merged registry to match current inputs and validates the recorded observation schema/counts. It then independently checks current file availability and bytes; clean checkouts can have fewer available artifacts. Historical on-disk observations do not establish that absent bytes were freshly verified. Source hashes are integrity bindings, not authenticity signatures.", "",
        "Earlier phase scope limits remain in force. No simulations, native evaluations, geometry changes, physical gate checks, structural margin calculations or qualification are performed. Python modules are imported only to read their existing registry objects; their verifier entry points are not called.", "",
        "```sh", "python3 experiments/rsv2_07/audit_phase17_portable_schema.py",
        "python3 experiments/rsv2_07/audit_phase17_portable_schema.py --artifact-root saved_repository",
        "python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'", "```", "",
        "The command is read-only unless --write is supplied. --repo-root and --artifact-root support relocation without Git discovery. The only persistent paths are repository-relative. A write explicitly refreshes this report and the Phase-17 JSON receipt; it does not modify source receipts or artifacts.", "",
        "Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Phase-17 synthetic tests cover schema/type failures, path portability, conflicts, tampering, missing evidence, recorded receipt integrity and a relocated CLI without Git or original host paths. The full discovery suite and git diff --check are required before publication.", "",
    ])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--write", action="store_true", help="Write the Phase-17 JSON receipt and Markdown report")
    args = parser.parse_args()
    try:
        current = audit(args.repo_root, args.artifact_root)
        if args.write:
            (args.repo_root / RECEIPT).write_text(json.dumps(current, indent=2) + "\n")
            (args.repo_root / REPORT).write_text(report(current))
        else:
            check_receipt(load_json((args.repo_root / RECEIPT).read_bytes()), current)
    except (OSError, ValueError) as error:
        parser.exit(1, f"Phase-17 schema/registry audit: FAIL: {error}\n")
    counts = current["verification"]
    print(f"Phase-17 schema/registry: PASS; 7 phases; {counts['references']} references; {counts['unique_paths']} unique paths; "
          f"{counts['on_disk']} on-disk; {counts['manifest_only']} manifest-only. No evaluations performed.")


if __name__ == "__main__":
    main()
