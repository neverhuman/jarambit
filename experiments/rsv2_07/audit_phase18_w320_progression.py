#!/usr/bin/env python3
"""Audit saved w320 provenance only; mechanical deficit progression is not evaluated."""

import argparse
import csv
import hashlib
import json
from pathlib import Path
import re
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as receipts
from audit_phase13_screen_rounds import audit_documents as audit_rounds
import audit_phase17_portable_schema as schema_audit
from portable_audit import require

HERE = "experiments/rsv2_07/"
RECEIPT = HERE + "results/phase18_w320_progression_audit.json"
REPORT = HERE + "PHASE18_W320_PROGRESSION_AUDIT.md"
BASELINE_CANDIDATES = (HERE + "phase15_w315_w319_progression_audit.json",
                       HERE + "results/phase15_w315_w319_progression_audit.json")
ROUND_NAMES = {f"round_{r:02}_{kind}.json" for r in (0, 1) for kind in ("births", "screen")}
LABELS = {"w320_r01_p10_c01", "w320_r01_p14_c27", "w320_r02_p10_c22"}
LEADERBOARD_PIN = ("c86614effb2283001bf9e0cb0293226d2a06fd8e6aac8006b67744b03adb9a7c", 3418)
PINS = {
    "round_00_births.json": ("f725189be663ee30646a52a84dd049648a7b39c7e39a58e7d9384e5d626e83c7", 177413),
    "round_00_screen.json": ("adb40eb24b76e1ce1120c5255f5db90f69c4824fad1ca19af60b8aa17a6134f5", 23213),
    "round_01_births.json": ("195b11e4828697ea8db1a6898330a62e03adcfd7f12c2671a71e03ef80a0cbdb", 165083),
    "round_01_screen.json": ("a4494238b88b330502a6d36910eacb947df9d511b587cd656c388a682bd1f274", 20297),
    "by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792326648482434020/w320_r01_p10_c01_master_scorecard.md":
        ("10a050cc49de2347e04016d5a933e9009a16305571c1c81cf3d463b2a7d5a941", 6907),
    "by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_master_scorecard.md":
        ("f6eb03b5c36fe9e1ff0b7469431c858d9d39a1b0830d5d4352bc42498ed72557", 6907),
    "by-content/dea35f5fb19c5f11bad56e636be1f88434aed9a0fa2547428c2e58619c128599/attempt-1792329315786884441/w320_r02_p10_c22_master_scorecard.md":
        ("eef7003b75b22574d06f663019b9a757930058043dd014766cec036de0db331c", 6909),
}


def audit_payloads(payloads, pins=None):
    pins = PINS if pins is None else pins
    require(set(payloads) == set(pins) and len(pins) == 7 and ROUND_NAMES <= pins.keys(),
            "Exactly four round files and three attempt scorecards are required")
    for name, data in payloads.items():
        schema_audit.portable_path(name)
        sha, size = pins[name]
        require(type(size) is int and size >= 0, "Invalid pinned byte size")
        schema_audit.digest_metadata(sha, size)
        require(isinstance(data, bytes) and len(data) == size and hashlib.sha256(data).hexdigest() == sha,
                f"SHA-256/size mismatch: {name}")
    documents = {name: load_json(payloads[name]) for name in ROUND_NAMES}
    round_counts = audit_rounds("w320", documents)["rounds"]
    screens = {entry["label"]: entry for r in (0, 1) for entry in documents[f"round_{r:02}_screen.json"]}
    for entry in screens.values():
        receipts.fields(entry, receipts.SCREEN_NUMBERS, "screen observations")
        receipts.numbers(entry, receipts.SCREEN_NUMBERS)
    attempts = {}
    for name in sorted(set(payloads) - ROUND_NAMES):
        parts = Path(name).parts
        require(len(parts) == 4 and parts[0] == "by-content" and re.fullmatch(r"attempt-[0-9]+", parts[2]),
                "Invalid attempt scorecard path")
        receipts.digest_string(parts[1])
        suffix = "_master_scorecard.md"
        require(parts[3].endswith(suffix), "Invalid scorecard filename")
        label = parts[3].removesuffix(suffix)
        require(label in LABELS and label not in attempts and label in screens
                and screens[label]["certificate_identity"] == parts[1], "Attempt identity/screen mismatch")
        lines = payloads[name].decode("utf-8").splitlines()
        require(lines and lines[0] == f"# Master CAD Scorecard — `{label}`", "Scorecard title/label mismatch")
        require([line for line in lines if line.startswith("- **Verdict**:")]
                == ["- **Verdict**: **FAIL (Gate Violation)**"], "Stored attempt verdict mismatch")
        attempts[label] = {"label": label, "certificate_identity": parts[1], "path": "out/w320/" + name,
                           "stored_verdict": "FAIL (Gate Violation)", "qualification": "not_established_without_leaderboard"}
    require(set(attempts) == LABELS, "Missing requested attempt scorecard")
    return {"stage1": {"rounds": {r: {key: row[key] for key in ("births", "screens")} for r, row in round_counts.items()},
                       "births": sum(row["births"] for row in round_counts.values()), "pending_screens": len(screens)},
            "stage2_attempts": [attempts[label] for label in sorted(attempts)]}


