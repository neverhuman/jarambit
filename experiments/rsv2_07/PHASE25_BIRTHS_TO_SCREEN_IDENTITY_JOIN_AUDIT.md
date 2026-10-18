# Phase-25 births-to-screen certificate identity join audit

**PASS for same-wave, same-round identity membership.** Across 12 saved wave/round pairs, 542 screen rows join against 1196 birth rows. Matched screen rows: 542; orphan screen rows: 0. Every saved leaderboard retains qualified_count=0 and champion=null; 23 assessed events and 1 separate unverified attempt remain recorded.

| Wave | Round | Birth rows | Screen rows | Matched screen rows | Orphan rows | Distinct birth IDs | Distinct screen IDs |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| w315 | 0 | 100 | 83 | 83 | 0 | 88 | 71 |
| w315 | 1 | 96 | 93 | 93 | 0 | 82 | 79 |
| w316 | 0 | 104 | 20 | 20 | 0 | 90 | 12 |
| w316 | 1 | 96 | 14 | 14 | 0 | 82 | 6 |
| w317 | 0 | 104 | 20 | 20 | 0 | 90 | 12 |
| w317 | 1 | 96 | 13 | 13 | 0 | 82 | 5 |
| w318 | 0 | 104 | 49 | 49 | 0 | 97 | 45 |
| w318 | 1 | 96 | 41 | 41 | 0 | 89 | 37 |
| w319 | 0 | 104 | 56 | 56 | 0 | 95 | 50 |
| w319 | 1 | 96 | 48 | 48 | 0 | 87 | 42 |
| w320 | 0 | 104 | 56 | 56 | 0 | 97 | 52 |
| w320 | 1 | 96 | 49 | 49 | 0 | 89 | 45 |

Each screen certificate_identity must occur in the birth set supplied by that same wave and saved round. Identical digests in another wave or round cannot satisfy membership. Each digest requires exactly 64 lowercase hexadecimal characters. Comparison uses the exact saved strings, without prefix matching, case normalization or fresh native identity calculation.

This is an existence join: each screen row is counted once. Repeated birth identities do not create Cartesian row multiplication; repeated screen identities still count as their original number of screen rows. Distinct-identity counts are per wave/round, not global novelty counts. Empty screen arrays have zero orphans; an empty birth array with screens reports those screens as orphans. Rows are never sorted or changed.

