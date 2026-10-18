#!/usr/bin/env python3
"""Verify saved w320 r02 attempt bytes and receipt schema sync, without evaluations."""

import argparse
import csv
import hashlib
import json
from pathlib import Path
import subprocess

from audit_phase6 import load_json
from audit_phase13_screen_rounds import audit_documents as audit_rounds
import audit_phase17_portable_schema as schema_audit
import audit_phase18_w320_progression as phase18
from portable_audit import require

HERE = "experiments/rsv2_07/"
RECEIPT = HERE + "results/phase19_w320_r02_provenance_audit.json"
REPORT = HERE + "PHASE19_W320_R02_PROVENANCE_AUDIT.md"
LABEL = "w320_r02_p14_c93"
IDENTITY = "80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572"
ATTEMPT = f"by-content/{IDENTITY}/attempt-1792330667555204194"
FILES = {
    "w320_r02_p14_c93_skfem_stdout.txt": ("d56c6b27efccda568ca784a10e41d9deeb37553c7af8939b667c3a9075ff235e", 1046),
    "d1_rescreen.csv": ("6debcbd0905004739b0ed104d01fb66b4ea36456faf2b736e89fac85ac5460d8", 886),
    "d1_rescreen.md": ("468397dc4357859e9bb0670f7bae2f3542fa95b4f164004276a68978ea44aab3", 2133),
    "plates.txt": ("40831797fd412f18cf3d7049452b8f155a3e55401c5f2b9bb63f905406a39f30", 20),
    "w320_r02_p14_c93.step": ("df15db4274fbc562fb5fd3f837a5d41a05f63ed359cd68c8080fe0b9b2aea8f7", 59704935),
    "w320_r02_p14_c93.stl": ("be25bd0130c83432d4db2738316fc8c180aeaa0034f4bbcbfad7fec05c182f79", 16397734),
    "w320_r02_p14_c93_D1.json": ("aed64b3c075a5421e7cc3b7e1df0ac09cf8ef5e730eac9bbdd7bebc3acb5f116", 5992649),
    "w320_r02_p14_c93_export_stdout.txt": ("b1f165232b9aaffb819145b977b8a58d17d18fffc613fb80897b3bf5bdaec0ce", 59),
    "w320_r02_p14_c93_genome.csv": ("1421b661bbdae629c3d5ca3d010150ee39c912c9468abd1adda582b6a6f53869", 2528),
    "w320_r02_p14_c93_grind_law.json": ("1c709a427bf642c2de7e26be8c5d9508d2bc58a8cf45fe970a7f758811747252", 111623),
    "w320_r02_p14_c93_plate_gci.json": ("b22951ca9ed5932a0c41c7d45be2fadbf8da8ab7b8c1d2b72d064f958410014a", 5688),
    "w320_r02_p14_c93_plate_gci_raw.json": ("83fcf87361f0a816e172794fa64be1c02d3635326fd108b0ed8d97a2f1b1a2ed", 1841),
    "w320_r02_p14_c93_runtime.json": ("2694431cf731a37e8d6dbf63638d271d193263205ce3eceafd1280903405548b", 558),
    "w320_r02_p14_c93_solid_mesh.npz": ("938ef9cb124b5f70a8dba5b26f046f173ba851f74be8901acde8bb8bac627d3a", 566987),
    "w320_r02_p14_c93_step_check.json": ("e5d66ffa7b079b8133f0de85e547c95b56aa5785155c327c9d5f76cb8a21c7db", 302166),
    "w320_r02_p14_c93_surface.json": ("7a51ff8fe5167d0e8c80a7eb9ef5d583a51ef61df2ad677d535699bc92dc048b", 3455168),
}
PINS = {**{name: pin for name, pin in phase18.PINS.items() if name in phase18.ROUND_NAMES},
        "leaderboard.json": phase18.LEADERBOARD_PIN,
        **{ATTEMPT + "/" + name: pin for name, pin in FILES.items()}}
