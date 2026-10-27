# RS-karambit-v5: provenance of every requirement

`requirements/RS-karambit-v5.toml` is the requirement set of the G5 generation (release v5.0.0; G4 was the withdrawn v4.0.0 prototype). Each value there names one or more
source keys in square brackets, and this page resolves every key. Grades say how strong the evidence is: **S** a
standard or handbook, **P** a peer-reviewed paper read in full, **p** a paper known through its abstract or a review,
**V** a vendor guide, **D** derived here from the cited values, **O** an owner decision.

The set was rebuilt on 2026-10-27 at the owner's request ("I want the BEST possible constraints for the best possible
knife, please do what is best"). Three literature reviews fed it: human hand loads and ergonomics, knife task loads
with edge and point geometry, and L-PBF Ti-6Al-4V properties and process rules. RS-karambit-v4 belongs to the withdrawn
v4.0.0 prototype and is not used.

## Owner decisions

| Key | Date | Decision |
|---|---|---|
| `owner-ring` | 2026-10-24/25 | Little finger in the ring. Bore 24.9 mm and lip 8 mm, both fixed. 300 N bearing on the bore inside closest to the handle (±60°) and 30 N on the ring back, both at SF 2 |
| `owner-reach` | 2026-10-25 | Blade reach at most 85.725 mm (3.375 in) in a straight line from the guard; sharpened arc at most 4.0 in per edge |
| `owner-balance` | 2026-10-24 | Spinning is a use case: centre of mass within 8 mm of the index choil, low spin inertia about the ring |
| `owner-objective` | 2026-10-27 | A continuous objective with no hard gates. Ergonomics is never traded for grams (E is a multiplier) |
| `owner-point` | 2026-10-27 | "Yes you should penalize a blunt point, that seems obvious": a continuous penalty on the planform point |
| `owner-grip` | 2026-10-27 | The evidence grip band (84–100 mm perimeter) replaces the compact 65–75 mm band of 2026-10-24 |
| `owner-serration` | 2026-10-27 | Medium serrations on both edges (pitch 4–6 mm, depth 0.8–1.5 mm, gullet radius ≥ 0.5 mm), ground after printing |
| — | 2026-10-27 | No batoning. The 3,427 N spine case is dropped, because a claw sharpened on both edges has no spine to strike, and the value came from a boxing-punch study [walilko2005] |
| — | 2026-10-27 | The tip twist of 1.82 N·m is replaced by the sourced tip set: no sharp point can carry it (capacity about 0.2–0.45 N·m at 2 mm, D), and it taught the G3 search to blunt the point |

## What changed from RS-karambit-v3, and why

| Requirement | v3 | v5 | Reason |
|---|---|---|---|
| Yield allowable | 828 MPa (wrought plate A-basis) ×0.912 | 825 MPa (F2924 minimum) × 0.91 at 80 °C × thin-wall law | L-PBF minimum, not a wrought value; the 0.912 derate equals about 78 °C [f2924, hdbk5j, xie2026] |
| Fatigue | Machined-steel Marin factors, modified Goodman | Strength by surface class, FKM mean stress (M 0.6), Kf = Kt on as-built faces | Marin overstates as-built strength about 2×, and Goodman understates the mean-stress effect [johnsen2024, benedetti2018, nicoletto2018] |
| Lateral | 180 / 260 N at the tip | 12 N·m limit, 18 N·m ultimate, applied 5 mm back from the point | Human wrist and arm strength; every bend standard loads well back from the point [yoshii2015, mil1472h, mil8662e, gost51501] |
| Body twist | 1.82 N·m (floor), 16.2 N·m human max | 7 N·m service, 16.2 N·m limit | 1.82 N·m is below cutlery handle-twist tests [en8442, osullivan2002] |
| Tip | 1.82 N·m twist | 150 N axial, 10 N side, 0.3 N·m twist at 3 mm | Owner decision (above) [en388, gilchrist2008, nolan2013] |
| Buckling | analytic lateral-torsional estimate | eigenvalue λ ≥ 1.5 under the tip axial load | A load normal to the flat blade has no buckling eigenvalue; in-plane compression of the point does (D) |
| Spine | 3,427 N | dropped | Owner decision (above) |
| Grip squeeze | 250 N, 450 MPa | 800 N limit, plus contact-pressure limits at 55 / 130 / 661 N | Above P95 male grip and the NASA crew maximum [dodds2014, hidh, tirloni2021] |
| Guard | soft, 12 mm | 15 mm target, 10 mm floor, 300 N load | Guard effectiveness levels off at 15.2 mm [cochran1986] |
| Edge angle | 17.5–22.5° per side | 20–25° per side | Titanium rolls at steel angles; equal edge bending strength needs tan θ about 1.4–1.5× steel's [thomas2018, zhang2023, landes; D] |
| Serrations | pitch 5–9 mm, depth 0.8–2.2 mm | pitch 4–6 mm, depth 0.8–1.5 mm, gullet ≥ 0.5 mm, end teeth checked | Owner decision [us4823467] |
| Point | thickness only | angle at 1 / 3 mm, width at 2 mm, tip radius | Owner decision; tip radius controls piercing [hainsworth2008] |
| Grip | perimeter 65–75 mm, H/T 1.5–2.0 | 84–100 mm (floor 78.5), H/T 1.5–1.8, length ≥ 69 mm | Owner decision [kong2005, seo2011, faa-hfds] |
| Ports | ≥ 2.5 mm | ≥ 3.0 mm, two per cavity, no closed voids measured on the geometry | [nasa6030, xometry, materialise, solukon] |
| Drainage | a cavity drained if any voxel path reached the outside | only passages ≥ 1 mm wide drain powder | A one-voxel gap drained a cavity on one grid and sealed it on another (G3 `g00840`: 51.3 g at h 0.4, 75.2 g at h 0.2) [carrozza2023] |
| Margins | SF 1 | SF 1.2 on yield, 1.3 on ultimate; ring SF 2 kept | NASA-STD-6030 low-demand class [nasa6030]; owner ring |

