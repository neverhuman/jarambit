# Phase-15 Gate S and margin progression assignment: artifact subset

**Phase-15 is partially completed. PASS applies only to the saved artifact hashes, scorecard references, stored verdict metadata and Markdown table layout.** Sharpening thresholds, Gate S pass conditions and rates, structural gate validity, exact margin gaps and cross-wave mechanical progression were not evaluated. This audit does not establish the assignment's proposed 100% Gate S claim.

Analysis of sharpening thresholds and structural margins to evaluate weapon designs is outside the work performed. No new geometry, simulation, gate decision, certificate or solver result is produced. The requested filenames and test-class name are retained for task traceability; they do not imply implementation of Gate S or margin calculations.

## Verified available files

| Wave | Assessed records | Referenced scorecards | Verified files | Stored qualified count | Stored champion |
| --- | ---: | ---: | ---: | ---: | --- |
| w315 | 4 | 4 | 5 | 0 | null |
| w316 | 4 | 4 | 5 | 0 | null |
| w317 | 4 | 4 | 5 | 0 | null |
| w318 | 4 | 4 | 5 | 0 | null |
| w319 | 4 | 4 | 5 | 0 | null |

These are 20 assessment records/scorecards and five leaderboards, for **25 verified files**. Every assessed row is explicitly unqualified, and every referenced scorecard has the matching candidate title and the saved single FAIL (Gate Violation) verdict. Pending or physical qualification claims are not inferred from layout checks. Repeated identities across waves are separate assessment events; no cross-wave performance comparison is made.

The read-only default uses the live saved files under `/home/ubuntu/jarambit/out/w315` through `out/w319`. Their leaderboard digests match the earlier Phase-12/14 pins. The card byte digests are now pinned to the available files; changing either a file or an attempt reference requires an explicit review of the pins. `0` of these 25 live paths are tracked by Git in the common repository. This report does not present the live scorecard bytes as historical committed scorecard copies.

## Requested archive coverage

The named archive root is `/home/ubuntu/jarambit/evidence/evolution_waves/2026-10-17/xbabe2/out`.

| Wave | Archive directory exists | Referenced attempt scorecards present in archive |
| --- | --- | ---: |
| w315 | true | 0/4 |
| w316 | true | 0/4 |
| w317 | true | 0/4 |
| w318 | true | 0/4 |

The archived w315–w317 leaderboards refer to attempt scorecards absent from their archived wave directories. There is no archived w318 directory at this location. These missing archived scorecard bytes are not verified and are not treated as equivalent to a complete archive. The available live files are audited separately, with that source substitution disclosed here. No files are copied into or written inside the archive or live wave directories.

## Checks and limits

The verifier requires four assessed rows and four referenced scorecards per selected wave. It checks SHA-256 syntax/digest/byte size, strict JSON parsing, board schema/count types, stored zero qualified count and null champion, empty unverified list, false assessed qualification flags, certificate-identity syntax and reference-path binding, unique labels, matching scorecard titles, and the literal saved FAIL verdict metadata. It maps saved repository paths to pinned wave-relative suffixes without following those original paths. Traversal and noncanonical references are rejected. Certificate identities remain opaque references rather than regenerated hashes of design inputs.

For each scorecard the layout check requires three rectangular tables, their exact saved column headers, 8 component rows, 16 numbered gate rows and 18 evaluator rows. Gate ordinals and identifier uniqueness are checked. Measured-value, requirement and status cells remain opaque strings: the verifier does not evaluate their semantics, derive a verdict from gate rows, calculate any gap or summarize any gate's pass rate. A missing row or altered header/column structure fails the generic schema check. Any changed bytes also fail the pinned hash check, even when the layout remains valid. Other files referenced in scorecards are not audited.

All selected files are read upfront and reread after the group validation to reject changes during the audit. Missing required files fail closed. In the default/out-root mode w315–w318 are required and w319 is included when its directory is present. An explicit single-wave fixture must contain all five pinned files for its selected wave. No missing file is reported as verified.

## Portability and tests

