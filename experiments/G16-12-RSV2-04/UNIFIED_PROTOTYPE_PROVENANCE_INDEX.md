# Portable diagnostic report provenance index — V5 through V14 and audits

status: IN_PROGRESS
qualified_count: 0

Task: `G16-12-RSV2-04-p24`. Baseline: `fbc1596`; earlier index tasks: Phase18, Phase21 and Phase22.

This index covers exactly14 diagnostic reports: V5 throughV14 plus the certification-code, synthetic-schema, static gate/FEM invariant, and static Stage-1/Stage-2 pipeline audit receipts. It verifies document bytes, line counts, table structure and portable paths; it does not generate or evaluate geometry or certify physical claims. IN_PROGRESS and qualified_count0 apply to the provenance receipt for each row, including historical reports without their own metadata declarations. They do not rewrite historical numerical results or claim qualification.

Phase21 normalized V12’s worktree reference. Phase22 normalizes remaining host-specific display references in V5,V6,V8,V9,V10,V11, retaining original bytes for all12 reports/index/verifier in its receipt. No numerical table or archived JSON is changed. V8’s external mirror destination is a portable display label, with the original destination preserved.

## Report index

Paths use POSIX separators relative to the repository root. All indexed Markdown, this index, and the verifier source have zero literal host-home prefixes; the runtime detector is constructed dynamically. Historical artifact availability outside these indexed documents is not inferred.

|Report|Report path|SHA-256|Lines|Bytes|Tables|Table-schema SHA-256|Status|qualified_count|
|---|---|---|---|---|---|---|---|---|
|V5|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V5_REPORT.md|7c252aa9c6176c49a2606fd71c5bf848081554450d190f1193693b5d7a203351|134|15130|9|294819df23ad64010c603cc760a26890d6a386fe58959aed62a283cce09e4c9b|IN_PROGRESS|0|
|V6|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V6_REPORT.md|8877256d3fd2159b9f65bf07af9fd133c7d25cee46ee349da8c2ff2d9f127ca4|196|20555|14|7802fb6c5d741549afbc159fd1ef34e7e530b0cac32f352e62266fd581707bdb|IN_PROGRESS|0|
|V7|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V7_REPORT.md|b670e18da2b33d6472cb33e60986c18ecbb62b8c4dc0ec4b6d1f77715dfc8542|223|21419|16|c7901d997d0cc64d6b3023c1d98f54131ac0cbde268f95bb43f13c1224338e9c|IN_PROGRESS|0|
|V8|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V8_REPORT.md|32ffe2022406b5e5a4f5e64075ab1dfa01861fe2629cd99c004b21934c6eebb9|187|21073|7|02ad7644e6ee7dc742380f48ed23291eb5fe0bbc077dacdc31ab927599711790|IN_PROGRESS|0|
|V9|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V9_REPORT.md|0349e72721832000535671dbc1803286237ae5932c2ff5982275e9aa737e8b53|162|17345|10|f3aa9c4c96e6c7dca3ba00fb36e3ece34df9ac46a0c70dc2b81462f6c69cf6b6|IN_PROGRESS|0|
|V10|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V10_REPORT.md|6513ade301bec20a793923d77db4f4e9b514339bdff15287553c9b19a6ee198a|248|31702|8|f1093f661df30a9961dd8bb9ae4034ffd277d52612fc37a448b7d81474b54b5b|IN_PROGRESS|0|
|V11|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V11_REPORT.md|eeb87c5707f475eff32d2827cb82e6fc8f32b614df3866e05c47346c9cf0ff18|214|31087|5|c26168458065c8479d62ca3cc9a8085ccefea5a0999f9d84cae2ca916526cb25|IN_PROGRESS|0|
|V12|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V12_REPORT.md|d3d6a70a743afbb9ed4cc8069dea9311f45a08cd096a880213aec4c5931e4b00|188|24987|6|09a884778c9d9c518f0d5505fe7ce43efb7d98ce92ac0888b1b09cc0eaf52369|IN_PROGRESS|0|
|V13|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V13_REPORT.md|b3175d02f0415168c0e7e65ba88cce3181b5c8dc6f7f78d1c817bb3ea49afbe8|313|41531|5|1a3148ab57966afd1d266f74b08056ec2ee0a2a6bff300182646ad50e67307dd|IN_PROGRESS|0|
|V14|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V14_REPORT.md|d3a897cc3914c760be4e66373ca3d6924744ac1d9350715d87015871b7c123ee|1062|54507|31|f64088803efe8ec3dccbb86cb5bff46ee94f6aea95c2c208a0df61787829471a|IN_PROGRESS|0|
|CODE_AUDIT|experiments/G16-12-RSV2-04/CERTIFICATION_MEASUREMENT_CODE_AUDIT.md|1409141a8d7c7174245b8ba9b8b901397d11225d7602f56497ccb5ee27230b3a|168|15527|6|42212ab2618b5be7937ca24f7b0662d57de18a7602d75acecb7a12124c8b26df|IN_PROGRESS|0|
|SCHEMA_AUDIT|experiments/G16-12-RSV2-04/SYNTHETIC_MEASUREMENT_SCHEMA_AUDIT.md|29d2ef4a6910a4fc6346f50ca7fb32b61c18e6958d5bea642ceddfc1cd0e8f60|143|19029|6|3da8be6624495207947af638fd701ad831223ef6d11d3945aa4f8ab0f5c5edd8|IN_PROGRESS|0|
|INVARIANT_AUDIT|experiments/G16-12-RSV2-04/STATIC_GATES_AND_FEM_INVARIANT_AUDIT.md|f729f23b66ae4bb022a7b8ce84f16b9115ccef5a7761132a4610fdb1d56f3892|84|9191|4|8070860f447c896ab098ebc07811d607be8a1738765e64d27cf3e21ed2309482|IN_PROGRESS|0|
|PIPELINE_AUDIT|experiments/G16-12-RSV2-04/STATIC_STAGE1_STAGE2_PIPELINE_AUDIT.md|96e8d433091642d28500760f4470078051c0a33d5473f4b8ac7994139adbb531|71|10424|3|2d3ab14bd97f3ec6a1f802b3c7ff901a564c94da8f2c7a6d1179a5ab2e7860cd|IN_PROGRESS|0|

