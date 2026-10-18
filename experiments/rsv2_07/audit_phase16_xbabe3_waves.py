#!/usr/bin/env python3
"""Check xbabe3 receipt integrity and accounting; paired physical metrics are not evaluated."""

import argparse
import csv
import hashlib
from pathlib import Path
import re
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as receipts
from audit_phase13_screen_rounds import audit_documents as audit_rounds
from audit_phase14_w319_receipts import BOARD_HEADER
from portable_audit import require

PINS = {
    "w405c3": {
        "leaderboard.json": [
            "faf231f9b2c96dfc3665750303f6d18c00bf41ba43c40df94054c5db7b75cb50",
            5953
        ],
        "leaderboard.md": [
            "0c4680b9391f128c826c869e36b9d373cf02ec60ea428b8a80b0e0ce16a35bbe",
            865
        ],
        "RESULT.md": [
            "e847b5a139cba2ed8e4fb64bdb2477be2b5c7c0bad86bf0dfd15543d6218399a",
            2286
        ],
        "round_00_births.json": [
            "8cea1247b4eb2f4bece786d2b0f13807272d1550bbfa39a5750bc436cc6a0e03",
            102946
        ],
        "round_00_screen.json": [
            "f6eacf9fb8b1a2d7f6ac9867fcb521ec0a2d72c1f29d99706a708fbfbb2ed848",
            14203
        ],
        "round_01_births.json": [
            "b0144a3100827d9665ee6d3e8ef2ce9e6ac4688f6082293c32de58c0d7f4d4e1",
            94467
        ],
        "round_01_screen.json": [
            "b181c543b1faaf6d02bb4a5b85fb78ac0a9d1ea68918c25b27bc4701236a4778",
            20032
        ],
        "round_02_births.json": [
            "996a8b5167288c6fa774f1004e39221a853e865304355e1f1d16bbd54de708ac",
            94497
        ],
        "round_02_screen.json": [
            "fffa1f8686c70c7588d702622f67f0756a30bb22f99b2bb14bb0e745187f45f1",
            19606
        ]
    },
    "w406c3": {
        "leaderboard.json": [
            "56915cfc513a27e7e17a3a87ad3788e84cb65c23f5009472d3ca3def7c271af9",
            5940
        ],
        "leaderboard.md": [
            "54f007869daf779589324d4f89dadd9931df2ff460b06932e781306c9202b6fd",
            865
        ],
        "RESULT.md": [
            "08e3874db6f651149b09ff6081132e05c2cc7da2e105ff5ff5f95e212f4b8dc2",
            2331
        ],
        "round_00_births.json": [
            "c29223d66e1c1d25b4712efc966d53ebab4b0b3eed901a8508b543e610e5d57e",
            103086
        ],
        "round_00_screen.json": [
            "538f24eb8666c9a700a117e8984a9280d552342c59ae160551c9ff537e8461c4",
            13785
        ],
        "round_01_births.json": [
            "26e9cb38f8761b5a2ec091f79977ee0d4c1152ccfb9b9614d33b53a3cb6ebaea",
            94480
        ],
        "round_01_screen.json": [
            "83fc240d6aa42a1259d6f3996ecaae8c2821ef4d08c02a9cdd829fea99e8deff",
            19192
        ],
        "round_02_births.json": [
            "977f2d8ea8fecd4849566ba59666c30fc25994b6f2802f285bb49849c0323833",
            94778
        ],
        "round_02_screen.json": [
            "f96febdadc6ab90295608d1e29d1b2fb689f345caf871bc77c0dffff3c8dd1e8",
            20020
        ]
    },
    "w406t3": {
        "leaderboard.json": [
            "376f268e74c7de5abb5af1e74c96098b5cb43ab6afddad9e2a02a8660bdd62eb",
            5947
        ],
        "leaderboard.md": [
            "3cbb5ac73722cbe806ad9973d891b2c1362964b7215df60a0f99eb32c406ee69",
            865
        ],
        "RESULT.md": [
            "c016fc2d1957fb0d81370d5624c963c61ba4e57d1d5ecde01fa349f401e41792",
            2345
        ],
        "round_00_births.json": [
            "3ce17611374d93addd17832f7f7619dac604affbef383211a131a972652d3a08",
            103091
        ],
        "round_00_screen.json": [
            "82ba7f42cb4e0731f00bf3161ab7cc86565bba643e16cc0da05f91debdb9e2f0",
            13779
        ],
        "round_01_births.json": [
            "e213bd3e7d9a06acd2e0ec6dee69c04e3de43a05a38384712d7191f6e0271c36",
            95400
        ],
        "round_01_screen.json": [
            "fc79be15d82110cc28d0b314cf896343ac0a253f6f7ffdee7343130489353340",
            20030
        ],
        "round_02_births.json": [
            "97e8ff212f4d9c00af103d719754486f1bd7995ed7347cd0f4cd2a3ca89bba6e",
            94545
        ],
        "round_02_screen.json": [
            "a575bb45ff0265763327f91de045979dab4b8fe47673d7f433e096291facb2ed",
            19597
        ]
    },
    "w407c3": {
        "leaderboard.json": [
            "7da34b9df4f76434e6fffe7f9a5a3e60713478bfaf340c54cc26afa9ae4ccd61",
            5362
        ],
        "leaderboard.md": [
            "f022b6d18fe9cbe59307658b2ec26ab698fa6f4f980bf4e741562c6299975268",
            775
        ],
        "RESULT.md": [
            "9f1b68aea03e795b58498c616ad38594ce60e5ea7bddaf2d207756ad4ce4d45d",
            2587
        ],
        "round_00_births.json": [
            "9ef56544e0ac01fb82e99b6e3055479a4079ba7d5dc62122e5f7737dc9897260",
            130824
        ],
        "round_00_screen.json": [
            "1b02d667993aca396947df422b16b5f14c18b910fadd2a78d3d15387277a7578",
            4958
        ],
        "round_01_births.json": [
            "b68196822df5091bb3f0432430ea4b947b39b774303d70a4c357347750c8820d",
            93112
        ],
        "round_01_screen.json": [
            "464571849e44876c65cde9a1aed2b2ad4ce0780d3fe7660eec3e3269a92685c1",
            1618
        ],
        "round_02_births.json": [
            "0ae65d6da9468b23d00db842ca0fa85b109bf33f6f88fd8e819dae87a4b53e1c",
            46904
        ],
        "round_02_screen.json": [
            "d1bfaf33f2000aeeeae056165b6427be3506920bc2731d422bef555096c6757b",
            14543
        ]
    },
    "w407t3": {
        "leaderboard.json": [
            "a57a19ee420f859d96c870f977ded1e3def391926fc03d3e17a54719d4e063e6",
            5887
        ],
        "leaderboard.md": [
            "26d182c3f8e566a6a37aa3f638964acbeb57ef980dd3dc7afa1120bfa4fec805",
            865
        ],
        "RESULT.md": [
            "a61559b367773be4b94522bd3a40077c04569c95faff12c3dc8d4204b4e824b8",
            2481
        ],
        "round_00_births.json": [
            "8fe6b6ddbc819dcc795f48927ffa25e63f957f535fd84a2176c03f01427dde5d",
            130818
        ],
        "round_00_screen.json": [
            "d571555c69df2feeecdb25ea9f9c53831a0b5f17009faaf5cfcad5984fda0764",
            4955
        ],
        "round_01_births.json": [
            "c00813c5da5c114c64748d3d86d9b71d361df5356a933e5920be8fa5131d8e7d",
            93033
        ],
        "round_01_screen.json": [
            "cc79d18eddffbc9ae101c6fcaaa5d784ca4b2ae674b2738d4f57a6a0e3e6bdcc",
            1617
        ],
        "round_02_births.json": [
            "adf131a214ccc5fba6c685ca89bb3b8fcdde3f07cf595af43f91008a1ed959ef",
            46910
        ],
        "round_02_screen.json": [
            "f6e5e264baf2072041b6f972d38447c25436aa297ac4d4c45a5f4930a3559f03",
            14541
        ]
    },
    "w408c3": {
        "leaderboard.json": [
            "5928d01586e72e8d315359a3c63adc570693ea732098add8808fea2811370a41",
            5911
        ],
        "leaderboard.md": [
            "5bedd2e34dc8ac7a3c7f1df650847da1a39499457b7762480899e8a655eed3f8",
            865
        ],
        "RESULT.md": [
            "e8d17126558127da4e57f790f9bc32a21ac61a5d1806abbb1e5542642569a8c2",
            2497
        ],
        "round_00_births.json": [
            "a0522ae67c4469b403538cd7a9fd2290624a531653d6378eeb17ebb3d2045374",
            104170
        ],
        "round_00_screen.json": [
            "f820510b5d4cdcaba5402ff8860551ff9330ca71faf7038d111e060ad6e81b3c",
            3717
        ],
        "round_01_births.json": [
            "eb81298ab751e440c353b041015a7405dc8476d8624698f370fbb6e5ff46c9f7",
            85292
        ],
        "round_01_screen.json": [
            "546dce60033c9750c2ad0ee1e6df0080cc691dcaafaa6b0a1cb11b30e738629b",
            6468
        ],
        "round_02_births.json": [
            "26fdd0443143e2b67f3cb5a2f641b519c6969a31c6cbac9d3eca00b8fa96db3b",
            71972
        ],
        "round_02_screen.json": [
            "4b2db1d8dd0f32685ec5cbd2a7ec55af82ab050bd998b6dc725ac2f7311591c5",
            18989
        ]
    },
    "w408t3": {
        "leaderboard.json": [
            "e9b69682cf090106083d7db4f3bcdb5610d8849fb1e5b98ce873466e6f43f036",
            5875
        ],
        "leaderboard.md": [
            "5aa53e8ff3d2821fc7402415adf08c69fbc01c759a23758a136a7f79f1f6f8cb",
            865
        ],
        "RESULT.md": [
            "354184e29711a8791230c93e51db3d8b68ecd2615889965eed5f1307093278a2",
            2468
        ],
        "round_00_births.json": [
            "5ccb5b6d5aff1f6fae2d23e6cdd02337cd5c0df97e5f4a2bfbb96aeb95a80155",
            104202
        ],
        "round_00_screen.json": [
            "506cb14ef2da97f53f44715e37a92c583004c89dd290453e685aec4a46e3474e",
            3717
        ],
        "round_01_births.json": [
            "7e6620590a690fded1edd45616759797b37b7e3e630abd5f9d0992521a68fb68",
            85219
        ],
        "round_01_screen.json": [
            "3d7c985c7e3b6a274ffeafede39ca0a88e4421e2ac737bb5af2bf2260e343938",
            6059
        ],
        "round_02_births.json": [
            "5fad653f1056375ca108bed4c2c66a78cfa7d87c5520250a961077b56c6032a1",
            62895
        ],
        "round_02_screen.json": [
            "44be6efb28723839f8350b9bca8dc92602074316427fe4f1043c8df563a8789c",
            19377
        ]
    }
}

