#!/usr/bin/env python3
"""Phase-15 artifact subset: saved scorecard integrity/layout, without gate or margin evaluation."""

import argparse
import hashlib
from pathlib import Path, PurePosixPath
import re
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as receipts
from audit_phase12_cross_wave import PINS as EARLIER_PINS
from audit_phase14_w319_receipts import PINS as W319_PINS
from portable_audit import require

CARD_PINS = {
    "w315": {
        "by-content/74ed6ed4ab8bf8a042352e73ca6a51561821d515acc09f21ed83bbaa880ad94e/attempt-1792292704052296518/w315_r01_p10_c03_master_scorecard.md":
            ("d7b1b95f220cc021cba6c0f819fdd7e1324a021a25a3118632afe2644feb9e05", 6907),
        "by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792295146972012229/w315_r02_p10_c12_master_scorecard.md":
            ("33ebc7a24e0d013c0e5969a09f9f659dd81cfe3becaf35a52c38da4d9740410f", 6907),
        "by-content/b77c59c158cf89b22a5493bfe380f31397eeb3a77c05a56fb9db46a584c7c0cb/attempt-1792304121638424690/w315_r02_p14_c29_master_scorecard.md":
            ("4aaa9bb5e9c8de226186418c3c24926abc28453af519501a21baa6e773da5528", 6907),
        "by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792293914730895747/w315_r01_p14_c50_master_scorecard.md":
            ("5dee2e79ee1684ca3d0a946950d0a5518bc4706a9df6b37dd1c83ae95c3fbdc7", 6907),
    },
    "w316": {
        "by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792305689989032813/w316_r01_p10_c01_master_scorecard.md":
            ("886cf60381547a5fb2f22982294e385c3b97bf053b6d701ac97b5c3da7bfa249", 6907),
        "by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792307082308955840/w316_r01_p14_c27_master_scorecard.md":
            ("7f7b99b9eab07b90f96a03cd5628b14e5000c4ff74df09b0fa03b02a71a3ec60", 6907),
        "by-content/d715c60095b56f3f9fe20eb89dcff8fca15ad48eccd50f0e8d968e7117b2b55f/attempt-1792309463348233325/w316_r02_p14_c88_master_scorecard.md":
            ("1cd137a1ae4ba436b0e4502a5a44bec34b8aa425bd3b8fffdda127c21c9b75ee", 6909),
        "by-content/234a9e89d25b63ad7300825df9ac88a8f78cac41957b17f43853d69184b5bd70/attempt-1792308261805508531/w316_r02_p10_c64_master_scorecard.md":
            ("bd44256b86335361e456ebc78f0d90dd4c4c500affae8588382a63256bcac932", 6909),
    },
    "w317": {
        "by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792310544027082374/w317_r01_p10_c01_master_scorecard.md":
            ("5015c9b9f7f29b81c36cb8b920dd89714022403e362cd3f53de399b7f30a46e8", 6907),
        "by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792312163831746164/w317_r01_p14_c27_master_scorecard.md":
            ("64367a5de1d2c69e41bdbc05e5821c3ef40871de8ddd751297092603d0be8a9f", 6907),
        "by-content/d715c60095b56f3f9fe20eb89dcff8fca15ad48eccd50f0e8d968e7117b2b55f/attempt-1792314572567943870/w317_r02_p14_c96_master_scorecard.md":
            ("56a75afe3e792a2d5d81b3b68b37d19cee4692a708fe1bf2823964207cb8108b", 6909),
        "by-content/234a9e89d25b63ad7300825df9ac88a8f78cac41957b17f43853d69184b5bd70/attempt-1792313495662966937/w317_r02_p10_c56_master_scorecard.md":
            ("1fc1d7486f9b21f32cbb6463491ca693d5cc21334297fa7304278be9be481ca3", 6909),
    },
    "w318": {
        "by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792315783786269184/w318_r01_p10_c01_master_scorecard.md":
            ("ace16bc6ffd57881114bef41770498b43d30f69acb9f13b0c1385232265e60aa", 6907),
        "by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792316960856419476/w318_r01_p14_c27_master_scorecard.md":
            ("64a8e8fd72508a7f4453ace2587c3bd402df807c175c38c19687b2bcd5c78433", 6907),
        "by-content/3eb40f1b9d74669c3a9afd22833888e26542a7c52ae512bbfa3f77fe5525b2db/attempt-1792319745170851375/w318_r02_p14_c92_master_scorecard.md":
            ("9f863dd83a5e9cf488e3e5f70ef6ad9eda103642555c8fce2e21f8f41706b2ca", 6909),
        "by-content/44ac7f99fd172d52780482c32dffe9e012282355e941ecc8ebbf0c40ebc35a43/attempt-1792318358578003318/w318_r02_p10_c10_master_scorecard.md":
            ("fb5a7109e2d683bdd2b20647661ccc16fef83b9924cd17a6a1ad042d1283f5aa", 6909),
    },
}
PINS = {wave: {"leaderboard.json": EARLIER_PINS[wave]["leaderboard.json"], **cards}
        for wave, cards in CARD_PINS.items()}
