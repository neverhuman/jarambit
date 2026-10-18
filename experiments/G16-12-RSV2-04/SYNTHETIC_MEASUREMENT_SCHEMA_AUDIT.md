# Static and synthetic measurement schema audit

status: IN_PROGRESS (diagnostic audit receipt; qualified_count=0)
qualified_count: 0

Task: `G16-12-RSV2-04-p20`. Baseline: `4df1a672b0b652eacccb808aa39c62e3ffb8c38d`.

This receipt covers static source inspection, existing document schemas, exact file digests, and synthetic non-weapon JSON fixtures. No production candidate optimization, design comparison, geometry generation, or physical evaluation is performed. No production source, historical JSON, or existing archive seal is changed. Phase21 updates the host-path policy and makes the single worktree reference in V12 portable, refreshing its index byte/hash entry; the original report/index bytes are preserved in the Phase21 receipt.

## Findings

The assignment names an interface that differs from this checkout. `point_profile` exists, but `edge_profile` and several requested fields do not. All five named root JSON files are absent. The actual recursive tracked inventory contains37 JSON files:36 object roots and1 array root. All37 parse strictly and match the existing Phase4 archive SHA-256 entries. **27 historical files contain cataloged legacy host-home prefixes**, with340 occurrences in decoded JSON keys/strings. The other10 tracked JSON files have zero occurrences.

Phase20 applied a blanket zero-prefix test and recorded20 passing tests/1 failure. Task `G16-12-RSV2-04-p21` explicitly revises that policy: the10 portable JSON files and six listed Phase15–20 Markdown reports must have zero occurrences; the27 legacy JSON files must match this exact per-file catalog, byte digests, shape digests and archive-seal entries. No new uncataloged host-path regression is allowed. Matching only the aggregate340 count is insufficient. No test is skipped or marked expected-failure. Diagnostic status stays IN_PROGRESS with qualified_count0; passing this revised provenance policy does not make legacy artifacts portable or establish semantic qualification.

## Static source and archive binding

|Repository-relative input|SHA-256|
|---|---|
|crates/crescent/src/certification.rs|eeefc8b1ecbf21df85aefb967e351ada4f0070f8b481959afef419379215c4ac|
|crates/crescent/tests/cert_integrity.rs|6fb98daf1e0eb4ebc44498bd5791aceceb58ccee66f7f1a3f5b804beaecd1917|
|experiments/G16-12-RSV2-04/phase4/SHA256SUMS|54f4bbfc5174708dc77e8fab356c5b1d6270cfedf52e5e4f8d6cad64d1f5160d|

## Exact current control flow

`point_profile(s, tip, bisector, core)` in `crates/crescent/src/certification.rs` returns an optional `PointProfile`. It first requests thickness at distances.25,1,2. Any unavailable query returns None. It calls `stations(s,tip,bisector,10,[.25,1,2,5])`: seed0 and10 plus required distances, add intersections of the projected sampling line with projected triangle edges, sort with `f64::total_cmp`, and deduplicate station distances within1e-10. Sampling is mesh-derived, not a fixed21-point grid. Every resulting station is queried; any missing thickness makes the whole profile unavailable.

Monotonicity is exactly `samples.windows(2).all(|w| w[1].1 >= w[0].1)`. There is no monotonicity allowance. The station-deduplication tolerance and triangle containment tolerances are geometric rules, not permission for a negative thickness step.

Set `level=.95*core`. If the first thickness already reaches it, `taper_mm=Some(0)`. Otherwise find the first upward level crossing and linearly interpolate its distance. No crossing yields `taper_mm=None` and a measured lower bound10, not an invented exact crossing. `pass` is the AND of the three required thickness predicates, strict monotonicity, and the taper lower-bound predicate. The existing code audit gives the full formulas; this receipt introduces no alternative measurement rule.

