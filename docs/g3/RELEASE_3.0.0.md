# Jarambit v3.0.0: the G3 generation

v3.0.0 makes G3 the headline of the repository: a one-piece Ti-6Al-4V karambit for laser powder-bed fusion, designed
around the hand and evolved from scrambled Sobol samples. The README and the G3 paper
(`docs/g3/paper/`; the v3.0.0 PDF is attached to the [v3.0.0 release](https://github.com/neverhuman/jarambit/releases/tag/v3.0.0)) describe the best individual of the cohort of record. Both are
generated from one receipt, [`evidence/g3/v3.0.0/release.json`](../../evidence/g3/v3.0.0/release.json); no number in either
is typed by hand.

## Owner requests behind this release (2026-10-26)

- "The blade edge seems weird, junky and dull, maybe the wrong angle … one of the ideal sharpness angles for a knife for
  the entire two-sided blade."
- "The last leading knife was missing the two serrated edges", and from the owner via AGY: "serrated specific tests …
  the most formidable, but also we don't want to break serrated teeth".
- "Address ALL of these issues" (the G3.1 red-team audit and the draft-d self-audit).
- "The README is 100 % accurate with the best individual (no older data), also it should show the evolution", and
  "README and the paper 100 % up to date, with the best possible structure and make-up".
- "Do whatever you need to squash/merge these … no need to wait for CI … We want v3.0.0 rolled out ASAP."

## What changed

**Decoder r2 (G3.0 schema; PRs #170, #171).**
- EDGE-1..5: one primary grind per edge at 17.5–22.5° per side.
  - The angle is measured normal to the actual serrated edge and the grind is carried at full height, with no
    micro-bevel and no shoulder.
  - The grind runs unbroken to the point; only a 5–12 % plunge by the guard is unsharpened.
  - Gate S3 reads the true edge-normal section over 0–2 mm.
- SERR-1..4: both edges serrated.
  - Depth is 0.8–2.2 mm on the inner edge and 0.8–2.0 mm on the spine edge, with at least 3 full-depth scallops per edge.
  - The pitch is 5–9 mm and the depth at most a quarter of the pitch.
  - The serrated span starts past the plunge and ends at least 18 mm from the point.
  - Every tooth between two scallops is checked under a 180 N snag along the edge (times an Inglis gullet notch
    factor) and a 60 N pry across the blade: in the decoder tests on 60 Sobol designs and on the released design. The
    check is not a term of the score.
- Measured terms that were formulas or dead weights:
  - human-max twist is the blade-body plate solve scaled to 16.2 N·m, where it used to be capped at 0.337;
  - buckling is a lateral-torsional estimate, where it used to be zero credit.
- The solid-tip TET10 mesh converges on sharp points (0.15 mm² production mesh).
- Every island logs its evolution history.

**Release pipeline.**
- The cohort of record runs at the merged decoder commit.
- The ranking of record is measured at h 0.2 mm, the grid of the print file, and re-run on Linux x86_64 as the platform
  of record. The cohort's own h 0.4 leader turned out to be a coarse-grid artifact: its mass rose by about half at
  h 0.2. The README and paper show the grid check.
- The winner is certified with a hex8 voxel FE of its printed geometry.
- The winner's blade is re-meshed at 0.05 mm from the exact decoder law for the CAD and renders.
- Binary STL files are in millimetres.

**Documents.**
- `ops/ci/gen_readme.py` now writes the whole README from the receipt. Its tests fail if an older design or run
  identifier appears, or if a figure or CAD file is missing.
- The G3 paper is new (`docs/g3/paper/`); `g3-release-tex` turns the receipt into its macros.
- The v1 white paper stays in the repository as the historical record of the earlier study (header bumped to v3.0.0).
- The claim checker no longer requires the v1 RS verdict block in the README, because the README no longer reports the
  v1 design. The README is still scanned for unscoped PASS claims, and the verdict block stays required in
  `docs/PROVISIONAL.md`.

## Process notes

- Under the owner's "no need to wait for CI" instruction, #171 and the release PR were merged without waiting for CI or a
  fresh exact-head neverhumanbot review. This is labelled on each PR. Post-merge findings will be fixed in v3.0.x.
- The release cohort is shortened (see the README for its size) so that the release could ship the same day.
- G3.1 (#169: SI-50, GRIP-μ, W_ERGOLIGHT_v2) is not in this release.
  - Its decoder lives in the same hashed source files, so any change there after the cohort started would invalidate
    this release's receipts.
  - Its documents still carried smoke-test numbers that the audit measured differently.
  - It is planned for v3.1 after a rebase onto this release, a frozen decoder and its own cohort.

## Audit items and where they stand

| Item | Status in v3.0.0 |
|---|---|
| Unground, thick point (A1); s1 gene prior left the tip end of the spine edge unground (S5) | Fixed by EDGE-1..5 |
| Human-max twist capped at 0.337 (S2) | Fixed: measured on the blade body; certified by voxel FE |
| Buckling weight dead (S3) | Fixed: analytic lateral-torsional estimate, labelled as a surrogate |
| Hand-edge radii read from genes but labelled "measured" (S4) | Relabelled `design_law` in the tier provenance |
| Tip FE mesh not converged on sharp points | Fixed: 0.15 mm² production mesh, checked against 0.08 mm² |
| Coarse-grid mass undercount (A8) | Still present in G3.0 at the h 0.4 search grid (the cohort's leader exploited it). Ranking and mass of record are now measured at h 0.2; a finer search grid or a convergence check during the search is v3.1 work |
| Results differ between arm64 and x86_64 (S6) | Final ranking re-run on Linux x86_64, the platform of record |
| Grip strength formula optimistic against voxel FE | Reported side by side with voxel FE in the README and paper |
| G3.1 items (A2–A7, B1–B9, C1–C3) | Deferred with G3.1 to v3.1 |