WAVES = tuple(PINS)
NAMES = set(PINS[WAVES[0]])
ROUND_NAMES = NAMES - {"leaderboard.json", "leaderboard.md", "RESULT.md"}
RESULT_HEADER = ["label", "qualified", "mass_g", "mass_3d_g", "w_cad", "g1", "solid_tip_sf",
                 "strict_hunter_sf", "seq", "fat_sf", "all_16", "rs_v2_all_hard", "battery"]


def table_rows(text, header, count):
    rows = [[cell.strip() for cell in line.strip().strip("|").split("|")]
            for line in text.splitlines() if line.startswith("|")]
    require(len(rows) == count + 2 and rows[0] == header
            and all(len(row) == len(header) for row in rows)
            and all(re.fullmatch(r":?-+:?", cell) for cell in rows[1]), "Markdown table schema/count mismatch")
    return rows[2:]


def check_reports(wave, board, markdown, result):
    require(markdown.startswith("# Evolution certification attempts\n"), "Leaderboard Markdown title mismatch")
    require(result.startswith(f"# {wave} RESULT "), "RESULT title/wave mismatch")
    rows = table_rows(markdown, BOARD_HEADER, len(board["designs"]))
    result_rows = table_rows(result, RESULT_HEADER, len(board["designs"]))
    for design, row, result_row in zip(board["designs"], rows, result_rows):
        require(row[0] == f"`{design['label']}`" and result_row[0] == design["label"], "Markdown candidate/order mismatch")
        require(row[1] == "false" and row[6] == str(design["rs_v2_all_hard"]).lower()
                and result_row[1] == "False" and result_row[10] == str(design["all_16_gates"])
                and result_row[11] == str(design["rs_v2_all_hard"]), "Markdown qualification flag mismatch")
    source = ("Source `leaderboard.json` (schema jarambit.auto_evolve.leaderboard.v2): "
              f"assessed {board['assessed_count']}, qualified 0, unverified {len(board['unverified'])}")
    require([line for line in result.splitlines() if line.startswith("Source `leaderboard.json`")] == [source],
            "RESULT source accounting mismatch")
    summaries = [line for line in result.splitlines() if line.startswith("**Summary:")]
    require(len(summaries) == 1 and summaries[0].startswith(f"**Summary: assessed {board['assessed_count']}, qualified 0;"),
            "RESULT summary accounting mismatch")
    errors = [line for line in result.splitlines() if line.startswith("- unverified:")]
    require(errors == [f"- unverified: {entry['label']}: {entry['reason']}" for entry in board["unverified"]],
            "RESULT unverified attempt mismatch")
    # All numeric report cells and performance prose remain opaque; no paired comparison.


