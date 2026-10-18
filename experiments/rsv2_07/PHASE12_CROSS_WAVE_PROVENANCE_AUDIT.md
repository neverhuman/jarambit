# Phase-12 cross-wave receipt provenance audit

**PASS for the pinned saved artifacts.** Twenty JSON file digests and byte sizes match. Each wave declares `qualified_count=0`, contains four explicitly unqualified assessed rows and has `champion=null` and an empty `unverified` list. Reused certificate identities have no conflicting saved CSV bindings.

| Wave | Birth records | Pending screens | Assessment events | Qualified | Unverified |
| --- | ---: | ---: | ---: | ---: | ---: |
| w315 | 196 | 176 | 4 | 0 | 0 |
| w316 | 200 | 34 | 4 | 0 | 0 |
| w317 | 200 | 33 | 4 | 0 | 0 |
| w318 | 200 | 90 | 4 | 0 | 0 |

These are 796 birth records, 333 pending screen records and **16 assessment events referencing eight distinct assessed identities**. Reassessing a saved identity in another wave creates another assessment event; it does not create a new distinct identity. Labels are scoped to their wave. Across all births, 640 distinct identities appear, and 16 occur in more than one wave. Repeated identical birth entries within a wave are retained in record counts.

## Cross-wave identity membership

| Assessed certificate identity | Waves with an assessed row |
| --- | --- |
| 234a9e89d25b63ad7300825df9ac88a8f78cac41957b17f43853d69184b5bd70 | w316, w317 |
| 3eb40f1b9d74669c3a9afd22833888e26542a7c52ae512bbfa3f77fe5525b2db | w318 |
| 44ac7f99fd172d52780482c32dffe9e012282355e941ecc8ebbf0c40ebc35a43 | w318 |
| 6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac | w315, w316, w317, w318 |
| 74ed6ed4ab8bf8a042352e73ca6a51561821d515acc09f21ed83bbaa880ad94e | w315 |
| b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc | w315, w316, w317, w318 |
| b77c59c158cf89b22a5493bfe380f31397eeb3a77c05a56fb9db46a584c7c0cb | w315 |
| d715c60095b56f3f9fe20eb89dcff8fca15ad48eccd50f0e8d968e7117b2b55f | w316, w317 |

Each wave reuses the Phase-11 structural and fail-closed checks: required fields/types, finite numeric observations, digest syntax, two-round CSV header/row width, pending Stage-1 status, same-round birth-to-screen identity/family references, unique local labels, and screen-to-assessed identity/family references. `assessed_count` must equal the number of design rows. Every assessed `qualified` flag must be the Boolean false, qualified counts must be integer zero, and contradictory stored Boolean gate flags, champion claims or unverified errors fail.

The cross-wave check additionally requires every reused identity to retain the exact same saved CSV header and row. It does not assume that equal CSV rows must have equal certificate identities: native identities also bind source/environment context. Certificate identities remain opaque references; their native derivation is not regenerated. No numeric performance thresholds are applied, gate decisions are not recomputed, and no geometry, solver, new certificate or structural calculation is run.

Stage-1 pending rows are not counted as qualifications or as entries in the leaderboard's literal `unverified` error list. Only the five JSON artifacts per wave are audited. Paths to external CSV and scorecard files remain references and are not followed or claimed to be independently validated.

## Pinned artifact bytes

