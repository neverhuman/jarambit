#!/usr/bin/env python3
"""Verify saved inventory, w320 schema provenance and the cross-phase receipt catalogue."""

import argparse
import hashlib
import json
import re
from pathlib import Path
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as receipts
import audit_phase17_portable_schema as schema
import audit_phase20_stage2_identity_dedup as phase20
from portable_audit import require

HERE = "experiments/rsv2_07/"
RECEIPT = HERE + "phase21_round0_seed_vs_mutated_inventory_audit.json"
REPORT = HERE + "PHASE21_ROUND0_SEED_VS_MUTATED_INVENTORY_AUDIT.md"
REVISION = "141e694532bb73a720bf76c1e54e19940a93abd1"
SOURCE = "crates/cli/src/auto_evolve.rs"
SOURCE_PIN = ("1eaadb175b1ba3ee89eedf7cf0cbae2fac8131a05374da44c6721032f5513524", 98188)
PHASE20_PIN = ("ed95015c3f9b753a78a1756b203f29f2e92f1482d9905aaeae99ee3a5bce4817", 873563)
# These are repeated identities from Phase 20, not a reconstructed launch seed set.
REFERENCE_IDENTITIES = frozenset((
    "b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc",
    "6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac",
))


def metadata(data):
    return {"sha256": hashlib.sha256(data).hexdigest(), "size_bytes": len(data)}


def pinned(data, pin, label):
    require(metadata(data) == {"sha256": pin[0], "size_bytes": pin[1]}, f"SHA-256/size mismatch: {label}")
    return data


def inventory(rows, references):
    """Count occurrences and distinct identifiers without using order or metrics."""
    for identity in references:
        schema.digest_metadata(identity, None)
    groups = {}
    for row in rows:
        family, identity = receipts.identifier(row)
        groups.setdefault(family, []).append(identity)
    def counts(identities):
        return {"entries": len(identities), "distinct_identities": len(set(identities)),
                "reference_matching_entries": sum(identity in references for identity in identities),
                "other_entries": sum(identity not in references for identity in identities)}
    return {**counts([identity for identities in groups.values() for identity in identities]),
            "families": {family: counts(identities) for family, identities in sorted(groups.items())}}


def wave_inventory(wave, documents):
    events, errors = phase20.board_events(wave, documents)
    screens = {row["label"]: row for round_id in (0, 1) for row in documents[f"round_{round_id:02}_screen.json"]}
    round0 = {row["label"] for row in documents["round_00_screen.json"]}
    subsets = {"round0_births": documents["round_00_births.json"]["genomes"],
               "round0_screens": documents["round_00_screen.json"],
               "round0_assessed": [screens[row["label"]] for row in events if row["label"] in round0],
               "later_round_assessed": [screens[row["label"]] for row in events if row["label"] not in round0],
               "all_rounds_assessed": [screens[row["label"]] for row in events]}
    return {"seed_input_provenance": "not_established_by_saved_wave_documents",
            "seed_vs_mutated_counts": None,
            "repeated_identity_reference_inventory": {name: inventory(rows, REFERENCE_IDENTITIES) for name, rows in subsets.items()},
            "qualified_count": 0, "champion": None,
            "unverified_events": len(errors),
            "round0_unverified_events": sum(row["label"] in round0 for row in errors)}


