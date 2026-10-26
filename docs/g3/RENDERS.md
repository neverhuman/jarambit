# G3 renders: the v3.0.0 best individual

Every image of the G3 design in this repository shows the best individual of the v3.0.0 cohort of record. Its
identifier, numbers and provenance are in [`evidence/g3/v3.0.0/release.json`](../../evidence/g3/v3.0.0/release.json),
which the README and the G3 paper are generated from. This page states no numbers of its own.

| Files | What they show | Made by |
|---|---|---|
| `docs/g3/v3.0.0/img/{hero,side,top,grip,ring,blade,tip,tip_edge,edge_a,edge_b}.jpg` | Views of the exported meshes | Blender 4.2 Cycles (OptiX), 96 samples; anodized hand zone, satin blade; 35° edge split, nothing smoothed |
| `docs/g3/v3.0.0/img/{xray,cut_full,cut_x,cut_b}.jpg` | The cavities drawn in amber inside a glass shell, and sections of the print file (AM build) | Same scene |
| `docs/g3/v3.0.0/img/lineage_*.jpg` | The winning island's best design at each logged checkpoint | Same side camera, light and material for every frame; envelope at the search grid |
| `docs/g3/v3.0.0/img/evolution_*`, `edge_sections*` | Evolution curves and true-scale edge sections | Matplotlib, from the logged island histories and the measured edge profile |
| `docs/paper/figures/g3/g3-v3.0.0-winner.jpg` | Paper B's illustration of the follow-on cohort: hero (A), x-ray (B), lengthwise section (C), grip section (D) | A 2×2 composite of the renders above |

Geometry: the envelope and the AM build were exported from the decoder at a 0.2 mm grid. For the renders, the blade at
x ≥ 92 mm was re-meshed at 0.05 mm from the exact decoder law, so the edges and serrations are not voxel-aliased.

The pilot candidate shown before v3.0.0 is kept, with its own provenance, in
[`history/pilot-hz4i012_g00085/`](history/pilot-hz4i012_g00085/RENDERS.md).
