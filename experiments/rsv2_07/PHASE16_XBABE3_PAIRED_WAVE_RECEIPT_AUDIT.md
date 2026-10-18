# Phase-16 xbabe3 paired-wave receipt audit

**Phase-16 is partially completed. PASS applies to saved artifact integrity, Stage-1/Stage-2 accounting, candidate references and Markdown qualification metadata.** Paired c3/t3 structural performance reporting/comparison is not performed. The verifier does not assess weapon-design performance, derive mechanical statistics or recompute physical qualification. Structural metric cells and narrative claims remain opaque text/bytes protected by their file hashes.

All **63 requested saved files** match their pinned SHA-256 digests and byte sizes: seven `leaderboard.json`, seven `leaderboard.md`, seven `RESULT.md` and 42 birth/screen JSON files across three rounds per wave. All 63 available files were read upfront and reread after validation; the round-file inventory is checked before and after. All seven required waves must be present.

## Accounting results

| Wave | Birth records | Pending screen records | Assessed rows | Qualified | Separate unverified attempts | Champion |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| w405c3 | 148 | 129 | 6 | 0 | 0 | null |
| w406c3 | 148 | 127 | 6 | 0 | 0 | null |
| w406t3 | 148 | 128 | 6 | 0 | 0 | null |
| w407c3 | 153 | 52 | 5 | 0 | 1 | null |
| w407t3 | 153 | 52 | 6 | 0 | 0 | null |
| w408c3 | 151 | 72 | 6 | 0 | 0 | null |
| w408t3 | 151 | 72 | 6 | 0 | 0 | null |

The totals are **1,052 birth records, 632 pending screens, 41 assessed rows, zero qualified rows and one separate unverified attempt**. These are record/event counts rather than counts of distinct cross-wave identities. Every assessed `qualified` flag is the Boolean false, every `qualified_count` is integer zero, and every `champion` is null.

`w407c3` has five assessed rows and one separate unverified attempt, `w407c3_r02_p14_c20`, with saved identity `f36332f32466e66386ea1d7a14d69c9910dc1356739dc1712447d155850d343c`. Its saved reason reports an independent evaluator failure with exit status 1. This audit preserves that error without rerunning or diagnosing its evaluator. It does not count the attempt as assessed or qualified. The error's label and identity must link to a saved pending screen, and `RESULT.md` must retain the matching error entry and unverified count.

The saved wave membership is three c3/t3 file pairs, `w406c3/w406t3`, `w407c3/w407t3`, `w408c3/w408t3`, plus standalone `w405c3`. Both members of every pair are required. Pair membership is an accounting check; treatment effects, physical margins and structural performance differences are not calculated.

## Saved Stage-1 rounds

| Wave | Zero-based round | Birth records | Pending screen records |
| --- | ---: | ---: | ---: |
| w405c3 | 00 | 52 | 34 |
| w405c3 | 01 | 48 | 48 |
| w405c3 | 02 | 48 | 47 |
| w406c3 | 00 | 52 | 33 |
| w406c3 | 01 | 48 | 46 |
| w406c3 | 02 | 48 | 48 |
| w406t3 | 00 | 52 | 33 |
| w406t3 | 01 | 48 | 48 |
| w406t3 | 02 | 48 | 47 |
| w407c3 | 00 | 69 | 12 |
| w407c3 | 01 | 48 | 4 |
| w407c3 | 02 | 36 | 36 |
| w407t3 | 00 | 69 | 12 |
| w407t3 | 01 | 48 | 4 |
| w407t3 | 02 | 36 | 36 |
| w408c3 | 00 | 55 | 9 |
| w408c3 | 01 | 48 | 16 |
| w408c3 | 02 | 48 | 47 |
| w408t3 | 00 | 55 | 9 |
| w408t3 | 01 | 48 | 15 |
| w408t3 | 02 | 48 | 48 |

Each round checks birth provenance fields, integer round/seed values, CSV header and row width, repeated-identity bindings and the same-round screen references. Screen labels must name the expected wave, one-based round, family and exact global birth-array slot. Duplicate screen labels/slots and non-pending Stage-1 status fail. Screen membership and finite numeric field types are checked; no thresholds or physical pass/fail decisions are derived.