def audit_documents(wave, documents):
    require(wave in WAVES and set(documents) == NAMES, "Exactly nine wave artifacts are required")
    rounds = audit_rounds(wave, {name: documents[name] for name in ROUND_NAMES})
    require(rounds["unavailable_rounds"] == [], "All three saved rounds are required")
    screened = {entry["label"]: entry for r in range(3) for entry in documents[f"round_{r:02}_screen.json"]}
    for entry in screened.values():
        receipts.fields(entry, receipts.SCREEN_NUMBERS, "screen observations")
        receipts.numbers(entry, receipts.SCREEN_NUMBERS)
    board = documents["leaderboard.json"]
    receipts.fields(board, {"schema", "designs", "assessed_count", "qualified_count", "champion", "unverified"}, "board")
    require(board["schema"] == "jarambit.auto_evolve.leaderboard.v2" and isinstance(board["designs"], list), "Invalid board schema")
    expected = 5 if wave == "w407c3" else 6
    require(type(board["assessed_count"]) is int and board["assessed_count"] == len(board["designs"]) == expected,
            "Assessed row count mismatch")
    require(type(board["qualified_count"]) is int and board["qualified_count"] == 0 and board["champion"] is None,
            "qualified_count must be integer zero and champion null")
    require(isinstance(board["unverified"], list) and len(board["unverified"]) == (1 if wave == "w407c3" else 0),
            "Unverified attempt count mismatch")
    receipts.no_qualification_claims(board)
    labels = set()
    for design in board["designs"]:
        receipts.fields(design, receipts.DESIGN_NUMBERS | {"label", "family", "certificate_identity", "qualified",
                        "all_16_gates", "rs_v2_all_hard", "battery_pass_count", "genome_csv", "scorecard_md"}, "assessed row")
        label = design["label"]
        require(isinstance(label, str) and label not in labels, "Duplicate/invalid assessed label")
        source = screened.get(label)
        require(source is not None and receipts.identifier(source) == receipts.identifier(design), "Assessed identity/screen mismatch")
        labels.add(label)
        require(design["qualified"] is False and type(design["all_16_gates"]) is bool
                and type(design["rs_v2_all_hard"]) is bool and not (design["all_16_gates"] and design["rs_v2_all_hard"]),
                "Contradictory stored qualification flags")
        receipts.numbers(design, receipts.DESIGN_NUMBERS)
        require(type(design["battery_pass_count"]) is int and design["battery_pass_count"] >= 0, "Invalid battery count")
        for key in ("genome_csv", "scorecard_md"):
            require(isinstance(design[key], str) and bool(design[key].strip()), "Missing saved artifact reference")
    for error in board["unverified"]:
        receipts.fields(error, {"label", "certificate_identity", "reason", "status"}, "unverified attempt")
        receipts.digest_string(error["certificate_identity"])
        label = error["label"]
        require(isinstance(label, str) and label in screened and label not in labels
                and error["certificate_identity"] == screened[label]["certificate_identity"], "Unverified identity/screen mismatch")
        require(error["status"] == "unverified" and isinstance(error["reason"], str) and bool(error["reason"].strip()),
                "Invalid unverified status/reason")
        labels.add(label)
    check_reports(wave, board, documents["leaderboard.md"], documents["RESULT.md"])
    return {"rounds": {r: {key: counts[key] for key in ("births", "screens")} for r, counts in rounds["rounds"].items()},
            "births": sum(r["births"] for r in rounds["rounds"].values()),
            "screens": len(screened), "assessed": len(board["designs"]), "qualified": 0,
            "unverified": len(board["unverified"]), "champion": None}


