# Phase-22 w320 leaderboard and screen schema provenance audit

**PASS for saved schema, byte provenance and cross-consistency.** Six saved files match SHA-256 and byte-size pins. The JSON ledger records 3 assessed rows, qualified_count=0, champion=null and 1 separate unverified attempt. The Markdown table contains the three assessed rows only; it does not list the unverified attempt.

| Saved round | Birth entries | Pending screen rows |
| --- | ---: | ---: |
| 0 | 104 | 56 |
| 1 | 96 | 49 |

Births require integer round/RNG seed metadata, a nonempty source, a unique CSV header and rows of matching width. Round numbering, seed/source consistency and cross-round identity/CSV bindings are checked. Screens require unique, correctly formed labels, same-round birth position/family/identity matches, pending status and finite nonboolean numeric fields. Stored numbers are checked as schema values only.

The v2 leaderboard requires integer assessment/qualification counts, boolean false qualified flags, a null champion, three distinct assessed labels and one separate unverified record. Every assessed/error identity links to the saved screen and birth rows. Genome/scorecard/error references must identify the same candidate and attempt; only normalized repository-relative references are used internally. Error reason text and original host paths are not copied into the receipt.

Markdown checks require the saved title, eight-column header/separator, assessed-row count/order/labels and matching stored flags. Four displayed numeric strings must match the JSON's documented decimal formatting; the displayed battery count must match its integer JSON value and the saved /18 format. This is transcription verification, without recalculating physical metrics or gate verdicts. Omitted unverified entries are recorded as a presentation coverage limit, not recast as assessed or qualified.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| out/w320/leaderboard.json | 3418 | c86614effb2283001bf9e0cb0293226d2a06fd8e6aac8006b67744b03adb9a7c |
| out/w320/leaderboard.md | 590 | 2f994cb80d04604b482645878da23bfc80a4fffd0d707ae7ef7d93d0780b8755 |
| out/w320/round_00_births.json | 177413 | f725189be663ee30646a52a84dd049648a7b39c7e39a58e7d9384e5d626e83c7 |
| out/w320/round_00_screen.json | 23213 | adb40eb24b76e1ce1120c5255f5db90f69c4824fad1ca19af60b8aa17a6134f5 |
| out/w320/round_01_births.json | 165083 | 195b11e4828697ea8db1a6898330a62e03adcfd7f12c2671a71e03ef80a0cbdb |
| out/w320/round_01_screen.json | 20297 | a4494238b88b330502a6d36910eacb947df9d511b587cd656c388a682bd1f274 |

The five JSON pins reuse the reviewed Phase-20/w320 registry. The Markdown pin is newly recorded from its saved bytes for Phase-22. Inputs are live saved artifacts, not newly generated simulations or claimed committed wave snapshots. All six source files and the Phase-21 receipt are reread to detect mutation during the audit. Missing files, invalid digests/sizes, altered bytes, duplicate JSON keys, nonfinite values, schema errors and inconsistent references fail closed.

Phase-21 binding: experiments/rsv2_07/phase21_round0_seed_vs_mutated_inventory_audit.json (SHA-256 bc665634397714b3af19dda65b3c36b00c1c02d81132957ad8cd3d38b2c7547d). The recomputed w320 inventory must match that receipt exactly. Phase-21 seed-versus-mutated classification remains partial because launch seed provenance is absent; this audit supplies no replacement seed evidence.

All emitted filesystem paths are canonical repository-relative POSIX paths. --artifact-root and --repo-root permit relocation without Git discovery. Normal runs verify the complete saved Phase-22 receipt and never write source artifacts. --write explicitly refreshes this report and receipt. No simulations, candidate selection, design changes, optimization or physical qualification checks are performed.

```sh
python3 experiments/rsv2_07/verify_rsv2_07_receipts.py --phase 22
python3 -m unittest discover -s experiments/rsv2_07 -p 'test_*.py' -v
git diff --check
```

Execution uses CPUs14,15 at inherited nice10 with CARGO_BUILD_JOBS=2. Controlling study hashes are synchronized for the new receipt, report and verifier/tests.