|Serialized PointProfile field|Declared Rust type|Meaning in current source|
|---|---|---|
|`pass`|`bool`|Conjunction of existing predicates|
|`t_0p25_mm`|`f64`|Required thickness query|
|`t_1p00_mm`|`f64`|Required thickness query|
|`t_2p00_mm`|`f64`|Required thickness query|
|`monotonic`|`bool`|Every adjacent thickness is nondecreasing|
|`taper_mm`|`Option<f64>`|First interpolated crossing or unavailable crossing|
|`taper_lower_bound_mm`|`f64`|Crossing distance, or10 when no crossing was observed|
|`samples`|`Vec<(f64,f64)>`|Returned mesh-derived distance/thickness pairs|

|Requested interface/field|Static finding|
|---|---|
|`point_profile`|Present; control flow described above|
|`tip_angle_deg`|Absent from certification source and PointProfile|
|`w_at_5mm`|Absent from certification source and PointProfile|
|`edge_profile`|No such function exists in certification source|
|`apex_t_p95`|Absent; no percentile value is inferred|
|`max_apex_mm`|Present in Sharpness, produced by sharpness; not an edge_profile field|
|`bevel_deg_p50`|Absent; no median value is inferred|

`Sharpness` currently serializes `pass`, `complete`, `max_apex_mm`, `max_half_angle_deg`, and `samples`. A maximum is not relabeled as a percentile or median. No missing function, schema field, or diagnostic observation is synthesized.

## Existing synthetic fixture review

The existing `brep_point_measures_intersections_and_rejects_reversals_after_five_mm` regression in `crates/crescent/tests/cert_integrity.rs` uses small rectangular strip meshes. Source inspection shows these assertions:

|Fixture|Existing assertion|
|---|---|
|`good`|Profile passes, monotonicity true, required query agrees with its fixture value|
|`bad`|Later reversal makes monotonicity and profile result false|
|`small_reversal`|Small interior reversal is rejected without a tolerance waiver|
|`early`|Combined profile result is false|

The Phase19 audit records the previous exact synthetic Rust execution. This task does not rerun production or geometry tests. New runtime tests operate only on synthetic JSON documents, document copies and byte digests; they do not call a geometric evaluator.

## Requested root artifact availability

|Requested filename|Tracked at baseline|Exists at baseline|
|---|---|---|
|diag_summary.json|False|False|
|gate_s_recheck_receipt.json|False|False|
|s1_root_cause_audit.json|False|False|
|top_candidates_gate_diag.json|False|False|
|unified_prototype_audit.json|False|False|

These missing files have no observed schema or digest. They are not created as empty substitutes. The inventory below recursively includes every actual tracked JSON, including archived inputs and receipts; limiting the audit to the absent root names would conceal the existing files.

## Complete tracked JSON inventory

Artifact paths are POSIX repository-relative paths. SHA-256 hashes exact bytes. JSON shape SHA-256 hashes canonical UTF-8 JSON (`ensure_ascii=False`, sorted keys, compact separators, no trailing newline) of the observed type shape: objects retain sorted keys recursively; arrays contain sorted unique element shapes; primitives distinguish null, boolean, integer, finite number and string. Empty arrays have an empty element-shape set. This describes observed structure, not conformance to an unavailable formal measurement schema.

Host-prefix counts inspect decoded string values and object keys recursively, so escaped spellings cannot evade detection. This receipt itself contains no forbidden host-home prefix. Every file digest is additionally checked against the unchanged `phase4/SHA256SUMS` archive seal.

