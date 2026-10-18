#!/usr/bin/env python3
"""Verify historical Round-0 identity inventory; seed lineage requires saved input provenance."""

import argparse
import hashlib
import json
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


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--source-blob", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        artifact_root = args.artifact_root if args.artifact_root is not None else receipts.default_wave_directory().parents[1]
        source = args.source_blob.read_bytes() if args.source_blob is not None else subprocess.check_output(
            ["git", "-C", str(artifact_root), "show", REVISION + ":" + SOURCE], stderr=subprocess.PIPE)
        current = audit(args.repo_root, artifact_root, source)
        if args.write:
            (args.repo_root / RECEIPT).write_text(json.dumps(current, indent=2) + "\n")
            (args.repo_root / REPORT).write_text(report(current))
        else:
            saved = load_json((args.repo_root / RECEIPT).read_bytes())
            require(json.dumps(current, sort_keys=True) == json.dumps(saved, sort_keys=True), "Phase-21 receipt mismatch")
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Phase-21 inventory: FAIL: {error}\n")
    print("Phase-21 inventory integrity: PASS; seed-versus-mutated classification PARTIAL (seed input provenance unavailable); "
          "qualified_count=0 for all six saved waves; no selection counterfactual performed.")


if __name__ == "__main__":
    main()