`--wave-dir /relocated/fixture --wave w319` selects one relocated saved wave and bypasses Git metadata and original host paths. `--out-root /relocated/out` selects the wave folders beneath another root without Git discovery. The default discovers the common Git root and uses its live `out` folder. Neither override invents missing archive evidence.

The 11 new `Phase15GateSMarginAuditTests` cases use synthetic opaque observations and requirements. They cover layout and reference accounting, every-file byte tampering, missing synthetic rows/tables, changed headers/separators/column counts/ordinals, duplicate identifiers, stored verdict contradictions/duplicates, missing/extra files, malformed pins, false qualification/count metadata, reference identity/path/title mismatch, relocated CLI execution without Git, mutation during reread and unsafe pin paths before filesystem access. These tests do not test Gate S thresholds or real physical gate-margin consistency. The existing 64 tests are retained; the requested discovery suite now contains 75 tests.

```sh
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 experiments/rsv2_07/audit_phase15_gate_s_and_margins.py
taskset -c 14,15 env CARGO_BUILD_JOBS=2 python3 -m unittest discover -s experiments/rsv2_07 -p 'test_portable_audits.py'
git diff --check
```

The process inherits CPUs14,15 at nice10; another `nice -n 10` is not added because it would change niceness a second time. Actual command results, hashes and logs are recorded under `target/rsv2-07/phase15-scorecard-artifact-subset/20261018T125639Z/summary.json`. The Phase-10 test-source hash and Phase-7/8 study-file bindings are refreshed. Original numerical receipts and all wave files remain unchanged. No commit, push or PR is made.

## Pinned available bytes