WATCHED = ("round_02_births.json", "round_02_screen.json", "leaderboard.json", ATTEMPT + "/" + LABEL + "_master_scorecard.md")


def audit_payloads(payloads, pins=None):
    pins = PINS if pins is None else pins
    require(set(payloads) == set(pins) == set(PINS), "Exactly twenty-one pinned files are required")
    for name, data in payloads.items():
        schema_audit.portable_path(name)
        sha, size = pins[name]
        require(type(size) is int and size >= 0, "Invalid pinned byte size")
        schema_audit.digest_metadata(sha, size)
        require(isinstance(data, bytes) and len(data) == size and hashlib.sha256(data).hexdigest() == sha,
                f"SHA-256/size mismatch: {name}")
    rounds = {name: load_json(payloads[name]) for name in phase18.ROUND_NAMES}
    counts = audit_rounds("w320", rounds)["rounds"]
    births, screens = rounds["round_01_births.json"], rounds["round_01_screen.json"]
    require(len(births["genomes"]) >= 93 and births["genomes"][92]["certificate_identity"] == IDENTITY
            and births["genomes"][92]["family"] == "p14", "Target birth identity/position mismatch")
    target = [row for row in screens if row["label"] == LABEL]
    require(len(target) == 1 and target[0]["certificate_identity"] == IDENTITY and target[0]["family"] == "p14",
            "Target pending screen mismatch")
    leaderboard = phase18.check_leaderboard(payloads["leaderboard.json"], rounds)
    require(leaderboard["unverified"] == [{"label": LABEL, "certificate_identity": IDENTITY, "status": "unverified"}],
            "Target must remain the separate unverified attempt")
    formats, json_documents, csv_documents = {}, {}, {}
    for filename in FILES:
        name = ATTEMPT + "/" + filename
        data = payloads[name]
        metadata = {"format": "opaque_bytes"}
        if filename.endswith(".json"):
            document = load_json(data)
            require(isinstance(document, dict), "Expected JSON object in attempt")
            for key in ("certificate_identity", "genome_sha256"):
                if key in document:
                    schema_audit.digest_metadata(document[key], None)
            if "certificate_identity" in document:
                require(document["certificate_identity"] == IDENTITY, "JSON candidate identity mismatch")
            if "label" in document:
                expected_label = LABEL + "_D1" if filename == LABEL + "_D1.json" else LABEL
                require(document["label"] == expected_label, "JSON candidate label mismatch")
            json_documents[filename] = document
            metadata = {"format": "strict_json_object", "top_level_fields": len(document)}
        elif filename.endswith(".csv"):
            rows = list(csv.reader(data.decode("utf-8").splitlines(), strict=True))
            require(len(rows) == 2 and bool(rows[0]) and all(rows[0]) and len(set(rows[0])) == len(rows[0])
                    and len(rows[0]) == len(rows[1]), "CSV header/row shape mismatch")
            csv_documents[filename] = rows
            metadata = {"format": "csv_single_record", "header_columns": len(rows[0]), "data_rows": 1}
        formats["out/w320/" + name] = metadata
    for suffix in ("_D1.json", "_plate_gci.json", "_surface.json"):
        require(json_documents[LABEL + suffix].get("certificate_identity") == IDENTITY,
                "Missing required JSON candidate identity")
    for suffix in ("_D1.json", "_grind_law.json"):
        expected_label = LABEL + "_D1" if suffix == "_D1.json" else LABEL
        require(json_documents[LABEL + suffix].get("label") == expected_label, "Missing required JSON candidate label")
    genome = csv_documents[LABEL + "_genome.csv"]
    require(genome[0] == next(csv.reader([births["genome_header"]]))
            and genome[1] == next(csv.reader([births["genomes"][92]["row"]])), "Genome CSV/birth binding mismatch")
    return {"stage1": {"births": sum(row["births"] for row in counts.values()),
                       "pending_screens": sum(row["screens"] for row in counts.values()),
                       "target": {"label": LABEL, "certificate_identity": IDENTITY, "family": "p14",
                                  "saved_round": 1, "birth_position": 93, "qualification_status": target[0]["qualification_status"]}},
            "format_checks": formats, "genome_csv_matches_saved_birth": True, "leaderboard": leaderboard}


