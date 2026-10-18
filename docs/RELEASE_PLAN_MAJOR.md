# Jarambit Major Release Plan (`docs/RELEASE_PLAN_MAJOR.md`)

**Authors:** Jepson Taylor, `super_captain`, & `captain_jarambit` (`2740538.captain_jarambit`)
**Authority:**
1. `/home/ubuntu/xbabe2_kicker/captains-20261017/gb_jarambit_release_1120.txt` (`GB ORDER 11:25 AM MDT Oct 18 — JARAMBIT MAJOR RELEASE + STALL ROOT CAUSE`)
2. `/home/ubuntu/xbabe2_kicker/captains-20261017/JEPSON_RULING_20261018T1658Z_speed_over_ci.md` (`SC-DEC-078`: Speed Over CI)
3. `/home/ubuntu/xbabe2_kicker/captains-20261017/JEPSON_RULING_20261018T1736Z_jarambit_release_a.md` (`11:36 AM MDT Oct 18 2026 — Jarambit major release = OPTION (a)` + `ADDENDUM 11:36 AM MDT — Jepson: weighted objectives, no hard fails`)

---

## 1. Executive Summary & Jepson Ruling `2026-10-18T17:36Z` Re-Scope (`J6(a)` + Weighted Objectives Addendum)

Per `JEPSON_RULING_20261018T1736Z_jarambit_release_a.md`:
1. **Option (a) Release Scope:** Ship the pipeline, the 26 verified per-gate checks (explicitly labelled `"check built + verified — not all-26 hard-gate design passed"`), the **Ultralight Running Knife** weighted-objective evolution pipeline (`W_ULTRALIGHT_v1`), and an honest `"no design qualified on all 26 legacy hard gates yet / best-so-far weighted design"` report. Legacy all-26 hard-gate winner certification (`CERT-01`, `CERT-02`, `DOC-03`) moves to the next release (`v0.32.0`).
2. **Weighted Objectives, No Hard Fails in Selection (Addendum 11:36 AM MDT):**
   - Replace the 26 hard pass/fail gates in the evolution loop with a weighted global fitness (`W_ULTRALIGHT_v1`, `[0.0, 100.0]`) tailored for an **ULTRALIGHT RUNNING KNIFE** (carry mass/weight `30%` and carry ergonomics/ring comfort `25%` weighted highest; cutting/B-rep geometry `15%`; structural/safety checks `30%` kept as smooth normalized objectives $\min(1.0, \text{achieved}/\text{target})$, not vetoes).
   - **No hard failures in selection:** Keep logging every raw per-gate metric and its legacy threshold so reports show exactly how far each design is from the old hard limits.
   - **Monotone non-decreasing global fitness (Hall-of-Fame elitism):** Plotted per run (`fitness_curve.png` / `fitness_curve.json`), with the best-so-far design and its 26-gate breakdown saved to S3 each generation.
   - **Multi-host weighted runs:** Launch on `xbabe1` (`102` CPUs) and `xbabe3` (never `xbabe0`) and report the first fitness curve.
3. **Target Schedule:**
   - **`J3` Feasibility Verdict (`docs/FEASIBILITY_VERDICT_J3.md`):** `19:30Z` (`1:30 PM MDT`) by `1242467.codex_jarambit_gate_diag`.
   - **Release Candidate (`RC-01`..`RC-04`):** `~22:30Z` (`4:30 PM MDT`).
   - **Final Validation (`FV-01` — full CI on final `main`, all receipts re-verified on exact SHA, independent red-team by `2132048.codex_jarambit`):** `~00:00Z` (`6:00 PM MDT`).

### Classification of `G16-15` (`plate_gci_valid_3level`)
- **Dual-Predicate Classification:**
  1. **`W_CAD_v2` Gate (`ops/ci/evaluate_cad_candidate.py:654–657`):** Evaluates `bool(gci.get("sequence_valid") and gci.get("observable_valid"))` on the 3-level ($4.0, 1.0, 0.25\text{ mm}^2$) failure-sequence mesh study bound to solved `fem::report_fields` triangulation hashes (`mesh_hash`). **Baseline:** **Met (`true` on `28/28` Stage-2 candidates across `w311`–`w318`)**.
  2. **Standalone Battery Check (`ops/ci/cad_battery.py:333–343`, `plate.gci`):** Evaluates `bool(gci.get("seq_ratio_gci", {}).get("admitted"))` (`fit_status == "admissible_estimate"`). **Baseline:** **Met (`2/28` in `w311` and `1/6` on `w408t3_r03_p14_c17`)**.
