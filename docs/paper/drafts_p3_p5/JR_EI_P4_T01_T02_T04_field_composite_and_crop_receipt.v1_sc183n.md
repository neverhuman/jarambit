# JR-EI-P4-T01 / T02 / T04 — White-Background Horizontal Hero, Normalized Field Composite, and >= 90% Saliency FEM Crops

## 1. Scope & Governance (`SC-DEC-183N` §3 & `SC-DEC-183M`)
- **Tasks Executed:** `JR-EI-P4-T01`, `JR-EI-P4-T02`, and `JR-EI-P4-T04` (bounded inside `docs/paper/drafts_p3_p5/`).
- **Validation Posture:** `implemented_unvalidated` (`CI_HOLD_UNTIL_P8 = true`; no `git push` to `origin`; no new CAD or FEM solver runs executed).
- **Reader-Facing Naming (`JR-R4`):** Uses integer label **Individual 2000** (`assessed, not qualified`; `qualified_count = 0`, `champion = null`). Internal filenames, hostnames, paths, and raw SHA-256 digests are confined to this machine-auditable receipt and excluded from figure panels and manuscript prose.

## 2. Source-Bound Provenance & Pose Convention
- **Source D1 Plate Artifact:** `evidence/evolve_certifiable/automatic/by-content/db5d8e32e99d0b2d8ef6d7c7f9622a0601487c25b80879bbc8be1a502693c55e/attempt-1792206525341070917/cert_pipeline_r01_p14_c02_D1.json` (`5,711,420` bytes, `sha256: c2811ed8877904254ddc953dc888aedc3d7791b69c040dd6513fe0b763997bab`).
- **Proper 3D Pose Transform:** $R = R_z(-6^\circ)\,R_x(180^\circ) \in \mathrm{SO}(3)$ with $\det(R) = +1.0$ (no 2D mirrored reflection), placing the retention ring at left, the handle horizontal within $2^\circ$, and the blade curving upward to the right on a pure white (`#FFFFFF`) background.

## 3. Normalized Multi-Load Composite (`Recipe A` vs `Recipe B`) & Contributor Coverage
- **Mesh Support:** `13,503` nodes, `25,209` triangles, total plate area `3,816.272554 mm²`.
- **Per-Map Support:**
  - `env_vm` (scaled multi-case von Mises envelope, $\mathrm{MPa}$): `25,134` active cells, `75` tip-excluded cells ($\le 4.0\,\mathrm{mm}$ from blade tip forced to `0.0` by producer, rendered in neutral gray), peak `1,388.6695 MPa` (triangle `10564`).
  - `lat` (unit lateral bending response, $\mathrm{MPa/N}$): `25,209` active cells, peak `0.5878 MPa/N`.
  - `ring` (unit ring-traction T6 centroid response, $\mathrm{MPa/N}$): `10,334` active cells (`ring > 0`), `14,875` seated-ring masked cells (`ring == 0`, rendered in neutral gray), peak `0.0264 MPa/N` (triangle `7071`).
  - `twist` (unit chord-torsion response, $\mathrm{MPa/(N\,mm)}$): `25,209` active cells, raw peak `34.8589 MPa/(N mm)` (triangle `10685`, inside $4\,\mathrm{mm}$ tip exclusion), post-exclusion capacity peak `0.6611 MPa/(N mm)` (triangle `10564`).
- **Contributor-Count Map (`Recipe A`, 4-Map Mean Percentile):**
  - `4/4` maps active: `10,334` triangles (unmasked ring/neck/handle domain)
  - `3/4` maps active: `14,800` triangles (seated ring/blade domain outside tip exclusion)
  - `2/4` maps active: `75` triangles ($4\,\mathrm{mm}$ tip exclusion where `env_vm` and `ring` are masked)
  - `0/4` maps active: `0` triangles
- **Sensitivity (`Recipe A` 4-Map vs `Recipe B` 3-Map Primitive-Only Excluding `env_vm`):**
  - Area-weighted $L_1$ rank difference: `0.029395` (`max_abs_rank_diff = 0.249384`), confirming that double-counting between `env_vm` and primitive unit responses shifts area-weighted composite ranks by less than $2.94\%$ on average.

## 4. Source-Bound `>= 90%` Integrated-Saliency Crops & PIP Locators (`JR-EI-P4-T04`)
| Panel | Field | Posed Crop BBox $[x_{\min}, y_{\min}, x_{\max}, y_{\max}]$ ($\mathrm{mm}$) | Active Triangles in Crop | Integrated Saliency Fraction | Peak Value & Triangle |
| --- | --- | --- | ---: | ---: | --- |
| **A** | Ring-traction unit response ($\mathrm{MPa/N}$) | `[-26.555, -24.691, 82.411, 21.330]` | `10,334 / 10,334` | `100.0%` ($\ge 90\%$) | `0.0264 MPa/N` (tri `7071`) |
| **B** | Terminal blade-tip chord torsion ($\mathrm{MPa/(N\,mm)}$) | `[167.325, 5.158, 197.448, 36.099]` | `2,062 / 2,062` | `100.0%` ($\ge 90\%$) | `34.8589 MPa/(N mm)` raw (tri `10685`); `0.6611 MPa/(N mm)` post-exclusion (tri `10564`) |
| **C** | Multi-case von Mises envelope ($\mathrm{MPa}$) | `[-0.352, -25.637, 208.646, 34.654]` | `23,659 / 25,134` | `99.83%` ($\ge 90\%$) | `1,388.6695 MPa` (tri `10564`) |

## 5. Emitted Artifacts
- `JR_EI_P4_T01_T02_hero_white_field_composite.svg` (`859,620` B, `sha256: 95867fd0fc1dab80138bf2c719dc2d0d05adf54a857c48955b6ad610dfec9f3d`)
- `JR_EI_P4_T01_T02_hero_white_field_composite.png` (`1,032,602` B, `sha256: ebb58fb3967906ea539cd6aed66f1f903ef75037c1db24471398a9da208017d9`)
- `JR_EI_P4_T04_fem_saliency_crops_and_locators.svg` (`753,400` B, `sha256: 51a29ad23d9a9100f3192e024124a8772dc9b9d6b62b4aba037f8231e8d988b7`)
- `JR_EI_P4_T04_fem_saliency_crops_and_locators.png` (`847,014` B, `sha256: 76571ad303ba6b9f0d1ab5f4f141d3d312d8bc3a6d19ee4594f0fa36e171c05d`)