Each assessed JSON row must reference the matching saved screen label, family and certificate identity. Its required fields/types, finite numeric observations, nonnegative battery count, false qualification flag and internally noncontradictory stored Boolean gate flags are checked. Saved genome and scorecard references remain strings: those external files are not followed or independently verified. Certificate identities are opaque references; no native identity or certificate is regenerated.

## Markdown and byte checks

`leaderboard.md` and `RESULT.md` must contain the expected table headers, exact assessed row counts, the same candidate labels/order as JSON and matching saved qualification flags. `RESULT.md` must report the exact JSON assessed, qualified and unverified counts in its source line and assessed/qualified counts in its summary line. Its unverified error list must match JSON. Numeric performance table cells, best-candidate prose and pairwise structural claims are not interpreted or validated for physical correctness.

Hashes and byte sizes pin all requested document bytes, including opaque performance prose. File tampering, malformed digest/size pins, duplicate/nonfinite JSON, missing/extra round files, inconsistent counts, positive qualification claims, identity mismatches and changes during the audit fail closed. `HEAD.txt`, `seeds.txt`, external scorecards and solver artifacts are outside the requested 63-file set.

## Source and portability

The audited source is `evidence/evolution_waves/2026-10-17/xbabe3/out`. Against the common repository's current HEAD at audit time, **63 of 63 files have identical committed blob bytes and sizes**. This is a saved-receipt provenance check, not a claim that source-run certification was independently repeated. No archive or live wave directory is changed.

The default resolves this archive from the common Git root. `--out-root /relocated/xbabe3/out` supplies a folder with the seven wave directories and bypasses Git discovery and original host paths. Missing files fail; absent evidence is never reported as verified. The fixed pins are not refreshed automatically from supplied files.

