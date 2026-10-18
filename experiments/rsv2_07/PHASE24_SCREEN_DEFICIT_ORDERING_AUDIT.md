# Phase-24 saved screen tuple ordering provenance audit

**PASS for saved tuple ordering.** 12 screen arrays across w315–w320 contain 542 rows and 530 adjacent comparisons. There are 0 descending adjacent pairs. Each saved leaderboard retains qualified_count=0 and champion=null; 23 assessed events and 1 separate unverified attempt remain unchanged.

| Wave | Saved round | Rows | Adjacent pairs | Descending pairs | Equal keys | Non-decreasing |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| w315 | 0 | 83 | 82 | 0 | 17 | True |
| w315 | 1 | 93 | 92 | 0 | 16 | True |
| w316 | 0 | 20 | 19 | 0 | 2 | True |
| w316 | 1 | 14 | 13 | 0 | 1 | True |
| w317 | 0 | 20 | 19 | 0 | 2 | True |
| w317 | 1 | 13 | 12 | 0 | 3 | True |
| w318 | 0 | 49 | 48 | 0 | 1 | True |
| w318 | 1 | 41 | 40 | 0 | 1 | True |
| w319 | 0 | 56 | 55 | 0 | 1 | True |
| w319 | 1 | 48 | 47 | 0 | 2 | True |
| w320 | 0 | 56 | 55 | 0 | 0 | True |
| w320 | 1 | 49 | 48 | 0 | 0 | True |

The checked key is the exact lexicographic tuple (screen_deficit, mass_g). Each row requires both finite, nonboolean numeric fields. A smaller first component permits any second component; the second component orders only equal first components. Equal complete keys are allowed. No tolerance, rounding, deficit recalculation or row sorting is applied, and no candidate labels or key values are copied into this ordering report.

This checks the saved sequence's predicate and byte reproducibility only. Equal keys do not establish a unique order or the producer's stable tie-breaking behavior. No source algorithm is replayed, no engineering margin is derived, and no candidate is selected, recommended or newly qualified. Ordering failures are recorded without repairing source arrays; verification exits unsuccessfully if any descending pair is present.

All 30 saved root files (two birth/screen pairs and a leaderboard per wave) match existing reviewed SHA-256/size pins. Birth/screen schema and same-round label/family/identity linkage are reused from the provenance verifier. The leaderboard count/boolean/null and assessed-versus-unverified checks retain the original statuses. Source bytes are reread after checking; no wave files are written. The fixed scope covers rounds 00/01, not an entire wave-directory census.

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

All emitted paths are repository-relative. --artifact-root selects relocated saved inputs without Git discovery, and --repo-root selects the receipt checkout. Normal runs compare the complete recomputed receipt and exact report; --write explicitly refreshes outputs. Both Phase-24 outputs are checked for literal host prefixes. The Phase-23 catalogue retains its Phase-1..22 scope; its final Phase-7/8 byte bindings and the study hashes are refreshed after adding these outputs.

```sh
python3 experiments/rsv2_07/verify_rsv2_07_receipts.py --phase 24
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
git diff --check
```

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. No simulations, geometry changes or physical qualification checks are performed.
