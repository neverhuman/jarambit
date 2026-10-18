# Genotype/phenotype constant-contract audit: limited provenance receipt

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p26`. Baseline: `dd1e3b6`.

Assignment outcome: PARTIAL. Parameter/constant parity: NOT_ASSESSED. This receipt records exact file provenance and generic import/API observations only. The two sources encode weapon geometry; design-specific constant values, parameter transformations and parity validation are outside this receipt. No full constant-contract assessment is claimed.

The companion JSON is `docs/audit/genotype_phenotype_constant_contract_audit.json`. Its diagnostic status describes the receipt, not an assessed design. No production module was executed or modified, no geometry was generated, and no physical or optimization tests were run.

## Source fingerprints

SHA-256 hashes exact bytes; Bytes is the byte length and Lines is the UTF-8 decoded split-line count. File equality detects changes relative to these anchors; it does not establish semantic correctness, authorship or a passing physical result.

|Source path|SHA-256|Bytes|Lines|
|---|---|---|---|
|crates/genotype/src/lib.rs|f4cbf3c277c4b7aa7d2acce877984c82fe74c747f0764409f30b94197729db20|20657|355|
|crates/phenotype/src/lib.rs|25179aae64b9f8b5bfb454bc4f040b52cf6c92abc2fce4ca25fae519094f3f5d|17659|327|

## Generic contract observations

The phenotype imports the genotype `limits` module and `Genotype` type. Its `from_genotype` entry point accepts a borrowed `Genotype`. This establishes a source-level API dependency only. It does not establish that corresponding values agree, that every parameter is consumed, or that a transformation preserves any physical requirement. No parameter maps or numerical constant values are extracted here.

The JSON explicitly records `assignment_outcome: PARTIAL` and `parity_assessment: NOT_ASSESSED`. The verifier rejects a substituted completion/parity claim. The exact scope exclusions remain visible in both formats; successful provenance verification cannot turn an unassessed contract into a passing contract.

## Sixteen-report catalog

The requested JSON index and verifier did not exist at baseline. `docs/audit/audit_provenance_index.json` therefore bootstraps an ordered catalog from the existing fifteen Markdown reports in `experiments/G16-12-RSV2-04`, followed by this report as entry sixteen. The companion JSON is a supplemental document, not a seventeenth report. The historical fifteen-report Markdown index and verifier remain unchanged and continue to bind their original catalog.

Each report record binds repository-relative path, exact SHA-256, byte count, line count, table count, table-schema SHA-256 and diagnostic status/qualified_count. The supplemental JSON has its own exact fingerprint. The JSON record binds both source-file fingerprints, and the new verifier checks all these references without importing either production module. Earlier reports remain historical evidence; their inclusion asserts document identity only.

## Verification and limitations

`tools/verify_gate_feasibility_audit.py` uses the established document parser and strict JSON helpers from the historical verifier. It enforces exact ordered coverage, assigned paths, digest/count metadata, JSON diagnostic status and explicit unassessed parity. It rejects duplicate JSON keys, invalid numeric representations and literal or decoded host-home prefixes in the new JSON. Portable path resolution rejects unsafe paths and symlink escapes. Report/index/verifier text must retain zero literal host-home prefixes; the detector is constructed dynamically.

The index has no self-digest. Output digests are sealed in the ignored local task receipt. Coordinated replacement of data and trusted anchors is outside hash-based integrity checking. The checks validate document provenance and basic JSON shape only; they do not parse or validate a Rust semantic contract.

Verification runs on CPUs 10,11 using `nice -n 10` and `CARGO_BUILD_JOBS=2`: the new command, its synthetic corruption controls, the unchanged historical Python discovery suite, and `git diff --check`. Effective inherited nice is 19 on this host. Fixtures are inert document/source copies and synthetic JSON; they execute no production code. Historical reports and the archived JSON catalog remain unchanged. The requested weapon-specific parity assessment remains unperformed regardless of test outcomes. Changes are left uncommitted.