## Sources

| Key | Grade | Source | Used for |
|---|---|---|---|
| `f2924` | S | ASTM F2924-14(2021), *Standard Specification for Additive Manufacturing Titanium-6 Aluminum-4 Vanadium with Powder Bed Fusion*; minima via totalmateria.com/en-us/material/3300111 | Yield 825, ultimate 895 MPa, elongation 10 % |
| `hdbk5j` | S | MIL-HDBK-5J (2003), §5.4.1, Ti-6Al-4V annealed: Table 5.4.1.0(b); Figs 5.4.1.1.1 and 5.4.1.1.4 (temperature) | E, G, ν, density; yield 0.91 / ultimate 0.92 / modulus 0.97 at 80 °C |
| `ncamp` | S | NIAR/NCAMP ND-ME 004, L-PBF Ti-6Al-4V material specification (2025) | Stress-relief route, density minimum |
| `nasa6030` | S | NASA-STD-6030, *Additive Manufacturing Requirements for Spaceflight Systems* (2021) | Margins (yield MS ≥ 0.2, ultimate ≥ 0.3); 60 % fatigue rule for as-built surfaces; wall ≥ 8 scan traces; no trapped powder |
| `eos` | V | EOS Titanium Ti64 Grade 5 material data sheet (M290) | Stress-relieved properties, accuracy ±50 µm, finishing allowance |
| `vrancken` | P | Vrancken B, Thijs L, Kruth J-P, Van Humbeeck J (2012), "Heat treatment of Ti6Al4V produced by selective laser melting", *J Alloys Compd* 541:177, doi:10.1016/j.jallcom.2012.07.022 | E 109–115 GPa; as-built vs stress-relieved properties |
| `cain2015` | p | Cain V et al. (2015), "Crack propagation and fracture toughness of Ti6Al4V alloy produced by selective laser melting", *Addit Manuf* 5:68, doi:10.1016/j.addma.2014.12.006 | Toughness about 50 MPa√m stress-relieved |
| `xie2026` | P | Xie et al. (2026), *Materials* 19:1945, doi:10.3390/ma19101945 | Thin-wall law σ = σ₀(1 − 2Ts/t)^n, Ts = 26.5 µm, 0.2–1.0 mm walls |
| `murchio2024` | P | Murchio S et al. (2024), *Int J Fatigue* 180:108102, doi:10.1016/j.ijfatigue.2023.108102 | Thin as-built struts: strength and fatigue by build angle; down-skin Ts (D) |
| `benedetti2018` | P | Benedetti M et al. (2018), *Int J Fatigue* 107:96, doi:10.1016/j.ijfatigue.2017.10.021 | As-built fatigue 221 MPa (R = −1); FKM and SWT fit, Goodman fails |
| `nicoletto2018` | P | Nicoletto G (2018), *Int J Fatigue* 106:124, doi:10.1016/j.ijfatigue.2017.10.004 | As-built fatigue 150–180 MPa; Kf about Kt on as-built notches |
| `wycisk2014` | P | Wycisk E et al. (2014), *Phys Procedia* 56:371, doi:10.1016/j.phpro.2014.08.120 | Stress-relieved as-built vs polished fatigue |
| `wycisk2015` | P | Wycisk E et al. (2015), *Front Mater* 2:72, doi:10.3389/fmats.2015.00072 | HIP, machined fatigue about 575 MPa (R = −1) |
| `pagliari2025` | P | Pagliari et al. (2025), *Eng Fail Anal* 180:109825, doi:10.1016/j.engfailanal.2025.109825 | Surface factors 0.23 / 0.46 / 1 (as-built / machined / polished) |
| `johnsen2024` | P | Johnsen et al. (2024), *Welding in the World* 68:361, doi:10.1007/s40194-023-01604-5 | Meta-analysis: surface factor vs Ra, mean-stress sensitivity 0.62 |
| `kahlin2017` | p | Kahlin M, Ansell H, Moverare J (2017), *Int J Fatigue* 101:51, doi:10.1016/j.ijfatigue.2017.04.009 | Notched as-built fatigue; HIP gives nothing while the as-built surface remains |
| `beretta2017` | P | Beretta S, Romano S (2017), *Int J Fatigue* 94:178, doi:10.1016/j.ijfatigue.2016.06.020 | Subsurface defects 200–250 µm deep (finishing allowance) |
| `barricelli2022` | P | Barricelli, Patriarca, du Plessis, Beretta (2022), *Int J Fatigue*, doi:10.1016/j.ijfatigue.2022.107401 | Roughness by orientation (down-skin Rv up to 140 µm) |
| `naghavi2022` | P | Naghavi SA et al. (2022), *Bioengineering* 9:504, doi:10.3390/bioengineering9100504 | Sheet gyroid and Schwarz-D scaling laws |
| `timercan2021` | P | Timercan A, Sheremetyev V, Brailovski V (2021), *Sci Technol Adv Mater* 22:285, doi:10.1080/14686996.2021.1907222 | Sheet gyroid strength scaling |
| `kelly2019` | p | Kelly CN et al. (2019), *Acta Biomater* 94:610, doi:10.1016/j.actbio.2019.05.046 | Sheet TPMS stronger and less anisotropic than strut TPMS |
| `lietaert2018` | P | Lietaert K et al. (2018), *Sci Rep* 8:4957, doi:10.1038/s41598-018-23414-2 | Strut lattice tension vs compression |
| `benedetti2021` | P | Benedetti M, du Plessis A et al. (2021), *Mater Sci Eng R* 144:100606, doi:10.1016/j.mser.2021.100606 | Lattice fatigue well below 0.4 σy* at 1e6 |
| `alana2021` | P | Alaña M et al. (2021), *Sci Rep* 11, doi:10.1038/s41598-021-98631-3 | Lattice fatigue 0.15–0.31 σy* |
| `renishaw2017` | V | Renishaw (2017), *Design for metal AM: a beginner's guide* | Overhang 45°, holes, ledges |
| `carrozza2023` | p | Carrozza A, Bircher B, Aversa A, Biamino S (2023), "Investigating complex geometrical features in LPBF-produced parts: a material-based comparison between different titanium alloys", *Met Mater Int*, doi:10.1007/s12540-023-01460-4 | Minimum walls, channels, holes. 300 µm walls and 1 mm channels print in Ti-6Al-4V, but residual powder sticks in the channels: 1 mm is the lower bound for a drainage passage |
| `xometry`, `materialise`, `solukon` | V | Xometry DMLS design guide; Materialise metal design guide; Solukon depowdering guidance | Powder ports 2–5 mm, ≥ 3 mm; length/diameter about 4 |
| `dodds2014` | P | Dodds RM et al. (2014), "Grip strength across the life course", *PLoS ONE* 9:e113637, doi:10.1371/journal.pone.0113637 | Grip strength percentiles; P95 male 661 N (D) |
| `hidh` | S | NASA Human Integration Design Handbook, NASA/SP-2010-3407/REV1 (2014), §9.12.2, App. B Table 8 | Maximum crew grip 783 N; edge radii; pull 449–587 N; finger entrapment |
| `mil1472h` | S | MIL-STD-1472H (2020), *Human Engineering*, §5.7.6.3–4 and Fig. 23 | Edge radii; arm push/pull/inward/outward strength |
| `niosh2004` | S | Cal/OSHA–NIOSH (2004), *Easy Ergonomics: A Guide to Selecting Non-Powered Hand Tools*, Pub. 2004-164 | No sharp edges or deep finger grooves; handle size |
| `kong2005` | P | Kong Y-K, Lowe BD (2005), "Optimal cylindrical handle diameter for grip force tasks", *Int J Ind Ergon* 35:495, doi:10.1016/j.ergon.2004.11.003 | Comfort band of handle size; finger force shares (little finger 13–19 %) |
| `seo2008hf` | p | Seo NJ, Armstrong TJ (2008), *Human Factors* 50:734, doi:10.1518/001872008X354192 | Sum of normal forces about 2.3× grip force |
| `seo2011` | P | Seo NJ, Armstrong TJ (2011), "Effect of elliptic handle shape on … twisting ability", *Ergonomics* 54:961, doi:10.1080/00140139.2011.606923 | Elliptic sections (1.6–1.7) twist better; friction radius |
| `ansur2` | S | Gordon CC et al. (2014), ANSUR II, NATICK/TR-15/007 | Hand length and breadth percentiles |
| `faa-hfds` | S | FAA Human Factors Design Standard, Appendix B (anthropometry) | Finger widths (index P95 male 23 mm) |
| `tirloni2021` | P | Tirloni SF et al. (2021), "Cutting force measurement …: a systematic review", *EXCLI J* 20:727, doi:10.17179/excli2021-3167 | Working grip 53 N mean, 131 N peak; moments in knife use |
| `cochran1986` | P | Cochran DJ, Riley MW (1986), "An evaluation of knife handle guarding", *Human Factors* 28:295, doi:10.1177/001872088602800305 | Guard height vs slip force (plateau at 15.2 mm, 178 N); fillet no effect |
| `buchholz1988` | P | Buchholz B, Frederick LJ, Armstrong TJ (1988), *Ergonomics* 31:317, doi:10.1080/00140138808966676 | Skin friction on aluminium 0.33 (dry) |
| `fransson1993` | p | Fransson-Hall C, Kilbom Å (1993), "Sensitivity of the hand to surface pressure", *Appl Ergon* 24:181 | 104 kPa acceptable all day |
| `johansson1999` | p | Johansson L et al. (1999), "Perception of surface pressure applied to the hand", *Ergonomics* 42:1274, doi:10.1080/001401399184947 | Discomfort and pain thresholds by hand zone |
| `aldien2005` | P | Aldien Y et al. (2005), *Int J Ind Ergon* 35:267, doi:10.1016/j.ergon.2004.09.005 | Contact pressure at the hand–handle interface |
| `osullivan2002` | p | O'Sullivan LW, Gallwey TJ (2002), *J Electromyogr Kinesiol* 12:275, doi:10.1016/S1050-6411(02)00014-7 | Maximum supination 16.2 N·m |
| `yoshii2015` | P | Yoshii Y et al. (2015), *BioMed Eng OnLine* 14:115 | Maximum wrist torques (flexion up to 11.9 N·m) |
| `kong2007` | P | Kong Y-K et al. (2007), *Ergonomics* (screwdriving torque) | Maximum screwdriving 4.7–5.9 N·m |
| `bogazici2010` | p | Boğaziçi University thesis, hand torque on cylindrical handles (257 men) | Hand torque 6.97 ± 1.85 N·m |
| `en8442` | S | EN/ISO 8442-1 and -2, handle twist tests (via CATRA) | Handle twist 2.2–4.5 N·m |
| `iso8442-5` | S | ISO 8442-5, sharpness and edge retention test (via CATRA) | 50 N, 40 mm stroke; initial cut > 50 mm; retention ≥ 150 mm plain, ≥ 1,500 mm serrated |
| `gost51501` | S | GOST R 51501-99, tourist knives, cl. 9.5.1 | 5 mm point deflection, residual ≤ 1 mm |
| `mil8662e` | S | MIL-K-8662E (1974), pilot survival knife | Blade bend clamped 38 mm from the point |
| `mil810h` | S | MIL-STD-810H, Method 516.8, Procedure IV | Drop 1.22 m, 26 orientations |
| `fl1` | S | ANSI/PLATO FL1-2019 | Drop ≥ 1 m onto concrete |
| `en388` | S | EN 388 puncture levels (20 / 60 / 100 / 150 N) | Axial tip load 150 N |
| `gilchrist2008` | P | Gilchrist MD et al. (2008), *Forensic Sci Int* 177:52 | Penetration force ≤ 40 N in skin simulant |
| `nolan2013` | p | Nolan G, Hainsworth SV, Rutty GN (2013), *J Forensic Sci* 58:372 | Clothing adds 8–21 N |
| `hainsworth2008` | P | Hainsworth SV et al. (2008), *Int J Legal Med* 122:281, doi:10.1007/s00414-007-0202-6 | Tip radius controls penetration |
| `thomas2018` | V | Thomas L (2018), knifesteelnerds.com, CATRA regression and edge-geometry articles | Edge retention vs angle and behind-edge thickness |
| `zhang2023` | P | Zhang et al. (2023), *Materials* 16:5375 | Cut depth vs angle 18–54° |
| `landes` | p | Landes R (2006), *Messerklingen und Stahl* (via Thomas) | Edge stability vs hardness |
| `us4823467` | p | US Patent 4,823,467, serrated knife | Serration geometry |
| `mcgorry2005` | p | McGorry RW, Dowd PC, Dempsey PG (2005), *Appl Ergon* | Moments in knife use |
| `walilko2005` | P | Walilko TJ, Viano DC, Bir CA (2005), "Biomechanics of the head for Olympic boxer punches to the face", *Br J Sports Med* 39:710 | Origin of the former 3,427 N spine load |
| `rs-v3` | O | `requirements/RS-karambit-v3.toml` | Blade-before-hand yield sequence ≥ 1.2 |
| `g3-diagnostics` | D | `evidence/g3/v3.0.0/diagnostics/seq_per_lc.json` | The sequence must be checked per load case |

