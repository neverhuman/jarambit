# Portable report provenance index — V6 through V14

Task: `G16-12-RSV2-04-p18`. Baseline committed reports: `e78026a2af16b45b030776daa109eb3b28fef3a4`.

This index verifies document integrity and table structure only. It does not generate or evaluate geometry, interpret measured values, or certify scientific claims. The nine historical reports are unchanged.

## Report index

All report paths below use POSIX separators and are relative to the repository root. The verifier resolves them from its own location; no original checkout path, Git metadata, network, ignored `target/` contents, or third-party Python packages are needed. Absolute paths and artifact references inside the historical reports are preserved as document bytes; their availability and portability are outside this index.

|Version|Report path|SHA-256|Bytes|Tables|Table-schema SHA-256|
|---|---|---|---|---|---|
|V6|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V6_REPORT.md|fb27660b08e2c7c440b1e568651e7cef549ec0f81e9e7e8a2093a699c5919616|20583|14|7802fb6c5d741549afbc159fd1ef34e7e530b0cac32f352e62266fd581707bdb|
|V7|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V7_REPORT.md|b670e18da2b33d6472cb33e60986c18ecbb62b8c4dc0ec4b6d1f77715dfc8542|21419|16|c7901d997d0cc64d6b3023c1d98f54131ac0cbde268f95bb43f13c1224338e9c|
|V8|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V8_REPORT.md|7c6088ad4af07774d9e23f8733e90c25adf5b33a4142a08cce6b7a3824a2c5fc|20952|7|02ad7644e6ee7dc742380f48ed23291eb5fe0bbc077dacdc31ab927599711790|
|V9|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V9_REPORT.md|930bdb8767434f0b34fe4a5f70d5638c851b261ed793ee73510086e48a8fe570|17373|10|f3aa9c4c96e6c7dca3ba00fb36e3ece34df9ac46a0c70dc2b81462f6c69cf6b6|
|V10|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V10_REPORT.md|9183bbecbf3898c16f37a647866b922cce5070db7775224a99182ac42127fb15|31730|8|f1093f661df30a9961dd8bb9ae4034ffd277d52612fc37a448b7d81474b54b5b|
|V11|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V11_REPORT.md|91d60f4f64e89a2f221c7a588aaf7d359c4859333733c2c7117a796865500c64|31115|5|c26168458065c8479d62ca3cc9a8085ccefea5a0999f9d84cae2ca916526cb25|
|V12|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V12_REPORT.md|c44e4be496ac202e2943a16de02387cf9b16ddd13cfbc00463a1c32ebb77d7ae|24997|6|09a884778c9d9c518f0d5505fe7ce43efb7d98ce92ac0888b1b09cc0eaf52369|
|V13|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V13_REPORT.md|b3175d02f0415168c0e7e65ba88cce3181b5c8dc6f7f78d1c817bb3ea49afbe8|41531|5|1a3148ab57966afd1d266f74b08056ec2ee0a2a6bff300182646ad50e67307dd|
|V14|experiments/G16-12-RSV2-04/UNIFIED_PROTOTYPE_V14_REPORT.md|d3a897cc3914c760be4e66373ca3d6924744ac1d9350715d87015871b7c123ee|54507|31|f64088803efe8ec3dccbb86cb5bff46ee94f6aea95c2c208a0df61787829471a|

## Hash and schema rules

- SHA-256 hashes the exact file bytes, without newline or encoding normalization. Bytes records their length. Digests are 64 lowercase hexadecimal characters.
- Each pipe-delimited Markdown table must have nonempty header cells, a matching separator row (at least three hyphens per column, optional alignment colons), and a consistent width in every data row. Fenced code examples are ignored. Escaped pipes do not split cells.
- The schema is an ordered JSON array of objects with `header_line` (one-based), `headers` (trimmed header cell strings), and `rows` (data-row count). Serialize using Python `json.dumps(schema, ensure_ascii=False, sort_keys=True, separators=(",", ":"))`, encode UTF-8 without a trailing newline, then SHA-256 those bytes. This binds table order, header text, header position, widths and row counts. File SHA-256 independently binds every data cell.
- Exactly one row for each V6 through V14, in that order, is required. Missing, duplicate, reordered or unexpected entries fail. Indexed paths must match the assigned report names, remain inside the chosen root after symlink resolution, and contain no absolute prefix, backslash, drive/scheme colon, traversal/dot/empty component, whitespace, percent escape or fragment marker.

## Verification

From any copy of the repository root:

```sh
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 python3 -m unittest experiments/G16-12-RSV2-04/test_report_provenance.py
```

The tests read this fixed index; they never update expected digests. Besides verifying all nine reports, they exercise relocation without `.git` or `target`, report tampering, invalid digests/counts/schema digests, coverage errors, unsafe paths, symlink escape, malformed tables, and fenced/escaped-pipe parsing. Temporary fixtures contain document copies only.

SHA-256 detects differences relative to this index; it does not establish authorship or protect against coordinated replacement of both the reports and index. Review and version control provide that trust boundary. The index has no self-digest; its exact digest is recorded in the task handoff.
