#!/usr/bin/env python3
"""Check saved cross-wave receipt provenance and accounting; never evaluate designs."""

import argparse
import csv
from pathlib import Path
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as single_wave
from portable_audit import require

PINS = {
    "w315": {
        "leaderboard.json": ("49f9dcc245b9a2d81518ccbd992fe9b9c445d521a25080baf9b0eb98a611924d", 3961),
        "round_00_births.json": ("bc9441f863812257b119520370563da8cfe606fe56d76c8649ce3077838ce0d2", 183491),
        "round_00_screen.json": ("4ed6def0fb861038ddc7c8184c08c824325531f2830de41c3772abe0046e1689", 34524),
        "round_01_births.json": ("e149fae3512408544de0205c562d995838aa166f0c4f64386a00dd40b961863c", 175784),
        "round_01_screen.json": ("a45a5f4559b64702c9cf77e71e5f734e14af996ce1b20e34b4afde1523137698", 38662),
    },
    "w316": {
        "leaderboard.json": ("63df2d7b2c3613a4411b619b48200ec227ca9ef03515bc53e5e5dacb7405f8f5", 3955),
        "round_00_births.json": ("c1ce1969c2cee585db9cde609acd957635398acfc227f57c595bf08c0415db14", 180331),
        "round_00_screen.json": ("1b5cb4d4ff4504bcd073b0fd1276a5a3887ddbb06447a91a2ad2c478921ed56e", 8172),
        "round_01_births.json": ("a4fc583720d23a8fc0876ea398dde7d862d261a8199bfe496476573acec4ffe8", 167965),
        "round_01_screen.json": ("2a7a0f86ac92ea7e983179761b639ef7e7bf585274d96cee8d81cccfb589b872", 5647),
    },
    "w317": {
        "leaderboard.json": ("429efbe4be1db44befc662ed5fe7a4a1de8ebe3cb53beaa0902a88179b72bb0e", 3955),
        "round_00_births.json": ("aa4f9c4c0a888a9fb723ccd1245bb0a8c10d33be7743767ab53c71edbe73290c", 179197),
        "round_00_screen.json": ("1895c91af815e83e87e5d820705f84537327a50a493b37a7bfdf98b41b8de240", 8172),
        "round_01_births.json": ("b5df4627717dc2b76fc0b2f7ad18f92307b9ba82c268ed792d78ea7730efdda8", 166685),
        "round_01_screen.json": ("afd1e87a79c21f6996df5b0774df4b9653a413dafe2a7d83a2bb9b5c65822663", 5244),
    },
    "w318": single_wave.EXPECTED,
}


def cross_wave_summary(waves, summaries):
    bindings, birth_membership, assessed_membership = {}, {}, {}
    for wave, documents in waves.items():
        for round_id in (0, 1):
            births = documents[f"round_{round_id:02}_births.json"]
            header = births["genome_header"]
            for row in births["genomes"]:
                identity = row["certificate_identity"]
                binding = (header, row["row"])
                require(identity not in bindings or bindings[identity] == binding,
                        f"Cross-wave identity binding mismatch: {wave}/{identity}")
                bindings[identity] = binding
                birth_membership.setdefault(identity, set()).add(wave)
        for row in documents["leaderboard.json"]["designs"]:
            assessed_membership.setdefault(row["certificate_identity"], set()).add(wave)
    return {
        "waves": summaries,
        "totals": {key: sum(value[key] for value in summaries.values())
                   for key in ("births", "screens", "assessed", "qualified", "unverified")},
        "unique_birth_identities": len(bindings),
        "birth_identities_in_multiple_waves": sum(len(waves) > 1 for waves in birth_membership.values()),
        "unique_assessed_identities": len(assessed_membership),
        "assessed_identity_waves": {identity: sorted(waves) for identity, waves in sorted(assessed_membership.items())},
    }


def audit_documents(waves):
    require(isinstance(waves, dict) and set(waves) == set(PINS), "Exactly waves w315 through w318 are required")
    summaries = {wave: single_wave.audit_documents(documents) for wave, documents in waves.items()}
    return cross_wave_summary(waves, summaries)


def audit_payloads(payloads, pins=None):
    pins = PINS if pins is None else pins
    require(set(payloads) == set(pins) == set(PINS), "Exactly four wave byte/hash collections are required")
    summaries, documents = {}, {}
    for wave, files in payloads.items():
        try:
            summaries[wave] = single_wave.audit_payloads(files, pins[wave])
        except ValueError as error:
            raise ValueError(f"{wave}: {error}") from error
        documents[wave] = {name: load_json(data) for name, data in files.items()}
    return cross_wave_summary(documents, summaries)


def audit_directories(directories, pins=None):
    require(set(directories) == set(PINS), "Exactly four wave directories are required")
    payloads = {wave: {name: (Path(directory) / name).read_bytes() for name in single_wave.EXPECTED}
                for wave, directory in directories.items()}
    result = audit_payloads(payloads, pins)
    require(all((Path(directories[wave]) / name).read_bytes() == data
                for wave, files in payloads.items() for name, data in files.items()),
            "Cross-wave artifacts changed during audit")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive-root", type=Path, help="Directory containing saved w315, w316 and w317 folders")
    parser.add_argument("--live-wave-dir", type=Path, help="Directory containing the five saved w318 artifacts")
    args = parser.parse_args()
    try:
        if args.archive_root is None or args.live_wave_dir is None:
            repository = single_wave.default_wave_directory().parents[1]
        archive = args.archive_root if args.archive_root is not None else repository / "evidence/evolution_waves/2026-10-17/xbabe2/out"
        live = args.live_wave_dir if args.live_wave_dir is not None else repository / "out/w318"
        directories = {wave: archive / wave for wave in ("w315", "w316", "w317")}
        directories["w318"] = live
        result = audit_directories(directories)
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Cross-wave receipt audit: FAIL: {error}\n")
    for wave, counts in result["waves"].items():
        print(f"{wave}: births={counts['births']}; pending screens={counts['screens']}; "
              f"assessed_count={counts['assessed']}; qualified_count=0; unverified=0.")
    print(f"Cross-wave receipt audit: PASS; 20 file digests; {result['totals']['assessed']} assessment events; "
          f"{result['unique_assessed_identities']} distinct assessed identities; no conflicting identity bindings.")
    print("Qualification accounting only; no certificate regeneration or structural calculations performed.")


if __name__ == "__main__":
    main()