def check_leaderboard(data, rounds):
    board = load_json(data)
    receipts.fields(board, {"schema", "designs", "assessed_count", "qualified_count", "champion", "unverified"}, "leaderboard")
    require(board["schema"] == "jarambit.auto_evolve.leaderboard.v2" and isinstance(board["designs"], list)
            and type(board["assessed_count"]) is int and board["assessed_count"] == len(board["designs"]) == 3,
            "Invalid saved leaderboard assessment count")
    require(type(board["qualified_count"]) is int and board["qualified_count"] == 0 and board["champion"] is None,
            "Invalid saved qualification count/champion")
    receipts.no_qualification_claims(board)
    screens = {row["label"]: row for r in (0, 1) for row in rounds[f"round_{r:02}_screen.json"]}
    labels = set()
    for row in board["designs"]:
        receipts.fields(row, {"label", "family", "certificate_identity", "qualified"}, "assessed row")
        label = row["label"]
        require(isinstance(label, str) and label in LABELS and label not in labels and label in screens
                and receipts.identifier(row) == receipts.identifier(screens[label]) and row["qualified"] is False,
                "Saved assessed identity/screen mismatch")
        labels.add(label)
    require(labels == LABELS and isinstance(board["unverified"], list) and len(board["unverified"]) == 1,
            "Saved assessed/unverified coverage mismatch")
    error = board["unverified"][0]
    receipts.fields(error, {"label", "certificate_identity", "status", "reason"}, "unverified attempt")
    label = error["label"]
    require(isinstance(label, str) and label in screens and label not in labels and error["status"] == "unverified"
            and error["certificate_identity"] == screens[label]["certificate_identity"]
            and isinstance(error["reason"], str) and bool(error["reason"].strip()), "Invalid unverified attempt binding")
    return {"path": "out/w320/leaderboard.json", "status": "pinned_saved_receipt", "assessed_count": 3,
            "qualified_count": 0, "champion": None, "unverified_count": 1,
            "unverified": [{"label": label, "certificate_identity": error["certificate_identity"], "status": "unverified"}]}


def baseline_context(root):
    document_bytes = (root / schema_audit.RECEIPT).read_bytes()
    saved = load_json(document_bytes)
    current = schema_audit.audit(root)
    schema_audit.check_receipt(saved, current)
    entries = [entry for entry in saved["registry"] if 15 in entry["phases"]]
    require(len(entries) == 25 and {entry["path"].split("/")[1] for entry in entries} == {f"w{n}" for n in range(315, 320)},
            "Invalid Phase-15 registry context")
    source = HERE + "audit_phase15_gate_s_and_margins.py"
    return {"requested_progression_receipt": {"candidate_paths": list(BASELINE_CANDIDATES), "status": "missing"},
            "available_registry_context": {"phase15_source": source, "sha256": saved["source_sha256"][source],
                                           "phase15_references": 25, "waves": [f"w{n}" for n in range(315, 320)]},
            "phase17_receipt": {"path": schema_audit.RECEIPT, "sha256": hashlib.sha256(document_bytes).hexdigest(),
                                "status": "schema_and_registry_verified"}}


