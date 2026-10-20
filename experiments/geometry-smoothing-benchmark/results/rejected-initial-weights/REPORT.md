# Synthetic geometry smoothing benchmark

Result: **FAIL** on a made-up rough sphere; radius 30 mm. Geometry changed at 642 / 642 vertices. Topology remains 1280 triangles.

| Measurement | Before | After |
|---|---:|---:|
| Vertex radial RMS error, mm | 0.423986 | 0.214189 |
| Vertex radial p95 error, mm | 0.748254 | 0.364465 |
| Vertex radial maximum error, mm | 0.799081 | 0.590599 |
| Mean radius, mm | 30.000000 | 30.181263 |
| Enclosed volume, mm³ | 112120.866237 | 114185.960517 |

RMS error reduction: 49.482%. Maximum movement: 1.097698 mm (budget 2 mm). Volume difference from the clean reference mesh: 1.838998%.

Declared pass criteria: vertices move; RMS error falls at least 50%; p95 improves; maximum error does not increase; volume differs from the clean reference mesh by at most 1%; closed connected oriented mesh retains Euler characteristic 2; no degenerate or reversed faces; total displacement budget holds. All metrics use all vertices/faces.

Method: uniform Taubin-style lambda/mu filtering, 20 iterations, lambda 0.5, mu -0.53. Pins and open boundaries are fixed. The method uses only noisy input positions and adjacency; the known sphere is used only for assessment.

[Wireframe comparison](comparison.svg) · [Vertex coordinates](vertices.csv) · [Machine-readable measurements](metrics.json) · [Rough STL](rough.stl) · [Smoothed STL](smoothed.stl) · [Reference STL](reference.stl)

Limits: a synthetic mesh benchmark, not reconstructed analytic B-rep/STEP CAD. Vertex-to-sphere error is exact for the sampled vertices; no continuous Hausdorff bound is claimed. Volume uses the triangulated reference, not the analytic sphere. No exhaustive triangle-intersection check, general feature-preservation claim, functional evaluation, or real-object qualification is supplied. Passing this fixture does not establish success on arbitrary CAD.