|Artifact path|Bytes|SHA-256|JSON shape SHA-256|Host-prefix occurrences|Root type|
|---|---|---|---|---|---|
|experiments/G16-12-RSV2-04/phase4/ARCHIVE_PROVENANCE.json|1015|39596f7ac9769902412107a345569d7a2ead800c35f21eed4c8b9f6dea991c14|b744041c4b21d76fb90f4b0c3fe561cb80a8aa58709da367058bcb2cb6f78c2c|1|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_config.json|645|f4176e593e8773b900a863c74ebd7caa15a5dfcbf17f6462fcaacc8ced4c6a0c|a0eaa53a9ce447f6748ec983e223965d1f9904a94454d38fd316cd92e3d15aea|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d048_export.json|67353|4fbeeb070144c3e59acda370ca60e3b6b5e52739a348168e362f234a724df7b7|9fde194fcf9df0db4daac4154721cf05bce111a98d5cce2d91f2f4ee5aac149b|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d048_export_invocation.json|1117|81d04a975098fa5c04f38ee0df961a46664ca60edc4e1c4d78b9f6bf5f1e0831|e9f4ca94c905770f51c3cc0d913b58035d4387a86c22887a7eba0d262e7715cc|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d048_fem.json|28801|144d6839cce1d6c062290e0af3c5998c50fb85b46eb7f0f894edc3c4fbf7e882|3091d82f5bad21a2a9b3380376b6dfe0df4a6dafb70ddbf592b3c971cb91c36f|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d048_fem_invocation.json|1111|aa9769ec6e1e136e9bc41fc01cc1df9f677b4d7f1ce65fbf4eb35ab223e7d366|e9f4ca94c905770f51c3cc0d913b58035d4387a86c22887a7eba0d262e7715cc|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d055_export.json|67707|2ada295a03dc7dc47cd0cff223f04295894029bda97866da6fb26ecca481dbf7|9fde194fcf9df0db4daac4154721cf05bce111a98d5cce2d91f2f4ee5aac149b|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d055_export_invocation.json|1117|fe2b98fba5b87140127c07a0c2b592fa8807d4bd2aaf3227cdf10a7706703d6e|e9f4ca94c905770f51c3cc0d913b58035d4387a86c22887a7eba0d262e7715cc|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d055_fem.json|28789|28d35dc03d8fe101457fd4a581b7ec2c46a7c9544f214f1e21a940c7534a3161|3091d82f5bad21a2a9b3380376b6dfe0df4a6dafb70ddbf592b3c971cb91c36f|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d055_fem_invocation.json|1111|1d56b8328cdedfd9b8970d7d0e5fdb7f131e0d2dae3b75279bd008f7fef68962|e9f4ca94c905770f51c3cc0d913b58035d4387a86c22887a7eba0d262e7715cc|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d065_export.json|69180|98d05a99fe70d144c9ed11e2c8643a764558a41159c594321d9a7a5e4278189d|9fde194fcf9df0db4daac4154721cf05bce111a98d5cce2d91f2f4ee5aac149b|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d065_export_invocation.json|1117|fc2000a708c443922afb72a4b19e31881e5668c1de714b7d3097115f08e83b3b|e9f4ca94c905770f51c3cc0d913b58035d4387a86c22887a7eba0d262e7715cc|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d065_fem.json|28803|b63566cd8517642243ee584e1e7566b7d578232b7b119b5f0e6ea0cfbb88a49b|3091d82f5bad21a2a9b3380376b6dfe0df4a6dafb70ddbf592b3c971cb91c36f|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d065_fem_invocation.json|1111|6f55a34817d750a5982e378149570228ad13a2a6d311bbf5a1d6e1e68cad8567|e9f4ca94c905770f51c3cc0d913b58035d4387a86c22887a7eba0d262e7715cc|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d065_h025_export.json|79459|57300842a602466b881dfc7d69b3216a7af85462e32cce0a85a12059faf27484|9fde194fcf9df0db4daac4154721cf05bce111a98d5cce2d91f2f4ee5aac149b|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/combined_d065_h025_export_invocation.json|1123|a1c490fc3ed7fc13c678187744f8c828b9a4da217bdbcc3bba5319e49e2a3bb8|e9f4ca94c905770f51c3cc0d913b58035d4387a86c22887a7eba0d262e7715cc|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/control_verification.json|353|8e045b86bc35731a410b37e014b514fd162c7bad1df7686953becda9ad7eec26|92a04bd4060c9d66fea3017e858339cb92a5c0325eb47c014d0f4cfca94b079e|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/fem_summary.json|1400|d3ab60ad2f4fed7ea24821276b957734e648b15f008f20cd039f6227e5a420b2|a40f1f91455b6bd05a005c5f55ad14b32caa4e2bb3c15e9afb28e711e1c773f1|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/hotspot_control_fem.json|20530|0913855f0c1a3a47551878328546ad741b7688c0e04a1c669bef397e132ba06b|3091d82f5bad21a2a9b3380376b6dfe0df4a6dafb70ddbf592b3c971cb91c36f|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/hotspot_control_fem_invocation.json|737|0f0e1d01fc12057b3fb8f0bf018bcbe450c5ab378ae32ade36d93a8cd18d1aa3|c829d0fe598b7f60f83645f3148cdcc8ece87bfd3d5658dcf0c63e2231c93514|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/input_provenance.json|5019|8c1c609ba90a09fe1461667b388946ea8082e4d26431d3a93da09c97f4c00056|7dfa443bf1c60d96b366322137d8d3f1dd7c81ae6af81e8411800791193e346b|19|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/inputs/hotspot/target/rsv2-07/v3/hotspot_strict_3/invocation.json|731|9a8bde9f68fd1756f33381812cd2d5c084d3f9311d73e16164b3786862ac6afe|347d1322bf38accc51033501afe42f5c9c14de03cda94beba28c034200c6366e|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/inputs/hotspot/target/rsv2-07/v3/hotspot_strict_3/result.json|4643|cdb2eece4e15c228003eedc8b8ed499e59706e9980fde3c31dce8db4392b39d2|a457213b5c89ce43bb01efe8f18e9d2823fd83408b35ca261d4ba40dfa9b9835|1|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/inputs/hotspot/target/rsv2-07/v3/hotspot_strict_summary.json|18766|7822ea1bed2de87a2f7ae06126f51fac6db9ad5eb1ca98088a3f723b6f84dbe4|8da4e71ca50af2699e1f6ef7a82f47bf07a2bc05f76c342081e0f4e052b7c3f9|0|array|
|experiments/G16-12-RSV2-04/phase4/unified_study/inputs/neck/experiments/G16-06-RSV2-08/candidate_config.json|618|f3d5921b90dfe712940fbd4fd98346aa78be985fbdaf8f11f874cad7934eebb9|9b081cf6cbb23c2b56ef9bfb48b7840593f8134b1966e41fc0a3175a92b6bf32|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/inputs/neck/target/G16-06-RSV2-08/candidate_validation.json|70330|532a5a16d4da589dad6f99285a6d7bff11c5d4fe99656565889d6b6dc4404957|4fa9d10d38664ad12bdbd3a8ccf2f5a517cfde9258fcf6f04bc47bfca595c296|34|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/inputs/neck/target/G16-06-RSV2-08/generated/source_provenance.json|8805|21b658437ff7bb51c0294c15571c4dc5287dc36bc9fc71d6123def2cd792f7aa|b87511335a902cee465945860f61d5661a4477c5fc520028068470d823fc43e1|33|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/inputs/neck/target/G16-06-RSV2-08/receipt.json|2287|9426a76359e8e60c280749851371962a286c5cb9f99fbb4d585d74786f70c2c3|4d32649ab0553194360c8a5094c5e88e79119094b97d1969bdf2a19d311efb3e|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/neck_control_fem.json|28835|d496e7b53594ec74eb0d082bd266f4150213048c9f611dacec4f57ffd9694284|3091d82f5bad21a2a9b3380376b6dfe0df4a6dafb70ddbf592b3c971cb91c36f|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/neck_control_fem_invocation.json|1082|bf324897bad6e1ec80b67c432d070c7193abb319b0b6ecd441da96d2e8f8ec82|edc4ea887185b1513e16c278b83412784ecafc3a286a154e5e844c35e171ba7e|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/neck_skyline_partial.json|25946|60955ef7d459d13e719bac7073914472f16067c12db88d19cdbbbace6aa67e9e|0de58bb2a21fdb096dee12927fe4b9133f65823caaf65ac0c7f3fe7d5410d9b5|20|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/neck_skyline_partial_invocation.json|1085|62749e1f1fd2020500a7e9d1193f90d401f0d10bb33e3bd83e879739b68439aa|edc4ea887185b1513e16c278b83412784ecafc3a286a154e5e844c35e171ba7e|3|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/run_provenance.json|2287|0ed10cdad0a4139bb81410917a8bdb7a7758c23be971b8bce1097fb45e6d354a|2850677bb18adfbe3f1bbb8ed79b4729c0011d147524c049a9ae54e491975da5|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/solver_import_note.json|750|9337823dd968f2abfa033dd94d82bbfcdede57bb563cf69f36c4040c5105ae55|beed9d17e775dc2def3d9e5be7ebac9ff107d9ed6decbb32f66b87ff0b74f2ca|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/source_provenance.json|13631|fd69d454b49dd503ef6b773cc9e5cf4c090ba7d5c086a3792c9959d3e1929315|9377dad31eba4ddb3fc2fd279e680c73da24c658b61c886b7f76c9d407540654|19|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/summary.json|216744|3de8847512a084e96c63e6bfaa94692fd6b13e3aae13a194c8235470e2160c7c|e31bda5ee1e7f9bdcba50a7bf3c5fae1707ce62d37fefe1d62ead8e2e6e9e5a3|0|object|
|experiments/G16-12-RSV2-04/phase4/unified_study/verification.json|1216|f86256f51f67cdf212a017fbfd22cfa7ee3ffd93c07fc91abd23fe56757ecc7f|359f35b56217e2b49a9078a5d9d5b74b05d506300fc2d51555c6ae812af3ca5d|0|object|

