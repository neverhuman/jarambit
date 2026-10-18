#!/usr/bin/env python3
"""Verify diagnostic document/source provenance without executing production code."""

import argparse
import importlib.util
import json
from pathlib import Path


INDEX = "docs/audit/audit_provenance_index.json"
REPORT = "docs/audit/GENOTYPE_PHENOTYPE_CONSTANT_CONTRACT_AUDIT.md"
CONTRACT = "docs/audit/genotype_phenotype_constant_contract_audit.json"
LEGACY = "experiments/G16-12-RSV2-04/test_report_provenance.py"
SELF = "tools/verify_gate_feasibility_audit.py"
SOURCES = ("crates/genotype/src/lib.rs", "crates/phenotype/src/lib.rs")
TASK = "G16-12-RSV2-04-p26"
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
    require(type(index["report_count"]) is int and index["report_count"] == 16,
            "expected sixteen reports")
    expected = [str(path) for _, path in m.REPORTS] + [REPORT]
    require(len(m.REPORTS) == 15, "expected fifteen historical reports")
    require(isinstance(index["reports"], list) and len(index["reports"]) == 16,
            "report coverage mismatch")
    for row, path in zip(index["reports"], expected):
        require(isinstance(row, dict) and row.get("path") == path,
                "report path/order mismatch")
        diagnostic(row)
        require(row == record(m, root, path, markdown=True), "report fingerprint mismatch")
        require(all(type(row[k]) is int for k in ("bytes", "lines", "tables")),
                "report counts must be integers")
    require(index["supplemental_documents"] == [record(m, root, CONTRACT)],
            "supplemental fingerprint mismatch")
    contract = read_json(CONTRACT)
    require(set(contract) == {"schema_version", "task", "status", "qualified_count",
                              "assignment_outcome", "parity_assessment", "scope",
                              "source_files", "observations", "not_assessed"},
            "invalid contract schema")
    require(type(contract["schema_version"]) is int and contract["schema_version"] == 1,
            "unsupported contract schema")
    require(contract["task"] == TASK, "unexpected contract task")
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
    for path in expected + [INDEX, CONTRACT, SELF, LEGACY]:
        require(m.FORBIDDEN_HOME not in m.portable_path(root, path).read_text(encoding="utf-8"),
                "literal host prefix")
    require(m.verify(root) == 15, "historical index mismatch")
    return {"task": TASK, "status": STATUS, "assignment_outcome": OUTCOME,
            "qualified_count": 0, "reports": 16, "source_files": 2,
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