All 30 saved root JSON files (two birth/screen pairs and a leaderboard per wave) require the existing reviewed SHA-256 and byte-size pins. A shared provenance helper checks strict JSON, birth/screen schemas, exact same-round birth-slot/family/label bindings and the original assessed/unverified ledger references. Those stronger linkage checks remain in addition to the explicit identity membership counts. Invalid schemas or reference mismatches fail closed before receipt admission.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| out/w315/leaderboard.json | 3961 | 49f9dcc245b9a2d81518ccbd992fe9b9c445d521a25080baf9b0eb98a611924d |
| out/w315/round_00_births.json | 183491 | bc9441f863812257b119520370563da8cfe606fe56d76c8649ce3077838ce0d2 |
| out/w315/round_00_screen.json | 34524 | 4ed6def0fb861038ddc7c8184c08c824325531f2830de41c3772abe0046e1689 |
| out/w315/round_01_births.json | 175784 | e149fae3512408544de0205c562d995838aa166f0c4f64386a00dd40b961863c |
| out/w315/round_01_screen.json | 38662 | a45a5f4559b64702c9cf77e71e5f734e14af996ce1b20e34b4afde1523137698 |
| out/w316/leaderboard.json | 3955 | 63df2d7b2c3613a4411b619b48200ec227ca9ef03515bc53e5e5dacb7405f8f5 |
| out/w316/round_00_births.json | 180331 | c1ce1969c2cee585db9cde609acd957635398acfc227f57c595bf08c0415db14 |
| out/w316/round_00_screen.json | 8172 | 1b5cb4d4ff4504bcd073b0fd1276a5a3887ddbb06447a91a2ad2c478921ed56e |
| out/w316/round_01_births.json | 167965 | a4fc583720d23a8fc0876ea398dde7d862d261a8199bfe496476573acec4ffe8 |
| out/w316/round_01_screen.json | 5647 | 2a7a0f86ac92ea7e983179761b639ef7e7bf585274d96cee8d81cccfb589b872 |
| out/w317/leaderboard.json | 3955 | 429efbe4be1db44befc662ed5fe7a4a1de8ebe3cb53beaa0902a88179b72bb0e |
| out/w317/round_00_births.json | 179197 | aa4f9c4c0a888a9fb723ccd1245bb0a8c10d33be7743767ab53c71edbe73290c |
| out/w317/round_00_screen.json | 8172 | 1895c91af815e83e87e5d820705f84537327a50a493b37a7bfdf98b41b8de240 |
| out/w317/round_01_births.json | 166685 | b5df4627717dc2b76fc0b2f7ad18f92307b9ba82c268ed792d78ea7730efdda8 |
| out/w317/round_01_screen.json | 5244 | afd1e87a79c21f6996df5b0774df4b9653a413dafe2a7d83a2bb9b5c65822663 |
| out/w318/leaderboard.json | 3964 | 7eeeb41a5dd36718ee6d300b626c76b01e0be5215bd36c97d9c37ca354c654a9 |
| out/w318/round_00_births.json | 176486 | 294c9d71e64d0963c24c71fcd8e76f9ec6b8e08bd5089bb8c78c44dfc4e3d75f |
| out/w318/round_00_screen.json | 20298 | 2416936cd05b93217ed8912fc27d5cc8b9308ce0fe04e69fd35278daacded1d8 |
| out/w318/round_01_births.json | 164034 | f384d9fc60eaad7d2880e47c8ae467e9c42a13688050d0ffa80331b3736f2f21 |
| out/w318/round_01_screen.json | 16949 | 243404dc45c5903b03a2ea3ddf476a5b0f6eeee16e38a7e8b24832ac81e53b28 |
| out/w319/leaderboard.json | 3983 | df129159ec3d3e3d95044827f190c082b0ccd4165fc935dca002011b3b05fadf |
| out/w319/round_00_births.json | 174410 | 8ac6de8f7f19cc83c4c9d7f871006e3455300d5ffe71c3f51bedfeb66cd0167c |
| out/w319/round_00_screen.json | 23223 | 067756640f297d949b7f8f0aa898e3100ce8f61f8070ad2307e9542c7ecf575d |
| out/w319/round_01_births.json | 162120 | a8098af848a7f65cf6f107e732bc888ace0b06bed7433af0749ed39776f12fee |
| out/w319/round_01_screen.json | 19893 | af58989acf8b6f9f9c27dcac34f5476eaec73d0cc5adb1630818f1ba8bd34a3a |
| out/w320/leaderboard.json | 3418 | c86614effb2283001bf9e0cb0293226d2a06fd8e6aac8006b67744b03adb9a7c |
| out/w320/round_00_births.json | 177413 | f725189be663ee30646a52a84dd049648a7b39c7e39a58e7d9384e5d626e83c7 |
| out/w320/round_00_screen.json | 23213 | adb40eb24b76e1ce1120c5255f5db90f69c4824fad1ca19af60b8aa17a6134f5 |
| out/w320/round_01_births.json | 165083 | 195b11e4828697ea8db1a6898330a62e03adcfd7f12c2671a71e03ef80a0cbdb |
| out/w320/round_01_screen.json | 20297 | a4494238b88b330502a6d36910eacb947df9d511b587cd656c388a682bd1f274 |

Source bytes are reread after auditing and never written. The scope is rounds 00/01 under out/w315 through out/w320. Output paths are canonical repository-relative paths. --artifact-root supports relocated saved inputs without Git discovery; --repo-root selects the receipt checkout. Normal runs compare the complete receipt and exact report. --write explicitly refreshes these outputs, which are also checked for literal host prefixes. A failed join exits unsuccessfully rather than claiming zero orphans.

Phase-7/8 study hashes and Phase-23's final control-receipt byte bindings are synchronized. The existing Phase-24 receipt and report remain unchanged. No metrics, margins, candidate recommendations, simulations, geometry changes or physical qualification are computed by this join audit.

```sh
python3 experiments/rsv2_07/verify_rsv2_07_receipts.py --phase 25
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
git diff --check
```

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2.