def audit(root, directory, pins=None):
    root, directory = Path(root), Path(directory)
    pins = PINS if pins is None else pins
    for name in pins:
        schema_audit.portable_path(name)
    leaderboard_present = (directory / "leaderboard.json").exists()
    require(not any((root / name).exists() for name in BASELINE_CANDIDATES), "Unpinned progression receipt appeared; review required")
    payloads = {name: (directory / name).read_bytes() for name in pins}
    result = audit_payloads(payloads, pins)
    leaderboard = {"path": "out/w320/leaderboard.json", "status": "missing", "assessment_and_qualification_counts": "not_established"}
    leaderboard_bytes = None
    if leaderboard_present:
        leaderboard_bytes = (directory / "leaderboard.json").read_bytes()
        require(isinstance(leaderboard_bytes, bytes)
                and (hashlib.sha256(leaderboard_bytes).hexdigest(), len(leaderboard_bytes)) == LEADERBOARD_PIN,
                "Unpinned leaderboard bytes; saved snapshot requires review")
        leaderboard = check_leaderboard(leaderboard_bytes, {name: load_json(payloads[name]) for name in ROUND_NAMES})
        for attempt in result["stage2_attempts"]:
            attempt["qualification"] = "stored_unqualified"
    context = baseline_context(root)
    require(all((directory / name).read_bytes() == data for name, data in payloads.items()), "Wave artifacts changed during audit")
    require((directory / "leaderboard.json").exists() == leaderboard_present and not any((root / name).exists() for name in BASELINE_CANDIDATES),
            "Input availability changed during audit")
    require(not leaderboard_present or (directory / "leaderboard.json").read_bytes() == leaderboard_bytes,
            "Leaderboard changed during audit")
    require(hashlib.sha256((root / schema_audit.RECEIPT).read_bytes()).hexdigest() == context["phase17_receipt"]["sha256"],
            "Baseline context changed during audit")
    artifacts = {"out/w320/" + name: {"sha256": sha, "size_bytes": size} for name, (sha, size) in sorted(pins.items())}
    if leaderboard_present:
        artifacts["out/w320/leaderboard.json"] = {"sha256": LEADERBOARD_PIN[0], "size_bytes": LEADERBOARD_PIN[1]}
    return {"schema": "jarambit.rsv2_07.phase18_w320_provenance.v1", "task": "RSV2-07-p18", "wave": "w320",
            "status": "partial_provenance_only", "scope": "saved_artifact_integrity_and_reference_accounting",
            "artifacts": artifacts, **result, "leaderboard": leaderboard,
            "baseline_context": context, "mechanical_deficit_comparison": "not_performed"}


