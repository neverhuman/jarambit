# Phase-14 w319 saved wave receipt audit

**PASS for the ten pinned saved artifacts and their reporting/linkage.** Wave w319 contains 200 birth records, 104 pending Stage-1 screen records and four assessed rows. `assessed_count=4`, `qualified_count=0`, `champion=null`, and `unverified=[]`. Every assessed `qualified` flag is the Boolean false. The Markdown leaderboard lists the same four candidates in the same order and marks all four unqualified. Each of the four referenced attempt scorecards has the matching candidate title and a stored **FAIL (Gate Violation)** verdict.

| Saved round | Birth records | Pending screen records |
| --- | ---: | ---: |
| 00 | 104 | 56 |
| 01 | 96 | 48 |

Counts refer to records/events. The four assessed rows reference four distinct saved identities. Pending screen records are not qualifications and are not entries in the leaderboard's literal `unverified` error list.

## Assessed receipt bindings

| Candidate label | Opaque certificate identity | Attempt directory | Qualified | Stored scorecard verdict |
| --- | --- | --- | --- | --- |
| w319_r01_p10_c01 | b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc | attempt-1792321160626983948 | false | FAIL |
| w319_r01_p14_c27 | 6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac | attempt-1792322311344751353 | false | FAIL |
| w319_r02_p14_c45 | 47d60b04044f32fc84f17d915af58c44e0896e1f25ab97dda6587950f44b2850 | attempt-1792325188215181150 | false | FAIL |
| w319_r02_p10_c21 | 89363bf867d1e07b9f91a4f72c181024f8823b4268987517fdc3d00a638d0d3c | attempt-1792323687719656617 | false | FAIL |

The verifier checks the JSON birth-to-screen-to-leaderboard family and identity references using the Phase-11 schema/accounting checks. Phase-13's label/position checks additionally verify that each screen label names the correct wave, round, family and exact saved birth-array position. JSON parsing rejects duplicate keys and nonfinite numbers. The saved CSV headers/row widths and repeated-identity bindings must be consistent; numeric fields receive schema/type/finite-value checks only. No numerical thresholds or qualification decisions are recomputed.

The Markdown leaderboard must contain the expected table schema and exactly the four JSON labels in order. Its Qualified and RS-v2 all_hard cells must agree with the saved JSON flags. Every assessed scorecard reference must end in `out/w319/by-content/{certificate_identity}/attempt-{digits}/{label}_master_scorecard.md`; its identity directory, attempt directory, filename and Markdown title are checked. Exactly four pinned scorecards must be referenced once each. Their single stored Verdict line must be FAIL (Gate Violation).

The scorecard bodies have no standalone certificate-identity field: the checked identity binding comes from the saved JSON reference and by-content path. Identities are opaque references, not newly regenerated certificates. Stored numeric scores, gate details and solver claims in these documents are preserved as bytes and not independently verified. External files cited inside scorecards and the JSON `genome_csv` references are outside this ten-file audit.

## Pinned bytes and portability

The default path is `out/w319` under the common Git repository, currently `/home/ubuntu/jarambit/out/w319`. All files are read before validation and reread afterward to detect changes during the audit. Missing files, malformed pins, mismatching bytes/sizes, inconsistent links or qualification claims fail closed. The command does not modify any wave file or directory.

`--wave-dir /relocated/w319` uses a supplied saved copy without Git discovery or original host data. JSON scorecard references remain unchanged: repository-relative and original absolute references are mapped to a canonical wave-relative suffix and matched against the ten-file pin list. The command never follows those saved reference paths. Traversal and noncanonical paths are rejected before access. This is a fixed snapshot audit; it does not auto-trust later replacement attempts. Other unpinned files in the wave directory are outside scope.

| Artifact relative to w319 | Bytes | SHA-256 |
| --- | ---: | --- |
| by-content/47d60b04044f32fc84f17d915af58c44e0896e1f25ab97dda6587950f44b2850/attempt-1792325188215181150/w319_r02_p14_c45_master_scorecard.md | 6909 | d7e2160333e21f8a7d63392c746d6b6acb3805bee982220b24c3c02514e05e0e |
| by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792322311344751353/w319_r01_p14_c27_master_scorecard.md | 6907 | 73806aaf467b4d64e2ce79b4be286c93953ce88b041d9ba0ffcb98e3b0ea3804 |
| by-content/89363bf867d1e07b9f91a4f72c181024f8823b4268987517fdc3d00a638d0d3c/attempt-1792323687719656617/w319_r02_p10_c21_master_scorecard.md | 6909 | f57710afa103528c24ab2305e96f009d6d23effebab3bbff731c26054b3c0a5e |
| by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792321160626983948/w319_r01_p10_c01_master_scorecard.md | 6907 | 542d65a9c1b4dbbd4ba4393d916b6c14d5e2d7dbdfd039f8e6a922e60f4a1606 |
| leaderboard.json | 3983 | df129159ec3d3e3d95044827f190c082b0ccd4165fc935dca002011b3b05fadf |
| leaderboard.md | 679 | b3edff5fe8a5a0aff7783966271aa51add0313a420bffc48a0855c878d36bd99 |
| round_00_births.json | 174410 | 8ac6de8f7f19cc83c4c9d7f871006e3455300d5ffe71c3f51bedfeb66cd0167c |
| round_00_screen.json | 23223 | 067756640f297d949b7f8f0aa898e3100ce8f61f8070ad2307e9542c7ecf575d |
| round_01_births.json | 162120 | a8098af848a7f65cf6f107e732bc888ace0b06bed7433af0749ed39776f12fee |
| round_01_screen.json | 19893 | af58989acf8b6f9f9c27dcac34f5476eaec73d0cc5adb1630818f1ba8bd34a3a |

## Validation

Thirteen new synthetic unit tests exercise ten-file verification, all-file byte tampering, missing/extra files, malformed digest/size pins, exact birth-slot and assessed-screen mismatch rejection, zero-count/champion/unverified constraints, Boolean qualification rejection, Markdown disagreement/missing rows, scorecard title and verdict mismatch, identity/attempt/label/path tampering, original absolute references, unreferenced attempts, strict duplicate-key parsing, relocated CLI paths without Git, mutation during rereading and unsafe pin-path rejection before file access. These tests require no host wave data. They passed as a focused suite; the requested complete discovery suite includes all 64 tests.

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase14_w319_receipts.py
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
git diff --check
```

The launcher already sets niceness 10 and CPU affinity 14,15; commands preserve those settings instead of adding niceness a second time. Command logs and actual validation results are recorded in `target/rsv2-07/phase14-w319-wave-receipt-audit/20261018T124158Z/summary.json` and adjacent logs.

Phase-10's test-source hash and Phase-7/8 study manifests are refreshed for the new verifier/report and updated tests. Original numerical receipts and all ten wave files remain unchanged. This work performs no builds, geometry generation, solver runs or physical qualification. No commit, push or PR is made.