def audit_payloads(waves, pins=None):
    pins = PINS if pins is None else pins
    require(set(waves) == set(pins) == set(WAVES), "Exactly seven wave collections are required")
    results = {}
    for wave, files in waves.items():
        require(set(files) == set(pins[wave]) == NAMES, "Exactly nine pinned files are required per wave")
        documents = {}
        for name, data in files.items():
            sha, size = pins[wave][name]
            receipts.digest_string(sha)
            require(type(size) is int and size >= 0, "Invalid pinned byte size")
            require(isinstance(data, bytes) and len(data) == size and hashlib.sha256(data).hexdigest() == sha,
                    f"SHA-256/size mismatch: {wave}/{name}")
            documents[name] = load_json(data) if name.endswith(".json") else data.decode("utf-8")
        results[wave] = audit_documents(wave, documents)
    totals = {key: sum(value[key] for value in results.values()) for key in ("births", "screens", "assessed", "qualified", "unverified")}
    require(totals["assessed"] == 41 and totals["unverified"] == 1, "Cross-wave attempt accounting mismatch")
    return {"waves": results, "totals": totals, "pairs": {f"w{number}": [f"w{number}c3", f"w{number}t3"] for number in (406, 407, 408)},
            "standalone": ["w405c3"], "verified_files": 63}


