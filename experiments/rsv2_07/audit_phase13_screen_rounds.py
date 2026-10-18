#!/usr/bin/env python3
"""Check saved Stage-1 artifact integrity and ID linkage, without design evaluation."""

import argparse
import csv
import hashlib
from pathlib import Path
import re
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as receipt
from audit_phase12_cross_wave import PINS as CROSS_WAVE_PINS
from portable_audit import require

PINS = {wave: {name: pin for name, pin in files.items() if name.startswith("round_")}
        for wave, files in CROSS_WAVE_PINS.items()}
PINS["w319"] = {
    "round_00_births.json": ("8ac6de8f7f19cc83c4c9d7f871006e3455300d5ffe71c3f51bedfeb66cd0167c", 174410),
    "round_00_screen.json": ("067756640f297d949b7f8f0aa898e3100ce8f61f8070ad2307e9542c7ecf575d", 23223),
    "round_01_births.json": ("a8098af848a7f65cf6f107e732bc888ace0b06bed7433af0749ed39776f12fee", 162120),
    "round_01_screen.json": ("af58989acf8b6f9f9c27dcac34f5476eaec73d0cc5adb1630818f1ba8bd34a3a", 19893),
}
REQUIRED_WAVES = {"w315", "w316", "w317", "w318"}
NAMES = tuple(f"round_{r:02}_{kind}.json" for r in range(3) for kind in ("births", "screen"))
METRIC_KEYS = ("lateral_sf", "tip_sf", "twist_deg_per_nm")
SLOTS = (6, 7, 8)


def audit_documents(wave, documents):
    require(isinstance(documents, dict) and bool(documents) and documents.keys() <= set(NAMES),
            "Invalid round artifact collection")
    rounds, bindings, labels, previous_header = {}, {}, set(), None
    receipt.no_qualification_claims(documents)
    for round_id in range(3):
        birth_name, screen_name = NAMES[2 * round_id:2 * round_id + 2]
        if birth_name not in documents and screen_name not in documents:
            continue
        require(birth_name in documents and screen_name in documents, "Incomplete birth/screen pair")
        births, screens = documents[birth_name], documents[screen_name]
        receipt.fields(births, {"round", "seed", "source", "genome_header", "genomes"}, "births")
        require(type(births["round"]) is int and births["round"] == round_id, "Round identity mismatch")
        require(type(births["seed"]) is int and isinstance(births["source"], str)
                and bool(births["source"].strip()), "Invalid birth provenance")
        require(isinstance(births["genome_header"], str), "Invalid CSV header")
        header = next(csv.reader([births["genome_header"]]), [])
        require(bool(header) and all(header) and len(header) == len(set(header)), "Invalid CSV header")
        require(previous_header is None or previous_header == header, "Birth CSV headers differ between rounds")
        previous_header = header
        require(isinstance(births["genomes"], list) and isinstance(screens, list), "Invalid record array")
        born = []
        for entry in births["genomes"]:
            receipt.fields(entry, {"family", "certificate_identity", "row"}, "birth")
            identity = receipt.identifier(entry)
            require(isinstance(entry["row"], str) and len(next(csv.reader([entry["row"]]), [])) == len(header),
                    "Birth CSV/header width mismatch")
            binding = (births["genome_header"], entry["row"])
            require(identity[1] not in bindings or bindings[identity[1]] == binding, "Conflicting identity binding")
            bindings[identity[1]] = binding
            born.append(identity)
        screened_slots = set()
        for entry in screens:
            receipt.fields(entry, {"label", "family", "certificate_identity", "qualification_status"}, "screen")
            identity = receipt.identifier(entry)
            require(isinstance(entry["label"], str), "Invalid screen label")
            match = re.fullmatch(re.escape(f"{wave}_r{round_id + 1:02}_{entry['family']}_c") + r"([0-9]{2,})", entry["label"])
            require(match is not None, "Screen label wave/round/family mismatch")
            slot = int(match[1])
            require(1 <= slot <= len(born) and identity == born[slot - 1], "Screen identity/birth slot mismatch")
            require(entry["label"] not in labels and slot not in screened_slots, "Duplicate screen label/slot")
            labels.add(entry["label"])
            screened_slots.add(slot)
            require(entry["qualification_status"] == receipt.PENDING, "Stage-1 status must remain pending")
        rounds[str(round_id)] = {
            "births": len(born), "screens": len(screens),
            "slots": {f"c{slot:02}": {"birth_present": slot <= len(born), "screen_present": slot in screened_slots}
                      for slot in SLOTS},
            # Field coverage only: never aggregate or interpret mechanical observations.
            "metric_field_presence": {key: sum(key in row for row in screens) for key in METRIC_KEYS},
        }
    require(list(rounds) == [str(i) for i in range(len(rounds))], "Completed rounds must be contiguous from 00")
    return {"rounds": rounds, "unavailable_rounds": [r for r in range(3) if str(r) not in rounds]}