## Verification and limits

The extended portable verifier anchors this receipt's SHA-256, bytes, table count and table-schema digest, validates every indexed JSON strictly, checks its byte/shape digests and archive-seal entry, and checks exact tracked inventory coverage. With Git metadata it uses `git ls-files`; in a relocated document fixture without Git it discovers the copied JSON recursively. No network, original checkout path or ignored build output is required.

JSON decoding rejects duplicate keys, NaN/Infinity and numeric overflow to a non-finite value. Synthetic JSON tests cover valid simple records, ambiguous/invalid inputs, boolean-versus-integer shape distinction, and a Unicode-escaped forbidden prefix. Tampering and relocated-copy controls verify this receipt and archived JSON. The Phase21 policy test requires zero occurrences in the10 cataloged portable JSON files and all six listed Markdown reports. For the27 legacy JSON files, it requires exact per-file catalog consistency, including340 total historical occurrences. A changed legacy file, newly contaminated portable JSON, any contaminated listed Markdown report, or an uncataloged tracked JSON fails validation. Regression subtests inject synthetic host paths into both JSON classes and each of the six Markdown reports; the suite retains21 unittest methods.

The zero-occurrence Markdown scope is `UNIFIED_PROTOTYPE_V12_REPORT.md`, `UNIFIED_PROTOTYPE_V13_REPORT.md`, `UNIFIED_PROTOTYPE_V14_REPORT.md`, `UNIFIED_PROTOTYPE_PROVENANCE_INDEX.md`, `CERTIFICATION_MEASUREMENT_CODE_AUDIT.md`, and `SYNTHETIC_MEASUREMENT_SCHEMA_AUDIT.md`. The Phase21 change touches only provenance policy/tests and portable report wording; all37 JSON byte digests and the catalog above remain unchanged.

Phase22 extends the zero-literal host-prefix policy to all12 indexed Markdown reports (V5 throughV14 and both audit receipts), the provenance index, and the verifier source. The verifier constructs its detector dynamically. Index rows record exact line/byte sizes, SHA-256, table schema and diagnostic IN_PROGRESS/qualified_count0 for every report; these provenance labels do not revise historical scientific conclusions. Earlier host-specific display references are normalized with original bytes retained in the Phase22 receipt; all37 archived JSON and their catalog remain unchanged.

Run the requested checks from a repository copy:

```sh
nice -n 10 taskset -c 10,11 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/G16-12-RSV2-04 -p 'test_*.py' -v
git diff --check
```

Execution uses CPUs10,11 and jobs2 with the requested niceness increment; inherited effective niceness is19. Results are retained in the task handoff and ignored receipt directory. The historical blanket portability failure remains an audit finding; passing the revised policy is not a successful qualification. No field-value physics, candidate comparison, external embedded artifact reference, or missing diagnostic file is evaluated or invented. Hash checks establish consistency relative to trusted repository anchors, not authorship or physical validity.