def audit(root, artifact_root, source_bytes):
    root, artifact_root = Path(root), Path(artifact_root)
    pinned(source_bytes, SOURCE_PIN, "immutable committed source")
    prior_bytes = pinned((root / phase20.RECEIPT).read_bytes(), PHASE20_PIN, "Phase-20 receipt")
    prior = load_json(prior_bytes)
    artifacts, payloads, waves = {}, {}, {}
    for wave in phase20.WAVES:
        documents = {}
        for name in sorted(phase20.ROOT_NAMES):
            path = f"out/{wave}/{name}"
            data = (artifact_root / path).read_bytes()
            pinned(data, phase20.ROOT_PINS[wave][name], path)
            record = metadata(data)
            require(record == prior["artifacts"][phase20.ROOT_DIRECTORIES[wave] + "/" + name], "Phase-20 root binding mismatch")
            artifacts[path], payloads[path], documents[name] = record, data, load_json(data)
        waves[wave] = wave_inventory(wave, documents)
    for name, data in payloads.items():
        require((artifact_root / name).read_bytes() == data, "Wave input changed during audit")
    require((root / phase20.RECEIPT).read_bytes() == prior_bytes, "Phase-20 receipt changed during audit")
    subset_names = next(iter(waves.values()))["repeated_identity_reference_inventory"].keys()
    totals = {name: {key: sum(row["repeated_identity_reference_inventory"][name][key] for row in waves.values())
                     for key in ("entries", "reference_matching_entries", "other_entries")} for name in subset_names}
    return {"schema": "jarambit.rsv2_07.phase21_round0_inventory.v1", "task": "RSV2-07-p21",
            "status": "partial_seed_input_provenance_missing", "scope": "historical_inventory_and_identity_linkage_only",
            "committed_source": {"revision": REVISION, "path": SOURCE, **metadata(source_bytes),
                                 "historical_wave_producer_version": "not_established"},
            "phase20_receipt": {"path": phase20.RECEIPT, **metadata(prior_bytes)},
            "reference_identity_basis": "two_repeated_Phase20_identities_not_a_verified_wave_seed_set",
            "reference_identities": sorted(REFERENCE_IDENTITIES), "artifacts": artifacts, "waves": waves,
            "reference_inventory_totals": totals, "qualified_count": 0,
            "unverified_events": sum(row["unverified_events"] for row in waves.values()),
            "counterfactual_selection": "not_performed"}