- **Status:** Closed as `DONE` (`check built + verified — not all-26 design passed`) in PR #119.

---

## 2. Full 37-Row Truth Table (`J2` + `JEPSON_RULING_20261018T1736Z_jarambit_release_a.md`)

| Task ID | Current Status | Blocker Class | Owner Screen | Next Action (Single Concrete Step + PR Target) | ETA (UTC / MDT) |
|---|---|---|---|---|---|
| `WP-01` | `DONE` | `none` | `2740538.captain_jarambit` | Complete on `main` (`d046093432864ac3cc619dc40c177c452e1e967a`); re-verify receipt at `FV-01`. | `DONE` |
| `AUTO-01` | `DONE` | `none` | `2291514.agy_jarambit_screen` | Complete on `main` (`253944829cb58e262a16b13f4a72ff9adc745425`); re-verify receipt at `FV-01`. | `DONE` |
| `AUTO-02` | `DONE` | `none` | `2291514.agy_jarambit_screen` | Complete on `main` (`7cb952ef2a30e3e0e3fb35d0429be70192817eb6`); re-verify receipt at `FV-01`. | `DONE` |
| `AUTO-03` | `DONE` | `none` | `2291514.agy_jarambit_screen` | Complete on `main` (`513e215e20c84651583daedb31660fe74f6462a3`, 57/57 CLI unit + 2/2 integration tests). | `DONE` |
| `AUTO-04` | `IN_PROGRESS` | `compute` (re-scoped to `W_ULTRALIGHT_v1` weighted runs) | `2291514.agy_jarambit_screen` + `2455097.jarambit_evo3c_pair` | Land `W_ULTRALIGHT_v1` weighted-objective scoring + Hall-of-Fame elitism + per-gen S3 sync (PR #120), run weighted waves on `xbabe1` (102 CPUs) & `xbabe3`, commit first monotone fitness curve + best-so-far report (PR #121). | `19:30Z` (`1:30 PM MDT`) |
| `G16-01` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`crates/crescent/src/{export,step_export,cad_ingest}.rs`, `ops/ci/evaluate_cad_candidate.py`). | `DONE` (PR #119) |
| `G16-02` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`crates/crescent/src/gates.rs`, `crates/redteam/src/lib.rs`). | `DONE` (PR #119) |
| `G16-03` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`crates/crescent/src/{pheno,aesthetic,certification}.rs`). | `DONE` (PR #119) |
| `G16-04` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`crates/crescent/src/aesthetic.rs`). | `DONE` (PR #119) |
| `G16-05` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `1908142.codex_jarambit_w2` | Wire normalized weighted objective $\min(1, F_{\text{lat,cert}}/180.0)$ (`3.5%` weight) + raw threshold/deficit logger + unit test in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `G16-06` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `1908142.codex_jarambit_w2` | Wire normalized weighted objective $\min(1, F_{\text{lat,cert}}/260.0)$ (`2.5%` weight) + raw threshold/deficit logger + unit test in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `G16-07` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `1908142.codex_jarambit_w2` | Wire normalized weighted objective $\min(1, F_{\text{lat,strict}}/260.0)$ (`2.0%` weight) + raw threshold/deficit logger + unit test in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `G16-08` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`ring_rob_cert_n >= 1500.0` check verified). | `DONE` (PR #119) |
| `G16-09` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`twist_body_sf >= 1.0` check verified). | `DONE` (PR #119) |
| `G16-10` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `2199667.codex_jarambit_tip_w1` | Wire normalized weighted objective $\min(1, \text{SF}_{\text{tip}})$ (`1.25%` weight) + raw threshold/deficit logger + unit test in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `G16-11` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `2199667.codex_jarambit_tip_w1` | Wire normalized weighted objective $\min(1, \text{SF}_{\text{tip,strict}})$ (`1.25%` weight) + raw threshold/deficit logger + unit test in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `G16-12` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `2199667.codex_jarambit_tip_w1` | Wire normalized 3D $P_2$ solid-tip objective $\min(1, \text{SF}_{\text{solid}})$ (`3.0%` weight) + raw threshold/deficit logger + unit test in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `G16-13` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`ops/ci/side_a4.py`, `crates/physics/src/side_a4.rs`). | `DONE` (PR #119) |
| `G16-14` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`seq_ratio_cert >= 1.20` check verified). | `DONE` (PR #119) |
| `G16-15` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`crates/crescent/src/gci.rs`, `fem_solid/gci.py`). | `DONE` (PR #119) |
| `G16-16` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`fem_solid/cross_check.py`, `certification.rs::independent_pass`). | `DONE` (PR #119) |
| `RSV2-01` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`bench.rs::rs_v2_eval` `/gates/aesthetic_and_geometric`). | `DONE` (PR #119) |
| `RSV2-02` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `1908142.codex_jarambit_w2` | Wire non-veto weighted objective score + legacy `/gates/floors_828_joint` deficit report in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `RSV2-03` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `2199667.codex_jarambit_tip_w1` | Wire non-veto weighted objective score + legacy `/gates/twist_floor_body_and_tipsub_strict` deficit report in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `RSV2-04` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `2199667.codex_jarambit_tip_w1` | Wire non-veto weighted objective score + legacy `/gates/solid_tip_sf` deficit report in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `RSV2-05` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `1908142.codex_jarambit_w2` | Wire non-veto weighted objective score + legacy `/gates/strict_hunter_lateral` deficit report in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `RSV2-06` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`bench.rs::rs_v2_eval` `/gates/seq_hard`). | `DONE` (PR #119) |
| `RSV2-07` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `1908785.codex_jarambit_hm_w3` | Separate `bore_collar_comfort` (`9.0%` weight) from `human_max_overload` (`2.0%` weight) in `W_ULTRALIGHT_v1` + log legacy deficits in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `RSV2-08` | `IN_PROGRESS` | `decision` -> resolved (`W_ULTRALIGHT_v1`) | `1908142.codex_jarambit_w2` | Wire normalized Goodman L7 objective $\min(1, \text{SF}_{\text{Goodman}})$ (`2.0%` weight) + legacy deficit report in PR #120 and close row as `"check built + verified"`. | `18:30Z` (`12:30 PM MDT`) |
| `RSV2-09` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`certification.rs::sharpness`, `#77`). | `DONE` (PR #119) |
| `RSV2-10` | `DONE` | `none` (check built + verified) | `2291514.agy_jarambit_screen` | Closed as `"check built + verified — not all-26 design passed"` in PR #119 (`certification.rs::point_profile`, `#78`). | `DONE` (PR #119) |
| `CERT-01` | `BLOCKED` | `winner-bound` (deferred to `v0.32.0`) | `2291514.agy_jarambit_screen` + `2132048.codex_jarambit` | Deferred to `v0.32.0` per `JEPSON_RULING_20261018T1736Z_jarambit_release_a.md` ("Winner certification (design passing all 26 gates) moves to the NEXT release"). | Deferred (`v0.32.0`) |
| `CERT-02` | `BLOCKED` | `winner-bound` (deferred to `v0.32.0`) | `2740538.captain_jarambit` + `1409298.agy_jarambit_paper` | Pre-authorized via `SC-DEC-001`; deferred to `v0.32.0` alongside `CERT-01`. | Deferred (`v0.32.0`) |
| `DOC-01` | `DONE` | `none` | `1409298.agy_jarambit_paper` | Complete on `main` (`21425fe2948ab3941eedbe9710ebc3d29427faae`); re-verify double-build PDFs at `FV-01`. | `DONE` |
| `DOC-02` | `DONE` | `none` | `1409298.agy_jarambit_paper` | Complete on `main` (`9443b11a76f8e552ec692de7f4aee106b19fbbd0`); update with weighted fitness curve at `RC-03`. | `DONE` |
| `DOC-03` | `BLOCKED` | `winner-bound` (deferred to `v0.32.0`) | `1409298.agy_jarambit_paper` | Provisional 19 `.webp` plates committed & verified; final winner re-render deferred to `v0.32.0` with `CERT-01`. | Deferred (`v0.32.0`) |
| `DOC-04` | `DONE` | `none` (`J5` verified on `main`) | `1409298.agy_jarambit_paper` | Closed in PR #119 (`5/5` check commands exit `0` on `main` `513e215e20c84651583daedb31660fe74f6462a3`, receipt `/home/ubuntu/jarambit-audits/2026-10-18T1729Z-doc04-closure/RECEIPT.md` `SHA-256: 0f89d92fddf5f1aabd6405cc3a04e73fa74236139acc7caa050d02fab330976b`). | `DONE` (PR #119) |

---

## 3. Release-Candidate Checklist (`J8` — `RC-01` .. `RC-04` & `FV-01`)

- [x] **`RC-01` — Scope Decision Recorded (`J6(a)` + `JEPSON_RULING_20261018T1736Z_jarambit_release_a.md`):**
  - Major release (`v0.31.0`) ships the `./evolve.sh` pipeline, the 26 verified per-gate checks (labelled `"check built + verified — not all-26 design passed"`), the `W_ULTRALIGHT_v1` Ultralight Running Knife weighted-objective evolution loop (monotone Hall-of-Fame global fitness + S3 per-generation sync), the `J3` Feasibility Verdict (`docs/FEASIBILITY_VERDICT_J3.md`), and the best-so-far design report listing raw per-gate metrics vs. legacy hard thresholds.
  - All-26 legacy hard-gate winner certification (`CERT-01`, `CERT-02`, `DOC-03`) is explicitly deferred to `v0.32.0`.
- [ ] **`RC-02` — All In-Scope Major-Release Rows Closed (`34 / 37 DONE`, `3 / 37 DEFERRED TO v0.32.0`):**
  - **PR #119 (`21 / 37 DONE`):** Closes `DOC-04` + 14 baseline-met gate checks (`G16-01..04, 08, 09, 13..16, RSV2-01, 06, 09, 10`) and commits the `W_ULTRALIGHT_v1` weight table + rationale.
  - **PR #120 (`33 / 37 DONE`, ETA `18:30Z` / `12:30 PM MDT`):** Implements `W_ULTRALIGHT_v1` global weighted fitness in `crates/cli/src/auto_evolve.rs` and `ops/ci/evaluate_cad_candidate.py` (no hard failures in selection, monotone Hall-of-Fame elitism, S3 per-generation upload, legacy threshold deficit table) and closes the remaining 12 gate rows (`G16-05..07, 10..12, RSV2-02..05, 07, 08`) as `"check built + verified + weighted objective active"`.
  - **PR #121 (`34 / 37 DONE`, ETA `19:30Z` / `1:30 PM MDT`):** Commits `docs/FEASIBILITY_VERDICT_J3.md` (`J3`) + first weighted evolution fitness curve & best-so-far design breakdown from `xbabe1` (`102` CPUs) and `xbabe3` (`AUTO-04`).
- [ ] **`RC-03` — Regenerate & Verify Public Documentation (`ETA 21:30Z` / `3:30 PM MDT`):**
  - Run `gen_status.py`, `gen_readme.py`, `gen_whitepaper.py`, `verify_figures.py`, `verify_media.py`, and `build_pdf.py --verify-reproducible` so `README.md`, `STATUS.md`, and `docs/WHITEPAPER.md` state plainly that the 26 legacy hard gates were replaced by `W_ULTRALIGHT_v1` weighted objectives per Jepson (`11:36 AM MDT Oct 18`), report the monotone fitness curve, and list which legacy thresholds the best-so-far ultralight design does not meet.
- [ ] **`RC-04` — Version Bump & `RELEASES.md` (`ETA 22:30Z` / `4:30 PM MDT`):**
  - Bump workspace version to `0.31.0`, author `RELEASES.md` (`v0.31.0`), and open the release-candidate PR.
- [ ] **`FV-01` — FINAL VALIDATION Gate (`ETA 00:00Z` / `6:00 PM MDT` — Mandatory Before Tagging `v0.31.0`):**
  - **(a) Full GitHub Actions CI green on final `main` SHA** (`version-check`, `d1-gates`, `render-marketing-smoke` covering every commit merged under `local-CI only`).
  - **(b) Receipt Re-Verification:** Re-verify every task receipt (`WP-01`, `AUTO-01..04`, `G16-01..16`, `RSV2-01..10`, `DOC-01`, `DOC-02`, `DOC-04`) against the final `main` SHA.
  - **(c) Independent Red-Team Audit:** `2132048.codex_jarambit` executes an independent red-team audit of the final `main` SHA and issues a sealed `PASS` receipt under `/home/ubuntu/jarambit-audits/`.
