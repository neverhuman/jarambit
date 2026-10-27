# G3 provenance: the documents behind RS-karambit-v3 and the G3 decoder

`requirements/RS-karambit-v3.toml` and the G3 sources cite several documents that were not in the repository. This
table records each one, with its SHA-256, and what G3 took from it. The four constraint documents are the owner's
private review inputs and are not published here; their hashes pin the exact versions used.

| Cited as | Document | SHA-256 | What G3 took from it |
|---|---|---|---|
| `tips/constraint/design.md` | *Engineering Specification: Ergonomic Grip, Finger Groove, and Low-Inertia Ring Redesign* (JARAMBIT-SPEC-ERG-002), a review of the v1 design `ec3i042_r01_p10_c56` | `c0195ec468ece862…` | Zones 1–3 of the hand zone (fixed blade; the folding Zone 4 is omitted): finger pitches 22.0 / 21.0 / 19.5 mm and choils (CHK-GRIP-01), the 250 N grip squeeze at σ_vM ≤ 450 MPa (CHK-FEA-01), and the low-inertia ring behind the spin-balance factor |
| `tips/constraint/jarambit_ec3i042_grip_ring_engineering_spec.md` | *Jarambit ec3i042 — Grip & Ring Redesign: Engineering Spec and Critical Review* | `945956cc87293690…` | The 24.9 mm bore (kept rather than its 24.0 mm option), the 8.0 mm comfort-fit lip, and the guard-height target (≥ 10 mm, 12 mm target). Its grip-perimeter advice (82–90 mm) was not taken: the owner chose a compact grip (65–75 mm, height-to-width 1.5–2.0) |
| `tips/constraint/Jarambit_Grip_and_Ring_Engineering_Spec.md` | *Grip & Ring Engineering Specification: Critical Feedback and Revision Recommendations* | `271b56d8079acfdb…` | A second review of the same ring and grip, consistent with the bore and lip above |
| `tips/constraint/Jarambit_Handling_Safety_Review.md` | *Jarambit CAD: handling comfort and safety review* (24 October 2026, preliminary desktop review) | `ed39679605829592…` | Its caution against sharp hand-contact edges, behind the hand-edge radius factor of the ergonomic multiplier E; it states that only a physical review can establish comfort |
| `G3_GENOME_SPEC.md` v1.1 | The G3 genome draft (2026-10-25) | `2f5a930fcd6ce822…` | Encoding, strict loader, construction rules and module layout; committed here as [`G3_GENOME_SPEC.md`](G3_GENOME_SPEC.md) |
| `tools_frozen_v3/hzm.py` | The Python prototype of the hand-zone decoder, whose first 94 genes `g3_genome.rs` matches in order | `a5dce5f7840db1a8…` | Gene order of the hand zone; parity fixtures `crates/g3/tests/fixtures/g3hz_parity_*.json`. Not committed: the repository adds no Python |

The owner's directives of 2026-10-24 and 2026-10-25 listed in the RS-karambit-v3 header (fixed blade, no sheath, 300 N
ring bearing and 30 N ring back at SF 2, the 85.725 mm reach with a 4.0 in sharpened arc per edge, compact grip,
spinning balance) are recorded there and in `docs/RELEASES.md`.
