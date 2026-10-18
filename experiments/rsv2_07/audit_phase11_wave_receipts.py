#!/usr/bin/env python3
"""Audit saved wave receipt bytes and accounting without running any evaluation."""

import argparse
import csv
import hashlib
import math
from pathlib import Path
import re
import subprocess

from audit_phase6 import load_json
from portable_audit import require

EXPECTED = {
    "leaderboard.json": ("7eeeb41a5dd36718ee6d300b626c76b01e0be5215bd36c97d9c37ca354c654a9", 3964),
    "round_00_births.json": ("294c9d71e64d0963c24c71fcd8e76f9ec6b8e08bd5089bb8c78c44dfc4e3d75f", 176486),
    "round_00_screen.json": ("2416936cd05b93217ed8912fc27d5cc8b9308ce0fe04e69fd35278daacded1d8", 20298),
    "round_01_births.json": ("f384d9fc60eaad7d2880e47c8ae467e9c42a13688050d0ffa80331b3736f2f21", 164034),
    "round_01_screen.json": ("243404dc45c5903b03a2ea3ddf476a5b0f6eeee16e38a7e8b24832ac81e53b28", 16949),
}
PENDING = "pending independent top-k assessment"
SCREEN_NUMBERS = {"fat_sf", "mass_g", "ring_bearing_w_mm", "screen_deficit", "seq_ratio"}
DESIGN_NUMBERS = {"buckling_eigenvalue_a4", "fat_sf", "g1", "mass_3d_g", "mass_g",
                  "ring_bearing_w_mm", "seq_ratio", "solid_tip_sf", "strict_hunter_sf", "w_cad"}


def fields(record, required, label):
    require(isinstance(record, dict) and required <= record.keys(), f"Missing schema fields: {label}")


def digest_string(value):
    require(isinstance(value, str) and re.fullmatch(r"[0-9a-f]{64}", value) is not None,
            "Invalid SHA-256 digest")


def identifier(record):
    digest_string(record["certificate_identity"])
    require(isinstance(record["family"], str) and bool(record["family"].strip()), "Invalid family")
    return record["family"], record["certificate_identity"]


def numbers(record, keys):
    for key in keys:
        value = record[key]
        require(type(value) in (int, float) and math.isfinite(value), f"Invalid numeric field: {key}")


def no_qualification_claims(value):
    if isinstance(value, dict):
        for key, item in value.items():
            if key == "qualified":
                require(item is False, "qualified must be the boolean false")
            no_qualification_claims(item)
    elif isinstance(value, list):
        for item in value:
            no_qualification_claims(item)


