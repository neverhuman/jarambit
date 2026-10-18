#!/usr/bin/env python3
"""Index saved digest fields and deduplicate assessment provenance, without evaluations."""

import argparse
import csv
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess

from audit_phase6 import load_json
import audit_phase11_wave_receipts as receipts
from audit_phase12_cross_wave import PINS as EARLIER_PINS
from audit_phase13_screen_rounds import audit_documents as audit_rounds
from audit_phase14_w319_receipts import PINS as W319_PINS
import audit_phase17_portable_schema as schema_audit
from audit_phase19_w320_r02_provenance import PINS as W320_PINS
from portable_audit import require

HERE = "experiments/rsv2_07/"
RECEIPT = HERE + "results/phase20_stage2_identity_dedup_audit.json"
REPORT = HERE + "PHASE20_STAGE2_IDENTITY_DEDUP_AUDIT.md"
WAVES = tuple(f"w{number}" for number in range(315, 321))
ROOT_NAMES = set(receipts.EXPECTED)
ROOT_DIRECTORIES = {wave: ("evidence/evolution_waves/2026-10-17/xbabe2/out/" if wave in WAVES[:3] else "out/") + wave
                    for wave in WAVES}
ROOT_PINS = {**EARLIER_PINS,
             "w319": {name: W319_PINS[name] for name in ROOT_NAMES},
             "w320": {name: W320_PINS[name] for name in ROOT_NAMES}}
KINDS = ("certificate_identity", "genome_sha256")


def digest_fields(value, location=()):
    if isinstance(value, dict):
        for key, item in value.items():
            child = location + (key,)
            if key in KINDS:
                schema_audit.digest_metadata(item, None)
                yield key, item, list(child)
            if isinstance(item, (dict, list)):
                yield from digest_fields(item, child)
    elif isinstance(value, list):
        for index, item in enumerate(value):
            if isinstance(item, (dict, list)):
                yield from digest_fields(item, location + (index,))


def reference(value, wave, identity, filename=None):
    require(isinstance(value, str), "Invalid saved reference")
    path = PurePosixPath(value)
    require(str(path) == value and ".." not in path.parts and "\\" not in value, "Noncanonical saved reference")
    width = 6 if filename is not None else 5
    parts = path.parts[-width:]
    require(len(parts) == width and parts[:4] == ("out", wave, "by-content", identity)
            and re.fullmatch(r"attempt-[0-9]+", parts[4]) and (path.is_absolute() or len(path.parts) == width),
            "Saved reference wave/identity/attempt mismatch")
    require(filename is None or parts[-1] == filename, "Saved reference label mismatch")
    return schema_audit.portable_path("/".join(parts))


def board_events(wave, documents):
    audit_rounds(wave, {name: documents[name] for name in ROOT_NAMES if name.startswith("round_")})
    screens = {row["label"]: row for r in (0, 1) for row in documents[f"round_{r:02}_screen.json"]}
    board = documents["leaderboard.json"]
    receipts.fields(board, {"schema", "designs", "assessed_count", "qualified_count", "champion", "unverified"}, "board")
    require(board["schema"] == "jarambit.auto_evolve.leaderboard.v2" and isinstance(board["designs"], list), "Invalid board schema")
    require(type(board["assessed_count"]) is int and board["assessed_count"] == len(board["designs"])
            == (3 if wave == "w320" else 4), "Assessed event count mismatch")
    require(type(board["qualified_count"]) is int and board["qualified_count"] == 0 and board["champion"] is None,
            "Invalid qualification count/champion")
    require(isinstance(board["unverified"], list) and len(board["unverified"]) == (1 if wave == "w320" else 0),
            "Unverified event count mismatch")
    receipts.no_qualification_claims(board)
    events, errors, labels = [], [], set()
    for row in board["designs"]:
        receipts.fields(row, {"label", "family", "certificate_identity", "genome_csv", "scorecard_md", "qualified"}, "assessed row")
        label = row["label"]
        require(isinstance(label, str) and label not in labels and label in screens
                and receipts.identifier(row) == receipts.identifier(screens[label]) and row["qualified"] is False,
                "Assessed identity/screen mismatch")
        labels.add(label)
        identity = row["certificate_identity"]
        genome = reference(row["genome_csv"], wave, identity, label + "_genome.csv")
        scorecard = reference(row["scorecard_md"], wave, identity, label + "_master_scorecard.md")
        require(PurePosixPath(genome).parent == PurePosixPath(scorecard).parent, "Scorecard/genome attempt mismatch")
        events.append({"event": wave + "/" + label, "wave": wave, "label": label, "certificate_identity": identity,
                       "genome_csv": genome, "attempt_directory": str(PurePosixPath(genome).parent)})
    for row in board["unverified"]:
        receipts.fields(row, {"label", "certificate_identity", "status", "reason"}, "unverified row")
        label = row["label"]
        require(isinstance(label, str) and label not in labels and label in screens and row["status"] == "unverified"
                and row["certificate_identity"] == screens[label]["certificate_identity"], "Unverified identity/screen mismatch")
        require(isinstance(row["reason"], str) and "raw artifacts retained in " in row["reason"], "Missing unverified attempt reference")
        identity = row["certificate_identity"]
        directory = reference(row["reason"].rpartition("raw artifacts retained in ")[2], wave, identity)
        errors.append({"event": wave + "/" + label, "wave": wave, "label": label, "certificate_identity": identity,
                       "genome_csv": directory + "/" + label + "_genome.csv", "attempt_directory": directory, "status": "unverified"})
        labels.add(label)
    return events, errors


