#!/usr/bin/env python3
"""Verify diagnostic document/source provenance without executing production code."""

import argparse
import importlib.util
import json
import tomllib
from pathlib import Path


INDEX = "docs/audit/audit_provenance_index.json"
REPORT = "docs/audit/GENOTYPE_PHENOTYPE_CONSTANT_CONTRACT_AUDIT.md"
CONTRACT = "docs/audit/genotype_phenotype_constant_contract_audit.json"
LEGACY = "experiments/G16-12-RSV2-04/test_report_provenance.py"
SELF = "tools/verify_gate_feasibility_audit.py"
SOURCES = ("crates/genotype/src/lib.rs", "crates/phenotype/src/lib.rs")
CONTRACT_TASK = "G16-12-RSV2-04-p26"
MANIFEST_TASK = "G16-12-RSV2-04-p27"
DIGEST_TASK = "G16-12-RSV2-04-p28"
CLI_TASK = "G16-12-RSV2-04-p29"
CRESCENT_TASK = "G16-12-RSV2-04-p30"
CERT_EXPORT_TASK = "G16-12-RSV2-04-p31"
AESTHETIC_GCI_TASK = "G16-12-RSV2-04-p32"
TASK = "G16-12-RSV2-04-p33"
BATTERY_G1_REPORT = "docs/audit/BATTERY_G1_SOURCE_PROVENANCE_AUDIT.md"
BATTERY_G1_JSON = "docs/audit/battery_g1_source_provenance_audit.json"
BATTERY_G1_SOURCES = ("crates/crescent/src/battery.rs", "crates/crescent/src/g1.rs")
AESTHETIC_GCI_REPORT = "docs/audit/AESTHETIC_GCI_SOURCE_PROVENANCE_AUDIT.md"
AESTHETIC_GCI_JSON = "docs/audit/aesthetic_gci_source_provenance_audit.json"
AESTHETIC_GCI_SOURCES = ("crates/crescent/src/aesthetic.rs", "crates/crescent/src/gci.rs")
CERT_EXPORT_REPORT = "docs/audit/CERT_EXPORT_SOURCE_PROVENANCE_AUDIT.md"
CERT_EXPORT_JSON = "docs/audit/cert_export_source_provenance_audit.json"
CERT_EXPORT_SOURCES = ("crates/crescent/src/certification.rs", "crates/crescent/src/export.rs")
CRESCENT_REPORT = "docs/audit/CRESCENT_SOURCE_PROVENANCE_AUDIT.md"
CRESCENT_JSON = "docs/audit/crescent_source_provenance_audit.json"
CRESCENT_SOURCES = ("crates/crescent/src/lib.rs", "crates/crescent/src/gates.rs")
CLI_REPORT = "docs/audit/CLI_SOURCE_PROVENANCE_AUDIT.md"
CLI_JSON = "docs/audit/cli_source_provenance_audit.json"
CLI_SOURCES = ("crates/cli/src/main.rs", "crates/cli/src/auto_evolve.rs")
DIGEST_REPORT = "docs/audit/EVOLVE_PHYSICS_SOURCE_PROVENANCE_AUDIT.md"
DIGEST_JSON = "docs/audit/evolve_physics_source_provenance_audit.json"
DIGEST_SOURCES = ("crates/evolve/src/lib.rs", "crates/physics/src/lib.rs")
MANIFEST_REPORT = "docs/audit/WORKSPACE_MANIFEST_PROVENANCE_AUDIT.md"
MANIFEST_JSON = "docs/audit/workspace_manifest_provenance_audit.json"
STATUS = "IN_PROGRESS"
OUTCOME = "PARTIAL"