def audit_documents(documents):
    require(set(documents) == set(EXPECTED), "Exactly five wave artifacts are required")
    no_qualification_claims(documents)
    headers, known_rows, screened = [], {}, {}
    births_count = screen_count = 0
    round_counts = {}
    for round_id in (0, 1):
        births = documents[f"round_{round_id:02}_births.json"]
        fields(births, {"genome_header", "genomes", "round", "seed", "source"}, "births")
        require(type(births["round"]) is int and births["round"] == round_id, "Round identity mismatch")
        require(type(births["seed"]) is int, "Invalid seed")
        require(isinstance(births["source"], str) and bool(births["source"].strip()), "Missing birth provenance")
        require(isinstance(births["genome_header"], str), "Invalid CSV header")
        header = next(csv.reader([births["genome_header"]]))
        require(bool(header) and all(header) and len(header) == len(set(header)), "Invalid CSV header")
        headers.append(header)
        require(isinstance(births["genomes"], list), "Invalid births array")
        born = set()
        for entry in births["genomes"]:
            fields(entry, {"row", "family", "certificate_identity"}, "birth")
            identity = identifier(entry)
            require(isinstance(entry["row"], str) and len(next(csv.reader([entry["row"]]))) == len(header),
                    "Birth CSV/header width mismatch")
            certificate = identity[1]
            require(certificate not in known_rows or known_rows[certificate] == entry["row"],
                    "Conflicting rows for one certificate identity")
            known_rows[certificate] = entry["row"]
            born.add(identity)
        screen = documents[f"round_{round_id:02}_screen.json"]
        require(isinstance(screen, list), "Invalid screen array")
        for entry in screen:
            fields(entry, SCREEN_NUMBERS | {"label", "family", "certificate_identity", "qualification_status"}, "screen")
            require(identifier(entry) in born, "Screen identity absent from same-round births")
            require(isinstance(entry["label"], str) and bool(entry["label"].strip()), "Missing screen label")
            require(entry["label"] not in screened, "Duplicate screen label")
            require(entry["qualification_status"] == PENDING, "Stage-1 status must remain pending")
            numbers(entry, SCREEN_NUMBERS)
            screened[entry["label"]] = entry
        round_counts[str(round_id)] = {"births": len(births["genomes"]), "screens": len(screen)}
        births_count += len(births["genomes"])
        screen_count += len(screen)
    require(headers[0] == headers[1], "Birth CSV headers differ between rounds")
    board = documents["leaderboard.json"]
    fields(board, {"schema", "designs", "assessed_count", "qualified_count", "champion", "unverified"}, "leaderboard")
    require(board["schema"] == "jarambit.auto_evolve.leaderboard.v2", "Invalid leaderboard schema")
    require(isinstance(board["designs"], list), "Invalid assessed designs array")
    require(type(board["assessed_count"]) is int and board["assessed_count"] == len(board["designs"]),
            "Assessed count does not match design rows")
    require(type(board["qualified_count"]) is int and board["qualified_count"] == 0, "qualified_count must be zero")
    require(board["champion"] is None, "champion must be null")
    require(isinstance(board["unverified"], list) and len(board["unverified"]) == 0, "unverified must be an empty list")
    labels = set()
    for entry in board["designs"]:
        fields(entry, DESIGN_NUMBERS | {"label", "family", "certificate_identity", "qualified", "all_16_gates",
               "rs_v2_all_hard", "battery_pass_count", "genome_csv", "scorecard_md"}, "assessed design")
        require(entry["qualified"] is False, "Assessed qualified flag must be false")
        require(type(entry["all_16_gates"]) is bool and type(entry["rs_v2_all_hard"]) is bool,
                "Invalid stored qualification flags")
        require(not (entry["all_16_gates"] and entry["rs_v2_all_hard"]), "Contradictory stored qualification flags")
        require(isinstance(entry["label"], str) and entry["label"] not in labels, "Duplicate/invalid assessed label")
        labels.add(entry["label"])
        source = screened.get(entry["label"])
        require(source is not None and identifier(entry) == identifier(source), "Assessed identity/screen mismatch")
        require(type(entry["battery_pass_count"]) is int and entry["battery_pass_count"] >= 0, "Invalid battery count")
        for key in ("genome_csv", "scorecard_md"):
            require(isinstance(entry[key], str) and bool(entry[key].strip()), f"Missing artifact reference: {key}")
        numbers(entry, DESIGN_NUMBERS)
    return {"rounds": round_counts, "births": births_count, "screens": screen_count,
            "assessed": len(board["designs"]), "qualified": 0, "unverified": 0, "champion": None}


def audit_payloads(payloads, expected=None):
    expected = EXPECTED if expected is None else expected
    require(set(payloads) == set(EXPECTED) == set(expected), "Exactly five byte/hash records are required")
    documents = {}
    for name, data in payloads.items():
        sha, size = expected[name]
        digest_string(sha)
        require(type(size) is int and size >= 0, f"Invalid byte size: {name}")
        require(isinstance(data, bytes) and len(data) == size and hashlib.sha256(data).hexdigest() == sha,
                f"SHA-256/size mismatch: {name}")
        documents[name] = load_json(data)
    return audit_documents(documents)


def audit_wave_directory(directory, expected=None):
    directory = Path(directory)
    payloads = {name: (directory / name).read_bytes() for name in EXPECTED}
    result = audit_payloads(payloads, expected)
    require(all((directory / name).read_bytes() == data for name, data in payloads.items()),
            "Wave artifacts changed during audit")
    return result


def default_wave_directory():
    root = Path(__file__).resolve().parents[2]
    common = subprocess.check_output(["git", "rev-parse", "--git-common-dir"], cwd=root, text=True).strip()
    common = Path(common) if Path(common).is_absolute() else root / common
    return common.resolve().parent / "out/w318"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--wave-dir", type=Path, help="Directory holding all five saved wave JSON artifacts")
    args = parser.parse_args()
    try:
        directory = args.wave_dir if args.wave_dir is not None else default_wave_directory()
        result = audit_wave_directory(directory)
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Wave receipt audit: FAIL: {error}\n")
    print(f"Wave receipt audit: PASS; 5 file digests; {result['births']} births; {result['screens']} pending screens; "
          f"{result['assessed']} assessed; qualified=0; unverified=0; champion=null.")
    print("Identity cross-references checked; no certificate regeneration or structural calculations performed.")


if __name__ == "__main__":
    main()