## Evidence gaps and the physical validation they need

These values rest on weak evidence. Physical tests on printed coupons and parts should replace them before any claim
of fitness for use.
- **Serration tooth loads.** No measured per-tooth load data exists. The 180 N snag and 60 N pry are human-strength
  bounds.
- **Tip loads.** The side and twist loads at the point are derived, and the off-axis share of piercing load is an
  assumption.
- **Edge retention of Ti-6Al-4V.** No independent CATRA data exists; ISO 8442-5 on printed and ground edges is
  needed.
- **Ring loads.** No little-finger hook strength data exists. The ring is a finger-entrapment hole by design (HIDH),
  and ring avulsion can occur at modest force, so the ring must be treated as a hazard.
- **Grip below 78.5 mm.** No data exists for smaller handles.
- **Friction.** No data exists for as-printed Ti-6Al-4V on skin; the wet value of 0.15 is an assumption.
- **Thin walls and down-skin faces.** The thin-wall law comes from one study in one orientation, and the down-skin
  skin depth is derived.
- **TPMS.** The stiffness constants are compression-test lower bounds. FE homogenisation of the actual cell is
  needed, and tension fatigue of sheet TPMS is extrapolated.
- **Temperature.** No L-PBF tensile data exists at 60–100 °C; the derate uses the wrought handbook curve, which is
  conservative.