def sync_context(root, directory):
    phase18_bytes = (root / phase18.RECEIPT).read_bytes()
    saved = load_json(phase18_bytes)
    current = phase18.audit(root, directory)
    require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-18 receipt is not synchronized")
    phase17_bytes = (root / schema_audit.RECEIPT).read_bytes()
    require(hashlib.sha256(phase17_bytes).hexdigest() == saved["baseline_context"]["phase17_receipt"]["sha256"],
            "Phase-17/18 receipt binding mismatch")
    return {"phase17": {"path": schema_audit.RECEIPT, "sha256": hashlib.sha256(phase17_bytes).hexdigest()},
            "phase18": {"path": phase18.RECEIPT, "sha256": hashlib.sha256(phase18_bytes).hexdigest()},
            "status": "verified_schema_and_provenance_sync"}


def availability(directory):
    return {"out/w320/" + name: ("present_pinned" if name in PINS else "present_unpinned")
            if (directory / name).is_file() else "missing" for name in WATCHED}


def attempt_inventory(directory):
    return {str(path.relative_to(directory)) for path in directory.glob(ATTEMPT + "/*") if path.is_file()}


def audit(root, directory, pins=None):
    root, directory = Path(root), Path(directory)
    pins = PINS if pins is None else pins
    require(set(pins) == set(PINS), "Exactly twenty-one pinned files are required")
    inventory, observed = attempt_inventory(directory), availability(directory)
    require({ATTEMPT + "/" + name for name in FILES} <= inventory, "Missing pinned attempt files")
    payloads = {name: (directory / name).read_bytes() for name in pins}
    result = audit_payloads(payloads, pins)
    context = sync_context(root, directory)
    require(all((directory / name).read_bytes() == data for name, data in payloads.items()), "Pinned artifacts changed during audit")
    require(inventory == attempt_inventory(directory) and observed == availability(directory), "Input inventory changed during audit")
    for key in ("phase17", "phase18"):
        require(hashlib.sha256((root / context[key]["path"]).read_bytes()).hexdigest() == context[key]["sha256"],
                "Receipt context changed during audit")
    return {"schema": "jarambit.rsv2_07.phase19_w320_r02_provenance.v1", "task": "RSV2-07-p19",
            "status": "partial_unverified_attempt", "scope": "saved_artifact_integrity_format_and_identity_only",
            "wave": "w320", "attempt_directory": "out/w320/" + ATTEMPT,
            "artifacts": {"out/w320/" + name: {"sha256": sha, "size_bytes": size} for name, (sha, size) in sorted(pins.items())},
            **result, "observed_outputs": observed,
            "additional_unpinned_attempt_files": ["out/w320/" + name for name in sorted(inventory - set(pins))],
            "receipt_sync": context, "assessment_completion": "unverified_attempt", "physical_qualification": "not_established"}