def round_inventory(root):
    return {wave: {path.name for kind in ("births", "screen") for path in (root / wave).glob(f"round_*_{kind}.json")}
            for wave in WAVES}


def audit_root(root, pins=None):
    root = Path(root)
    inventory = round_inventory(root)
    require(all(names == ROUND_NAMES for names in inventory.values()), "Round file inventory mismatch")
    waves = {wave: {name: (root / wave / name).read_bytes() for name in NAMES} for wave in WAVES}
    result = audit_payloads(waves, pins)
    require(inventory == round_inventory(root), "Round inventory changed during audit")
    require(all((root / wave / name).read_bytes() == data for wave, files in waves.items() for name, data in files.items()),
            "Wave artifacts changed during audit")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out-root", type=Path, help="Relocated folder containing all seven saved wave directories; no Git needed")
    args = parser.parse_args()
    try:
        root = args.out_root if args.out_root is not None else receipts.default_wave_directory().parents[1] / "evidence/evolution_waves/2026-10-17/xbabe3/out"
        result = audit_root(root)
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"xbabe3 receipt audit: FAIL: {error}\n")
    for wave, counts in result["waves"].items():
        print(f"{wave}: births={counts['births']}; pending screens={counts['screens']}; assessed={counts['assessed']}; "
              f"qualified=0; champion=null; unverified={counts['unverified']}.")
    print(f"xbabe3 receipt integrity/accounting: PASS; 63 file digests; 41 assessed rows; 1 separate unverified attempt.")
    print("Three c3/t3 file pairs and w405c3 accounted for; paired structural metrics NOT ASSESSED. No simulations performed.")


if __name__ == "__main__":
    main()