- **Drop.** The drop case is a physical test by design (MIL-STD-810H procedure, pass criteria assumed).

## Consistency notes

- **Behind-edge thickness.** With one grind (EDGE-1), the thickness behind the edge follows from the angle and the
  apex flat: t(d) = apex + 2 d tan θ. This gives 0.36–0.47 mm plus the apex at 0.5 mm. A separate limit would either
  repeat the angle band or contradict it, so none is set.
- **Ring bore.** The 24.9 mm bore is larger than the 8 mm horizontal-hole rule allows. It is printed with a
  support-free teardrop crown and machined round, inside the finishing allowance.
- **Thin-wall law, nominal section.** Xie et al. fit k_t = (1 − 2Ts/t)^n to stress on the nominal section. G5
  therefore uses the nominal thickness for stiffness and stress and applies k_t once, to the allowable. Reducing the
  section as well would count the rough layer twice. A wall's two faces each contribute their own Ts (up-skin
  0.03 mm, down-skin 0.10 mm, ground 0).
- **Permanent set.** The computation is linear-elastic, so "no yield at the limit load with SF 1.2" stands in for
  "no permanent set". The GOST-style 1 mm set test is a physical test.
- **Objective weights.** The weights and exponents of W_ERGOLIGHT_v2 are design choices made under the owner's rules
  (`owner-objective`): mass 0.30, the worst safety-factor margin 0.25, the other terms 0.45, and E^5. They are tested
  for two properties before the decoder is frozen: raising any failing safety factor to its requirement must outscore
  the mass it costs, and 1 % of E must cost at least 10 g of mass credit.