def report(document):
    pins = [f"| {name} | {record['size_bytes']} | {record['sha256']} |" for name, record in document["artifacts"].items()]
    observed = [f"| {name} | {status} |" for name, status in document["observed_outputs"].items()]
    formats = [f"| {name} | {record['format']} |" for name, record in document["format_checks"].items()]
    return "\n".join([
        "# Phase-19 w320 r02 attempt provenance audit", "",
        "**Partial: the 21 pinned files pass byte integrity, format and identity checks.** These are four existing Stage-1 round files, the saved leaderboard and 16 pinned attempt files. The leaderboard reports three assessed rows, zero qualified rows, a null champion and one separate unverified attempt for the target. Its stored reason records an evaluator failure with exit status 1; the pinned stdout file preserves the available evidence without interpreting or diagnosing it. The target is not counted as assessed or qualified. No simulations, geometry generation, physical gate evaluation or mechanical comparisons are performed.", "",
        f"The target label is `{LABEL}` with opaque certificate identity `{IDENTITY}`. Its r02 label refers to one-based round 2, stored in round_01 files. It is not evidence that round_02 files (the third saved round) exist.", "",
        f"The existing saved rounds contain {document['stage1']['births']} birth records and {document['stage1']['pending_screens']} pending screens. The target is the 93rd global birth-array entry in saved round 01, family p14. Its pending screen label/family/identity must match that exact position. No native certificate identity is regenerated.", "",
        "The attempt genome CSV header and single record match the saved birth header/record exactly. Both CSV files have rectangular single-record tables. All seven pinned attempt JSON files parse strictly as objects; duplicate keys and nonfinite numbers fail. Present label/identity fields and required metadata in the D1, grind-law, plate-GCI and surface documents must agree with the target; the D1 label retains its producer's _D1 suffix. SHA-256 metadata is checked for syntax only where no independent derivation is specified. Numeric observations and solver/geometry claims are not interpreted. STEP, STL, mesh, text and Markdown files receive byte-integrity checks only.", "",
        "| Requested output | Availability at snapshot |", "| --- | --- |", *observed, "",
        "The table records availability of the target's master scorecard and saved third-round birth/screen files. Missing outputs are not counted as completed assessments or qualifications. A later present file outside the pin set is explicitly unpinned. Any additional attempt paths are listed as unverified in the JSON receipt. New or changed files during validation fail the snapshot check; later changes require explicit receipt review/refresh.", "",
        "Phase-17's current schema/registry audit and Phase-18's current provenance audit must match their saved receipts. Phase-18 now accepts the reviewed leaderboard digest/size and preserves its three assessed rows and separate unverified entry. Phase-18's binding to the refreshed Phase-17 JSON is checked. Both receipt hashes are retained and reread after file validation. Earlier physical scope limits and missing-progression-baseline observations remain intact.", "",
        "Only repository-relative paths are emitted. --wave-dir and --repo-root support relocated saved copies without Git discovery when both are supplied. The default uses the common repository's w320 directory. The verifier is read-only unless --write explicitly updates this report and the Phase-19 JSON receipt. Source artifacts and earlier verifiers are not modified by the command.", "",
        "```sh", "python3 experiments/rsv2_07/audit_phase19_w320_r02_provenance.py",
        "python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'", "```", "",
        "Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Phase-10's test-source binding, Phase-17/18 receipt links and Phase-7/8 study bindings are refreshed together. Unit tests use synthetic saved bytes and in-memory relocated files.", "",
        "| Attempt file | Format check |", "| --- | --- |", *formats, "",
        "| Pinned artifact | Bytes | SHA-256 |", "| --- | ---: | --- |", *pins, "",
    ])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--wave-dir", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        directory = args.wave_dir if args.wave_dir is not None else phase18.receipts.default_wave_directory().with_name("w320")
        current = audit(args.repo_root, directory)
        if args.write:
            (args.repo_root / RECEIPT).write_text(json.dumps(current, indent=2) + "\n")
            (args.repo_root / REPORT).write_text(report(current))
        else:
            saved = load_json((args.repo_root / RECEIPT).read_bytes())
            require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-19 saved receipt mismatch")
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"w320 r02 provenance audit: FAIL: {error}\n")
    print("Phase-19 provenance: PASS; 21 pinned files; target birth/screen/genome references agree; Phase-17/18 synchronized.")
    print("Leaderboard: 3 assessed, 0 qualified, champion=null; target is 1 separate UNVERIFIED attempt. Physical qualification NOT ESTABLISHED.")


if __name__ == "__main__":
    main()