def report(document):
    stage1 = document["stage1"]
    rows = [f"| {int(r):02} | {counts['births']} | {counts['screens']} |" for r, counts in stage1["rounds"].items()]
    cards = [f"| {a['label']} | {a['certificate_identity']} | {a['stored_verdict']} |" for a in document["stage2_attempts"]]
    pins = [f"| {name} | {pin['size_bytes']} | {pin['sha256']} |" for name, pin in document["artifacts"].items()]
    board = document["leaderboard"]
    ledger_note = ("The pinned saved leaderboard now reports three assessed rows, zero qualified rows, a null champion and one separate unverified attempt. The unverified attempt is retained as an error record and is not counted as assessed or qualified."
                   if board["status"] == "pinned_saved_receipt" else "The leaderboard is missing; assessment counts, qualification counts and champion state are not established.")
    return "\n".join([
        "# Phase-18 w320 saved provenance audit", "",
        f"**Partial: {len(document['artifacts'])} saved artifact hashes, schema and candidate references pass.** Mechanical deficit progression is not performed. The requested Phase-15 progression JSON is absent. " + ledger_note, "",
        "| Saved round | Birth records | Pending screen records |", "| --- | ---: | ---: |", *rows, "",
        f"There are {stage1['births']} birth records and {stage1['pending_screens']} pending screens. Each screen must match its exact same-round birth-array position, family and opaque certificate identity. CSV shape/provenance, finite observation types, pending status, duplicate labels and repeated-identity bindings are checked. No mechanical thresholds or deficits are calculated.", "",
        "| Requested attempt | Opaque certificate identity | Stored scorecard verdict |", "| --- | --- | --- |", *cards, "",
        "All three named attempt scorecards exist and their pinned paths, identity directories, filenames, titles and single stored FAIL verdicts link to the saved screens. A saved scorecard alone does not independently establish physical qualification. A present leaderboard is accepted only against its reviewed digest/size and candidate/error references. External files and performance claims cited within scorecards are not revalidated. Additional attempt artifacts are outside this fixed snapshot.", "",
        "The requested phase15_w315_w319_progression_audit.json is absent at both repository-relative candidate locations recorded in the JSON receipt. The existing Phase-15 registry and verified Phase-17 schema receipt provide provenance context for 25 references across w315–w319. They are not substituted numerical progression evidence; no deficit improvement or deterioration is inferred. The original Phase-15 scope limits remain in force.", "",
        "The verifier checks the pinned SHA-256 digests and byte sizes and rereads their bytes after validation. Missing required files, unpinned leaderboard bytes, invalid pins, malformed JSON, reference mismatches or mutation fail closed. Leaderboard availability and absence of the requested baseline are checked before and after. Any new baseline or unreviewed leaderboard requires explicit review. Phase-17 context is also reread to reject mutation.", "",
        "Only portable repository-relative paths are emitted. --wave-dir accepts a relocated folder and --repo-root a relocated checkout. With both supplied, no Git discovery or original host paths are required. The default wave directory resolves from the common repository. The command is read-only unless --write explicitly updates this report and the Phase-18 JSON receipt. Read-only runs compare the saved receipt against a fresh provenance audit.", "",
        "```sh", "python3 experiments/rsv2_07/audit_phase18_w320_progression.py",
        "python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'", "```", "",
        "Execution uses CPUs14,15 at inherited nice10 and CARGO_BUILD_JOBS=2. The Phase-10 test hash, Phase-17 schema inventory and Phase-7/8 study bindings are refreshed for the new audit and tests. No geometry, simulation, gate recalculation or physical qualification is performed.", "",
        "| Saved artifact | Bytes | SHA-256 |", "| --- | ---: | --- |", *pins, "",
    ])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--wave-dir", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        directory = args.wave_dir if args.wave_dir is not None else receipts.default_wave_directory().with_name("w320")
        current = audit(args.repo_root, directory)
        if args.write:
            (args.repo_root / RECEIPT).write_text(json.dumps(current, indent=2) + "\n")
            (args.repo_root / REPORT).write_text(report(current))
        else:
            saved = load_json((args.repo_root / RECEIPT).read_bytes())
            require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-18 saved receipt mismatch")
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"w320 provenance audit: FAIL: {error}\n")
    print(f"w320 provenance subset: PASS; {len(current['artifacts'])} pinned artifacts; {current['stage1']['births']} births; "
          f"{current['stage1']['pending_screens']} pending screens; three linked attempt scorecards.")
    print(f"Phase-18: PARTIAL; requested progression JSON missing; leaderboard={current['leaderboard']['status']}; mechanical deficit comparison NOT PERFORMED.")


if __name__ == "__main__":
    main()
