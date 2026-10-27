# Jarambit `G4.0` Declarative Genome Table (130 Genes, SHA-256 `f012a9157649e5e5`)

| # | Gene | Mod | Kind | Bounds | Default | Unit | Description |
|---:|---|---|---|---|---:|---|---|
| 0 | `ring_od` | `H` | `f64` | `[28.8, 34]` | `30.2` | `mm` | Outer diameter of the finger retention ring around the 24.9 mm bore |
| 1 | `ring_crown` | `H` | `f64` | `[0.22, 0.38]` | `0.3` | `mm` | Axial crown convexity on the 8.0 mm wide ring lip |
| 2 | `ring_lip_d` | `H` | `f64` | `[1.6, 2.6]` | `1.95` | `mm` | Radial lip depth at the inner bore rim |
| 3 | `ring_taper_deg` | `H` | `f64` | `[4, 18]` | `10` | `deg` | Outward radial taper angle from the ring web to the crowned lip |
| 4 | `ring_web_t` | `H` | `f64` | `[2.2, 3.8]` | `2.65` | `mm` | Minimum radial wall thickness of the retention ring web |
| 5 | `ring_thumbpad` | `H` | `f64` | `[0, 2.2]` | `0.85` | `mm` | Dorsal thumb-pad radial prominence on the ring outer rim |
| 6 | `ring_od_tear` | `H` | `f64` | `[0, 2.5]` | `1.1` | `mm` | First harmonic teardrop eccentricity of the outer ring contour toward the neck |
| 7 | `ring_od_h2` | `H` | `f64` | `[-0.8, 1.2]` | `0.25` | `mm` | Second harmonic ellipticity of the outer ring contour |
| 8 | `ring_junction_deg` | `H` | `f64` | `[12, 32]` | `21` | `deg` | Angular span of the ring-to-grip neck transition flare |
| 9 | `ring_neck_r` | `H` | `f64` | `[4.2, 9]` | `6.2` | `mm` | Concave fillet radius at the ring-to-grip neck transition |
| 10 | `ring_bore_round` | `H` | `f64` | `[2.55, 3.45]` | `3` | `mm` | Corner round radius on the inner 24.9 mm ring bore |
| 11 | `ring_hollow_t` | `H` | `f64` | `[0, 1]` | `0.78` | `ratio` | Drained hollow gallery fraction inside the ring rim and neck bridge (L-4 mass lever) |
| 12 | `ring_port_d` | `H` | `f64` | `[2.55, 3.6]` | `2.85` | `mm` | Diameter of the ring-to-grip powder drainage gallery port (>= 2.5 mm) |
| 13 | `hand_scale` | `H` | `f64` | `[0.96, 1.05]` | `1` | `ratio` | Anthropometric scale factor multiplying M50 finger pitches (19.5, 21.0, 22.0 mm) |
| 14 | `grip_dl` | `H` | `f64` | `[2, 8]` | `5.2` | `mm` | Axial grip length offset added to scaled 3-finger pitch sum (L_g in 64-72 mm) |
| 15 | `grip_t0` | `H` | `f64` | `[11.8, 15.2]` | `13.4` | `mm` | Lateral grip thickness at the ring-end station |
| 16 | `grip_t_sw` | `H` | `f64` | `[13.4, 16.6]` | `14.8` | `mm` | Lateral grip thickness at the mid-palm swell station (13-17 mm band) |
| 17 | `grip_t_g` | `H` | `f64` | `[12.2, 15.8]` | `13.8` | `mm` | Lateral grip thickness at the forward guard station |
| 18 | `grip_h0` | `H` | `f64` | `[22.5, 27]` | `24.6` | `mm` | In-plane dorsal-to-ventral grip height at the ring-end station |
| 19 | `grip_h_sw` | `H` | `f64` | `[25.2, 27.8]` | `26.2` | `mm` | In-plane dorsal-to-ventral grip height at the palm swell station (25-28 mm band) |
| 20 | `grip_h_g` | `H` | `f64` | `[23.5, 27.5]` | `25.4` | `mm` | In-plane dorsal-to-ventral grip height ahead of the index choil |
| 21 | `grip_belly` | `H` | `f64` | `[-1.5, 2.5]` | `0.6` | `mm` | Dorsal palm-arch camber of the handle mid-line |
| 22 | `grip_n_exp` | `H` | `f64` | `[2.1, 3.2]` | `2.45` | `dimless` | Superellipse cross-section exponent of the grip shell (controls corner fullness) |
| 23 | `grip_edge_r` | `H` | `f64` | `[2.3, 3.8]` | `2.85` | `mm` | Hand-contact rail rounding radius along dorsal and ventral grip edges (>= 2.2 mm) |
| 24 | `groove_d1` | `G` | `f64` | `[1.6, 2.4]` | `1.95` | `mm` | Scallop depth of the ring-finger groove (1.5-2.5 mm comfort band) |
| 25 | `groove_d2` | `G` | `f64` | `[1.6, 2.4]` | `1.95` | `mm` | Scallop depth of the middle-finger groove (1.5-2.5 mm comfort band) |
| 26 | `groove_choil` | `G` | `f64` | `[3.6, 4.4]` | `4.1` | `mm` | Positive-stop depth of the compound index-finger choil (3.5-4.5 mm band) |
| 27 | `groove_r` | `G` | `f64` | `[10.5, 16]` | `12.2` | `mm` | Concave valley curvature radius of the finger grooves (>= 10.0 mm) |
| 28 | `crest_r` | `G` | `f64` | `[5.1, 8]` | `5.8` | `mm` | Convex saddle crest radius between adjacent finger grooves (>= 5.0 mm) |
| 29 | `ramp_h` | `G` | `f64` | `[1.5, 4.2]` | `2.6` | `mm` | Dorsal thumb-ramp elevation at the forward grip transition |
| 30 | `guard_h` | `G` | `f64` | `[12.2, 15.5]` | `13` | `mm` | Ventral index-guard protrusion height (>= 12.0 mm ergonomic requirement) |
| 31 | `guard_t` | `G` | `f64` | `[7.5, 12.5]` | `9.4` | `mm` | Lateral thickness of the parametric guard bolster |
| 32 | `guard_r` | `G` | `f64` | `[3.5, 7.5]` | `5` | `mm` | Fillet blend radius between the guard bolster and blade heel |
| 33 | `guard_bspline_c1` | `G` | `f64` | `[-2, 2.5]` | `0.4` | `mm` | First cubic B-spline control offset of the parametric dorsal guard profile (replaces v1 F075 lookup) |
| 34 | `guard_bspline_c2` | `G` | `f64` | `[-2, 2.5]` | `-0.3` | `mm` | Second cubic B-spline control offset of the parametric ventral guard lobe |
| 35 | `jimp_depth` | `G` | `f64` | `[0.4, 0.8]` | `0.58` | `mm` | Filleted thumb-ramp jimping groove depth (crest R >= 0.5 mm) |
| 36 | `blade_len` | `P` | `f64` | `[72.5, 84.5]` | `76.5` | `mm` | Arc length of the blade centre-line spine from guard face to point |
| 37 | `blade_reach_target` | `P` | `f64` | `[72.5, 84.5]` | `75.8` | `mm` | Straight-line chord reach from the guard datum to the blade point (REQ-FUN-1: 72-85.725 mm) |
| 38 | `claw_turn_deg` | `P` | `f64` | `[36, 78]` | `48` | `deg` | Total integrated centre-line turning angle Theta of the curved karambit claw (REQ-FUN-3 >= 35 deg) |
| 39 | `kappa_b0` | `P` | `f64` | `[0.2, 1.6]` | `0.85` | `dimless` | Bernstein B0 curvature weight at the blade root (controls root hook onset) |
| 40 | `kappa_b1` | `P` | `f64` | `[0.4, 1.8]` | `1.05` | `dimless` | Bernstein B1 curvature weight at the proximal mid-blade |
| 41 | `kappa_b2` | `P` | `f64` | `[0.4, 1.8]` | `1.15` | `dimless` | Bernstein B2 curvature weight at the distal mid-blade |
| 42 | `kappa_b3` | `P` | `f64` | `[0.1, 1.4]` | `0.7` | `dimless` | Bernstein B3 curvature weight approaching the blade point |
| 43 | `blade_belly` | `P` | `f64` | `[-2, 3.5]` | `0.8` | `mm` | Mid-blade width expansion of the inner concave edge profile |
| 44 | `spine_arch` | `P` | `f64` | `[-1.5, 3.5]` | `1.1` | `mm` | Dorsal spine arch expansion along the outer convex edge |
| 45 | `h_root` | `P` | `f64` | `[22, 30]` | `26.8` | `mm` | In-plane blade chord width at the guard root junction (boosts root section modulus) |
| 46 | `h_mid_ratio` | `P` | `f64` | `[0.58, 0.86]` | `0.72` | `ratio` | Ratio of mid-blade width (s = 0.5 L) to root width h_root |
| 47 | `theta_tip_deg` | `P` | `f64` | `[28, 50]` | `38` | `deg` | Decoupled planform included angle at 3.0 mm from the point (Gate S4 / REQ-FUN-4 <= 55 deg) |
| 48 | `l_tip_plan_mm` | `P` | `f64` | `[6, 14]` | `9.5` | `mm` | Length of the C2-blended linear-wedge point planform zone (prevents bullet-nose exploit) |
| 49 | `tip_radius_mm` | `P` | `f64` | `[0.06, 0.22]` | `0.12` | `mm` | Planform nose radius at the extreme tip apex (Gate S5 <= 0.25 mm) |
| 50 | `harm_in_1` | `P` | `f64` | `[-1.2, 1.2]` | `0.15` | `mm` | First spatial cosine harmonic offset on the inner concave blade outline |
| 51 | `harm_in_2` | `P` | `f64` | `[-0.8, 0.8]` | `-0.1` | `mm` | Second spatial cosine harmonic offset on the inner concave blade outline |
| 52 | `harm_sp_1` | `P` | `f64` | `[-1.2, 1.2]` | `0.2` | `mm` | First spatial cosine harmonic offset on the outer dorsal spine outline |
| 53 | `harm_sp_2` | `P` | `f64` | `[-0.8, 0.8]` | `0.05` | `mm` | Second spatial cosine harmonic offset on the outer dorsal spine outline |
| 54 | `camber_z1_mm` | `Z` | `f64` | `[-1.8, 1.8]` | `0` | `mm` | First-mode out-of-plane lateral camber z_c(s) of the 3D blade spine |
| 55 | `camber_z2_mm` | `Z` | `f64` | `[-1, 1]` | `0` | `mm` | Second-mode S-camber z_c(s) of the 3D blade spine |
| 56 | `twist_tip_deg` | `Z` | `f64` | `[-8, 8]` | `0` | `deg` | Axial Bishop-frame twist angle of the blade cross-section from root to tip |
| 57 | `chisel_bias` | `Z` | `f64` | `[-0.25, 0.25]` | `0` | `ratio` | Out-of-plane bevel symmetry bias (0 = symmetric double-ground, != 0 = asymmetric chisel shift) |
| 58 | `grip_cant_deg` | `Z` | `f64` | `[-5, 5]` | `0` | `deg` | Out-of-plane ergonomic cant angle of the grip axis relative to the blade plane |
| 59 | `ring_tilt_deg` | `Z` | `f64` | `[-6, 6]` | `0` | `deg` | Out-of-plane tilt angle of the finger retention ring bore axis |
| 60 | `build_theta_deg` | `Z` | `f64` | `[25, 65]` | `45` | `deg` | Co-evolved L-PBF build plate inclination angle theta_build |
| 61 | `build_phi_deg` | `Z` | `f64` | `[-30, 30]` | `0` | `deg` | Co-evolved L-PBF build plate azimuthal rotation angle phi_build |
| 62 | `t_core` | `X` | `f64` | `[4.2, 6.6]` | `5.1` | `mm` | Maximum blade spine thickness at the guard root transition (boosts root lateral & fatigue SF) |
| 63 | `t_tip_base` | `X` | `f64` | `[2.4, 4.6]` | `3.4` | `mm` | Nominal blade spine thickness entering the distal tip zone before crown reinforcement |
| 64 | `taper_x0` | `X` | `f64` | `[0.25, 0.75]` | `0.5` | `ratio` | Normalized arc-length station where distal spine thickness taper begins |
| 65 | `t_tip_crown_mm` | `X` | `f64` | `[0.15, 0.85]` | `0.52` | `mm` | Local diamond spine ridge reinforcement along the tip bisector over a in [3, 14] mm (L-8 lever: +90% tip torsion SF for +0.2 g) |
| 66 | `l_tip_crown_mm` | `X` | `f64` | `[6, 15]` | `11` | `mm` | Axial span of the distal tip-bisector diamond crown reinforcement |
| 67 | `ridge_amp` | `X` | `f64` | `[0, 1.2]` | `0.55` | `mm` | Medial structural ridge amplitude along the blade spine neutral line |
| 68 | `ridge_width_frac` | `X` | `f64` | `[0.18, 0.45]` | `0.28` | `ratio` | Transverse width fraction of the medial structural spine ridge |
| 69 | `fuller_amp` | `X` | `f64` | `[0, 1.1]` | `0.65` | `mm` | Bi-concave fuller groove depth removing low-leverage neutral-axis blade mass (L-5 lever) |
| 70 | `fuller_width_frac` | `X` | `f64` | `[0.2, 0.5]` | `0.34` | `ratio` | Transverse width fraction of the bi-concave fuller fluting |
| 71 | `corrug_amp` | `X` | `f64` | `[0, 0.45]` | `0.15` | `mm` | Micro-corrugation stiffness rib amplitude on the fuller flanks |
| 72 | `win_gate` | `X` | `f64` | `[0, 1]` | `0.3` | `ratio` | Smooth C2 sigmoid activation of medial blade fenestration windows |
| 73 | `win_length_frac` | `X` | `f64` | `[0.15, 0.45]` | `0.26` | `ratio` | Arc-length span fraction of the reinforced blade fenestration window |
| 74 | `win_width_frac` | `X` | `f64` | `[0.12, 0.3]` | `0.18` | `ratio` | Transverse width fraction of the blade fenestration window |
| 75 | `win_fillet_r` | `X` | `f64` | `[1.5, 3.5]` | `2.4` | `mm` | Stress-relieving corner fillet radius around fenestration window rims |
| 76 | `skin_t_base` | `I` | `f64` | `[0.78, 1.15]` | `0.86` | `mm` | Base exterior shell skin thickness (>= 0.75 mm printable floor) |
| 77 | `skin_flange_c2` | `I` | `f64` | `[0.05, 0.55]` | `0.38` | `mm` | Circumferential cos(2*theta) flange grading: thickens dorsal/ventral spine-bending chords while thinning neutral-axis flanks (L-1 lever) |
| 78 | `skin_dorsal_s1` | `I` | `f64` | `[-0.15, 0.25]` | `0.08` | `mm` | Dorsal-to-ventral sin(theta) asymmetry of the flange-graded grip skin |
| 79 | `skin_axial_taper` | `I` | `f64` | `[-0.2, 0.3]` | `0.12` | `mm` | Axial skin thickening toward the high-moment guard root junction |
| 80 | `rib_w` | `I` | `f64` | `[0, 1]` | `0.45` | `ratio` | Continuous weight of the internal I-beam shear web (>= 0.80 mm when active) |
| 81 | `rib_hole_frac` | `I` | `f64` | `[0.15, 0.68]` | `0.52` | `ratio` | Neutral-axis lightening-hole height fraction in the internal web (L-2 lever: removes 52% web mass for 14% stiffness) |
| 82 | `rib_arch_count` | `I` | `int` | `[3, 6]` | `4` | `count` | Number of Warren/trabecular lightening arches along the internal grip web |
| 83 | `bulkhead_t_mm` | `I` | `f64` | `[0.8, 1.3]` | `0.9` | `mm` | Thickness of transverse finger-station ring diaphragms (>= 0.80 mm printable floor, replacing 0.18 mm G3 webs) |
| 84 | `bulkhead_count` | `I` | `int` | `[2, 4]` | `3` | `count` | Number of printable transverse bulkheads supporting palm squeeze loads |
| 85 | `lat_mix` | `I` | `f64` | `[0, 1]` | `0.65` | `ratio` | Sheet-TPMS family blend (0.0 = Sheet-Gyroid, 1.0 = Sheet-Schwarz-D) |
| 86 | `lat_cell_mm` | `I` | `f64` | `[4.5, 7.5]` | `5.8` | `mm` | Unit-cell period of the internal Sheet-TPMS infill (sized for >= 0.80 mm struts and powder escape) |
| 87 | `rho_grip` | `I` | `f64` | `[0.05, 0.35]` | `0.08` | `ratio` | Relative density of the Sheet-TPMS core inside the grip cavity |
| 88 | `rho_guard` | `I` | `f64` | `[0.08, 0.45]` | `0.16` | `ratio` | Relative density of the Sheet-TPMS core inside the guard transition |
| 89 | `rho_ring` | `I` | `f64` | `[0.05, 0.35]` | `0.1` | `ratio` | Relative density of the Sheet-TPMS core inside the drained hollow ring cavity |
| 90 | `rho_blade` | `I` | `f64` | `[0.08, 0.5]` | `0.18` | `ratio` | Relative density of the continuous hollow blade core (decoupled from fenestration windows, L-6 lever) |
| 91 | `lat_stress_k` | `I` | `f64` | `[0, 0.35]` | `0.14` | `ratio` | Lamarckian stress-driven lattice density grading gain k (L-3 lever) |
| 92 | `lat_stress_p` | `I` | `f64` | `[0.8, 2.2]` | `1.3` | `dimless` | Exponent p of the stress-driven TPMS density grading field |
| 93 | `port_d_mm` | `I` | `f64` | `[2.6, 3.8]` | `3` | `mm` | Diameter of the exterior powder-evacuation ports (>= 2.50 mm requirement) |
| 94 | `tip_reserve_mm` | `E` | `f64` | `[6.5, 12]` | `9` | `mm` | Distal point zone length over which serrations are excluded and the diamond tip crown tapers |
| 95 | `t_0p25_mm` | `E` | `f64` | `[0.21, 0.29]` | `0.25` | `mm` | Point thickness along the bisector at 0.25 mm from the tip (Gate S2 <= 0.30 mm) |
| 96 | `t_1p00_mm` | `E` | `f64` | `[0.42, 0.58]` | `0.5` | `mm` | Point thickness along the bisector at 1.00 mm from the tip (Gate S2 <= 0.60 mm) |
| 97 | `t_2p00_mm` | `E` | `f64` | `[0.78, 0.98]` | `0.9` | `mm` | Point thickness along the bisector at 2.00 mm from the tip (Gate S2 <= 1.00 mm) |
| 98 | `plunge_in_mm` | `E` | `f64` | `[5, 11]` | `7.5` | `mm` | Unground ricasso/plunge distance from the guard on the inner concave edge |
| 99 | `grind_end_in_mm` | `E` | `f64` | `[0.8, 3.2]` | `1.6` | `mm` | Active distal grind termination offset on the inner edge (fixes retired s1_in gene F-G2) |
| 100 | `angle_in_deg` | `E` | `f64` | `[18, 22]` | `20` | `deg` | Primary bevel half-angle per side on the inner concave edge (EDGE-3: 17.5-22.5 deg) |
| 101 | `apex_in_mm` | `E` | `f64` | `[0.06, 0.11]` | `0.08` | `mm` | As-finished cutting edge apex thickness on the inner concave edge |
| 102 | `serr_teeth_in` | `E` | `int` | `[3, 7]` | `4` | `count` | Explicit integer number of full-depth serration scallops on the inner edge (SERR-1 >= 3) |
| 103 | `serr_depth_in_mm` | `E` | `f64` | `[0.9, 1.9]` | `1.35` | `mm` | Serration scallop depth on the inner concave edge (>= 0.80 mm) |
| 104 | `serr_duty_in` | `E` | `f64` | `[0.52, 0.78]` | `0.65` | `ratio` | Gullet-to-pitch duty cycle on the inner serrated edge (smooths gullet root radius rho_g) |
| 105 | `plunge_sp_mm` | `E` | `f64` | `[6, 12]` | `8.5` | `mm` | Unground plunge distance from the thumb ramp on the outer dorsal spine edge |
| 106 | `grind_end_sp_mm` | `E` | `f64` | `[0.8, 3.2]` | `1.6` | `mm` | Active distal grind termination offset on the outer spine edge (fixes retired s1_sp gene F-G2) |
| 107 | `angle_sp_deg` | `E` | `f64` | `[18, 22]` | `20.4` | `deg` | Primary bevel half-angle per side on the outer dorsal spine edge (EDGE-3: 17.5-22.5 deg) |
| 108 | `apex_sp_mm` | `E` | `f64` | `[0.06, 0.11]` | `0.085` | `mm` | As-finished cutting edge apex thickness on the outer dorsal spine edge |
| 109 | `serr_teeth_sp` | `E` | `int` | `[3, 8]` | `5` | `count` | Explicit integer number of full-depth serration scallops on the outer spine edge (SERR-1 >= 3) |
| 110 | `serr_depth_sp_mm` | `E` | `f64` | `[0.9, 1.8]` | `1.3` | `mm` | Serration scallop depth on the outer dorsal spine edge (>= 0.80 mm) |
| 111 | `serr_duty_sp` | `E` | `f64` | `[0.52, 0.78]` | `0.66` | `ratio` | Gullet-to-pitch duty cycle on the outer dorsal spine edge |
| 112 | `alien_gate` | `T` | `f64` | `[0, 1]` | `0.65` | `ratio` | Master blend weight of the Module T trabecular and 3D morphogen architecture inside keep-in zones |
| 113 | `trabec_arch_amp` | `T` | `f64` | `[0, 1]` | `0.72` | `ratio` | Tri-arched Bishop-frame trabecular load-path bridge across the ring-to-grip neck and guard root |
| 114 | `trabec_strut_r_mm` | `T` | `f64` | `[0.65, 1.4]` | `0.95` | `mm` | Cross-sectional capsule radius of the primary trabecular load-path struts (diameter >= 1.3 mm) |
| 115 | `trabec_aniso_x` | `T` | `f64` | `[1, 2]` | `1.45` | `ratio` | Principal-stress axial elongation ratio of the internal trabecular cells |
| 116 | `trabec_window_blend` | `T` | `f64` | `[0, 1]` | `0.5` | `ratio` | Exoskeletal windowing blend exposing internal trabecular struts along neutral-axis grip/blade flanks |
| 117 | `cppn_w0` | `T` | `f64` | `[-1.5, 1.5]` | `0.45` | `dimless` | 3D symmetric CPPN morphogen weight 0 (axial coordinates -> internal porosity modulation) |
| 118 | `cppn_w1` | `T` | `f64` | `[-1.5, 1.5]` | `-0.35` | `dimless` | 3D symmetric CPPN morphogen weight 1 (transverse coordinates -> flange rib modulation) |
| 119 | `cppn_w2` | `T` | `f64` | `[-1.5, 1.5]` | `0.6` | `dimless` | 3D symmetric CPPN morphogen weight 2 (radial distance -> shell perforation field) |
| 120 | `cppn_w3` | `T` | `f64` | `[-1.5, 1.5]` | `0.25` | `dimless` | 3D symmetric CPPN morphogen weight 3 (Gaussian radial basis activation) |
| 121 | `cppn_w4` | `T` | `f64` | `[-1.5, 1.5]` | `-0.4` | `dimless` | 3D symmetric CPPN morphogen weight 4 (harmonic cross-coupling) |
| 122 | `cppn_w5` | `T` | `f64` | `[-1.5, 1.5]` | `0.3` | `dimless` | 3D symmetric CPPN morphogen weight 5 (distal spine taper coupling) |
| 123 | `cppn_freq` | `T` | `f64` | `[0.5, 2.5]` | `1.2` | `dimless` | Spatial frequency multiplier of the 3D CPPN morphogen field |
| 124 | `tex_amp_mm` | `A` | `f64` | `[0.16, 0.24]` | `0.19` | `mm` | Zero-mean grip micro-texture amplitude (0.15-0.25 mm ergonomic band; zero net mass drift L-7) |
| 125 | `tex_pitch_mm` | `A` | `f64` | `[1.4, 3.2]` | `2.1` | `mm` | Spatial wavelength pitch of the zero-mean grip surface texture |
| 126 | `tex_angle_deg` | `A` | `f64` | `[20, 70]` | `42` | `deg` | Helical cross-hatch angle of the grip micro-texture |
| 127 | `tex_knurl_mix` | `A` | `f64` | `[0, 1]` | `0.55` | `ratio` | Zero-mean diamond-knurl basis weight |
| 128 | `tex_turing_mix` | `A` | `f64` | `[0, 1]` | `0.3` | `ratio` | Zero-mean reaction-diffusion Turing-stripe basis weight |
| 129 | `tex_voronoi_mix` | `A` | `f64` | `[0, 1]` | `0.15` | `ratio` | Zero-mean cellular Voronoi dimple basis weight |