def report(document):
    lines = ["# Phase-21 Round-0 seed versus mutated inventory audit", "",
             "**PARTIAL: historical counts and linkage verified; seed-input provenance is missing.** The saved wave JSONs do not record the launch seed CSV list or an explicit seed-identity set. Therefore exact seed-versus-mutated counts remain null, rather than treating repeated identities or family block positions as proven seeds.", "",
             "The table gives occurrence counts against the two repeated Phase-20 identity references. Each pair is reference-matching / other. Other means a different saved identity, not proven mutation or novelty. Counts include repeated rows; distinct-identity counts and family totals are retained in JSON.", "",
             "| Wave | Round-0 births | Round-0 screens | Round-0 assessed | Later-round assessed | All assessed | Qualified | Unverified |",
             "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for wave, row in document["waves"].items():
        groups = row["repeated_identity_reference_inventory"]
        cells = [f"{groups[name]['reference_matching_entries']} / {groups[name]['other_entries']}" for name in groups]
        lines.append(f"| {wave} | " + " | ".join(cells) + f" | 0 | {row['unverified_events']} |")
    totals = document["reference_inventory_totals"]
    lines.extend(["", f"Recorded totals: {totals['round0_births']['entries']} Round-0 birth entries, {totals['round0_screens']['entries']} Round-0 screen entries and {totals['round0_assessed']['entries']} Round-0 assessed events. Across both rounds, the ledgers contain {totals['all_rounds_assessed']['entries']} assessed events and {document['unverified_events']} separate unverified event.", "",
                  "Reference certificate identities:", "", *["- " + identity for identity in document["reference_identities"]], "",
                  "certificate_identity is the saved native identifier. It is not the SHA-256 of CSV bytes, and neither field alone establishes that a candidate was an input seed. No native identity computation or genome normalization is performed. No individual non-reference candidate recommendations or rankings are emitted.", "",
                  f"Source reference: {SOURCE} at commit {REVISION}, SHA-256 {document['committed_source']['sha256']}. Only the immutable git-show blob is inspected; the other checkout's working file is not read. The source permits overriding its default seed inputs and separately records Round-0 births. A source listing does not recover historical invocation arguments or prove the producer version of old wave artifacts.", "",
                  "The previous Phase-20 audit records 23 historical assessments, 11 distinct identities and 12 repeat events beyond first occurrence. Those 12 are duplicate identity events, not 12 independently established seed re-assessments. The two repeated reference identities account for ten of those repeats; the remaining repeats belong to two other identity groups. This inventory does not relabel that evidence as seed lineage.", "",
                  "All 30 saved root JSON files under out/w315 through out/w320 match the earlier reviewed SHA-256/size pins and Phase-20 bindings. These live files are not tracked Git wave snapshots in this checkout. The tracked Phase-20 receipt is under results/, rather than the parent directory named in the first assignment. The two requested verifier/test filenames did not exist and are added for this audit.", "",
                  "Screen rows must link to exact same-round birth positions, families and identities. Assessment membership uses saved screen labels to distinguish rounds: an identity seen again in a later round does not turn that event into a Round-0 assessment. Unverified events stay separate. qualified_count=0 and champion=null are required for every saved leaderboard; this checks recorded metadata only.", "",
                  "The command verifies the source blob and prior receipt pins, strict JSON parsing, all root file bytes and complete recomputed receipt. It rereads inputs to detect changes during the audit. Output paths are repository-relative. --artifact-root selects a relocated saved repository; --source-blob supplies a byte-identical exported committed blob without Git discovery. Missing inputs or altered hashes fail closed. --write explicitly writes the report and receipt; normal verification is read-only.", "",
                  "No simulations, selection-rule replay, geometry changes, optimization or physical qualification checks are performed. Completing the seed-versus-mutated portion requires a retained launch seed-input manifest or equivalent independently bound provenance for each wave.", "",
                  "```sh", "python3 experiments/rsv2_07/verify_rsv2_07_receipts.py",
                  "python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v", "git diff --check", "```", "",
                  "Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2.", ""])
    return "\n".join(lines)



PHASE22_RECEIPT = HERE + "phase22_w320_leaderboard_provenance_audit.json"
PHASE22_REPORT = HERE + "PHASE22_W320_LEADERBOARD_PROVENANCE_AUDIT.md"
PHASE21_PIN = ("bc665634397714b3af19dda65b3c36b00c1c02d81132957ad8cd3d38b2c7547d", 25515)
PHASE22_PINS = {**phase20.ROOT_PINS["w320"],
                "leaderboard.md": ("2f994cb80d04604b482645878da23bfc80a4fffd0d707ae7ef7d93d0780b8755", 590)}
PHASE22_MD_HEADER = ["Design", "Qualified", "Mass g", "W_CAD", "G1", "Solid tip SF", "RS-v2 all_hard", "Battery"]


def check_phase22_markdown(board, text):
    lines = text.splitlines()
    require(lines and lines[0] == "# Evolution certification attempts", "Invalid leaderboard Markdown title")
    table = [[cell.strip() for cell in line.strip().strip("|").split("|")]
             for line in lines if line.strip().startswith("|")]
    require(len(table) == len(board["designs"]) + 2 and table[0] == PHASE22_MD_HEADER
            and len(table[1]) == len(PHASE22_MD_HEADER) and all(re.fullmatch(r":?-+:?", cell) for cell in table[1]),
            "Leaderboard Markdown table/schema mismatch")
    for design, row in zip(board["designs"], table[2:]):
        require(len(row) == 8 and row[0] == f"`{design['label']}`", "Leaderboard Markdown label/order mismatch")
        require(row[1] == "false" and row[6] == str(design["rs_v2_all_hard"]).lower(),
                "Leaderboard Markdown flag mismatch")
        expected = [format(design[key], spec) for key, spec in
                    (("mass_g", ".3f"), ("w_cad", ".6f"), ("g1", ".6f"), ("solid_tip_sf", ".4f"))]
        require(row[2:6] == expected and row[7] == f"{design['battery_pass_count']}/18",
                "Leaderboard Markdown saved-field transcription mismatch")
    return {"assessed_rows": len(board["designs"]), "columns": len(PHASE22_MD_HEADER),
            "order_and_labels_match_json": True, "stored_flags_match_json": True,
            "formatted_json_fields": ["mass_g", "w_cad", "g1", "solid_tip_sf", "battery_pass_count"],
            "unverified_representation": "not_listed_in_assessed_markdown_table"}


def audit_phase22_payloads(payloads, pins=None):
    pins = PHASE22_PINS if pins is None else pins
    require(isinstance(payloads, dict) and set(payloads) == set(pins) == set(PHASE22_PINS),
            "Exactly six w320 receipt/hash records are required")
    for name, data in payloads.items():
        schema.portable_path(name)
        sha, size = pins[name]
        require(type(size) is int and size >= 0, "Invalid pinned byte size")
        schema.digest_metadata(sha, size)
        require(isinstance(data, bytes), "Expected source bytes")
        pinned(data, pins[name], name)
    documents = {name: load_json(payloads[name]) for name in phase20.ROOT_NAMES}
    events, errors = phase20.board_events("w320", documents)
    for round_id in (0, 1):
        births = documents[f"round_{round_id:02}_births.json"]
        require(0 <= births["seed"] < 2 ** 64, "Invalid unsigned RNG seed")
        for row in documents[f"round_{round_id:02}_screen.json"]:
            receipts.fields(row, receipts.SCREEN_NUMBERS, "screen numeric profile")
            receipts.numbers(row, receipts.SCREEN_NUMBERS)
    require(documents["round_00_births.json"]["seed"] == documents["round_01_births.json"]["seed"]
            and documents["round_00_births.json"]["source"] == documents["round_01_births.json"]["source"],
            "Round birth provenance mismatch")
    board = documents["leaderboard.json"]
    for row in board["designs"]:
        receipts.fields(row, receipts.DESIGN_NUMBERS | {"rs_v2_all_hard", "all_16_gates", "battery_pass_count"}, "assessed profile")
        receipts.numbers(row, receipts.DESIGN_NUMBERS)
        require(type(row["rs_v2_all_hard"]) is bool and type(row["all_16_gates"]) is bool,
                "Invalid stored boolean flag")
        require(type(row["battery_pass_count"]) is int and 0 <= row["battery_pass_count"] <= 18,
                "Invalid stored battery count")
    screens = {row["label"]: (r, row) for r in (0, 1) for row in documents[f"round_{r:02}_screen.json"]}
    def linkage(event):
        round_id, screen = screens[event["label"]]
        return {key: screen[key] for key in ("label", "family", "certificate_identity")} | {"saved_round": round_id}
    return {"rounds": {str(r): {"births": len(documents[f"round_{r:02}_births.json"]["genomes"]),
                                 "pending_screens": len(documents[f"round_{r:02}_screen.json"])} for r in (0, 1)},
            "leaderboard": {"schema": board["schema"], "assessed_count": len(events), "qualified_count": 0,
                            "champion": None, "assessed": [linkage(event) for event in events],
                            "unverified_count": len(errors),
                            "unverified": [{**linkage(event), "status": "unverified"} for event in errors]},
            "markdown": check_phase22_markdown(board, payloads["leaderboard.md"].decode("utf-8")),
            "phase21_w320_inventory": wave_inventory("w320", documents)}


def audit_phase22(root, artifact_root, pins=None):
    root, artifact_root = Path(root), Path(artifact_root)
    prior_bytes = pinned((root / RECEIPT).read_bytes(), PHASE21_PIN, "Phase-21 receipt")
    prior = load_json(prior_bytes)
    payloads = {name: (artifact_root / "out/w320" / name).read_bytes() for name in PHASE22_PINS}
    result = audit_phase22_payloads(payloads, pins)
    require(result["phase21_w320_inventory"] == prior["waves"]["w320"], "Phase-21 w320 inventory mismatch")
    for name, data in payloads.items():
        require((artifact_root / "out/w320" / name).read_bytes() == data, "w320 source changed during audit")
    require((root / RECEIPT).read_bytes() == prior_bytes, "Phase-21 receipt changed during audit")
    return {"schema": "jarambit.rsv2_07.phase22_w320_provenance.v1", "task": "RSV2-07-p22",
            "status": "pass_saved_schema_and_provenance", "scope": "static_receipt_schema_linkage_and_transcription_only",
            "artifacts": {"out/w320/" + name: metadata(data) for name, data in sorted(payloads.items())},
            "phase21_receipt": {"path": RECEIPT, **metadata(prior_bytes), "w320_inventory_matches": True,
                                "seed_lineage_status": "partial_seed_input_provenance_missing"}, **result}


def report_phase22(document):
    rows = [f"| {name} | {record['size_bytes']} | {record['sha256']} |" for name, record in document["artifacts"].items()]
    rounds = [f"| {r} | {row['births']} | {row['pending_screens']} |" for r, row in document["rounds"].items()]
    board = document["leaderboard"]
    return "\n".join([
        "# Phase-22 w320 leaderboard and screen schema provenance audit", "",
        f"**PASS for saved schema, byte provenance and cross-consistency.** Six saved files match SHA-256 and byte-size pins. The JSON ledger records {board['assessed_count']} assessed rows, qualified_count=0, champion=null and {board['unverified_count']} separate unverified attempt. The Markdown table contains the three assessed rows only; it does not list the unverified attempt.", "",
        "| Saved round | Birth entries | Pending screen rows |", "| --- | ---: | ---: |", *rounds, "",
        "Births require integer round/RNG seed metadata, a nonempty source, a unique CSV header and rows of matching width. Round numbering, seed/source consistency and cross-round identity/CSV bindings are checked. Screens require unique, correctly formed labels, same-round birth position/family/identity matches, pending status and finite nonboolean numeric fields. Stored numbers are checked as schema values only.", "",
        "The v2 leaderboard requires integer assessment/qualification counts, boolean false qualified flags, a null champion, three distinct assessed labels and one separate unverified record. Every assessed/error identity links to the saved screen and birth rows. Genome/scorecard/error references must identify the same candidate and attempt; only normalized repository-relative references are used internally. Error reason text and original host paths are not copied into the receipt.", "",
        "Markdown checks require the saved title, eight-column header/separator, assessed-row count/order/labels and matching stored flags. Four displayed numeric strings must match the JSON's documented decimal formatting; the displayed battery count must match its integer JSON value and the saved /18 format. This is transcription verification, without recalculating physical metrics or gate verdicts. Omitted unverified entries are recorded as a presentation coverage limit, not recast as assessed or qualified.", "",
        "| Artifact | Bytes | SHA-256 |", "| --- | ---: | --- |", *rows, "",
        "The five JSON pins reuse the reviewed Phase-20/w320 registry. The Markdown pin is newly recorded from its saved bytes for Phase-22. Inputs are live saved artifacts, not newly generated simulations or claimed committed wave snapshots. All six source files and the Phase-21 receipt are reread to detect mutation during the audit. Missing files, invalid digests/sizes, altered bytes, duplicate JSON keys, nonfinite values, schema errors and inconsistent references fail closed.", "",
        f"Phase-21 binding: {document['phase21_receipt']['path']} (SHA-256 {document['phase21_receipt']['sha256']}). The recomputed w320 inventory must match that receipt exactly. Phase-21 seed-versus-mutated classification remains partial because launch seed provenance is absent; this audit supplies no replacement seed evidence.", "",
        "All emitted filesystem paths are canonical repository-relative POSIX paths. --artifact-root and --repo-root permit relocation without Git discovery. Normal runs verify the complete saved Phase-22 receipt and never write source artifacts. --write explicitly refreshes this report and receipt. No simulations, candidate selection, design changes, optimization or physical qualification checks are performed.", "",
        "```sh", "python3 experiments/rsv2_07/verify_rsv2_07_receipts.py --phase 22",
        "python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Controlling study hashes are synchronized for the new receipt, report and verifier/tests.", "",
    ])


PHASE23_RECEIPT = HERE + "phase23_complete_receipt_manifest_audit.json"
PHASE23_REPORT = HERE + "PHASE23_COMPLETE_RECEIPT_MANIFEST_AUDIT.md"
HOST_PREFIX = b"/home/" + b"ubuntu"
LEGACY_PHASES = {"GCI_A4_VERIFICATION.md": 1, "results/gci_a4_validation.json": 1,
                 "SOLID_TIP_3D_VERIFICATION.md": 4, "results/solid_tip_3d_validation.json": 4}
INDIRECT_REPORT_RECORDS = {HERE + "results/phase7_saved_triplet_gci_audit.json",
                           HERE + "results/phase8_synthetic_gci_audit.json"}


def receipt_phase(name):
    schema.portable_path(name)
    require(name.startswith(HERE), "Receipt path is outside the study")
    relative = name.removeprefix(HERE)
    match = re.match(r"phase([1-9][0-9]*)(?:_|\.)", Path(name).name, re.IGNORECASE)
    return int(match[1]) if match else LEGACY_PHASES.get(relative)


def receipt_manifest_paths(root):
    sources, host_scope = set(), set()
    for path in (Path(root) / HERE).rglob("*"):
        if not path.is_file() or path.suffix not in (".md", ".json"):
            continue
        name = str(path.relative_to(root))
        phase = receipt_phase(name)
        if phase is None or 1 <= phase <= 22:
            sources.add(name)
        if phase is not None and 15 <= phase <= 23:
            host_scope.add(name)
    return sources, host_scope | {PHASE23_RECEIPT, PHASE23_REPORT}


def zero_literal_host_prefix(payloads):
    require(isinstance(payloads, dict) and bool(payloads), "Empty host-prefix scan")
    for name, data in payloads.items():
        schema.portable_path(name)
        require(isinstance(data, bytes) and HOST_PREFIX not in data, f"Literal host prefix in {name}")


def qualified_count_fields(value, location=()):
    fields = []
    if isinstance(value, dict):
        for key, child in value.items():
            if key == "qualified_count":
                require(type(child) is int and child == 0, "Stored qualified_count must be integer zero")
                fields.append({"field_location": list(location + (key,)), "value": child})
            fields.extend(qualified_count_fields(child, location + (key,)))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            fields.extend(qualified_count_fields(child, location + (index,)))
    return fields


def audit_phase23(root):
    root = Path(root)
    sources, host_scope = receipt_manifest_paths(root)
    require(bool(sources), "No study reports or receipts found")
    own_outputs = {PHASE23_RECEIPT, PHASE23_REPORT}
    payloads = {name: (root / name).read_bytes() for name in sorted(sources | (host_scope - own_outputs))}
    zero_literal_host_prefix({name: payloads[name] for name in host_scope - own_outputs})
    records = {}
    for name in sorted(sources):
        data, phase = payloads[name], receipt_phase(name)
        is_json = name.endswith(".json")
        document = load_json(data) if is_json else None
        fields = qualified_count_fields(document)
        records[name] = {"phase": phase, "phase_basis": "numbered_filename" if re.match(r"phase[0-9]+_", Path(name).name, re.IGNORECASE)
                         else "legacy_alias" if phase is not None else "unassigned_supporting_artifact",
                         "kind": "json_receipt" if is_json else "markdown_report", **metadata(data),
                         "schema": document.get("schema") if isinstance(document, dict) else None,
                         "declared_task": document.get("task") if isinstance(document, dict) else None,
                         "qualified_count": 0 if fields else None, "explicit_qualified_count_fields": fields,
                         "qualification_field_status": "explicit_zero_fields" if fields else "not_recorded_as_json_field"}
    phases = {}
    for phase in range(1, 23):
        reports = [name for name, row in records.items() if row["phase"] == phase and row["kind"] == "markdown_report"]
        receipts_for_phase = [name for name, row in records.items() if row["phase"] == phase and row["kind"] == "json_receipt"]
        phases[str(phase)] = {"reports": reports, "json_receipts": receipts_for_phase,
                              "coverage": "report_and_json" if reports and receipts_for_phase else
                              "report_only" if reports else "json_only" if receipts_for_phase else "no_current_dedicated_files"}
    require((sources, host_scope) == receipt_manifest_paths(root), "Receipt inventory changed during audit")
    for name, data in payloads.items():
        require((root / name).read_bytes() == data, "Receipt bytes changed during audit")
    current = {"schema": "jarambit.rsv2_07.phase23_receipt_manifest.v1", "task": "RSV2-07-p23",
               "status": "pass_current_inventory_with_documented_coverage_gaps", "scope": "file_inventory_hashes_explicit_counts_and_host_prefixes_only",
               "qualified_count": 0, "qualification_basis": "this_manifest_admits_no_candidates; absent_source_fields_remain_null",
               "artifacts": records, "phases": phases,
               "totals": {"files": len(records), "markdown_reports": sum(row["kind"] == "markdown_report" for row in records.values()),
                          "json_receipts": sum(row["kind"] == "json_receipt" for row in records.values()),
                          "unassigned_supporting_files": sum(row["phase"] is None for row in records.values()),
                          "explicit_zero_count_fields": sum(len(row["explicit_qualified_count_fields"]) for row in records.values())},
               "host_prefix_audit": {"phase_range": [15, 23], "paths": sorted(host_scope), "files": len(host_scope),
                                     "literal_matches": 0, "own_outputs": "checked_after_serialization_without_self_hashes"},
               "hash_dependencies": {"phase23_json_binds": sorted(INDIRECT_REPORT_RECORDS),
                                     "phase7_8_study_manifests_omit": PHASE23_RECEIPT,
                                     "phase23_report_defers_control_byte_metadata_to_json": True,
                                     "phase23_self_hash": "not_claimed"}}
    zero_literal_host_prefix({PHASE23_RECEIPT: (json.dumps(current, indent=2) + "\n").encode(),
                              PHASE23_REPORT: report_phase23(current).encode()})
    return current


def report_phase23(document):
    counts = document["totals"]
    rows = []
    for name, row in document["artifacts"].items():
        size, sha = ("See JSON", "Final bytes bound in JSON") if name in INDIRECT_REPORT_RECORDS else (row["size_bytes"], row["sha256"])
        rows.append(f"| {name} | {row['phase'] if row['phase'] is not None else 'support'} | {size} | {sha} |")
    coverage = [f"| {phase} | {len(row['reports'])} | {len(row['json_receipts'])} | {row['coverage']} |"
                for phase, row in document["phases"].items()]
    return "\n".join([
        "# Phase-23 complete current receipt manifest audit", "",
        f"**PASS for current-file inventory and host-prefix hygiene, with coverage gaps recorded.** The catalogue binds {counts['files']} saved files: {counts['markdown_reports']} Markdown reports and {counts['json_receipts']} JSON receipts/manifests. Every entry has exact SHA-256 and byte size in the JSON. {counts['unassigned_supporting_files']} supporting files have no phase number. Missing historical deliverables are not fabricated.", "",
        "| Phase | Reports | JSON receipts | Current coverage |", "| --- | ---: | ---: | --- |", *coverage, "",
        "Legacy aliases assign GCI_A4_VERIFICATION.md and results/gci_a4_validation.json to Phase-1, and SOLID_TIP_3D_VERIFICATION.md and results/solid_tip_3d_validation.json to Phase-4. These are catalogue conventions; no distinct Phase-2/3/9 files are invented from shared or overwritten deliverables. The original REPORT.md, validation.json and cad_manifest.json remain unassigned supporting artifacts. A report-only phase does not acquire an invented JSON receipt.", "",
        f"The manifest's qualified_count=0 means it admits no candidates. Source JSON contains {counts['explicit_zero_count_fields']} explicit qualified_count fields, all checked as integer zero; booleans, nonzero counts and invalid types fail. Missing fields and Markdown qualification text remain unasserted/null in each file record. Existing partial, unverified and incomplete scientific claims are not upgraded. No physical qualification is evaluated.", "",
        "| Artifact | Phase | Bytes | SHA-256 |", "| --- | ---: | ---: | --- |", *rows, "",
        "The Phase-23 JSON records final byte digests and sizes for both Phase-7/8 controlling receipts. Their two rows defer to JSON here so this report can itself be hashed by those study manifests without a cycle. Phase-7/8 omit the Phase-23 JSON from their hash maps; its own verifier checks all catalogue entries and this exact rendered report. No receipt claims a self hash. Other catalogue rows show their full byte metadata above.", "",
        f"The literal user-home prefix scan covers all {document['host_prefix_audit']['files']} current Phase-15 through Phase-23 reports and JSON receipts, including both Phase-23 outputs. Three old machine prefixes in the Phase-15/16 report prose were replaced with repository-relative paths; underlying saved evidence is unchanged. Earlier reports/receipts retain their original bytes and any older host-path evidence, which is outside this requested hygiene range.", "",
        "The command discovers current Markdown/JSON files recursively under the study, parses all JSON strictly and rereads the inventory and source bytes to catch changes during an audit. Read-only verification compares the complete recomputed manifest and exact report text, so changed, missing and newly added catalogue files require review. --write explicitly refreshes outputs. --repo-root supports a relocated checkout without Git discovery or external saved wave directories. Only metadata, explicit count fields and portable file paths are copied; simulations and engineering analyses are not run.", "",
        "```sh", "python3 experiments/rsv2_07/verify_rsv2_07_receipts.py --phase 23",
        "python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v", "git diff --check", "```", "",
        "Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Dependent Phase-10/17/18/19 and controlling study hashes are synchronized.", "",
    ])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--phase", type=int, choices=(21, 22, 23), default=21)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--source-blob", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        require(args.phase == 21 or args.source_blob is None, "--source-blob applies only to Phase-21")
        if args.phase == 23:
            require(args.artifact_root is None, "--artifact-root applies only to Phase-21/22")
            current = audit_phase23(args.repo_root)
            receipt_name, report_name, render = PHASE23_RECEIPT, PHASE23_REPORT, report_phase23
        else:
            artifact_root = args.artifact_root if args.artifact_root is not None else receipts.default_wave_directory().parents[1]
        if args.phase == 22:
            current = audit_phase22(args.repo_root, artifact_root)
            receipt_name, report_name, render = PHASE22_RECEIPT, PHASE22_REPORT, report_phase22
        elif args.phase == 21:
            source = args.source_blob.read_bytes() if args.source_blob is not None else subprocess.check_output(
                ["git", "-C", str(artifact_root), "show", REVISION + ":" + SOURCE], stderr=subprocess.PIPE)
            current = audit(args.repo_root, artifact_root, source)
            receipt_name, report_name, render = RECEIPT, REPORT, report
        if args.write:
            (args.repo_root / receipt_name).write_text(json.dumps(current, indent=2) + "\n")
            (args.repo_root / report_name).write_text(render(current))
        else:
            saved = load_json((args.repo_root / receipt_name).read_bytes())
            require(json.dumps(current, sort_keys=True) == json.dumps(saved, sort_keys=True), f"Phase-{args.phase} receipt mismatch")
        if args.phase == 23:
            outputs = {name: (args.repo_root / name).read_bytes() for name in (receipt_name, report_name)}
            zero_literal_host_prefix(outputs)
            require(outputs[report_name] == render(current).encode(), "Phase-23 report mismatch")
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Phase-{args.phase} inventory: FAIL: {error}\n")
    if args.phase == 23:
        print(f"Phase-23 receipt manifest: PASS; {current['totals']['files']} files; "
              f"{current['host_prefix_audit']['files']} Phase-15..23 host-prefix checks; qualified_count=0 (no new admissions).")
    elif args.phase == 22:
        print("Phase-22 schema/provenance: PASS; 6 pinned files; 3 assessed; qualified_count=0; champion=null; "
              "1 separate unverified attempt; Markdown table covers assessed rows only.")
    else:
        print("Phase-21 inventory integrity: PASS; seed-versus-mutated classification PARTIAL (seed input provenance unavailable); "
              "qualified_count=0 for all six saved waves; no selection counterfactual performed.")


if __name__ == "__main__":
    main()