PINS["w319"] = {name: pin for name, pin in W319_PINS.items()
                if name == "leaderboard.json" or name.startswith("by-content/")}
TABLE_LAYOUT = (
    (["Component", "Weight", "Normalized Subscore `[0, 1]`", "Weighted Contribution"], 8),
    (["#", "Gate ID", "Measured Value", "Requirement / Floor", "Status"], 16),
    (["Evaluator ID", "Kind", "Solver", "Status", "Evidence / Detail"], 18),
)


def check_layout(text, label):
    lines = text.splitlines()
    require(lines and lines[0] == f"# Master CAD Scorecard — `{label}`", "Scorecard title/label mismatch")
    require([line for line in lines if line.startswith("- **Verdict**:")]
            == ["- **Verdict**: **FAIL (Gate Violation)**"], "Stored overall verdict mismatch")
    tables, block = [], []
    for line in lines + [""]:
        if line.startswith("|"):
            require(line.endswith("|"), "Unterminated Markdown table row")
            block.append([cell.strip() for cell in line[1:-1].split("|")])
        elif block:
            tables.append(block)
            block = []
    require(len(tables) == len(TABLE_LAYOUT), "Missing or extra scorecard table")
    for table, (header, row_count) in zip(tables, TABLE_LAYOUT):
        require(len(table) == row_count + 2 and table[0] == header, "Table header/row count mismatch")
        require(all(len(row) == len(header) and all(row) for row in table), "Table column/empty-cell mismatch")
        require(all(re.fullmatch(r":?-+:?", cell) for cell in table[1]), "Invalid table separator")
    gates, evaluators = tables[1][2:], tables[2][2:]
    require([row[0] for row in gates] == [str(i) for i in range(1, 17)], "Gate ordinal mismatch")
    require(len({row[1] for row in gates}) == 16 and len({row[0] for row in evaluators}) == 18,
            "Duplicate table identifiers")
    # Values and requirements remain opaque text. No pass rates or margins are derived.
    return [len(table) - 2 for table in tables]


def scorecard_key(wave, design):
    reference = design["scorecard_md"]
    require(isinstance(reference, str), "Invalid scorecard reference")
    path = PurePosixPath(reference)
    require(str(path) == reference and ".." not in path.parts and "\\" not in reference,
            "Noncanonical scorecard reference")
    parts = path.parts[-6:]
    require(len(parts) == 6 and parts[:3] == ("out", wave, "by-content")
            and (path.is_absolute() or len(path.parts) == 6), "Wrong scorecard wave/path")
    require(parts[3] == design["certificate_identity"] and re.fullmatch(r"attempt-[0-9]+", parts[4])
            and parts[5] == design["label"] + "_master_scorecard.md", "Scorecard identity/attempt/label mismatch")
    return "/".join(parts[2:])