Fifteen new `Phase16Xbabe3WaveAuditTests` cases use synthetic data for all seven waves/three rounds and cover 41 assessments plus one separate error, all 63 file tamper controls, missing waves/files, extra rounds, malformed pins, exact birth slots, assessed/screen linkage, count and Boolean qualification constraints, retained/mislinked unverified attempts, both Markdown schemas/labels/flags, source/summary/error accounting, malformed CSV, duplicate screens, strict JSON, relocated CLI execution without Git, and inventory/byte mutation. They passed as a focused suite. The existing 75 tests remain; the requested full suite contains 90 tests.

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase16_xbabe3_waves.py
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'
git diff --check
```

The launcher already provides affinity CPUs14,15 and niceness10. These are preserved without adding niceness a second time. Actual command results, input hashes and logs are recorded at `target/rsv2-07/phase16-xbabe3-receipt-artifact-subset/20261018T131011Z/summary.json`. Phase-10's test-source hash and Phase-7/8 study hashes are refreshed for the new verifier/report and synthetic tests. Original numerical receipts remain unchanged. No simulations, builds, geometry changes, commit, push or PR are performed.

## Pinned requested bytes

| Artifact relative to xbabe3/out | Bytes | SHA-256 |
| --- | ---: | --- |
| w405c3/leaderboard.json | 5953 | faf231f9b2c96dfc3665750303f6d18c00bf41ba43c40df94054c5db7b75cb50 |
| w405c3/leaderboard.md | 865 | 0c4680b9391f128c826c869e36b9d373cf02ec60ea428b8a80b0e0ce16a35bbe |
| w405c3/RESULT.md | 2286 | e847b5a139cba2ed8e4fb64bdb2477be2b5c7c0bad86bf0dfd15543d6218399a |
| w405c3/round_00_births.json | 102946 | 8cea1247b4eb2f4bece786d2b0f13807272d1550bbfa39a5750bc436cc6a0e03 |
| w405c3/round_00_screen.json | 14203 | f6eacf9fb8b1a2d7f6ac9867fcb521ec0a2d72c1f29d99706a708fbfbb2ed848 |
| w405c3/round_01_births.json | 94467 | b0144a3100827d9665ee6d3e8ef2ce9e6ac4688f6082293c32de58c0d7f4d4e1 |
| w405c3/round_01_screen.json | 20032 | b181c543b1faaf6d02bb4a5b85fb78ac0a9d1ea68918c25b27bc4701236a4778 |
| w405c3/round_02_births.json | 94497 | 996a8b5167288c6fa774f1004e39221a853e865304355e1f1d16bbd54de708ac |
| w405c3/round_02_screen.json | 19606 | fffa1f8686c70c7588d702622f67f0756a30bb22f99b2bb14bb0e745187f45f1 |
| w406c3/leaderboard.json | 5940 | 56915cfc513a27e7e17a3a87ad3788e84cb65c23f5009472d3ca3def7c271af9 |
| w406c3/leaderboard.md | 865 | 54f007869daf779589324d4f89dadd9931df2ff460b06932e781306c9202b6fd |
| w406c3/RESULT.md | 2331 | 08e3874db6f651149b09ff6081132e05c2cc7da2e105ff5ff5f95e212f4b8dc2 |
| w406c3/round_00_births.json | 103086 | c29223d66e1c1d25b4712efc966d53ebab4b0b3eed901a8508b543e610e5d57e |
| w406c3/round_00_screen.json | 13785 | 538f24eb8666c9a700a117e8984a9280d552342c59ae160551c9ff537e8461c4 |
| w406c3/round_01_births.json | 94480 | 26e9cb38f8761b5a2ec091f79977ee0d4c1152ccfb9b9614d33b53a3cb6ebaea |
| w406c3/round_01_screen.json | 19192 | 83fc240d6aa42a1259d6f3996ecaae8c2821ef4d08c02a9cdd829fea99e8deff |
| w406c3/round_02_births.json | 94778 | 977f2d8ea8fecd4849566ba59666c30fc25994b6f2802f285bb49849c0323833 |
| w406c3/round_02_screen.json | 20020 | f96febdadc6ab90295608d1e29d1b2fb689f345caf871bc77c0dffff3c8dd1e8 |
| w406t3/leaderboard.json | 5947 | 376f268e74c7de5abb5af1e74c96098b5cb43ab6afddad9e2a02a8660bdd62eb |
| w406t3/leaderboard.md | 865 | 3cbb5ac73722cbe806ad9973d891b2c1362964b7215df60a0f99eb32c406ee69 |
| w406t3/RESULT.md | 2345 | c016fc2d1957fb0d81370d5624c963c61ba4e57d1d5ecde01fa349f401e41792 |
| w406t3/round_00_births.json | 103091 | 3ce17611374d93addd17832f7f7619dac604affbef383211a131a972652d3a08 |
| w406t3/round_00_screen.json | 13779 | 82ba7f42cb4e0731f00bf3161ab7cc86565bba643e16cc0da05f91debdb9e2f0 |
| w406t3/round_01_births.json | 95400 | e213bd3e7d9a06acd2e0ec6dee69c04e3de43a05a38384712d7191f6e0271c36 |
| w406t3/round_01_screen.json | 20030 | fc79be15d82110cc28d0b314cf896343ac0a253f6f7ffdee7343130489353340 |
| w406t3/round_02_births.json | 94545 | 97e8ff212f4d9c00af103d719754486f1bd7995ed7347cd0f4cd2a3ca89bba6e |
| w406t3/round_02_screen.json | 19597 | a575bb45ff0265763327f91de045979dab4b8fe47673d7f433e096291facb2ed |
| w407c3/leaderboard.json | 5362 | 7da34b9df4f76434e6fffe7f9a5a3e60713478bfaf340c54cc26afa9ae4ccd61 |
| w407c3/leaderboard.md | 775 | f022b6d18fe9cbe59307658b2ec26ab698fa6f4f980bf4e741562c6299975268 |
| w407c3/RESULT.md | 2587 | 9f1b68aea03e795b58498c616ad38594ce60e5ea7bddaf2d207756ad4ce4d45d |
| w407c3/round_00_births.json | 130824 | 9ef56544e0ac01fb82e99b6e3055479a4079ba7d5dc62122e5f7737dc9897260 |
| w407c3/round_00_screen.json | 4958 | 1b02d667993aca396947df422b16b5f14c18b910fadd2a78d3d15387277a7578 |
| w407c3/round_01_births.json | 93112 | b68196822df5091bb3f0432430ea4b947b39b774303d70a4c357347750c8820d |
| w407c3/round_01_screen.json | 1618 | 464571849e44876c65cde9a1aed2b2ad4ce0780d3fe7660eec3e3269a92685c1 |
| w407c3/round_02_births.json | 46904 | 0ae65d6da9468b23d00db842ca0fa85b109bf33f6f88fd8e819dae87a4b53e1c |
| w407c3/round_02_screen.json | 14543 | d1bfaf33f2000aeeeae056165b6427be3506920bc2731d422bef555096c6757b |
| w407t3/leaderboard.json | 5887 | a57a19ee420f859d96c870f977ded1e3def391926fc03d3e17a54719d4e063e6 |
| w407t3/leaderboard.md | 865 | 26d182c3f8e566a6a37aa3f638964acbeb57ef980dd3dc7afa1120bfa4fec805 |
| w407t3/RESULT.md | 2481 | a61559b367773be4b94522bd3a40077c04569c95faff12c3dc8d4204b4e824b8 |
| w407t3/round_00_births.json | 130818 | 8fe6b6ddbc819dcc795f48927ffa25e63f957f535fd84a2176c03f01427dde5d |
| w407t3/round_00_screen.json | 4955 | d571555c69df2feeecdb25ea9f9c53831a0b5f17009faaf5cfcad5984fda0764 |
| w407t3/round_01_births.json | 93033 | c00813c5da5c114c64748d3d86d9b71d361df5356a933e5920be8fa5131d8e7d |
| w407t3/round_01_screen.json | 1617 | cc79d18eddffbc9ae101c6fcaaa5d784ca4b2ae674b2738d4f57a6a0e3e6bdcc |
| w407t3/round_02_births.json | 46910 | adf131a214ccc5fba6c685ca89bb3b8fcdde3f07cf595af43f91008a1ed959ef |
| w407t3/round_02_screen.json | 14541 | f6e5e264baf2072041b6f972d38447c25436aa297ac4d4c45a5f4930a3559f03 |
| w408c3/leaderboard.json | 5911 | 5928d01586e72e8d315359a3c63adc570693ea732098add8808fea2811370a41 |
| w408c3/leaderboard.md | 865 | 5bedd2e34dc8ac7a3c7f1df650847da1a39499457b7762480899e8a655eed3f8 |
| w408c3/RESULT.md | 2497 | e8d17126558127da4e57f790f9bc32a21ac61a5d1806abbb1e5542642569a8c2 |
| w408c3/round_00_births.json | 104170 | a0522ae67c4469b403538cd7a9fd2290624a531653d6378eeb17ebb3d2045374 |
| w408c3/round_00_screen.json | 3717 | f820510b5d4cdcaba5402ff8860551ff9330ca71faf7038d111e060ad6e81b3c |
| w408c3/round_01_births.json | 85292 | eb81298ab751e440c353b041015a7405dc8476d8624698f370fbb6e5ff46c9f7 |
| w408c3/round_01_screen.json | 6468 | 546dce60033c9750c2ad0ee1e6df0080cc691dcaafaa6b0a1cb11b30e738629b |
| w408c3/round_02_births.json | 71972 | 26fdd0443143e2b67f3cb5a2f641b519c6969a31c6cbac9d3eca00b8fa96db3b |
| w408c3/round_02_screen.json | 18989 | 4b2db1d8dd0f32685ec5cbd2a7ec55af82ab050bd998b6dc725ac2f7311591c5 |
| w408t3/leaderboard.json | 5875 | e9b69682cf090106083d7db4f3bcdb5610d8849fb1e5b98ce873466e6f43f036 |
| w408t3/leaderboard.md | 865 | 5aa53e8ff3d2821fc7402415adf08c69fbc01c759a23758a136a7f79f1f6f8cb |
| w408t3/RESULT.md | 2468 | 354184e29711a8791230c93e51db3d8b68ecd2615889965eed5f1307093278a2 |
| w408t3/round_00_births.json | 104202 | 5ccb5b6d5aff1f6fae2d23e6cdd02337cd5c0df97e5f4a2bfbb96aeb95a80155 |
| w408t3/round_00_screen.json | 3717 | 506cb14ef2da97f53f44715e37a92c583004c89dd290453e685aec4a46e3474e |
| w408t3/round_01_births.json | 85219 | 7e6620590a690fded1edd45616759797b37b7e3e630abd5f9d0992521a68fb68 |
| w408t3/round_01_screen.json | 6059 | 3d7c985c7e3b6a274ffeafede39ca0a88e4421e2ac737bb5af2bf2260e343938 |
| w408t3/round_02_births.json | 62895 | 5fad653f1056375ca108bed4c2c66a78cfa7d87c5520250a961077b56c6032a1 |
| w408t3/round_02_screen.json | 19377 | 44be6efb28723839f8350b9bca8dc92602074316427fe4f1043c8df563a8789c |