def summarize(events, errors):
    groups, genome_groups, by_wave, seen = {}, {}, {}, set()
    for wave in WAVES:
        rows = [row for row in events if row["wave"] == wave]
        novel = set(row["certificate_identity"] for row in rows) - seen
        by_wave[wave] = {"assessment_events": len(rows), "first_seen_identities": len(novel),
                         "repeat_assessment_events": len(rows) - len(novel)}
        seen.update(novel)
        for row in rows:
            groups.setdefault(row["certificate_identity"], []).append(row)
            genome_groups.setdefault(row["genome_sha256"], []).append(row["event"])
    identities = []
    for identity, rows in sorted(groups.items()):
        genomes = {row["genome_sha256"] for row in rows}
        require(len(genomes) == 1, "One certificate identity binds conflicting genome CSV bytes")
        identities.append({"certificate_identity": identity, "genome_sha256": next(iter(genomes)),
                           "assessment_events": [row["event"] for row in rows], "first_seen_wave": rows[0]["wave"],
                           "repeat_events_beyond_first": len(rows) - 1})
    return {"assessment_events": len(events), "unique_assessed_identities": len(groups),
            "repeat_assessments_beyond_first": len(events) - len(groups),
            "unique_assessed_genome_hashes": len(genome_groups), "unverified_events": len(errors),
            "identities": identities, "genome_groups": dict(sorted(genome_groups.items())), "by_wave": by_wave}


def json_inventory(root):
    names = {directory + "/" + name for directory in ROOT_DIRECTORIES.values() for name in ROOT_NAMES}
    for wave in WAVES:
        names.update(str(path.relative_to(root)) for path in (root / "out" / wave / "by-content").rglob("*.json") if path.is_file())
    return names


