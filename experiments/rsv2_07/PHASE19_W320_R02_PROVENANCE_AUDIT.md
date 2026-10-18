# Phase-19 w320 r02 attempt provenance audit

**Partial: the 21 pinned files pass byte integrity, format and identity checks.** These are four existing Stage-1 round files, the saved leaderboard and 16 pinned attempt files. The leaderboard reports three assessed rows, zero qualified rows, a null champion and one separate unverified attempt for the target. Its stored reason records an evaluator failure with exit status 1; the pinned stdout file preserves the available evidence without interpreting or diagnosing it. The target is not counted as assessed or qualified. No simulations, geometry generation, physical gate evaluation or mechanical comparisons are performed.

The target label is `w320_r02_p14_c93` with opaque certificate identity `80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572`. Its r02 label refers to one-based round 2, stored in round_01 files. It is not evidence that round_02 files (the third saved round) exist.

The existing saved rounds contain 200 birth records and 105 pending screens. The target is the 93rd global birth-array entry in saved round 01, family p14. Its pending screen label/family/identity must match that exact position. No native certificate identity is regenerated.

The attempt genome CSV header and single record match the saved birth header/record exactly. Both CSV files have rectangular single-record tables. All seven pinned attempt JSON files parse strictly as objects; duplicate keys and nonfinite numbers fail. Present label/identity fields and required metadata in the D1, grind-law, plate-GCI and surface documents must agree with the target; the D1 label retains its producer's _D1 suffix. SHA-256 metadata is checked for syntax only where no independent derivation is specified. Numeric observations and solver/geometry claims are not interpreted. STEP, STL, mesh, text and Markdown files receive byte-integrity checks only.

| Requested output | Availability at snapshot |
| --- | --- |
| out/w320/round_02_births.json | missing |
| out/w320/round_02_screen.json | missing |
| out/w320/leaderboard.json | present_pinned |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_master_scorecard.md | missing |

The table records availability of the target's master scorecard and saved third-round birth/screen files. Missing outputs are not counted as completed assessments or qualifications. A later present file outside the pin set is explicitly unpinned. Any additional attempt paths are listed as unverified in the JSON receipt. New or changed files during validation fail the snapshot check; later changes require explicit receipt review/refresh.

Phase-17's current schema/registry audit and Phase-18's current provenance audit must match their saved receipts. Phase-18 now accepts the reviewed leaderboard digest/size and preserves its three assessed rows and separate unverified entry. Phase-18's binding to the refreshed Phase-17 JSON is checked. Both receipt hashes are retained and reread after file validation. Earlier physical scope limits and missing-progression-baseline observations remain intact.

Only repository-relative paths are emitted. --wave-dir and --repo-root support relocated saved copies without Git discovery when both are supplied. The default uses the common repository's w320 directory. The verifier is read-only unless --write explicitly updates this report and the Phase-19 JSON receipt. Source artifacts and earlier verifiers are not modified by the command.

```sh
python3 experiments/rsv2_07/audit_phase19_w320_r02_provenance.py
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'
```

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Phase-10's test-source binding, Phase-17/18 receipt links and Phase-7/8 study bindings are refreshed together. Unit tests use synthetic saved bytes and in-memory relocated files.

| Attempt file | Format check |
| --- | --- |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_skfem_stdout.txt | opaque_bytes |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/d1_rescreen.csv | csv_single_record |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/d1_rescreen.md | opaque_bytes |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/plates.txt | opaque_bytes |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93.step | opaque_bytes |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93.stl | opaque_bytes |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_D1.json | strict_json_object |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_export_stdout.txt | opaque_bytes |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_genome.csv | csv_single_record |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_grind_law.json | strict_json_object |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_plate_gci.json | strict_json_object |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_plate_gci_raw.json | strict_json_object |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_runtime.json | strict_json_object |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_solid_mesh.npz | opaque_bytes |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_step_check.json | strict_json_object |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_surface.json | strict_json_object |

| Pinned artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/d1_rescreen.csv | 886 | 6debcbd0905004739b0ed104d01fb66b4ea36456faf2b736e89fac85ac5460d8 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/d1_rescreen.md | 2133 | 468397dc4357859e9bb0670f7bae2f3542fa95b4f164004276a68978ea44aab3 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/plates.txt | 20 | 40831797fd412f18cf3d7049452b8f155a3e55401c5f2b9bb63f905406a39f30 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93.step | 59704935 | df15db4274fbc562fb5fd3f837a5d41a05f63ed359cd68c8080fe0b9b2aea8f7 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93.stl | 16397734 | be25bd0130c83432d4db2738316fc8c180aeaa0034f4bbcbfad7fec05c182f79 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_D1.json | 5992649 | aed64b3c075a5421e7cc3b7e1df0ac09cf8ef5e730eac9bbdd7bebc3acb5f116 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_export_stdout.txt | 59 | b1f165232b9aaffb819145b977b8a58d17d18fffc613fb80897b3bf5bdaec0ce |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_genome.csv | 2528 | 1421b661bbdae629c3d5ca3d010150ee39c912c9468abd1adda582b6a6f53869 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_grind_law.json | 111623 | 1c709a427bf642c2de7e26be8c5d9508d2bc58a8cf45fe970a7f758811747252 |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_plate_gci.json | 5688 | b22951ca9ed5932a0c41c7d45be2fadbf8da8ab7b8c1d2b72d064f958410014a |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_plate_gci_raw.json | 1841 | 83fcf87361f0a816e172794fa64be1c02d3635326fd108b0ed8d97a2f1b1a2ed |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_runtime.json | 558 | 2694431cf731a37e8d6dbf63638d271d193263205ce3eceafd1280903405548b |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_skfem_stdout.txt | 1046 | d56c6b27efccda568ca784a10e41d9deeb37553c7af8939b667c3a9075ff235e |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_solid_mesh.npz | 566987 | 938ef9cb124b5f70a8dba5b26f046f173ba851f74be8901acde8bb8bac627d3a |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_step_check.json | 302166 | e5d66ffa7b079b8133f0de85e547c95b56aa5785155c327c9d5f76cb8a21c7db |
| out/w320/by-content/80d912c4dffd42b1d68d8cbbaf85500abdd52dbdffdbf34a0ed866cfa0e84572/attempt-1792330667555204194/w320_r02_p14_c93_surface.json | 3455168 | 7a51ff8fe5167d0e8c80a7eb9ef5d583a51ef61df2ad677d535699bc92dc048b |
| out/w320/leaderboard.json | 3418 | c86614effb2283001bf9e0cb0293226d2a06fd8e6aac8006b67744b03adb9a7c |
| out/w320/round_00_births.json | 177413 | f725189be663ee30646a52a84dd049648a7b39c7e39a58e7d9384e5d626e83c7 |
| out/w320/round_00_screen.json | 23213 | adb40eb24b76e1ce1120c5255f5db90f69c4824fad1ca19af60b8aa17a6134f5 |
| out/w320/round_01_births.json | 165083 | 195b11e4828697ea8db1a6898330a62e03adcfd7f12c2671a71e03ef80a0cbdb |
| out/w320/round_01_screen.json | 20297 | a4494238b88b330502a6d36910eacb947df9d511b587cd656c388a682bd1f274 |