| Artifact relative to live out | Bytes | SHA-256 |
| --- | ---: | --- |
| w315/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792293914730895747/w315_r01_p14_c50_master_scorecard.md | 6907 | 5dee2e79ee1684ca3d0a946950d0a5518bc4706a9df6b37dd1c83ae95c3fbdc7 |
| w315/by-content/74ed6ed4ab8bf8a042352e73ca6a51561821d515acc09f21ed83bbaa880ad94e/attempt-1792292704052296518/w315_r01_p10_c03_master_scorecard.md | 6907 | d7b1b95f220cc021cba6c0f819fdd7e1324a021a25a3118632afe2644feb9e05 |
| w315/by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792295146972012229/w315_r02_p10_c12_master_scorecard.md | 6907 | 33ebc7a24e0d013c0e5969a09f9f659dd81cfe3becaf35a52c38da4d9740410f |
| w315/by-content/b77c59c158cf89b22a5493bfe380f31397eeb3a77c05a56fb9db46a584c7c0cb/attempt-1792304121638424690/w315_r02_p14_c29_master_scorecard.md | 6907 | 4aaa9bb5e9c8de226186418c3c24926abc28453af519501a21baa6e773da5528 |
| w315/leaderboard.json | 3961 | 49f9dcc245b9a2d81518ccbd992fe9b9c445d521a25080baf9b0eb98a611924d |
| w316/by-content/234a9e89d25b63ad7300825df9ac88a8f78cac41957b17f43853d69184b5bd70/attempt-1792308261805508531/w316_r02_p10_c64_master_scorecard.md | 6909 | bd44256b86335361e456ebc78f0d90dd4c4c500affae8588382a63256bcac932 |
| w316/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792307082308955840/w316_r01_p14_c27_master_scorecard.md | 6907 | 7f7b99b9eab07b90f96a03cd5628b14e5000c4ff74df09b0fa03b02a71a3ec60 |
| w316/by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792305689989032813/w316_r01_p10_c01_master_scorecard.md | 6907 | 886cf60381547a5fb2f22982294e385c3b97bf053b6d701ac97b5c3da7bfa249 |
| w316/by-content/d715c60095b56f3f9fe20eb89dcff8fca15ad48eccd50f0e8d968e7117b2b55f/attempt-1792309463348233325/w316_r02_p14_c88_master_scorecard.md | 6909 | 1cd137a1ae4ba436b0e4502a5a44bec34b8aa425bd3b8fffdda127c21c9b75ee |
| w316/leaderboard.json | 3955 | 63df2d7b2c3613a4411b619b48200ec227ca9ef03515bc53e5e5dacb7405f8f5 |
| w317/by-content/234a9e89d25b63ad7300825df9ac88a8f78cac41957b17f43853d69184b5bd70/attempt-1792313495662966937/w317_r02_p10_c56_master_scorecard.md | 6909 | 1fc1d7486f9b21f32cbb6463491ca693d5cc21334297fa7304278be9be481ca3 |
| w317/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792312163831746164/w317_r01_p14_c27_master_scorecard.md | 6907 | 64367a5de1d2c69e41bdbc05e5821c3ef40871de8ddd751297092603d0be8a9f |
| w317/by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792310544027082374/w317_r01_p10_c01_master_scorecard.md | 6907 | 5015c9b9f7f29b81c36cb8b920dd89714022403e362cd3f53de399b7f30a46e8 |
| w317/by-content/d715c60095b56f3f9fe20eb89dcff8fca15ad48eccd50f0e8d968e7117b2b55f/attempt-1792314572567943870/w317_r02_p14_c96_master_scorecard.md | 6909 | 56a75afe3e792a2d5d81b3b68b37d19cee4692a708fe1bf2823964207cb8108b |
| w317/leaderboard.json | 3955 | 429efbe4be1db44befc662ed5fe7a4a1de8ebe3cb53beaa0902a88179b72bb0e |
| w318/by-content/3eb40f1b9d74669c3a9afd22833888e26542a7c52ae512bbfa3f77fe5525b2db/attempt-1792319745170851375/w318_r02_p14_c92_master_scorecard.md | 6909 | 9f863dd83a5e9cf488e3e5f70ef6ad9eda103642555c8fce2e21f8f41706b2ca |
| w318/by-content/44ac7f99fd172d52780482c32dffe9e012282355e941ecc8ebbf0c40ebc35a43/attempt-1792318358578003318/w318_r02_p10_c10_master_scorecard.md | 6909 | fb5a7109e2d683bdd2b20647661ccc16fef83b9924cd17a6a1ad042d1283f5aa |
| w318/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792316960856419476/w318_r01_p14_c27_master_scorecard.md | 6907 | 64a8e8fd72508a7f4453ace2587c3bd402df807c175c38c19687b2bcd5c78433 |
| w318/by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792315783786269184/w318_r01_p10_c01_master_scorecard.md | 6907 | ace16bc6ffd57881114bef41770498b43d30f69acb9f13b0c1385232265e60aa |
| w318/leaderboard.json | 3964 | 7eeeb41a5dd36718ee6d300b626c76b01e0be5215bd36c97d9c37ca354c654a9 |
| w319/by-content/47d60b04044f32fc84f17d915af58c44e0896e1f25ab97dda6587950f44b2850/attempt-1792325188215181150/w319_r02_p14_c45_master_scorecard.md | 6909 | d7e2160333e21f8a7d63392c746d6b6acb3805bee982220b24c3c02514e05e0e |
| w319/by-content/6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac/attempt-1792322311344751353/w319_r01_p14_c27_master_scorecard.md | 6907 | 73806aaf467b4d64e2ce79b4be286c93953ce88b041d9ba0ffcb98e3b0ea3804 |
| w319/by-content/89363bf867d1e07b9f91a4f72c181024f8823b4268987517fdc3d00a638d0d3c/attempt-1792323687719656617/w319_r02_p10_c21_master_scorecard.md | 6909 | f57710afa103528c24ab2305e96f009d6d23effebab3bbff731c26054b3c0a5e |
| w319/by-content/b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc/attempt-1792321160626983948/w319_r01_p10_c01_master_scorecard.md | 6907 | 542d65a9c1b4dbbd4ba4393d916b6c14d5e2d7dbdfd039f8e6a922e60f4a1606 |
| w319/leaderboard.json | 3983 | df129159ec3d3e3d95044827f190c082b0ccd4165fc935dca002011b3b05fadf |