def audit_payloads(wave, payloads, pins):
    require(wave in PINS and set(payloads) == set(pins) and len(pins) == 5 and "leaderboard.json" in pins,
            "Exactly five scorecard/leaderboard hash records are required")
    for name, data in payloads.items():
        sha, size = pins[name]
        receipts.digest_string(sha)
        require(type(size) is int and size >= 0, "Invalid pinned byte size")
        require(isinstance(data, bytes) and len(data) == size and hashlib.sha256(data).hexdigest() == sha,
                f"SHA-256/size mismatch: {wave}/{name}")
    board = load_json(payloads["leaderboard.json"])
    receipts.fields(board, {"schema", "designs", "assessed_count", "qualified_count", "champion", "unverified"}, "leaderboard")
    require(board["schema"] == "jarambit.auto_evolve.leaderboard.v2" and isinstance(board["designs"], list), "Invalid board schema")
    require(type(board["assessed_count"]) is int and board["assessed_count"] == len(board["designs"]) == 4,
            "Assessed count mismatch")
    require(type(board["qualified_count"]) is int and board["qualified_count"] == 0
            and board["champion"] is None and board["unverified"] == [], "Stored qualification accounting mismatch")
    receipts.no_qualification_claims(board)
    bindings, used = {}, set()
    for design in board["designs"]:
        receipts.fields(design, {"label", "family", "certificate_identity", "scorecard_md", "qualified"}, "assessed row")
        receipts.identifier(design)
        require(design["qualified"] is False and isinstance(design["label"], str)
                and design["label"].startswith(wave + "_") and design["label"] not in bindings, "Invalid/duplicate assessed label")
        key = scorecard_key(wave, design)
        require(key in payloads and key not in used, "Missing or reused scorecard reference")
        used.add(key)
        check_layout(payloads[key].decode("utf-8"), design["label"])
        bindings[design["label"]] = key
    require(used == set(payloads) - {"leaderboard.json"}, "Unreferenced pinned scorecard")
    return {"assessed": 4, "qualified": 0, "champion": None, "scorecards": bindings, "verified_files": 5}


def audit_directories(directories, pins=None):
    pins = PINS if pins is None else pins
    require(bool(directories) and directories.keys() <= PINS.keys() and directories.keys() <= pins.keys(), "Invalid wave selection")
    for wave in directories:
        for name in pins[wave]:
            path = PurePosixPath(name)
            require(not path.is_absolute() and ".." not in path.parts and str(path) == name, "Invalid pinned artifact path")
    payloads = {wave: {name: (Path(directory) / name).read_bytes() for name in pins[wave]}
                for wave, directory in directories.items()}
    result = {wave: audit_payloads(wave, files, pins[wave]) for wave, files in payloads.items()}
    require(all((Path(directories[wave]) / name).read_bytes() == data
                for wave, files in payloads.items() for name, data in files.items()), "Scorecard artifacts changed during audit")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    paths = parser.add_mutually_exclusive_group()
    paths.add_argument("--wave-dir", type=Path, help="Relocated single-wave folder; bypasses Git discovery")
    paths.add_argument("--out-root", type=Path, help="Folder containing all required w315..w318 and optional w319")
    parser.add_argument("--wave", choices=tuple(PINS), default="w319", help="Wave for --wave-dir (default w319)")
    args = parser.parse_args()
    try:
        if args.wave_dir is not None:
            directories = {args.wave: args.wave_dir}
        else:
            root = args.out_root if args.out_root is not None else receipts.default_wave_directory().parent
            directories = {wave: root / wave for wave in PINS if wave != "w319" or (root / wave).is_dir()}
        result = audit_directories(directories)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Phase-15 artifact subset: FAIL: {error}\n")
    for wave, summary in result.items():
        print(f"{wave}: 5 pinned files; 4 assessed scorecard references and layouts; qualified_count=0; champion=null.")
    print(f"Phase-15 artifact subset: PASS; {sum(row['verified_files'] for row in result.values())} files. "
          "Gate S conditions, physical gate validity, pass-rate claims and margin progression NOT ASSESSED.")


if __name__ == "__main__":
    main()
