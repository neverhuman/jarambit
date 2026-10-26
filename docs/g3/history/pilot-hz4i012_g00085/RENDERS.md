# G3 renders: `hz4i012_g00085_4cdfea55`

> **Historical (v2.4.4, 2026-10-25).** This pilot candidate was superseded by the v3.0.0 cohort of record; see
> [`../../RENDERS.md`](../../RENDERS.md) and [`evidence/g3/v3.0.0/release.json`](../../../../evidence/g3/v3.0.0/release.json).

The leading G3 candidate on 2026-10-25. It is a computational pilot candidate: not qualified, not built, and not part of the papers' census. The running draft-d cohort will replace it.

| Image | View |
|---|---|
| `g3-hero.jpg` | Oblique three-quarter view; hand zone anodized Ti (blue), blade satin Ti |
| `g3-cavities-xray.jpg` | The envelope as translucent glass and the cavity body (envelope minus AM build) in orange |
| `g3-section-lengthwise.jpg` | AM build cut on the plane y = 3.5 mm, the grip's centre-line at the palm swell |
| `g3-section-cross.jpg` | AM build cut on the plane x = 45.6 mm, the palm-swell station |
| `g3-grip.jpg` | Close view of the finger grooves |

**Design.**
- Score: `W_ERGOLIGHT_v1` draft d 78.11 of 100 (draft c 79.79), Tier 2 at grid h = 0.4 mm. It is draft d's best among the 20 pilot designs re-checked at Tier 2 with the S9 code.
- Mass: 52.3 g as scored.
- Strength and comfort: 3D solid-tip SF 0.51; E = 0.977.
- Edges (hand zone): minimum edge radius 2.70 mm; section corners ≥ 4.05 mm; finger grooves ≥ 8.93 mm; crests ≥ 4.74 mm; ring-junction step 0.0 mm.

**Geometry.**
- Exported from the Rust decoder (`crates/g3` at commit `8ccb16326f4e`, the draft-d PR head; the decoder is byte-identical in the merged commit) at grid h = 0.2 mm.
- Envelope: 1,969,504 triangles, 26,133 mm³. AM build (skin, TPMS lattice, central web and powder ports): 4,800,988 triangles, 12,156 mm³. Both are watertight, with no boundary or non-manifold edges.
- Cavity share by zone, measured from these meshes:

  | Zone | Cavity share |
  |---|---|
  | Grip (x 8.8–75.6 mm) | 67.4 % |
  | Guard/neck (75.6–99.5 mm) | 28.0 % |
  | Blade (> 99.5 mm) | 27.0 % |
  | Ring zone (< 8.8 mm) | 12.1 %, all at the ring-to-grip junction; the ring itself has no internal void |
  | Overall | 53.5 % |

**Rendering.**
- Blender 4.2 Cycles with AgX, at 128–160 samples, using four area lights (key, rim, kick, top).
- The meshes are rendered exactly as exported; nothing is smoothed or re-shaped for display. Cut faces are flat grey.
- The scripts live outside the repository, in the hand-zone study's run directory: `tools_dev/g3_scene.py` (manifold3d plane trims, cavity body) and `tools_dev/g3_hero.py` (camera and lights).

**SHA-256.**

```
eb43c570ba3b24b85979fa4b11c1e206a6824c337a6a8f5397843fc1463e47fd  g3-cavities-xray.jpg
0de58d42640d64e979529b54bcc5ba73efcab581940c831ee089c52d9d4ead00  g3-grip.jpg
d3c63f890ce2253b32b2f75f2f0c7019cc13420699f558e79fce9a502f7ccb59  g3-hero.jpg
f174241509d63ccd8d1e0b47d8ecb702a10c07b489138685c66a3a36b008ce8c  g3-section-cross.jpg
ac6cd1c7d55481305eab0cbccbedcc339d40303f19ca8652139732b9567f76cc  g3-section-lengthwise.jpg
```
