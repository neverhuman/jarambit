# Jarambit Requirement & Load-Case Provenance Ledger (`RS-karambit-v3` & `RS-karambit-v4`)

This ledger documents the scenario, biomechanical or standards source, statistical percentile, dynamic amplification factor, and boundary conditions (`HM-1` Winkler hand contact vs legacy clamp) for every load case, material allowable, and geometric requirement in [`RS-karambit-v3.toml`](file:///home/ubuntu/jarambit/requirements/RS-karambit-v3.toml) and [`RS-karambit-v4.toml`](file:///home/ubuntu/jarambit/requirements/RS-karambit-v4.toml) (`P-14`, `F-R10`, `F-R11`, `C-11`).

---

## 1. Load-Case Ledger (`loads-v4.toml`)

| Load ID | Name & Scenario | Magnitude | Direction & Patch | Boundary Condition | Required SF | Biomechanical / Engineering Source |
|---|---|---:|---|---|---:|---|
| `LC-LAT-180` | **Hiker lateral pry / pull** (one-hand lateral blade push/pull near tip during field utility) | `180.0 N` | Out-of-plane $\pm\hat{\mathbf{z}}$ over distal `3.0 mm` tip patch ($a \in [4.0, 7.0]\text{ mm}$) | `HM-1` hand-contact pad (legacy v3: clamp at $x = x_{\text{heel}}$) | `1.00` (screening) | 75th-percentile male one-hand lateral push/pull at tool handle (`MIL-STD-1472H` §5.9; `NASA-STD-3000` Vol. I §4.9). |
| `LC-LAT-260` | **Hunter lateral pry** (95th-percentile one-hand lateral pry with dynamic factor $1.15$) | `260.0 N` | Out-of-plane $\pm\hat{\mathbf{z}}$ over distal `3.0 mm` tip patch ($a \in [4.0, 7.0]\text{ mm}$) | `HM-1` hand-contact pad | `1.15` (`300 N` cap) | 95th-percentile male one-hand lateral force (`226 N` static $\times 1.15$ dynamic factor = `260 N`; `MIL-STD-1472H` Table XVII). |
| `LC-TW-1820` | **Blade body torsion** (wrist supination/pronation twist with blade wedged in kerf) | `1,820.0 N·mm` (`1.82 N·m`) | Torque about blade chord axis applied over $a \in [4.0, 16.0]\text{ mm}$ | `HM-1` hand-contact pad | `1.20` | Working wrist supination/pronation torque on a compact tool handle (`NASA-STD-3000` Fig. 4.9.3-1; `Pheasant & Haslegrave 2006`). |
| `LC-TW-16200` | **Human-max two-hand / leverage torsion** (maximum isometric forearm + shoulder torque against locked blade root/body) | `16,200.0 N·mm` (`16.2 N·m`) | Torque about blade longitudinal axis applied at mid-blade ($s = 0.5 L_{\text{blade}}$) | `HM-1` hand-contact pad | `1.10` | 95th-percentile male maximum isometric pronation/supination + torso assist torque (`14.1 N·m` $\times 1.15$ dynamic factor = `16.2 N·m`; `MIL-STD-1472H`). |
| `LC-TIP-1` | **Solid-tip twist** (point twisted under full `RS-karambit-v3` torque) | `1,820.0 N·mm` (`1.82 N·m`) | Torque about origin-to-point / local tip-bisector axis over $a \in [0, 5.0]\text{ mm}$; readout over $a \in [3.0, 12.0]\text{ mm}$ | Fixed at $a = 20.0\text{ mm}$ submodel cut (`T-TIP-3` frame-invariant, cell-centre aligned raster) | `1.00` | Preserves the exact `RS-karambit-v3` `1.82 N·m` (`1,820 N·mm`) solid-tip twist value on the cell-centre-aligned 3D `TET10` tip submodel. |
| `LC-TIP-2` | **Local tip lateral pry** (point embedded `5.0 mm` and pried out-of-plane) | `60.0 N` | Out-of-plane force $\pm\hat{\mathbf{z}}$ over $a \in [0, 5.0]\text{ mm}$; readout over $a \in [3.0, 12.0]\text{ mm}$ | Fixed at $a = 20.0\text{ mm}$ submodel cut | `1.00` | Single-hand point breakout pry force (`60 N` at `5 mm` penetration; matches tooth-pry demand in `SERR-4`). |
| `LC-SPINE-PUSH` | **Static two-hand spine push** (controlled baton / two-hand downward push on dorsal spine) | `500.0 N` | In-plane $-\hat{\mathbf{y}}$ distributed traction along dorsal spine ($s \in [0.15, 0.85] L_{\text{blade}}$) | `HM-1` hand-contact pad | `1.20` (elastic first-yield) | 95th-percentile two-hand downward push force (`NASA-STD-3000`; `Chaffin, Andersson & Martin 2006`). |
| `LC-BATON` | **Dynamic spine impulse / strike** (`RS-karambit-v3` `3,427 N` spine load) | `3,427.0 N` | In-plane $-\hat{\mathbf{y}}$ distributed traction along dorsal spine | `HM-1` Winkler hand-contact pad (`k_n = 50 N/mm³` compliant glove/palm foundation) | `1.00` (first yield `828 MPa`) | Preserves the exact `RS-karambit-v3` `3,427 N` spine load judged on first yield (`828 MPa` with local thin-wall knockdown $k_t$, not ultimate strength). |
| `LC-RING-1` | **Ring bore bearing pull** (index/pinky finger pulling $+x$ against handle-side inner bore) | `300.0 N` | Cosine pressure distribution over $\theta \in [-60^\circ, +60^\circ]$ on inner bore wall | Grip supported on `HM-1` finger stations | `2.00` (`600 N` cap) | Owner directive `2026-10-25T02:15Z`; finger flexor limit in retention ring (`JARAMBIT-SPEC-ERG-002`). |
| `LC-RING-2` | **Ring outer back pull** (lanyard / retention tug on outer ring back in $-x$) | `30.0 N` | Cosine traction over $\theta \in [150^\circ, 210^\circ]$ on outer ring rim | Grip supported on `HM-1` finger stations | `2.00` (`60 N` cap) | Owner directive `2026-10-25T02:15Z` (`JARAMBIT-SPEC-ERG-002`). |
| `LC-SQUEEZE` | **Bilateral palm power-grip squeeze** (crushing check on hollow lattice-cored grip shell) | `250.0 N` | Opposing $\pm\hat{\mathbf{z}}$ normal pressures over `20.0 mm` $\times$ `14.0 mm` palm swell pads | Self-equilibrated bilateral pinch | `1.20` ($\sigma_{\text{vM}} \le 450\text{ MPa}$) | 95th-percentile male power-grip crush force (`Mathiowetz et al. 1985`, *Arch. Phys. Med. Rehabil.* 66:69–72; `CHK-FEA-01`). |
| `LC-SERR-SNAG` | **Single-tooth snag + lateral pry** (cord/fibrous snag on individual serration tooth) | `120.0 N` tangential + `60.0 N` lateral | Applied at tooth tip cusp over every tooth (including end teeth) | 2D quadratic `T6` tooth root submodel (`T-SERR-4`) | `1.00` | Preserves `RS-karambit-v3` tooth loads (`120 N` snag along edge, `60 N` pry across blade) evaluated on all teeth with Peterson notch factor $K_f$ (`Pilkey 2008`). |
| `LC-FAT-180` | **High-cycle lateral utility fatigue** ($10^7$ cycles, $R = 0$ pulsating lateral load) | `180.0 N` cyclic ($R = 0$) | Out-of-plane lateral tip load | `HM-1` hand-contact pad | `1.00` (Goodman + $K_f$) | `RS-karambit-v3` L-PBF Ti-6Al-4V stress-relieved endurance limit (`300 MPa` at $10^7$ cycles) with thin-wall knockdown $k_t(t)$ applied once (`materials-v4.toml`). |
| `LC-COMB-1` | **Combined lateral + torsion utility load** (simultaneous `70%` hunter lateral + `70%` body twist) | `182.0 N` + `1.274 N·m` | Superposed linear elastic stress tensor fields | `HM-1` hand-contact pad | `1.00` | Combined multi-axis hand-tool loading (`AG-24`, `T-HAND-1`). |

---

## 2. Material & Lattice Provenance (`materials-v4.toml`)

1. **Base Alloy**: Ti-6Al-4V produced by Laser Powder-Bed Fusion (`L-PBF`), stress-relieved per `ASTM F2924` (matching `RS-karambit-v3`, without assuming HIP).
   - Density $\rho = 4.43\text{ g/cm}^3$; Young's modulus $E = 113.8\text{ GPa}$; Poisson's ratio $\nu = 0.342$ (unified across all solvers, fixing `AG-26`).
   - Bulk tensile first-yield strength $\sigma_{y0} = 828.0\text{ MPa}$; ultimate tensile strength $\sigma_{u0} = 895.0\text{ MPa}$.
2. **Thin-Wall Knockdown $k_t(t_{\text{local}})$ (`AG-25`)**:
   - L-PBF thin walls ($t < 2.0\text{ mm}$) exhibit reduced effective yield strength due to surface valley roughness ($R_v \approx 25\text{--}40\text{ }\mu\text{m}$) and prior-$\beta$ grain aspect ratio (`Kranz et al. 2015`; `snowball` thin-wall coupon regression):
     $$k_t(t) = \operatorname{clamp}\!\left(0.82 + 0.09 \cdot \min(t, 2.0),\; 0.85,\; 1.00\right)$$
   - Evaluated using the **local wall/skin thickness $t_{\text{local}}(\mathbf{x})$**, not mid-blade `t_core`, and applied **once** (fixing the G3 double-knockdown bug).
3. **Surface-Class Fatigue Endurance Limit $S_e$ (`AG-08`, `F-M-2`)**:
   - Ground bevels & external blend fillets: $S_{e,\text{ground}} = 300.0\text{ MPa}$ at $R = -1$ (matching `RS-karambit-v3`, with $k_t(t_{\text{local}})$ applied once).
   - Grit-blasted exterior grip shell: $S_{e,\text{blasted}} = 210.0\text{ MPa}$ at $R = -1$.
   - As-built internal lattice & cavity surfaces: $S_{e,\text{as\_built}} = 150.0\text{ MPa}$ at $R = -1$.
4. **Sheet-TPMS Homogenised Scaling (`Gibson & Ashby 1997`; `Al-Ketan & Abu Al-Rub 2019`)**:
   - Sheet-Gyroid (`lat_mix = 0`) and Sheet-Schwarz-D (`lat_mix = 1`) unit-cell RVE fits over $\rho \in [0.06, 0.60]$:
     $$E^*(\rho) / E_s = C_E \rho^{n_E}, \quad G^*(\rho) / G_s = C_G \rho^{n_G}, \quad \sigma_y^*(\rho) / \sigma_{ys} = C_\sigma \rho^{n_\sigma}$$
     with $(C_E, n_E) = (0.52, 1.55)$ for Sheet-Gyroid and $(0.58, 1.50)$ for Sheet-Schwarz-D.