def audit(root, saved=None, root_pins=None):
    root = Path(root)
    root_pins = ROOT_PINS if root_pins is None else root_pins
    require(set(root_pins) == set(WAVES), "Exactly six root pin collections are required")
    inventory = json_inventory(root)
    artifacts, indices, events, errors, root_documents = {}, {kind: {} for kind in KINDS}, [], [], {}

    def read(name, pin=None):
        schema_audit.portable_path(name)
        data = (root / name).read_bytes()
        record = {"sha256": hashlib.sha256(data).hexdigest(), "size_bytes": len(data)}
        if pin is not None:
            require((record["sha256"], record["size_bytes"]) == tuple(pin), f"Pinned root bytes mismatch: {name}")
        if saved is not None:
            require(name in saved["artifacts"] and record == saved["artifacts"][name], f"Saved artifact digest/size mismatch: {name}")
        artifacts[name] = record
        return data

    def index_document(name, document, event=None):
        for kind, sha, location in digest_fields(document):
            if event is not None:
                expected = event[kind]
                require(sha == expected, f"Attempt {kind} binding mismatch: {name}")
            item = indices[kind].setdefault(sha, {"occurrences": [], "assessed_events": [], "unverified_events": []})
            item["occurrences"].append({"path": name, "field_location": location})

    for wave in WAVES:
        require(set(root_pins[wave]) == ROOT_NAMES, "Invalid wave root pin coverage")
        documents = {name: load_json(read(ROOT_DIRECTORIES[wave] + "/" + name, root_pins[wave][name])) for name in sorted(ROOT_NAMES)}
        root_documents.update({ROOT_DIRECTORIES[wave] + "/" + name: document for name, document in documents.items()})
        assessed, unverified = board_events(wave, documents)
        events.extend(assessed)
        errors.extend(unverified)
    attempts = {}
    for status, rows in (("assessed_events", events), ("unverified_events", errors)):
        for event in rows:
            require(event["attempt_directory"] not in attempts, "Duplicate attempt directory across events")
            attempts[event["attempt_directory"]] = event
            genome_bytes = read(event["genome_csv"])
            csv_rows = list(csv.reader(genome_bytes.decode("utf-8").splitlines(), strict=True))
            require(len(csv_rows) == 2 and csv_rows[0] and all(csv_rows[0]) and len(set(csv_rows[0])) == len(csv_rows[0])
                    and len(csv_rows[0]) == len(csv_rows[1]), "Invalid single-record genome CSV")
            event["genome_sha256"] = hashlib.sha256(genome_bytes).hexdigest()
            for kind in KINDS:
                item = indices[kind].setdefault(event[kind], {"occurrences": [], "assessed_events": [], "unverified_events": []})
                item[status].append(event["event"])
    for name in sorted(inventory):
        if name in root_documents:
            index_document(name, root_documents[name])
            continue
        candidates = [event for directory, event in attempts.items() if name.startswith(directory + "/")]
        require(len(candidates) == 1, "Unreferenced attempt JSON requires scope review")
        index_document(name, load_json(read(name)), candidates[0])
    require(saved is None or set(artifacts) == set(saved["artifacts"]), "Saved file inventory mismatch")
    require(inventory == json_inventory(root), "JSON inventory changed during audit")
    for name, record in artifacts.items():
        require(hashlib.sha256((root / name).read_bytes()).hexdigest() == record["sha256"], "Source bytes changed during audit")
    summary = summarize(events, errors)
    require(summary["assessment_events"] == 23 and len(errors) == 1, "Cross-wave event accounting mismatch")
    return {"schema": "jarambit.rsv2_07.phase20_stage2_identity_dedup.v1", "task": "RSV2-07-p20",
            "scope": "saved_digest_fields_genome_byte_bindings_and_event_accounting_only",
            "wave_sources": ROOT_DIRECTORIES, "artifacts": dict(sorted(artifacts.items())),
            "source_json_files": len(inventory), "source_genome_csv_files": len(events) + len(errors),
            "digest_occurrences": {kind: sum(len(item["occurrences"]) for item in index.values()) for kind, index in indices.items()},
            "digest_index": {kind: dict(sorted(index.items())) for kind, index in indices.items()},
            "assessed": events, "unverified": errors, "deduplication": summary}


