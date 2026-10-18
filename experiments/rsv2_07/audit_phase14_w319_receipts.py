#!/usr/bin/env python3
"""Verify the saved w319 receipt bytes and reporting, without evaluating designs."""

import argparse
import csv
import hashlib
from pathlib import Path, PurePosixPath
import re
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as wave_audit
from audit_phase13_screen_rounds import PINS as ROUND_PINS, audit_documents as audit_rounds
from portable_audit import require

PINS = {
    **ROUND_PINS["w319"],
    "leaderboard.json": ("df129159ec3d3e3d95044827f190c082b0ccd4165fc935dca002011b3b05fadf", 3983),
    "leaderboard.md": ("b3edff5fe8a5a0aff7783966271aa51add0313a420bffc48a0855c878d36bd99", 679),
    "by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792321160626983948/w319_r01_p10_c01_master_scorecard.md":
        ("542d65a9c1b4dbbd4ba4393d916b6c14d5e2d7dbdfd039f8e6a922e60f4a1606", 6907),
    "by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792322311344751353/w319_r01_p14_c27_master_scorecard.md":
        ("73806aaf467b4d64e2ce79b4be286c93953ce88b041d9ba0ffcb98e3b0ea3804", 6907),
    "by-content/47d60b04044f32fc84f17d915af58c44e0896e1f25ab97dda6587950f44b2850/attempt-1792325188215181150/w319_r02_p14_c45_master_scorecard.md":
        ("d7e2160333e21f8a7d63392c746d6b6acb3805bee982220b24c3c02514e05e0e", 6909),
    "by-content/89363bf867d1e07b9f91a4f72c181024f8823b4268987517fdc3d00a638d0d3c/attempt-1792323687719656617/w319_r02_p10_c21_master_scorecard.md":
        ("f57710afa103528c24ab2305e96f009d6d23effebab3bbff731c26054b3c0a5e", 6909),
}
BOARD_HEADER = ["Design", "Qualified", "Mass g", "W_CAD", "G1", "Solid tip SF", "RS-v2 all_hard", "Battery"]
BASE_FILES = set(wave_audit.EXPECTED) | {"leaderboard.md"}


def scorecard_key(reference, design):
    """Map a saved repository path to a local pinned suffix; never follow it."""
    require(isinstance(reference, str), "Invalid scorecard reference")
    path = PurePosixPath(reference)
    require(str(path) == reference and ".." not in path.parts and "\\" not in reference,
            "Noncanonical scorecard reference")
    parts = path.parts[-6:]
    require(len(parts) == 6 and parts[:3] == ("out", "w319", "by-content")
            and (path.is_absolute() or len(path.parts) == 6), "Invalid scorecard reference suffix")
    require(parts[3] == design["certificate_identity"], "Scorecard path certificate identity mismatch")
    require(re.fullmatch(r"attempt-[0-9]+", parts[4]) is not None
            and parts[5] == design["label"] + "_master_scorecard.md", "Scorecard attempt/label mismatch")
    return "/".join(parts[2:])


def check_markdown(board, text, scorecards):
    lines = text.splitlines()
    require(lines and lines[0] == "# Evolution certification attempts", "Invalid leaderboard Markdown title")
    table = [[cell.strip() for cell in line.strip().strip("|").split("|")]
             for line in lines if line.startswith("|")]
    require(len(table) == len(board["designs"]) + 2 and table[0] == BOARD_HEADER
            and len(table[1]) == len(BOARD_HEADER) and all(re.fullmatch(r":?-+:?", cell) for cell in table[1]),
            "Leaderboard Markdown table/schema mismatch")
    used = set()
    for design, row in zip(board["designs"], table[2:]):
        require(len(row) == len(BOARD_HEADER) and row[0] == f"`{design['label']}`",
                "Leaderboard Markdown candidate/order mismatch")
        require(row[1] == "false" and row[6] == str(design["rs_v2_all_hard"]).lower(),
                "Leaderboard Markdown qualification mismatch")
        key = scorecard_key(design["scorecard_md"], design)
        require(key in scorecards and key not in used, "Missing or reused assessed scorecard")
        used.add(key)
        card = scorecards[key].splitlines()
        require(card and card[0] == f"# Master CAD Scorecard — `{design['label']}`", "Scorecard title/label mismatch")
        verdicts = [line for line in card if line.startswith("- **Verdict**:")]
        require(verdicts == ["- **Verdict**: **FAIL (Gate Violation)**"], "Scorecard stored verdict mismatch")
    require(used == set(scorecards), "Unreferenced scorecard in pinned collection")
    return {design["label"]: scorecard_key(design["scorecard_md"], design) for design in board["designs"]}


def audit_payloads(payloads, pins=None):
    pins = PINS if pins is None else pins
    require(isinstance(payloads, dict) and set(payloads) == set(pins) and len(pins) == 10
            and BASE_FILES <= pins.keys(), "Exactly ten complete receipt/hash records are required")
    for name, data in payloads.items():
        path = PurePosixPath(name)
        require(not path.is_absolute() and ".." not in path.parts and str(path) == name, "Invalid pinned artifact path")
        sha, size = pins[name]
        wave_audit.digest_string(sha)
        require(type(size) is int and size >= 0, "Invalid pinned byte size")
        require(isinstance(data, bytes) and len(data) == size and hashlib.sha256(data).hexdigest() == sha,
                f"SHA-256/size mismatch: {name}")
    documents = {name: load_json(payloads[name]) for name in wave_audit.EXPECTED}
    result = wave_audit.audit_documents(documents)
    board = documents["leaderboard.json"]
    require(result["assessed"] == 4, "Exactly four assessed rows are required")
    audit_rounds("w319", {name: doc for name, doc in documents.items() if name.startswith("round_")})
    scorecards = {name: data.decode("utf-8") for name, data in payloads.items() if name not in BASE_FILES}
    result["scorecards"] = check_markdown(board, payloads["leaderboard.md"].decode("utf-8"), scorecards)
    result["verified_files"] = len(payloads)
    return result


def audit_directory(directory, pins=None):
    pins = PINS if pins is None else pins
    directory = Path(directory)
    # Validate keys before filesystem access, including caller-supplied pin sets.
    for name in pins:
        path = PurePosixPath(name)
        require(not path.is_absolute() and ".." not in path.parts and str(path) == name, "Invalid pinned artifact path")
    payloads = {name: (directory / name).read_bytes() for name in pins}
    result = audit_payloads(payloads, pins)
    require(all((directory / name).read_bytes() == data for name, data in payloads.items()),
            "Wave artifacts changed during audit")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--wave-dir", type=Path, help="Relocated w319 folder holding the ten pinned artifacts")
    args = parser.parse_args()
    try:
        directory = args.wave_dir if args.wave_dir is not None else wave_audit.default_wave_directory().with_name("w319")
        result = audit_directory(directory)
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"w319 receipt audit: FAIL: {error}\n")
    print(f"w319 receipt audit: PASS; {result['verified_files']} pinned file digests; {result['births']} birth records; "
          f"{result['screens']} pending screens; assessed_count={result['assessed']}; qualified_count=0; champion=null; unverified=0.")
    print("Four assessed scorecard paths, titles and stored FAIL verdicts agree; Markdown qualification rows agree with JSON.")
    print("Saved receipt integrity/accounting only; no certificate regeneration, solver runs or physical validation.")


if __name__ == "__main__":
    main()
