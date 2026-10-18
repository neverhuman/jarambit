# Phase-21 Round-0 seed versus mutated inventory audit

**PARTIAL: historical counts and linkage verified; seed-input provenance is missing.** The saved wave JSONs do not record the launch seed CSV list or an explicit seed-identity set. Therefore exact seed-versus-mutated counts remain null, rather than treating repeated identities or family block positions as proven seeds.

The table gives occurrence counts against the two repeated Phase-20 identity references. Each pair is reference-matching / other. Other means a different saved identity, not proven mutation or novelty. Counts include repeated rows; distinct-identity counts and family totals are retained in JSON.

| Wave | Round-0 births | Round-0 screens | Round-0 assessed | Later-round assessed | All assessed | Qualified | Unverified |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| w315 | 1 / 99 | 1 / 82 | 1 / 1 | 1 / 1 | 2 / 2 | 0 | 0 |
| w316 | 2 / 102 | 2 / 18 | 2 / 0 | 0 / 2 | 2 / 2 | 0 | 0 |
| w317 | 2 / 102 | 2 / 18 | 2 / 0 | 0 / 2 | 2 / 2 | 0 | 0 |
| w318 | 2 / 102 | 2 / 47 | 2 / 0 | 0 / 2 | 2 / 2 | 0 | 0 |
| w319 | 2 / 102 | 2 / 54 | 2 / 0 | 0 / 2 | 2 / 2 | 0 | 0 |
| w320 | 2 / 102 | 2 / 54 | 2 / 0 | 0 / 1 | 2 / 1 | 0 | 1 |

Recorded totals: 620 Round-0 birth entries, 284 Round-0 screen entries and 12 Round-0 assessed events. Across both rounds, the ledgers contain 23 assessed events and 1 separate unverified event.

Reference certificate identities:

- 6af25de9e52b5a836fab9764f644300d06f2f1abc056008721103cf27d8a0aac
- b2cf2b3384f20b399583fdcb3a55c5244bdba55accce6739cf63d4f32d7003cc

certificate_identity is the saved native identifier. It is not the SHA-256 of CSV bytes, and neither field alone establishes that a candidate was an input seed. No native identity computation or genome normalization is performed. No individual non-reference candidate recommendations or rankings are emitted.

Source reference: crates/cli/src/auto_evolve.rs at commit 141e694532bb73a720bf76c1e54e19940a93abd1, SHA-256 1eaadb175b1ba3ee89eedf7cf0cbae2fac8131a05374da44c6721032f5513524. Only the immutable git-show blob is inspected; the other checkout's working file is not read. The source permits overriding its default seed inputs and separately records Round-0 births. A source listing does not recover historical invocation arguments or prove the producer version of old wave artifacts.

The previous Phase-20 audit records 23 historical assessments, 11 distinct identities and 12 repeat events beyond first occurrence. Those 12 are duplicate identity events, not 12 independently established seed re-assessments. The two repeated reference identities account for ten of those repeats; the remaining repeats belong to two other identity groups. This inventory does not relabel that evidence as seed lineage.

All 30 saved root JSON files under out/w315 through out/w320 match the earlier reviewed SHA-256/size pins and Phase-20 bindings. These live files are not tracked Git wave snapshots in this checkout. The tracked Phase-20 receipt is under results/, rather than the parent directory named in the first assignment. The two requested verifier/test filenames did not exist and are added for this audit.

Screen rows must link to exact same-round birth positions, families and identities. Assessment membership uses saved screen labels to distinguish rounds: an identity seen again in a later round does not turn that event into a Round-0 assessment. Unverified events stay separate. qualified_count=0 and champion=null are required for every saved leaderboard; this checks recorded metadata only.

The command verifies the source blob and prior receipt pins, strict JSON parsing, all root file bytes and complete recomputed receipt. It rereads inputs to detect changes during the audit. Output paths are repository-relative. --artifact-root selects a relocated saved repository; --source-blob supplies a byte-identical exported committed blob without Git discovery. Missing inputs or altered hashes fail closed. --write explicitly writes the report and receipt; normal verification is read-only.

No simulations, selection-rule replay, geometry changes, optimization or physical qualification checks are performed. Completing the seed-versus-mutated portion requires a retained launch seed-input manifest or equivalent independently bound provenance for each wave.

```sh
python3 experiments/rsv2_07/verify_rsv2_07_receipts.py
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
git diff --check
```

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2.