def report(document):
    summary = document["deduplication"]
    groups = [f"| {row['certificate_identity']} | {len(row['assessment_events'])} | {row['repeat_events_beyond_first']} | {row['first_seen_wave']} |"
              for row in summary["identities"]]
    waves = [f"| {wave} | {row['assessment_events']} | {row['first_seen_identities']} | {row['repeat_assessment_events']} |"
             for wave, row in summary["by_wave"].items()]
    sources = [f"| {wave} | {path} |" for wave, path in document["wave_sources"].items()]
    return "\n".join([
        "# Phase-20 cross-wave Stage-2 identity deduplication audit", "",
        f"**PASS for saved provenance and exact digest/byte deduplication.** {summary['assessment_events']} assessed events reference {summary['unique_assessed_identities']} distinct certificate identities, leaving {summary['repeat_assessments_beyond_first']} repeat assessments beyond the first occurrence. They bind {summary['unique_assessed_genome_hashes']} distinct genome CSV byte hashes. One separate unverified attempt is indexed and excluded from all assessed/novelty totals.", "",
        "| Certificate identity | Assessed events | Repeats beyond first | First-seen wave |", "| --- | ---: | ---: | --- |", *groups, "",
        "The b2cf2b33 and 6af25de9 groups each appear in all six waves. Each contains six assessment events and five repeats beyond its first occurrence. Their ten repeat events are part of the overall repeat total; other repeated identities are listed above. A repeat here means the same saved certificate identity and identical genome CSV bytes, not byte-identical assessment output files. Reused identity fields are also checked against the SHA-256 of the actual genome CSV bytes. Equal genome bytes with different certificate identities remain distinct identity groups; both indexes are retained.", "",
        "| Wave | Assessed events | Identities first seen in this wave | Repeat events |", "| --- | ---: | ---: | ---: |", *waves, "",
        "Novelty means first occurrence in the listed w315–w320 wave order, not a claim about execution time, physical novelty or performance. Within-wave event rows retain the saved leaderboard order. Root/birth/screen digest occurrences never count as additional assessments. The w320_r02_p14_c93 unverified attempt remains separate and does not become an assessed event just because intermediate files exist.", "",
        f"The audit scans all exact certificate_identity and genome_sha256 fields recursively in {document['source_json_files']} selected source JSON files, including nested attempt JSON. It records {document['digest_occurrences']['certificate_identity']} certificate-identity and {document['digest_occurrences']['genome_sha256']} genome-hash field occurrences, plus byte bindings for {document['source_genome_csv_files']} genome CSV files. Field locations are arrays of JSON keys/indices, not filesystem paths. No other numerical data is copied into the index.", "",
        "| Wave | Actual saved root source |", "| --- | --- |", *sources, "",
        "The short evidence/evolution_waves/2026-10-17/w315 through w318 paths named in the assignment are absent in this checkout. The existing xbabe2/out archive supplies w315–w317 root documents; the reviewed out/w318 copy supplies w318. All candidate attempt JSON and genome CSV files are under the corresponding live out/w315–out/w320 directories. Archive root and live attempt sources are reported separately; absent archive attempt copies are not claimed as verified.", "",
        "Each root leaderboard and two birth/screen pairs must match the existing reviewed pins. Required schemas/types, exact same-round birth positions, assessed and error identity references, zero stored qualified counts and null champions are checked. JSON parsing rejects duplicate keys/nonfinite numbers. All indexed digest values require lowercase hexadecimal SHA-256 syntax. Attempt certificate-identity fields must match their directory/event, and every stored genome_sha256 field must match the actual associated CSV file hash. One certificate identity cannot bind conflicting genome bytes. Metadata errors fail closed without inferring physical qualification.", "",
        "The source JSON inventory and all source byte hashes are checked again after scanning. Read-only runs require the saved file inventory, SHA-256 values, sizes and complete recomputed receipt to agree. --write explicitly creates/refreshes the receipt and report; new source bytes are not trusted automatically during verification. Source artifact bytes are never modified. Non-JSON solver/geometry files and Markdown scorecard bodies are outside this digest-field scan; their saved references do not imply a fresh validation of their claims.", "",
        "All emitted filesystem paths are canonical repository-relative POSIX paths. --artifact-root selects a relocated saved repository without Git discovery; --repo-root selects the receipt/output checkout. Defaults use the common repository for artifacts. Missing source JSON/CSV files fail rather than being counted as verified. No simulations, native certificate generation, geometry changes, physical gate checks, structural comparisons or performance rankings are performed.", "",
        "```sh", "python3 experiments/rsv2_07/audit_phase20_stage2_identity_dedup.py",
        "python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'", "```", "",
        "Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Phase-17/18/19 receipt bindings and the existing study/test-source hashes are synchronized after adding the Phase-20 tests.", "",
    ])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        root = args.artifact_root if args.artifact_root is not None else receipts.default_wave_directory().parents[1]
        saved = None if args.write else load_json((args.repo_root / RECEIPT).read_bytes())
        current = audit(root, saved)
        if args.write:
            (args.repo_root / RECEIPT).write_text(json.dumps(current, indent=2) + "\n")
            (args.repo_root / REPORT).write_text(report(current))
        else:
            require(json.dumps(saved, sort_keys=True) == json.dumps(current, sort_keys=True), "Phase-20 saved receipt mismatch")
    except (OSError, ValueError, csv.Error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Stage-2 digest deduplication: FAIL: {error}\n")
    summary = current["deduplication"]
    print(f"Stage-2 digest deduplication: PASS; {summary['assessment_events']} assessed events; {summary['unique_assessed_identities']} distinct identities; "
          f"{summary['repeat_assessments_beyond_first']} repeats beyond first; {summary['unverified_events']} separate unverified attempt.")


if __name__ == "__main__":
    main()
