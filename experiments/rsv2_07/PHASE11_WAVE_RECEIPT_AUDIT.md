# Phase-11 wave receipt audit

**PASS for the pinned w318 snapshot:** five file digests and byte sizes match; 200 births, 90 pending Stage-1 screen rows and four assessed designs have consistent identity references. All four stored `qualified` flags are the boolean false; `qualified_count=0`, `champion=null`, and the `unverified` list has length zero.

| Artifact | Schema profile | Records | Bytes | SHA-256 |
| --- | --- | ---: | ---: | --- |
| leaderboard.json | leaderboard.v2 | 4 | 3964 | 7eeeb41a5dd36718ee6d300b626c76b01e0be5215bd36c97d9c37ca354c654a9 |
| round_00_births.json | births structural profile | 104 | 176486 | 294c9d71e64d0963c24c71fcd8e76f9ec6b8e08bd5089bb8c78c44dfc4e3d75f |
| round_00_screen.json | Stage-1 list profile | 49 | 20298 | 2416936cd05b93217ed8912fc27d5cc8b9308ce0fe04e69fd35278daacded1d8 |
| round_01_births.json | births structural profile | 96 | 164034 | f384d9fc60eaad7d2880e47c8ae467e9c42a13688050d0ffa80331b3736f2f21 |
| round_01_screen.json | Stage-1 list profile | 41 | 16949 | 243404dc45c5903b03a2ea3ddf476a5b0f6eeee16e38a7e8b24832ac81e53b28 |

The source files were read from `/home/ubuntu/jarambit/out/w318/`. Round 0 has 104 births and 49 screens; round 1 has 96 births and 41 screens. Births and screens have no schema identifier, so the verifier checks their existing structural profiles without modifying them. The leaderboard must declare `jarambit.auto_evolve.leaderboard.v2`.

Checks cover required fields and types, finite numeric observations, CSV header/row width, round IDs, unique screen/assessment labels, and lowercase 64-hex certificate identities. Repeated identical birth rows are allowed. One identity cannot bind conflicting CSV rows, including across families/rounds. Each screen identity/family must occur in that round's births; each assessed label/identity/family must match a screen record.

Stage-1 rows must retain `qualification_status="pending independent top-k assessment"`. A missing Stage-1 qualification flag is not interpreted as a qualification verdict. Every assessed row must explicitly contain `qualified=false`; missing flags, truthy substitutes, contradictory stored Boolean gate flags, nonzero qualified counts, Boolean count substitutes, an assessed count that differs from the design rows, a non-null champion, and nonempty or incorrectly typed `unverified` values fail closed. The unverified field is the leaderboard's literal error list; pending Stage-1 rows are not reclassified into that list.

SHA-256 and size pins are stored in the audit module and cover the exact bytes of all five JSON artifacts. The audit rereads the files to detect changes during verification. Certificate identities are treated as opaque, source-bound identifiers: their format and cross-references are checked, but their native source/environment-dependent derivation is not regenerated. Referenced CSV and scorecard paths remain references; this audit does not read additional certification artifacts or establish their validity.

The module uses the Python standard library and the existing strict JSON parser, which rejects duplicate keys and nonfinite JSON. No geometry generation, structural calculation, solver, certificate issuance or promotion is performed. The saved wave files remain unchanged.

## Portability and tests

The default source is `out/w318` beside the repository's common Git directory, so no username or worktree location is hardcoded. `--wave-dir` accepts another directory containing the five byte-identical saved artifacts. Missing artifacts, snapshot hash changes and incomplete schemas fail rather than claiming zero unverified records from absent evidence. There is no CPU affinity restriction.

**The requested full discovery suite passed all 30 tests on CPUs14,15.** Ten new synthetic tests use in-memory documents and file reads, with no dependency on the host's live wave, target directory or certificate/solver runtimes. They exercise accounting, portable directory access, byte tampering, digest/size syntax, missing files/fields, qualification and unverified failures, pending statuses, identity/label mismatches, conflicting CSV rows, and concurrent artifact mutation. Existing Phase-7/8 and receipt-portability regressions remain in the same suite.

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase11_wave_receipts.py
taskset -c 14,15 python3 experiments/rsv2_07/audit_phase11_wave_receipts.py --wave-dir /path/to/out/w318
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
```

Execution uses the inherited CPU affinity14,15 and nice10. The related Phase-7/8 study manifests and Phase-10 test-file binding are refreshed only for the new audit/report and updated test source; earlier numerical receipts and wave files are unchanged.