w315..w317 were read from `evidence/evolution_waves/2026-10-17/xbabe2/out/` under the common repository root; w318 was read from `out/w318/`. Both locations can be overridden. All files are read before validation and reread afterward to reject changes during the cross-wave audit.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| w315/leaderboard.json | 3961 | 49f9dcc245b9a2d81518ccbd992fe9b9c445d521a25080baf9b0eb98a611924d |
| w315/round_00_births.json | 183491 | bc9441f863812257b119520370563da8cfe606fe56d76c8649ce3077838ce0d2 |
| w315/round_00_screen.json | 34524 | 4ed6def0fb861038ddc7c8184c08c824325531f2830de41c3772abe0046e1689 |
| w315/round_01_births.json | 175784 | e149fae3512408544de0205c562d995838aa166f0c4f64386a00dd40b961863c |
| w315/round_01_screen.json | 38662 | a45a5f4559b64702c9cf77e71e5f734e14af996ce1b20e34b4afde1523137698 |
| w316/leaderboard.json | 3955 | 63df2d7b2c3613a4411b619b48200ec227ca9ef03515bc53e5e5dacb7405f8f5 |
| w316/round_00_births.json | 180331 | c1ce1969c2cee585db9cde609acd957635398acfc227f57c595bf08c0415db14 |
| w316/round_00_screen.json | 8172 | 1b5cb4d4ff4504bcd073b0fd1276a5a3887ddbb06447a91a2ad2c478921ed56e |
| w316/round_01_births.json | 167965 | a4fc583720d23a8fc0876ea398dde7d862d261a8199bfe496476573acec4ffe8 |
| w316/round_01_screen.json | 5647 | 2a7a0f86ac92ea7e983179761b639ef7e7bf585274d96cee8d81cccfb589b872 |
| w317/leaderboard.json | 3955 | 429efbe4be1db44befc662ed5fe7a4a1de8ebe3cb53beaa0902a88179b72bb0e |
| w317/round_00_births.json | 179197 | aa4f9c4c0a888a9fb723ccd1245bb0a8c10d33be7743767ab53c71edbe73290c |
| w317/round_00_screen.json | 8172 | 1895c91af815e83e87e5d820705f84537327a50a493b37a7bfdf98b41b8de240 |
| w317/round_01_births.json | 166685 | b5df4627717dc2b76fc0b2f7ad18f92307b9ba82c268ed792d78ea7730efdda8 |
| w317/round_01_screen.json | 5244 | afd1e87a79c21f6996df5b0774df4b9653a413dafe2a7d83a2bb9b5c65822663 |
| w318/leaderboard.json | 3964 | 7eeeb41a5dd36718ee6d300b626c76b01e0be5215bd36c97d9c37ca354c654a9 |
| w318/round_00_births.json | 176486 | 294c9d71e64d0963c24c71fcd8e76f9ec6b8e08bd5089bb8c78c44dfc4e3d75f |
| w318/round_00_screen.json | 20298 | 2416936cd05b93217ed8912fc27d5cc8b9308ce0fe04e69fd35278daacded1d8 |
| w318/round_01_births.json | 164034 | f384d9fc60eaad7d2880e47c8ae467e9c42a13688050d0ffa80331b3736f2f21 |
| w318/round_01_screen.json | 16949 | 243404dc45c5903b03a2ea3ddf476a5b0f6eeee16e38a7e8b24832ac81e53b28 |

## Portability and tests

The module uses only the Python standard library and the existing receipt validators. It has no username or CPU affinity restriction. Default paths resolve from the common Git directory. With both `--archive-root` and `--live-wave-dir`, no Git or original checkout path is needed. Missing waves/files and mismatching bytes fail closed rather than inferring qualification from absent evidence. The saved files are never modified.

**The requested full discovery suite passed all 40 tests on CPUs14,15.** Ten new synthetic test cases cover separate event/identity counts, repeated wave-local labels, every wave's Stage-1/2 linkage, conflicting cross-wave rows/headers, distinct source-bound identities for equal rows, malformed counts/qualification, missing waves/files, digest tampering, in-memory relocated directories, cross-wave mutation and explicit CLI paths without Git access. They use no host wave data or target artifacts. The existing 30 tests remain in the requested discovery suite.

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase12_cross_wave.py
taskset -c 14,15 python3 experiments/rsv2_07/audit_phase12_cross_wave.py --archive-root /path/to/archive/out --live-wave-dir /path/to/live/w318
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
```

The audit is executed on CPUs14,15 at inherited nice10. Related Phase-7/8 study hashes and the Phase-10 test-source binding are refreshed for the new audit/report and modified test source. Earlier receipts and saved wave artifacts remain unchanged. No commit or push is made for this task.
