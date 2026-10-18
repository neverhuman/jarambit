# Static workspace manifest provenance audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p27`. Baseline: `9fc1ff9`. Assignment outcome: COMPLETE for the static manifest metadata audit.

This receipt inventories the root `Cargo.toml` and every `crates/*/Cargo.toml` file as local TOML data. It records exact SHA-256, byte/UTF-8 split-line counts, edition/version declarations and inheritance, workspace member coverage, and dependency keys. No dependency resolution, downloads, Cargo metadata/build/test commands, build scripts, production source execution or geometry evaluation were performed. The companion JSON is `docs/audit/workspace_manifest_provenance_audit.json`.

## Workspace and package metadata

The root is a virtual workspace: it has a `[workspace]` table and no `[package]` table. Its resolver is `2`; `[workspace.package]` supplies version `0.1.0` and edition `2021`. All ten crate manifests declare `version.workspace = true` and `edition.workspace = true`, so their effective values are the shared defaults. Declared inheritance and effective values are both preserved in JSON, together with their origin paths.

The workspace lists ten explicit members. Their manifest paths match the ten sorted `crates/*/Cargo.toml` paths exactly, without missing, duplicate or extra crate manifests. The JSON preserves declared member order; manifest records use root first, then sorted crate paths. The verifier requires this explicit-member coverage for this checkout. It is a focused audit of this layout, not a general Cargo glob/exclusion resolver.

|Manifest path|Package|Version|Edition|Version/edition origin|
|---|---|---|---|---|
|Cargo.toml|(virtual workspace)|0.1.0 (default)|2021 (default)|workspace defaults; no root package|
|crates/cli/Cargo.toml|jarambit|0.1.0|2021|workspace.package|
|crates/crescent/Cargo.toml|jarambit-crescent|0.1.0|2021|workspace.package|
|crates/docs/Cargo.toml|jarambit-docs|0.1.0|2021|workspace.package|
|crates/evidence/Cargo.toml|jarambit-evidence|0.1.0|2021|workspace.package|
|crates/evolve/Cargo.toml|jarambit-evolve|0.1.0|2021|workspace.package|
|crates/genotype/Cargo.toml|jarambit-genotype|0.1.0|2021|workspace.package|
|crates/monte_carlo/Cargo.toml|jarambit-monte-carlo|0.1.0|2021|workspace.package|
|crates/phenotype/Cargo.toml|jarambit-phenotype|0.1.0|2021|workspace.package|
|crates/physics/Cargo.toml|jarambit-physics|0.1.0|2021|workspace.package|
|crates/redteam/Cargo.toml|jarambit-redteam|0.1.0|2021|workspace.package|

## Exact file fingerprints

SHA-256 hashes exact bytes with no normalization. Bytes is their length; Lines is `len(text.splitlines())` after UTF-8 decoding. Hashes bind the reviewed inputs; they do not establish publisher authenticity or dependency safety.

|Manifest path|SHA-256|Bytes|Lines|
|---|---|---|---|
|Cargo.toml|aa338525eac815da0ddf6557d150c0932f10c170985e9e7ff1cef1b4bcc50a1e|347|13|
|crates/cli/Cargo.toml|be4282631bd8747ad295b497fcb5fedde6088967488d7f1fd7aa1c011fbe8f74|590|18|
|crates/crescent/Cargo.toml|07970e5be1e735f13fcc6a73bc47da51015405a82e9e26b301733875ccf68556|497|18|
|crates/docs/Cargo.toml|43fb035ddd464a871c42bb84d86c66f5fb9b7b6f77f4b12bbe89c8f05f43cc94|125|8|
|crates/evidence/Cargo.toml|e26a3c2d78bd94d147785187d5e69fb3ce93b68b045f7b306a3230b8698aded2|296|12|
|crates/evolve/Cargo.toml|0dcb86df2ec5963c9e40687427126c368521c559b6f015667b1c57897d0db866|451|17|
|crates/genotype/Cargo.toml|5c95bb1aeb4871de5e23479fb4e420224d499d1d826fde033100b3efdb48da27|129|8|
|crates/monte_carlo/Cargo.toml|aad883b07f303ccd98812c0d51e8e6f877b374594cd5ba4afaaa3f4c020e2137|176|8|
|crates/phenotype/Cargo.toml|2662eac30c48eee8d638a9f86a6758f2eafd4e6aae2b368bdfd6365d4cc7af71|174|8|
|crates/physics/Cargo.toml|ea2d9a0e999c279e168d48df3825315157131399e48f31c16b35d4061caf4fff|471|18|
|crates/redteam/Cargo.toml|4708f2ca769c6978729d4583974f8d6f4b121bbee8f58aa908c5e14eea13063c|290|12|

