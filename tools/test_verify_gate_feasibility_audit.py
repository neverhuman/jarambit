"""Synthetic document-integrity controls; no production modules are executed."""

import json
from pathlib import Path
import shutil
import tempfile
import unittest

import verify_gate_feasibility_audit as audit


ROOT = Path(__file__).resolve().parents[1]


class AuditProvenanceTests(unittest.TestCase):
    def snapshot(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        root = Path(tmp.name)
        m = audit.legacy_verifier(ROOT)
        paths = [audit.INDEX, audit.REPORT, audit.CONTRACT, audit.MANIFEST_REPORT,
                 audit.MANIFEST_JSON, audit.LEGACY, audit.SELF,
                 str(m.INDEX_PATH), "Cargo.toml"] + [p.relative_to(ROOT).as_posix()
                 for p in sorted((ROOT / "crates").glob("*/Cargo.toml"))] + list(audit.SOURCES) + [str(p) for _, p in m.REPORTS]
        for path in paths:
            target = root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / path, target)
        return root

    def change(self, root, path, mutate):
        target = root / path
        doc = json.loads(target.read_text())
        mutate(doc)
        target.write_text(json.dumps(doc, indent=2) + "\n")

    def rebind_contract(self, root):
        m = audit.legacy_verifier(root)
        row = audit.record(m, root, audit.CONTRACT)
        self.change(root, audit.INDEX, lambda d: d.update(supplemental_documents=[
            row, audit.record(m, root, audit.MANIFEST_JSON)]))

    def test_current_and_relocated_catalog(self):
        for root in (ROOT, self.snapshot()):
            with self.subTest(root=root.name):
                result = audit.verify(root)
                self.assertEqual(result["reports"], 17)
                self.assertEqual(result["source_files"], 2)
                self.assertEqual(result["manifest_files"], 11)
                self.assertEqual(result["assignment_outcome"], "COMPLETE")
                self.assertEqual(result["parity_assessment"], "NOT_ASSESSED")
        root = self.snapshot()
        self.assertFalse((root / ".git").exists())
        self.assertFalse((root / "target").exists())

    def test_document_corruption(self):
        m = audit.legacy_verifier(ROOT)
        for path in [str(p) for _, p in m.REPORTS] + [audit.REPORT, audit.CONTRACT, audit.MANIFEST_REPORT, audit.MANIFEST_JSON]:
            with self.subTest(document=path):
                root = self.snapshot()
                p = root / path
                p.write_bytes(p.read_bytes() + b"\nsynthetic change\n")
                with self.assertRaisesRegex(ValueError, "fingerprint mismatch"):
                    audit.verify(root)

    def test_source_corruption(self):
        for path in audit.SOURCES:
            with self.subTest(source=path):
                root = self.snapshot()
                p = root / path
                p.write_bytes(p.read_bytes() + b"\n// synthetic change\n")
                with self.assertRaisesRegex(ValueError, "source fingerprint mismatch"):
                    audit.verify(root)

    def test_index_coverage_order_and_counts(self):
        changes = [lambda d: d.update(report_count=15),
                   lambda d: d["reports"].pop(),
                   lambda d: d["reports"].reverse(),
                   lambda d: d["reports"].__setitem__(1, d["reports"][0]),
                   lambda d: d.update(qualified_count=True),
                   lambda d: d.update(qualified_count=1),
                   lambda d: d.update(status="COMPLETE")]
        for i, change in enumerate(changes):
            with self.subTest(case=i):
                root = self.snapshot()
                self.change(root, audit.INDEX, change)
                with self.assertRaises(ValueError):
                    audit.verify(root)

    def test_report_metadata(self):
        for key, value in [("sha256", "f" * 64), ("bytes", 1), ("lines", 1),
                           ("tables", 0), ("table_schema_sha256", "f" * 64),
                           ("status", "COMPLETE"), ("qualified_count", 1)]:
            with self.subTest(column=key):
                root = self.snapshot()
                self.change(root, audit.INDEX, lambda d: d["reports"][-1].update({key: value}))
                with self.assertRaises(ValueError):
                    audit.verify(root)

    def test_source_metadata_and_unassessed_status(self):
        for key, value in [("bytes", 1), ("lines", 1), ("path", "../outside.rs")]:
            with self.subTest(source_column=key):
                root = self.snapshot()
                self.change(root, audit.CONTRACT,
                            lambda d: d["source_files"][0].update({key: value}))
                self.rebind_contract(root)
                with self.assertRaises(ValueError):
                    audit.verify(root)
        for key, value in [("assignment_outcome", "COMPLETE"),
                           ("parity_assessment", "PASS"), ("qualified_count", True)]:
            with self.subTest(status_column=key):
                root = self.snapshot()
                self.change(root, audit.CONTRACT, lambda d: d.update({key: value}))
                self.rebind_contract(root)
                with self.assertRaises(ValueError):
                    audit.verify(root)

    def test_strict_json_and_host_prefix(self):
        for suffix in [', "task": "duplicate"}', ', "extra": NaN}',
                       ', "extra": "\\u002fhome\\u002fubuntu"}']:
            with self.subTest(suffix=suffix):
                root = self.snapshot()
                p = root / audit.INDEX
                p.write_text(p.read_text().rstrip()[:-1] + suffix)
                with self.assertRaises(ValueError):
                    audit.verify(root)

    def test_paths_and_symlink_escape(self):
        for path in ("/absolute/report.md", "../outside.md", "docs\\audit\\report.md"):
            with self.subTest(path=path):
                root = self.snapshot()
                self.change(root, audit.INDEX, lambda d: d["reports"][-1].update(path=path))
                with self.assertRaises(ValueError):
                    audit.verify(root)
        root = self.snapshot()
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        external = Path(tmp.name) / "inert.rs"
        external.write_bytes((root / audit.SOURCES[0]).read_bytes())
        p = root / audit.SOURCES[0]
        p.unlink()  # Synthetic fixture only; never a checkout source.
        p.symlink_to(external)
        with self.assertRaisesRegex(ValueError, "escape"):
            audit.verify(root)


    def rebind_manifest(self, root):
        m = audit.legacy_verifier(root)
        self.change(root, audit.INDEX, lambda d: d.update(supplemental_documents=[
            audit.record(m, root, audit.CONTRACT), audit.record(m, root, audit.MANIFEST_JSON)]))

    def test_manifest_input_changes(self):
        m = audit.legacy_verifier(ROOT)
        for item in audit.manifest_inventory(m, ROOT)["manifests"]:
            path = item["path"]
            with self.subTest(manifest=path):
                root = self.snapshot()
                p = root / path
                p.write_text(p.read_text() + "\n# synthetic change\n")
                with self.assertRaisesRegex(ValueError, "manifest metadata/fingerprint mismatch"):
                    audit.verify(root)

    def test_manifest_receipt_metadata(self):
        for key, value in [("sha256", "f" * 64), ("bytes", 1), ("lines", 1),
                           ("package", None), ("dependency_keys", {})]:
            with self.subTest(column=key):
                root = self.snapshot()
                self.change(root, audit.MANIFEST_JSON,
                            lambda d: d["manifests"][1].update({key: value}))
                self.rebind_manifest(root)
                with self.assertRaisesRegex(ValueError, "manifest metadata/fingerprint mismatch"):
                    audit.verify(root)
        for key, value in [("manifest_count", 10), ("schema_version", True),
                           ("assignment_outcome", "PARTIAL"), ("qualified_count", 1),
                           ("workspace_package_defaults", {"edition": "2099", "version": "9.9.9"})]:
            with self.subTest(metadata=key):
                root = self.snapshot()
                self.change(root, audit.MANIFEST_JSON, lambda d: d.update({key: value}))
                self.rebind_manifest(root)
                with self.assertRaises(ValueError):
                    audit.verify(root)

    def test_manifest_member_coverage(self):
        root = self.snapshot()
        extra = root / "crates/synthetic_extra/Cargo.toml"
        extra.parent.mkdir()
        extra.write_text('[package]\nname="synthetic-extra"\nversion="1.0.0"\nedition="2021"\n')
        with self.assertRaisesRegex(ValueError, "member manifest coverage mismatch"):
            audit.verify(root)
        root = self.snapshot()
        (root / "crates/genotype/Cargo.toml").unlink()  # Synthetic fixture only.
        with self.assertRaises((ValueError, OSError)):
            audit.verify(root)

    def test_manifest_inheritance_and_dependency_sections(self):
        root = self.snapshot()
        p = root / "Cargo.toml"
        p.write_text(p.read_text().replace('edition = "2021"', 'edition = "2024"')
                     .replace('version = "0.1.0"', 'version = "9.9.9"')
                     + '\n[workspace.dependencies]\nsynthetic_shared="1"\n')
        p = root / "crates/genotype/Cargo.toml"
        p.write_text(p.read_text() + '\n[build-dependencies]\nsynthetic_build="1"\n'
                     + '\n[target.\'cfg(unix)\'.dev-dependencies]\nsynthetic_target="1"\n')
        m = audit.legacy_verifier(root)
        doc = audit.manifest_inventory(m, root)
        item = next(x for x in doc["manifests"] if x["path"] == "crates/genotype/Cargo.toml")
        self.assertEqual(item["package"]["edition"]["effective"], "2024")
        self.assertEqual(item["package"]["version"]["effective"], "9.9.9")
        self.assertEqual(item["dependency_keys"]["build-dependencies"], ["synthetic_build"])
        self.assertEqual(item["target_dependency_keys"]["cfg(unix)"]["dev-dependencies"],
                         ["synthetic_target"])
        self.assertEqual(doc["manifests"][0]["workspace_dependency_keys"], ["synthetic_shared"])
        p.write_text(p.read_text().replace('version.workspace = true', 'version = "2.0.0"')
                     .replace('edition.workspace = true', 'edition = "2021"'))
        item = next(x for x in audit.manifest_inventory(m, root)["manifests"]
                    if x["path"] == "crates/genotype/Cargo.toml")
        self.assertEqual(item["package"]["version"], {"declared": "2.0.0", "effective": "2.0.0",
                                                    "origin": "package.version"})
        self.assertEqual(item["package"]["edition"]["origin"], "package.edition")
        with self.assertRaisesRegex(ValueError, "manifest metadata/fingerprint mismatch"):
            audit.verify(root)
        p.write_text(p.read_text().replace('edition = "2021"', 'edition.workspace = 1'))
        with self.assertRaisesRegex(ValueError, "unsupported edition/version declaration"):
            audit.manifest_inventory(m, root)


if __name__ == "__main__":
    unittest.main()