def legacy_verifier(root):
    # This module contains document helpers only, not production solver imports.
    path = Path(root) / LEGACY
    if not path.resolve().is_relative_to(Path(root).resolve()):
        raise ValueError("legacy verifier escapes root")
    spec = importlib.util.spec_from_file_location("legacy_provenance", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def require(condition, message):
    if not condition:
        raise ValueError(message)


def diagnostic(doc):
    require(doc.get("status") == STATUS, "invalid diagnostic status")
    require(type(doc.get("qualified_count")) is int and doc["qualified_count"] == 0,
            "invalid diagnostic qualified_count")


def record(module, root, path, markdown=False):
    data = module.portable_path(root, path).read_bytes()
    text = data.decode("utf-8")
    result = {"path": path, "sha256": module.sha256(data),
              "bytes": len(data), "lines": len(text.splitlines())}
    if markdown:
        schema = module.table_schema(text)
        result.update(tables=len(schema), table_schema_sha256=module.schema_digest(schema),
                      status=STATUS, qualified_count=0)
    return result


def manifest_inventory(module, root):
    """Read local TOML metadata only; never resolve dependencies or invoke Cargo."""
    root = Path(root)
    paths = ["Cargo.toml"] + sorted(p.relative_to(root).as_posix()
                                     for p in (root / "crates").glob("*/Cargo.toml"))
    docs = {path: tomllib.loads(module.portable_path(root, path).read_text(encoding="utf-8"))
            for path in paths}
    workspace = docs["Cargo.toml"]["workspace"]
    defaults = {key: workspace["package"][key] for key in ("edition", "version")}
    require(all(isinstance(value, str) and bool(value) for value in defaults.values()),
            "workspace edition/version must be nonempty strings")
    members = workspace["members"]
    require(isinstance(members, list) and all(isinstance(x, str) for x in members),
            "invalid workspace members")
    member_paths = [f"{member}/Cargo.toml" for member in members]
    for path in member_paths:
        module.portable_path(root, path)
    require(len(member_paths) == len(set(member_paths))
            and sorted(member_paths) == paths[1:], "workspace/member manifest coverage mismatch")
    sections = ("dependencies", "dev-dependencies", "build-dependencies")
    records = []
    for path, doc in docs.items():
        item = record(module, root, path)
        item["package"] = None
        if "package" in doc:
            package = doc["package"]
            fields = {"name": package["name"]}
            for key in ("edition", "version"):
                declared = package[key]
                inherited = (isinstance(declared, dict) and set(declared) == {"workspace"}
                             and declared["workspace"] is True)
                require(inherited or (isinstance(declared, str) and bool(declared)),
                        "unsupported edition/version declaration")
                fields[key] = {"declared": declared,
                               "effective": defaults[key] if inherited else declared,
                               "origin": f"workspace.package.{key}" if inherited else f"package.{key}"}
            item["package"] = fields
        item["dependency_keys"] = {key: sorted(doc.get(key, {})) for key in sections}
        item["target_dependency_keys"] = {
            target: {key: sorted(table.get(key, {})) for key in sections}
            for target, table in sorted(doc.get("target", {}).items())}
        item["workspace_dependency_keys"] = sorted(doc.get("workspace", {}).get("dependencies", {}))
        records.append(item)
    return {"schema_version": 1, "task": MANIFEST_TASK, "status": STATUS, "qualified_count": 0,
            "assignment_outcome": "COMPLETE", "scope": "Static workspace manifest metadata only",
            "workspace_resolver": workspace["resolver"], "workspace_members": members,
            "workspace_package_defaults": defaults, "manifest_count": len(records),
            "manifests": records}


def digest_inventory(module, root, *, task=DIGEST_TASK, sources=DIGEST_SOURCES):
    """Hash inert source bytes and count UTF-8 lines; never inspect Rust semantics."""
    return {"schema_version": 1, "task": task, "status": STATUS, "qualified_count": 0,
            "assignment_outcome": "COMPLETE", "scope": "Static source byte provenance only",
            "source_count": len(sources),
            "source_files": [record(module, root, path) for path in sources]}


def verify(root):
    root = Path(root).resolve()
    m = legacy_verifier(root)

    def read_json(path):
        data = m.portable_path(root, path).read_bytes()
        require(m.FORBIDDEN_HOME.encode() not in data, "literal host prefix")
        doc = m.strict_json(data)
        require(m.home_occurrences(doc) == 0, "decoded host prefix")
        require(isinstance(doc, dict), "JSON document must be an object")
        return doc

    index = read_json(INDEX)
    require(set(index) == {"schema_version", "task", "status", "qualified_count",
                           "report_count", "reports", "supplemental_documents"},
            "invalid index schema")
    require(type(index["schema_version"]) is int and index["schema_version"] == 1,
            "unsupported index schema")
    require(index["task"] == TASK, "unexpected task")
    diagnostic(index)
    require(type(index["report_count"]) is int and index["report_count"] == 23,
            "expected twenty-three reports")
    expected = [str(path) for _, path in m.REPORTS] + [REPORT, MANIFEST_REPORT, DIGEST_REPORT, CLI_REPORT, CRESCENT_REPORT, CERT_EXPORT_REPORT, AESTHETIC_GCI_REPORT, BATTERY_G1_REPORT]
    require(len(m.REPORTS) == 15, "expected fifteen historical reports")
    require(isinstance(index["reports"], list) and len(index["reports"]) == 23,
            "report coverage mismatch")
    for row, path in zip(index["reports"], expected):
        require(isinstance(row, dict) and row.get("path") == path,
                "report path/order mismatch")
        diagnostic(row)
        require(row == record(m, root, path, markdown=True), "report fingerprint mismatch")
        require(all(type(row[k]) is int for k in ("bytes", "lines", "tables")),
                "report counts must be integers")
    require(index["supplemental_documents"] == [record(m, root, CONTRACT),
                                                   record(m, root, MANIFEST_JSON),
                                                   record(m, root, DIGEST_JSON),
                                                   record(m, root, CLI_JSON),
                                                   record(m, root, CRESCENT_JSON),
                                                   record(m, root, CERT_EXPORT_JSON),
                                                   record(m, root, AESTHETIC_GCI_JSON),
                                                   record(m, root, BATTERY_G1_JSON)],
            "supplemental fingerprint mismatch")
    contract = read_json(CONTRACT)
    require(set(contract) == {"schema_version", "task", "status", "qualified_count",
                              "assignment_outcome", "parity_assessment", "scope",
                              "source_files", "observations", "not_assessed"},
            "invalid contract schema")
    require(type(contract["schema_version"]) is int and contract["schema_version"] == 1,
            "unsupported contract schema")
    require(contract["task"] == CONTRACT_TASK, "unexpected contract task")
    diagnostic(contract)
    require(contract["assignment_outcome"] == OUTCOME
            and contract["parity_assessment"] == "NOT_ASSESSED",
            "unsupported parity/completion claim")
    require(isinstance(contract["scope"], str) and bool(contract["scope"]), "missing scope")
    for key in ("observations", "not_assessed"):
        require(isinstance(contract[key], list) and bool(contract[key])
                and all(isinstance(x, str) and bool(x) for x in contract[key]),
                "invalid scope observations")
    require(isinstance(contract["source_files"], list)
            and len(contract["source_files"]) == 2, "source coverage mismatch")
    for row, path in zip(contract["source_files"], SOURCES):
        require(isinstance(row, dict) and row.get("path") == path, "source path/order mismatch")
        require(row == record(m, root, path), "source fingerprint mismatch")
        require(all(type(row[k]) is int for k in ("bytes", "lines")),
                "source counts must be integers")
    text = m.portable_path(root, REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in text.splitlines()
            and "qualified_count: 0" in text.splitlines(), "invalid report status")
    manifest = read_json(MANIFEST_JSON)
    observed = manifest_inventory(m, root)
    require(json.dumps(manifest, sort_keys=True) == json.dumps(observed, sort_keys=True),
            "workspace manifest metadata/fingerprint mismatch")
    manifest_text = m.portable_path(root, MANIFEST_REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in manifest_text.splitlines()
            and "qualified_count: 0" in manifest_text.splitlines(), "invalid manifest report status")
    digest = read_json(DIGEST_JSON)
    require(json.dumps(digest, sort_keys=True) == json.dumps(digest_inventory(m, root), sort_keys=True),
            "source fingerprint mismatch: digest metadata")
    digest_text = m.portable_path(root, DIGEST_REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in digest_text.splitlines()
            and "qualified_count: 0" in digest_text.splitlines(), "invalid digest report status")
    cli_digest = read_json(CLI_JSON)
    require(json.dumps(cli_digest, sort_keys=True) == json.dumps(
        digest_inventory(m, root, task=CLI_TASK, sources=CLI_SOURCES), sort_keys=True),
        "source fingerprint mismatch: CLI digest metadata")
    cli_text = m.portable_path(root, CLI_REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in cli_text.splitlines()
            and "qualified_count: 0" in cli_text.splitlines(), "invalid CLI report status")
    crescent_digest = read_json(CRESCENT_JSON)
    require(json.dumps(crescent_digest, sort_keys=True) == json.dumps(
        digest_inventory(m, root, task=CRESCENT_TASK, sources=CRESCENT_SOURCES), sort_keys=True),
        "source fingerprint mismatch: crescent digest metadata")
    crescent_text = m.portable_path(root, CRESCENT_REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in crescent_text.splitlines()
            and "qualified_count: 0" in crescent_text.splitlines(), "invalid crescent report status")
    cert_export_digest = read_json(CERT_EXPORT_JSON)
    require(json.dumps(cert_export_digest, sort_keys=True) == json.dumps(
        digest_inventory(m, root, task=CERT_EXPORT_TASK, sources=CERT_EXPORT_SOURCES), sort_keys=True),
        "source fingerprint mismatch: certification/export digest metadata")
    cert_export_text = m.portable_path(root, CERT_EXPORT_REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in cert_export_text.splitlines()
            and "qualified_count: 0" in cert_export_text.splitlines(), "invalid certification/export report status")
    aesthetic_gci_digest = read_json(AESTHETIC_GCI_JSON)
    require(json.dumps(aesthetic_gci_digest, sort_keys=True) == json.dumps(
        digest_inventory(m, root, task=AESTHETIC_GCI_TASK, sources=AESTHETIC_GCI_SOURCES), sort_keys=True),
        "source fingerprint mismatch: aesthetic/gci digest metadata")
    aesthetic_gci_text = m.portable_path(root, AESTHETIC_GCI_REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in aesthetic_gci_text.splitlines()
            and "qualified_count: 0" in aesthetic_gci_text.splitlines(), "invalid aesthetic/gci report status")
    battery_g1_digest = read_json(BATTERY_G1_JSON)
    require(json.dumps(battery_g1_digest, sort_keys=True) == json.dumps(
        digest_inventory(m, root, task=TASK, sources=BATTERY_G1_SOURCES), sort_keys=True),
        "source fingerprint mismatch: battery/g1 digest metadata")
    battery_g1_text = m.portable_path(root, BATTERY_G1_REPORT).read_text(encoding="utf-8")
    require("status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)" in battery_g1_text.splitlines()
            and "qualified_count: 0" in battery_g1_text.splitlines(), "invalid battery/g1 report status")
    for path in expected + [INDEX, CONTRACT, MANIFEST_JSON, DIGEST_JSON, CLI_JSON,
                            CRESCENT_JSON, CERT_EXPORT_JSON, AESTHETIC_GCI_JSON,
                            BATTERY_G1_JSON, SELF, LEGACY]:
        require(m.FORBIDDEN_HOME not in m.portable_path(root, path).read_text(encoding="utf-8"),
                "literal host prefix")
    require(m.verify(root) == 15, "historical index mismatch")
    return {"task": TASK, "status": STATUS, "assignment_outcome": "COMPLETE",
            "qualified_count": 0, "reports": 23, "source_files": 2,
            "battery_g1_source_files": len(BATTERY_G1_SOURCES),
            "aesthetic_gci_source_files": len(AESTHETIC_GCI_SOURCES),
            "cert_export_source_files": len(CERT_EXPORT_SOURCES),
            "crescent_source_files": len(CRESCENT_SOURCES),
            "cli_source_files": len(CLI_SOURCES),
            "digest_source_files": len(DIGEST_SOURCES),
            "manifest_files": observed["manifest_count"], "prior_contract_assignment_outcome": OUTCOME,
            "parity_assessment": "NOT_ASSESSED"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        result = verify(args.root)
    except (ValueError, OSError, KeyError, TypeError, UnicodeError) as exc:
        parser.exit(1, f"audit provenance verification failed: {exc}\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
