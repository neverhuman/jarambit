# Phase-18 w320 saved provenance audit

**Partial: seven saved artifact hashes, schema and candidate references pass.** Mechanical deficit progression is not performed. The requested Phase-15 progression JSON and w320 leaderboard are absent. This report does not claim a completed Stage-2 leaderboard, a qualification count or a champion state.

| Saved round | Birth records | Pending screen records |
| --- | ---: | ---: |
| 00 | 104 | 56 |
| 01 | 96 | 49 |

There are 200 birth records and 105 pending screens. Each screen must match its exact same-round birth-array position, family and opaque certificate identity. CSV shape/provenance, finite observation types, pending status, duplicate labels and repeated-identity bindings are checked. No mechanical thresholds or deficits are calculated.

| Requested attempt | Opaque certificate identity | Stored scorecard verdict |
| --- | --- | --- |
| w320_r01_p10_c01 | b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc | FAIL (Gate Violation) |
| w320_r01_p14_c27 | 6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac | FAIL (Gate Violation) |
| w320_r02_p10_c22 | dea35f5fb19c5f11bad56e636be1f88434aed9a0fa2547428c2e58619c128599 | FAIL (Gate Violation) |

All three named attempt scorecards exist and their pinned paths, identity directories, filenames, titles and single stored FAIL verdicts link to the saved screens. A saved scorecard alone does not supply a completed leaderboard or independently establish physical qualification. External files and performance claims cited within scorecards are not revalidated. Additional later attempts are outside this fixed snapshot.

The requested phase15_w315_w319_progression_audit.json is absent at both repository-relative candidate locations recorded in the JSON receipt. The existing Phase-15 registry and verified Phase-17 schema receipt provide provenance context for 25 references across w315–w319. They are not substituted numerical progression evidence; no deficit improvement or deterioration is inferred. The original Phase-15 scope limits remain in force.

The verifier checks all seven SHA-256 digests and byte sizes and rereads their bytes after validation. Missing files, invalid pins, malformed JSON, reference mismatches or mutation fail closed. The absence of the leaderboard and requested baseline is checked before and after. If either appears later, the saved snapshot requires explicit review instead of silently extending its claims. Phase-17 context is also reread to reject mutation.

Only portable repository-relative paths are emitted. --wave-dir accepts a relocated folder and --repo-root a relocated checkout. With both supplied, no Git discovery or original host paths are required. The default wave directory resolves from the common repository. The command is read-only unless --write explicitly updates this report and the Phase-18 JSON receipt. Read-only runs compare the saved receipt against a fresh provenance audit.

```sh
python3 experiments/rsv2_07/audit_phase18_w320_progression.py
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'
```

Execution uses CPUs14,15 at inherited nice10 and CARGO_BUILD_JOBS=2. The Phase-10 test hash, Phase-17 schema inventory and Phase-7/8 study bindings are refreshed for the new audit and tests. No geometry, simulation, gate recalculation or physical qualification is performed.

| Saved artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| out/w320/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792327910595534301/w320_r01_p14_c27_master_scorecard.md | 6907 | f6eb03b5c36fe9e1ff0b7469431c858d9d39a1b0830d5d4352bc42498ed72557 |
| out/w320/by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792326648482434020/w320_r01_p10_c01_master_scorecard.md | 6907 | 10a050cc49de2347e04016d5a933e9009a16305571c1c81cf3d463b2a7d5a941 |
| out/w320/by-content/dea35f5fb19c5f11bad56e636be1f88434aed9a0fa2547428c2e58619c128599/attempt-1792329315786884441/w320_r02_p10_c22_master_scorecard.md | 6909 | eef7003b75b22574d06f663019b9a757930058043dd014766cec036de0db331c |
| out/w320/round_00_births.json | 177413 | f725189be663ee30646a52a84dd049648a7b39c7e39a58e7d9384e5d626e83c7 |
| out/w320/round_00_screen.json | 23213 | adb40eb24b76e1ce1120c5255f5db90f69c4824fad1ca19af60b8aa17a6134f5 |
| out/w320/round_01_births.json | 165083 | 195b11e4828697ea8db1a6898330a62e03adcfd7f12c2671a71e03ef80a0cbdb |
| out/w320/round_01_screen.json | 20297 | a4494238b88b330502a6d36910eacb947df9d511b587cd656c388a682bd1f274 |