def audit_payloads(wave, payloads, pins):
    require(set(payloads) == set(pins), "Artifact/hash coverage mismatch; unpinned or missing file")
    documents = {}
    for name, data in payloads.items():
        sha, size = pins[name]
        receipt.digest_string(sha)
        require(type(size) is int and size >= 0, "Invalid pinned byte size")
        require(isinstance(data, bytes) and len(data) == size and hashlib.sha256(data).hexdigest() == sha,
                f"SHA-256/size mismatch: {wave}/{name}")
        documents[name] = load_json(data)
    return audit_documents(wave, documents)


def audit_directories(root, pins=None):
    pins = PINS if pins is None else pins
    root = Path(root)
    require(set(pins) == REQUIRED_WAVES | {"w319"}, "Invalid wave pin collection")
    present = {wave: (root / wave).is_dir() for wave in pins}
    require(all(present[wave] for wave in REQUIRED_WAVES), "Missing required wave directory")
    inventory = {wave: {name for name in NAMES if (root / wave / name).is_file()}
                 for wave in pins if present[wave]}
    payloads = {wave: {name: (root / wave / name).read_bytes() for name in sorted(names)}
                for wave, names in inventory.items()}
    result = {wave: audit_payloads(wave, files, pins[wave]) for wave, files in payloads.items()}
    require(present == {wave: (root / wave).is_dir() for wave in pins}
            and inventory == {wave: {name for name in NAMES if (root / wave / name).is_file()}
                              for wave in inventory}, "Wave inventory changed during audit")
    require(all((root / wave / name).read_bytes() == data for wave, files in payloads.items() for name, data in files.items()),
            "Wave artifacts changed during audit")
    return {"waves": result, "absent_optional_waves": [] if present["w319"] else ["w319"],
            "verified_files": sum(map(len, payloads.values()))}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out-root", type=Path, help="Directory containing saved w315 through w319 directories")
    args = parser.parse_args()
    try:
        root = args.out_root if args.out_root is not None else receipt.default_wave_directory().parent
        result = audit_directories(root)
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Stage-1 artifact audit: FAIL: {error}\n")
    for wave, summary in result["waves"].items():
        for round_id, counts in summary["rounds"].items():
            print(f"{wave}/round_{int(round_id):02}: births={counts['births']}; screens={counts['screens']}; "
                  f"saved slot presence={counts['slots']}; field coverage={counts['metric_field_presence']}")
        print(f"{wave}: unavailable rounds={summary['unavailable_rounds']}")
    print(f"Stage-1 artifact integrity/linkage: PASS; {result['verified_files']} pinned file digests; "
          f"absent optional waves={result['absent_optional_waves']}.")
    print("Scope: artifact integrity and identifier representation only. Mechanical statistics and Q7 geometry semantics not assessed.")


if __name__ == "__main__":
    main()