## Hash, size and schema rules

- SHA-256 hashes exact bytes without newline or encoding normalization. Bytes is their length; Lines is Python `len(text.splitlines())` after UTF-8 decoding. Digests use64 lowercase hexadecimal characters.
- Each report table requires nonempty header cells, a matching separator row (three or more hyphens per column with optional alignment colons), and consistent data-row widths. Fenced code is ignored; escaped pipes do not split cells.
- Table schema is an ordered JSON array of objects containing one-based `header_line`, trimmed `headers`, and data-row count `rows`. Hash the UTF-8 serialization from `json.dumps(schema, ensure_ascii=False, sort_keys=True, separators=(",", ":"))`, without a trailing newline. This binds table order, positions, headers, widths and row counts; the file digest binds every cell.
- Exactly one ordered row for V5 throughV14, CODE_AUDIT, SCHEMA_AUDIT, INVARIANT_AUDIT and PIPELINE_AUDIT is required. Missing, duplicate, reordered or unexpected entries fail. Every row must declare Status `IN_PROGRESS` and `qualified_count` `0`.
- Indexed paths must match assigned filenames, use repository-relative components, and stay inside the chosen root after symlink resolution. Absolute prefixes, traversal/dot/empty components, backslashes, drive/scheme colons, whitespace, percent escapes and fragments are rejected.

## Verification

From a repository copy:

```sh
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/G16-12-RSV2-04 -p 'test_*.py' -v
git diff --check
```

The tests read fixed reviewed anchors and never refresh them. All14 report files have tampering subtests; index tests exercise missing/duplicate/reordered rows, invalid SHA/line/byte/table counts and schema digests, non-diagnostic status, nonzero qualification, and unsafe paths. Relocated document copies require no original checkout path, ignored build output or network. Source and archival JSON checks are separately retained.

Portable JSON must retain zero decoded host-prefix occurrences. Legacy JSON remains bound to its exact per-file catalog, byte/type-shape digests and archive seal (27 files/340 existing occurrences), with no uncataloged regression. Synthetic host-path injection checks cover every indexed Markdown report, this index, and the verifier source itself. Test fixtures are document copies and synthetic JSON only.

SHA-256 detects differences relative to trusted repository anchors; it does not establish authorship or protect against coordinated replacement of both files and anchors. This index has no self-digest; its exact digest is retained in the task handoff.