## Dependency key inventory

Keys are sorted within each table. The JSON distinguishes normal, development and build dependency tables, target-specific dependency tables, and workspace dependency keys. Empty lists mean the corresponding table is absent or empty, not that transitive dependencies are absent. There are no build, target-specific or workspace dependency keys in these eleven manifests. The table below lists normal and development keys; crate manifests may still depend transitively on other packages.

|Manifest path|Dependency keys|Dev-dependency keys|
|---|---|---|
|Cargo.toml|(none)|(none)|
|crates/cli/Cargo.toml|jarambit-crescent, jarambit-docs, jarambit-evidence, jarambit-evolve, jarambit-genotype, jarambit-monte-carlo, jarambit-phenotype, jarambit-physics, jarambit-redteam, mimalloc, serde_json|(none)|
|crates/crescent/Cargo.toml|jarambit-evidence, jarambit-genotype, jarambit-phenotype, jarambit-physics, jarambit-redteam, serde, serde_json|jarambit-physics, serde_json|
|crates/docs/Cargo.toml|(none)|(none)|
|crates/evidence/Cargo.toml|serde, serde_json, sha2, unicode-normalization, zstd|(none)|
|crates/evolve/Cargo.toml|jarambit-evidence, jarambit-genotype, jarambit-monte-carlo, jarambit-phenotype, jarambit-physics, jarambit-redteam, sha2|serde_json|
|crates/genotype/Cargo.toml|(none)|(none)|
|crates/monte_carlo/Cargo.toml|jarambit-genotype|(none)|
|crates/phenotype/Cargo.toml|jarambit-genotype|(none)|
|crates/physics/Cargo.toml|faer, jarambit-genotype, jarambit-phenotype, spade|criterion|
|crates/redteam/Cargo.toml|jarambit-genotype, jarambit-phenotype|jarambit-monte-carlo|

Dependency keys are manifest-level labels, including potential aliases. This inventory does not infer resolved crate versions, lockfile identities, selected features, platform activation, registry contents, vulnerabilities, dependency-source trust or successful compilation. A version range in a dependency declaration is not treated as a resolved package version. Full manifest bytes remain digest-bound even though dependency values are not expanded into this metadata inventory.

## Provenance and verification

The JSON provenance index now contains seventeen ordered Markdown reports: the original fifteen-report historical catalog, the unchanged Phase26 receipt, then this report. Both Phase26 and Phase27 JSON receipts are supplemental documents, not additional reports. Index rows bind report path, exact SHA-256, bytes, lines, table count/schema digest and diagnostic status/qualification. This audit's completed assignment status concerns static metadata only; the diagnostic receipt remains IN_PROGRESS with qualified_count zero. Phase26's PARTIAL/NOT_ASSESSED constant-parity scope is preserved unchanged.

The verifier reparses local TOML using Python's standard-library `tomllib`, reconstructs manifest metadata and compares it with the JSON receipt, in addition to checking document fingerprints and the existing historical catalog. New, missing or changed manifest files, stale edition/version inheritance, and changed dependency keys fail verification. Strict JSON parsing, portable paths, zero literal host-home prefixes, diagnostic flags and historical source fingerprints remain checked. Replacing both an input and its trusted anchors is outside the protection of these hashes; the index has no self-digest, and exact output digests are sealed separately in the local task receipt.

Synthetic tests exercise metadata corruption even after the supplemental JSON digest is deliberately rebound, plus changed/new/missing TOML fixtures, inherited/default changes, direct declarations, target/build/workspace dependency key capture, and relocation without Git or build output. Fixtures are inert files only. The required verifier, Python suites and whitespace checks run on CPUs 10,11 with `nice -n 10` and `CARGO_BUILD_JOBS=2`; inherited effective nice is 19. Existing reports, manifest inputs, source files and archived JSON remain unchanged. Changes are left uncommitted and LANE is emitted after verification.
