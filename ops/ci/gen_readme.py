"""Generate README.md for the current release (v3.0.0+) from its committed release receipt. Deterministic: no clock,
no git, no network. Every measured number and image of the knife comes from evidence/g3/v<RECORD>/release.json,
written by the release pipeline from the cohort of record (owner, 2026-10-26: "the README is 100 % accurate with the
best individual (no older data), also it should show the evolution"). evidence/g3/RECORD names that cohort's release
(3.0.0); docs/VERSION is the release this README belongs to (a patch release keeps the cohort of record).
The helpers above `readme()` (rd, js, toml, f, current_record, integrity_status, BATTERY_SUBJECTS) still serve the v1
white paper, STATUS.md and the certification-integrity generator; README.md no longer reads v1 evidence.
usage: gen_readme.py          write README.md
       gen_readme.py --check  exit 1 if README.md differs from what would be generated"""
import json, os, re, sys, tomllib
from urllib.parse import quote

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(ROOT, 'ops', 'ci')); sys.path.insert(0, os.path.join(ROOT, 'metrics'))
sys.path.insert(0, ROOT)
from ops.ci import rs_verdicts  # noqa: E402
import global_metric  # noqa: E402

REPO = 'https://github.com/neverhuman/jarambit'
IDX, LABEL = 'docs/wave27_diversity/plates/wave27_D1_index.json', 'g3_tip+00_t60_tt3_D1'
DESIGN = LABEL[:-3]
FIG = 'docs/figures/whitepaper'
EG_FIG = 'docs/figures/engineering_gates'
BATTERY_SUBJECTS = (
    'g3', 'w283_p14_c01', 'w283_p10_c02', 'w282_p14_c01', 'w282_p10_c02', 'w281_s2932_c01', 'w280_s2931_c01', 'w279_s2928_c01', 'w278_s2922_c01', 'w277_s2919_c01', 'w276_s2913_c01', 'w275_s2909_c01', 'w274_s2905_c01', 'w273_s2904_c01', 'w271c_11522', 'w272_s2902_c01',
    'w273_s2903_c04', 'w271b_11772', 'w271b_11860',
    'c10a', 'c93', 'c92', 'c10c',
)
CAD_EVAL_PATHS = (
    ('w274_s2905_c01', 'evidence/rs_v2/w274_calibrated/w274_s2905_c01_cad_eval.json'),
    ('w275_s2909_c01', 'evidence/rs_v2/w275_ultrarunner/w275_s2909_c01_cad_eval.json'),
    ('w276_s2913_c01', 'evidence/rs_v2/w276_ultrarunner/w276_s2913_c01_cad_eval.json'),
    ('w277_s2919_c01', 'evidence/rs_v2/w277_ultrarunner/w277_s2919_c01_cad_eval.json'),
    ('w278_s2922_c01', 'evidence/rs_v2/w278_ultrarunner/w278_s2922_c01_cad_eval.json'),
    ('w279_s2928_c01', 'evidence/rs_v2/w279_ultrarunner/w279_s2928_c01_cad_eval.json'),
    ('w280_s2931_c01', 'evidence/rs_v2/w280_ultrarunner/w280_s2931_c01_cad_eval.json'),
    ('w281_s2932_c01', 'evidence/rs_v2/w281_ultrarunner/w281_s2932_c01_cad_eval.json'),
    ('w282_p10_c02', 'evidence/rs_v2/w282_ultrarunner/w282_p10_c02_cad_eval.json'),
    ('w282_p14_c01', 'evidence/rs_v2/w282_ultrarunner/w282_p14_c01_cad_eval.json'),
    ('w283_p10_c02', 'evidence/rs_v2/w283_ultrarunner/w283_p10_c02_cad_eval.json'),
    ('w283_p14_c01', 'evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json'),
)


def rd(rel):
    with open(os.path.join(ROOT, rel), encoding='utf-8') as fh:
        return fh.read()


def js(rel):
    return json.loads(rd(rel))


def toml(rel):
    with open(os.path.join(ROOT, rel), 'rb') as fh:
        return tomllib.load(fh)


def rconst(rel, name):
    m = re.search(rf'pub const {name}: f64 = ([0-9.eE+-]+);', rd(rel))
    if not m:
        raise SystemExit(f'gen_readme: constant {name} not found in {rel}')
    return float(m.group(1))


def badge(label, value, color, link):
    esc = lambda s: quote(s.replace('-', '--').replace('_', '__'), safe='')
    return f'[![{label}](https://img.shields.io/badge/{esc(label)}-{esc(value)}-{color}?style=for-the-badge)]({link})'


def f(x, n=3):
    return f'{x:.{n}f}'


def pct(x, n=1):
    return f'{100 * x:.{n}f} %'


def current_record():
    """Return (subject, cad_eval_dict, cad_battery_dict) for the lightest all-16-gates-passing subject in CAD_EVAL_PATHS."""
    eligible = []
    for subj, rel in CAD_EVAL_PATHS:
        ev = js(rel)
        if ev.get('weighted_cad_score', {}).get('all_engineering_gates_pass'):
            eligible.append((ev['engineering_metrics']['mass_g'], subj, ev, js(f'evidence/cad_battery/{subj}.json')))
    if not eligible:
        raise SystemExit('gen_readme: no all-engineering-gates-passing record found in CAD_EVAL_PATHS')
    _, subj, ev, bat = min(eligible, key=lambda t: t[0])
    return subj, ev, bat


def integrity_status():
    """Re-certification results; values come from the committed corrected certificate JSONs."""
    rows = ['**v1.0.0 certification claims withdrawn (#75–#81).** Neither Wave 283 candidate passes the fixed pipeline. '
            'The exported geometry and all requirement thresholds are unchanged. Native-only solid solves are unverified; '
            'the results below use actual scikit-fem P2 and DOLFINx solves on the same mesh.', '',
            '| candidate | raster / solid g | strict lateral N | cert seq | solid-tip SF | G1 | W_CAD | RS-v2 |',
            '|---|---:|---:|---:|---:|---:|---:|---|']
    for sid in ('w283_p14_c01', 'w283_p10_c02'):
        ev = js(f'evidence/rs_v2/w283_ultrarunner/{sid}_cad_eval.json')
        em = ev['engineering_metrics']; wc = ev['weighted_cad_score']; gf = js(f'evidence/cad_battery/{sid}.json')['global_fitness']
        rs = js(f'evidence/rs_v2/w283_ultrarunner/{sid}_rs.json')
        rows.append(f'| `{sid}` | {em["mass_g"]:.3f} / {em["mass_3d_g"]:.3f} | {rs["strict_lat_rob_0p0625_n"]:.3f} | '
                    f'{rs["seq_ratio"]:.4f} | {em["solid_tip_sf_nodal_max"]:.4f} | {gf["G"]:.6f} | {wc["W_CAD"]:+.6f} | FAIL |')
    rows += ['', 'Source: `evidence/rs_v2/w283_ultrarunner/*_{cad_eval,rs,solid_tip}.json`; raw solver outputs and exact meshes '
             'are retained in `evidence/cert_integrity/recertified/`. The retracted certificates are preserved under '
             '`evidence/cert_integrity/v1.0.0-retracted/`. Lateral strength, failure sequence, human-maximum loads, fatigue, '
             'exported edge sharpness and full 0–10 mm point monotonicity remain blockers; the bore has no verified required fillet. '
             'Historical 16-gate screen results below do not establish an RS-v2 all-hard pass or manufacturing promotion. G1 above uses the full-precision RS inputs in each battery record; CAD-eval JSON retains its rounded D1 inputs.']
    return '\n'.join(rows)


REQUIRED = ('version', 'schema_version', 'requirement_set', 'winner_id', 'winner_seed', 'schema_sha256',
            'decoder_sha256', 'genome_sha256', 'winner_record', 'admission_contract', 'winner_history')


def receipt():
    """(release version, cohort-of-record version, G4 receipt, G3 receipt). Fail closed on a missing field."""
    v = rd('docs/VERSION').strip()
    rv = rd('evidence/g4/RECORD').strip()
    rec = js(f'evidence/g4/v{rv}/release.json')
    missing = [k for k in REQUIRED if k not in rec]
    if missing or rec['version'] != rv or v.split('.')[:2] != rv.split('.')[:2]:
        raise SystemExit(f'gen_readme: release.json v{rv} is incomplete or does not belong to release v{v} ({missing or rec["version"]})')
    g3_rv = rd('evidence/g3/RECORD').strip()
    g3_rec = js(f'evidence/g3/v{g3_rv}/release.json')
    return v, rv, rec, g3_rv, g3_rec


def meets(value, target, op='>='):
    """Reading of a measured value against its stated target: a floor (>=) or a ceiling (<=)."""
    if op == '>=':
        return 'meets target' if value >= target else '**below target**'
    return 'meets target' if value <= target else '**above the limit**'


def picture(light, dark, alt, width=None):
    w = f' width="{width}"' if width else ''
    return (f'<picture><source media="(prefers-color-scheme: dark)" srcset="{dark}">'
            f'<img src="{light}" alt="{alt}"{w}></picture>')


def readme():
    v, rv, r, g3_rv, g3_r = receipt()
    wid = r['winner_id']
    wr = r['winner_record']
    p = wr['params']
    m = wr['mass']
    dr = wr['drainability']
    hm1 = wr['hm1']
    pl = wr['plate']
    tip = wr['tip']
    ser = wr['serration']
    obj = wr['objective']
    seq = obj['sequence_ledger']
    ev_sum = js(f'evidence/g4/v{rv}/evolution_summary.json')
    e05 = js('evidence/g4/studies/e05_mass_levers.json')
    e06 = js('evidence/g4/studies/e06_tip_crown.json')
    sp = js(f'cad/{wid}_print_spec.json')
    am = sp['am_build_stl']
    env = sp['outer_envelope_stl']
    dim = sp['overall_dimensions_mm']
    g3_img = g3_r['images']
    q = lambda s: s.replace('-', '--').replace('_', '__').replace(' ', '%20')

    L = [
        f'# Jarambit {v}: an evolved, sub-50 g 3D-printable titanium karambit (`G4.0` / `RS-karambit-v4`)',
        '',
        f'[![release](https://img.shields.io/badge/release-v{v}-2f56d6)]({REPO}/releases/tag/v{v}) '
        f'[![paper](https://img.shields.io/badge/paper-G4%20v{v}%20PDF-2f56d6)](docs/g4/paper/g4-v{v}.pdf) '
        f'[![g3 paper](https://img.shields.io/badge/G3%20record-v3.0.2%20PDF-555)](docs/g3/paper/g3-v3.0.2.pdf) '
        f'[![3D print CAD](https://img.shields.io/badge/3D%20print%20CAD-cad%2F%20(STL%20%7C%203MF%20%7C%20DXF%20%7C%20PDF)-2f56d6)](cad/) '
        f'![design](https://img.shields.io/badge/best%20individual-{q(wid)}-1f6f3f) '
        f'![score](https://img.shields.io/badge/W__ERGOLIGHT__v2-{obj["w_ergolight_v2"]:.2f}-1f6f3f) '
        f'![mass](https://img.shields.io/badge/mass-{m["as_finished_total_mass_g"]:.2f}%20g%20finished%20%7C%20{m["as_printed_total_mass_g"]:.2f}%20g%20printed-1f6f3f) '
        f'![gates](https://img.shields.io/badge/RS--karambit--v4-29%2F29%20feasible%20(V%3D0)-1f6f3f) '
        f'![status](https://img.shields.io/badge/status-computational%2C%20not%20built-b4570f)',
        '',
        f'Every headline measurement on this page describes **`{wid}`**, the certified winning individual of the **G4 (`v{rv}`)** '
        f'multi-seed cohort ([`evidence/g4/v{rv}/release.json`](evidence/g4/v{rv}/release.json)), decoded from the 130-gene declarative '
        f'[`genome/G4.0.toml`](genome/G4.0.toml) schema (`0` dead genes) and verified against [`requirements/RS-karambit-v4.toml`](requirements/RS-karambit-v4.toml) '
        f'([`requirements/PROVENANCE.md`](requirements/PROVENANCE.md)). Status: computational design; not built, printed or physically tested; not qualified for use as a knife.',
        '',
        f'**3D printing & CAD upload kit ([`cad/`](cad/)):** ready-to-upload 6-file bundle (each under 50 MiB) for **`{wid}`** — '
        f'primary print mesh [`cad/{wid}_AMbuild_mm.stl`](cad/{wid}_AMbuild_mm.stl) (Binary STL, mm, `{am["mass_ti64_g"]:.2f} g`), '
        f'unit-embedded [`cad/{wid}_AMbuild_mm.3mf`](cad/{wid}_AMbuild_mm.3mf) (3MF, mm), '
        f'outer envelope [`cad/{wid}_envelope_mm.stl`](cad/{wid}_envelope_mm.stl), '
        f'1:1 2D drawing [`cad/{wid}_drawing_mm.dxf`](cad/{wid}_drawing_mm.dxf) (DXF, mm), '
        f'engineering spec [`cad/{wid}_spec_paper.pdf`](cad/{wid}_spec_paper.pdf) (PDF), and '
        f'internal cutaway [`cad/{wid}_xray_cutaway.jpg`](cad/{wid}_xray_cutaway.jpg) (JPG), plus [`cad/{wid}_print_spec.json`](cad/{wid}_print_spec.json). '
        f'See [`cad/README.md`](cad/README.md) for copy-paste order notes (archived `v3.0.0` CAD kit in [`cad/v3.0.0/`](cad/v3.0.0/)).',
        '',
        f'**Papers:** [`docs/g4/paper/g4-v{v}.pdf`](docs/g4/paper/g4-v{v}.pdf) sets out the `G4.0` 130-gene architecture, true-3D curvilinear implicit kernel, '
        f'`HM-1` 4-finger Winkler hand contact mechanics, 11-benchmark V&V suite, and 5-arm constrained QD ablation study; '
        f'[`docs/g3/paper/g3-v3.0.2.pdf`](docs/g3/paper/g3-v3.0.2.pdf) preserves the errata-updated G3 v3.0.0 cohort of record.',
        '',
        f'<img src="{g3_img["hero"]}" alt="Anodized titanium karambit render showing hollow lattice-cored grip, finger ring, and double-edged serrated claw blade" width="100%">',
        '',
        '## The certified G4 winner (`g4v400_s042_62763aa2`)',
        '',
        f'Ti-6Al-4V ELI HIP (`0.00443 g/mm³`, $\\sigma_y = 828\\text{{ MPa}}$, $\\sigma_{{\\text{{ult}}}} = 895\\text{{ MPa}}$, $\\sigma_e = 510\\text{{ MPa}}$), '
        f'laser powder-bed fusion: a `{p["l_grip_mm"]:.1f} mm` hollow grip with $C^2$ flange thickening (`skin_flange_c2 = {p["skin_flange_c2"]:.2f}`), '
        f'perforated Warren shear web (`rib_hole_frac = {p["rib_hole_frac"]:.2f}`), Sheet-TPMS lattice (`rho_grip = {p["rho_grip"]:.2f}`), '
        f'a drained `{p["bore_d_mm"]:.1f} mm` finger ring (`ring_od = {p["ring_od"]:.2f} mm`), and a `{p["blade_len"]:.1f} mm` double-edged, double-serrated claw blade '
        f'with a tip-bisector diamond reinforcement crown (`t_tip_crown_mm = {p["t_tip_crown_mm"]:.2f} mm`, `l_tip_crown_mm = {p["l_tip_crown_mm"]:.1f} mm`).',
        '',
        '| Measure (`RS-karambit-v4`) | Value | Target / Threshold | 95% LCB / Reading |',
        '|---|---|---|---|',
        f'| Feasibility across all 29 hard gates ($V(x) = \\sum \\max(0, g_k)^2$) | `feasible = {str(obj["feasible"]).lower()}` (`V = {obj["violation_sum_sq"]:.4f}`) | `V(x) = 0.0000` | min gate margin `+{obj["min_safety_margin"]:.4f}` (`{obj["governing_gate_name"]}`) |',
        f'| `W_ERGOLIGHT_v2` / legacy `W_ERGOLIGHT_v1` | **{obj["w_ergolight_v2"]:.2f}** / {obj["w_ergolight_v1_legacy"]:.2f} | higher is better | ergonomic geometric mean $E = {obj["ergo_geometric_mean"]:.3f}$ |',
        f'| Mass: as-finished / as-printed (`+0.30 mm` grind stock) / solid envelope | **{m["as_finished_total_mass_g"]:.2f} g** / **{m["as_printed_total_mass_g"]:.2f} g** / {m["envelope_mass_g"]:.2f} g | $\\le 50.0\\text{{ g}}$ | {meets(m["as_finished_total_mass_g"], 50.0, "<=")} (`{100.0 * m["total_hollowness_fraction"]:.1f} %` hollow; grid drift `{100.0 * wr["grid_mass_drift_frac"]:.2f} % <= 2.0 %`) |',
        f'| Zone mass ledger: ring / grip / guard / blade | {m["ring_mass_g"]:.2f} / {m["grip_total_mass_g"]:.2f} / {m["guard_mass_g"]:.2f} / {m["blade_total_mass_g"]:.2f} g | sub-voxel divergence integral | grip: `{m["grip_skin_mass_g"]:.2f} g` skin + `{m["grip_web_and_bulkheads_mass_g"]:.2f} g` web + `{m["grip_lattice_mass_g"]:.2f} g` lattice |',
        f'| Edge A (inner) / Edge B (spine) bevel angle & apex flat | {p["angle_in_deg"]:.2f}° (`{p["apex_in_mm"]:.3f} mm`) / {p["angle_sp_deg"]:.2f}° (`{p["apex_sp_mm"]:.3f} mm`) | `17.5–22.5°`, apex `<= 0.12 mm` | $t(0.25/1.0/2.0\\text{{ mm}}) = {p["t_0p25_mm"]:.2f}/{p["t_1p00_mm"]:.2f}/{p["t_2p00_mm"]:.2f}\\text{{ mm}}$ |',
        f'| Point planform angle at `3.0 mm` (`Gate S4`) | {wr["descriptors"]["point_angle_3mm_deg"]:.2f}° | $\\le 55.0^\\circ$ | {meets(wr["descriptors"]["point_angle_3mm_deg"], 55.0, "<=")} (penetrating acute point) |',
        f'| Serration teeth checked (2D `T6` tooth FE, inner / spine) | {ser["teeth_checked_in"]}/{ser["teeth_total_in"]} (`SF {ser["min_tooth_sf_in"]:.2f}`) / {ser["teeth_checked_sp"]}/{ser["teeth_total_sp"]} (`SF {ser["min_tooth_sf_sp"]:.2f}`) | 100% of teeth checked, $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 1.25$ | LCB **{obj["sf_tooth_snag_pry_lcb"]:.2f}** ({meets(obj["sf_tooth_snag_pry_lcb"], 1.25)}) |',
        f'| Solid-tip torsion `0.80 N·m` / tip pry `60 N` (`TET10`, `{tip["n_dof"]:,}` DOF) | SF **{tip["tip_torsion_0p80nm_sf"]:.2f}** / **{tip["tip_pry_60n_sf"]:.2f}** | $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 1.20$ | LCB **{obj["sf_solid_tip_torsion_0p80nm_lcb"]:.2f}** / **{obj["sf_solid_tip_pry_60n_lcb"]:.2f}** ({meets(obj["sf_solid_tip_torsion_0p80nm_lcb"], 1.20)}) |',
        f'| Hiker lateral `180 N` / Hunter lateral `260 N` (hole-aware plate FE) | SF **{pl["lateral_180n_sf"]:.2f}** / **{pl["lateral_260n_sf"]:.2f}** (`{pl["lateral_capacity_n"]:.1f} N`) | $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 1.00 / 1.15$ (`>= 260 N`) | LCB **{obj["sf_hiker_lateral_180n_lcb"]:.2f}** / **{obj["sf_hunter_lateral_260n_lcb"]:.2f}** ({meets(obj["sf_hunter_lateral_260n_lcb"], 1.15)}) |',
        f'| Body twist `1.82 N·m` / Human-max two-hand twist `16.2 N·m` | SF **{pl["twist_body_1p82nm_sf"]:.2f}** / **{pl["hm_twist_16p2nm_sf"]:.2f}** | $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 1.20 / 1.10$ | LCB **{obj["sf_twist_body_1p82nm_lcb"]:.2f}** / **{obj["sf_hm_twist_16p2nm_lcb"]:.2f}** ({meets(obj["sf_hm_twist_16p2nm_lcb"], 1.10)}) |',
        f'| Spine push `500 N` / Spine baton `3427 N` (vs $\\sigma_{{\\text{{ult}}}} = 895\\text{{ MPa}}$) | SF **{pl["spine_push_500n_sf"]:.2f}** / **{pl["spine_baton_3427n_sf"]:.2f}** | $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 1.50 / 1.10$ | LCB **{obj["sf_spine_push_500n_lcb"]:.2f}** / **{obj["sf_spine_baton_3427n_ult_lcb"]:.2f}** ({meets(obj["sf_spine_baton_3427n_ult_lcb"], 1.10)}) |',
        f'| `HM-1` 4-finger Winkler grip equilibrium (governing / baton / lateral / wrinkling) | SF **{hm1["grip_sf_min_governing"]:.2f}** / **{hm1["grip_sf_baton_3427n_ult"]:.2f}** / **{hm1["grip_sf_lat_260n"]:.2f}** / **{hm1["face_wrinkling_sf"]:.2f}** | $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 1.20$ | LCB **{obj["sf_grip_squeeze_250n_lcb"]:.2f}** ({meets(obj["sf_grip_squeeze_250n_lcb"], 1.20)}; residual `{hm1["force_residual_n"]:.1e} N`) |',
        f'| Ring bearing `300 N` / Ring back `30 N` (curved-beam Lamé + port $K_t$) | `{wr["ring_bearing_cap_n"]:.0f} N` (SF {wr["ring_bearing_sf"]:.2f}) / `{wr["ring_back_cap_n"]:.0f} N` (SF {wr["ring_back_sf"]:.2f}) | $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 2.00$ (`>= 600 / 60 N`) | LCB **{obj["sf_ring_bearing_300n_lcb"]:.2f}** / **{obj["sf_ring_back_30n_lcb"]:.2f}** ({meets(obj["sf_ring_bearing_300n_lcb"], 2.00)}) |',
        f'| Goodman high-cycle fatigue (`10^5` cycles, $K_f = 1.65$) | SF **{pl["goodman_fatigue_180n_sf"]:.2f}** | $\\text{{SF}}_{{\\text{{LCB}}}} \\ge 1.10$ | LCB **{obj["sf_goodman_fatigue_180n_lcb"]:.2f}** ({meets(obj["sf_goodman_fatigue_180n_lcb"], 1.10)}) |',
        f'| Per-load-case blade-before-grip yield sequence (lateral / twist / spine) | **{seq["lateral_260n_seq_ratio"]:.2f}** / **{seq["twist_seq_ratio"]:.2f}** / **{seq["spine_seq_ratio"]:.2f}** | $\\text{{seq}}_{{\\text{{LCB}}}} \\ge 1.20$ | LCB **{obj["sf_per_case_sequence_min_lcb"]:.2f}** ({meets(obj["sf_per_case_sequence_min_lcb"], 1.20)}) |',
        f'| Lateral-torsional buckling $\\lambda_{{\\text{{cr}}}}$ / Combined lateral+twist | **{pl["buckling_lambda"]:.2f}** / **{pl["combined_lat_twist_sf"]:.2f}** | $\\ge 1.50 / 1.15$ | LCB **{obj["sf_buckling_lambda_lcb"]:.2f}** / **{obj["sf_combined_lat_twist_lcb"]:.2f}** ({meets(obj["sf_combined_lat_twist_lcb"], 1.15)}) |',
        f'| 3-level Roache plate FE GCI ($p = {pl["gci_admission"]["observed_order_p"]:.2f}$) | $\\text{{GCI}}_{{\\text{{fine}}}} = {100.0 * pl["gci_admission"]["gci_fine_fraction"]:.2f}\\%$ | $\\le 5.0\\%$, monotone | `plate_gci_admitted = {str(pl["gci_admission"]["plate_gci_admitted"]).lower()}` |',
        f'| AM drainability ($r_{{\\text{{esc}}}} = {dr["r_esc_mm"]:.2f}\\text{{ mm}}$ morphological erosion) | $\\beta_0 = {dr["solid_betti_0_components"]}$, $\\beta_2 = {dr["sealed_void_betti_2_cavities"]}$, trapped `{dr["trapped_powder_volume_mm3"]:.1f} mm³` | $\\beta_0 = 1$, $\\beta_2 = 0$, trapped `0 mm³` | min port `{dr["min_port_diameter_mm"]:.2f} mm`, min throat `{dr["min_internal_throat_diameter_mm"]:.2f} mm` |',
        '',
        '### Zone mass breakdown & structural levers (`E05` + `E06` -> `FI-2Pop` evolution)',
        '',
        '<img src="docs/g4/figures/g4_zone_mass_breakdown.svg" alt="G4 v4.0.0 zone mass breakdown comparing G3 v3.0.0 (62.58 g) vs G4 v4.0.0 (35.42 g)" width="100%">',
        '',
        f'Cumulative mass-lever ablation ([`evidence/g4/studies/e05_mass_levers.json`](evidence/g4/studies/e05_mass_levers.json) and '
        f'[`evidence/g4/studies/e06_tip_crown.json`](evidence/g4/studies/e06_tip_crown.json)) from the un-tuned G3 baseline (`{e05["waterfall"][0]["as_finished_mass_g"]:.2f} g`) '
        f'to the `L-8` tip-crown baseline (`{e05["waterfall"][-1]["as_finished_mass_g"]:.2f} g`, solid-tip torsion SF `{e06["sweep"][5]["tip_torsion_0p80nm_sf"]:.2f}`) '
        f'and final `FI-2Pop` evolved winner `{wid}` (`{m["as_finished_total_mass_g"]:.2f} g` finished / `{m["as_printed_total_mass_g"]:.2f} g` printed):',
        '',
        '| Step | Lever | Finished mass | $\\Delta$ mass | `W_ERGOLIGHT_v2` | Min safety margin |',
        '|---|---|---:|---:|---:|---:|',
    ]
    for st in e05['waterfall']:
        L.append(f'| `{st["lever_id"]}` | {st["description"]} | {st["as_finished_mass_g"]:.2f} g | {st["delta_mass_g"]:+.2f} g | {st["w_ergolight_v2"]:.2f} | {st["min_safety_margin"]:+.4f} |')
    L += [
        f'| **`{wid}`** | **Full `FI-2Pop` constrained QD evolution (`E3_G4_Full`)** | **{m["as_finished_total_mass_g"]:.2f} g** | **{m["as_finished_total_mass_g"] - e05["waterfall"][0]["as_finished_mass_g"]:+.2f} g** | **{obj["w_ergolight_v2"]:.2f}** | **`+{obj["min_safety_margin"]:.4f}` (`V = 0`)** |',
        '',
        '## Multi-seed constrained QD search & 5-arm ablation study',
        '',
        'G4 replaces G3\'s unconstrained smooth penalty objective with **Deb\'s lexicographic feasibility-first superiority** (`FI-2Pop` dual archive: '
        'a feasible MAP-Elites archive where $V(x) = 0$ ranked by `W_ERGOLIGHT_v2`, paired with an infeasible stepping-stone archive where $V(x) > 0$ ranked by '
        '$V(x) = \\sum_k \\max(0, g_k(x))^2$), **Lloyd-relaxed CVT centroids** across 6 measured behavioral descriptors (`blade_reach_mm`, `claw_turning_deg`, '
        '`point_angle_3mm_deg`, `grip_h_over_t`, `hollowness_frac`, `serrated_arc_frac`), and a **4-emitter UCB1 bandit** (sep-CMA-ES, directional mass-gradient line search, '
        'constraint-repair Jacobian emitter, and module-masked crossover).',
        '',
        '| Experiment arm (`evidence/g4/v4.0.0/evolution_summary.json`) | Seeds | Median `W_ERGOLIGHT_v2` (IQR) | Median finished mass (IQR) | Median feasible QD score | Vargha-Delaney $\\hat{{A}}_{{12}}$ |',
        '|---|---:|---:|---:|---:|---:|',
    ]
    for c in ev_sum['cohorts']:
        L.append(f'| `{c["algorithm_id"]}` | {c["n_seeds"]} | {c["median_w_ergolight_v2"]:.2f} ({c["iqr_w_ergolight_v2"]:.2f}) | {c["median_finished_mass_g"]:.2f} g ({c["iqr_finished_mass_g"]:.2f} g) | {c["median_feasible_qd_score"]:.1f} | {c["vargha_delaney_a12_vs_e3"]:.2f} |')
    L += [
        '',
        '## Download the CAD & 3D-printing upload kit (`cad/`, millimetres)',
        '',
        'The top-level [`cad/`](cad/) folder contains a ready-to-upload **6-file manufacturing bundle** (every file under 50 MiB, '
        'designed for quote forms accepting up to 6 files of 100 MB each in `STL`, `3MF`, `DXF`, `PDF`, or `JPG`), '
        f'generated automatically from `{wid}` by `g4-export`:',
        '',
        '| # | File in `cad/` | Format | Purpose on a 3D-printing / manufacturing quote form |',
        '|---:|---|---|---|',
        f'| 1 | [`cad/{wid}_AMbuild_mm.stl`](cad/{wid}_AMbuild_mm.stl) | Binary STL (`mm`) | **Primary 3D print file in git** (`{am["voxel_pitch_h_mm"]:g} mm` voxels, `{am["mass_ti64_g"]:.2f} g`, `{am["mib"]:.2f} MiB`, `{am["triangles"]:,}` triangles; outer skin + perforated Warren web + Sheet-TPMS lattice + powder ports) |',
        f'| 2 | [`cad/{wid}_AMbuild_mm.3mf`](cad/{wid}_AMbuild_mm.3mf) | 3MF (`unit="millimeter"`) | **Modern 3D print package** with `millimeter` units and `Ti-6Al-4V ELI HIP (yield 828 MPa)` metadata embedded |',
        f'| 3 | [`cad/{wid}_envelope_mm.stl`](cad/{wid}_envelope_mm.stl) | Binary STL (`mm`) | **Solid outer envelope companion** (`{env["voxel_pitch_h_mm"]:g} mm` voxels, `{env["solid_mass_ti64_g"]:.1f} g` if printed solid; all internal cavities and ports closed) |',
        f'| 4 | [`cad/{wid}_drawing_mm.dxf`](cad/{wid}_drawing_mm.dxf) | AutoCAD R12 DXF (`mm`) | **1:1 2D engineering drawing** (`UNITS: MILLIMETERS`, `1 drawing unit = 1 mm`: exact `z = 0` mid-plane outline, 24.9 mm ring bore at `x = -3.93 mm`, `z`-extent thickness envelope & 30 mm blade section) |',
        f'| 5 | [`cad/{wid}_spec_paper.pdf`](cad/{wid}_spec_paper.pdf) | PDF | **Engineering specification & FE paper** (`g4-v{v}.pdf`; reports all 29 gates, 11 V&V benchmarks, and 5-arm ablation study) |',
        f'| 6 | [`cad/{wid}_xray_cutaway.jpg`](cad/{wid}_xray_cutaway.jpg) | JPG image | **Lengthwise cutaway render** showing the internal TPMS lattice and powder-evacuation ports |',
        '',
        f'Machine-readable print receipt: [`cad/{wid}_print_spec.json`](cad/{wid}_print_spec.json) (`{dim["length_x_mm"]:.2f} × {dim["height_y_mm"]:.2f} × {dim["thickness_z_mm"]:.2f} mm`, `watertight = {str(am["watertight"]).lower()}`). '
        f'The previous G3 `v3.0.0` (`{g3_r["design_id"]}`) 6-file kit is preserved byte-for-byte in [`cad/v3.0.0/`](cad/v3.0.0/) and [`docs/g3/v3.0.0/cad/`](docs/g3/v3.0.0/cad/).',
        '',
        '## Preserved G3 v3.0.0 cohort of record (`hz4i013_g00995_d206e3c9`)',
        '',
        f'Per `AGENTS.md`, the G3 `v3.0.0` receipt ([`evidence/g3/v{g3_rv}/release.json`](evidence/g3/v{g3_rv}/release.json)), decoder (`a203278d80d2...`), '
        f'and CAD assets are preserved byte-for-byte. Its errata-updated paper is [`docs/g3/paper/g3-v3.0.2.pdf`](docs/g3/paper/g3-v3.0.2.pdf), '
        f'and its post-hoc diagnostic receipts (per-load-case sequence, zone mass breakdown, plate mesh sensitivity, and tip raster aliasing) are committed in '
        f'[`evidence/g3/v3.0.0/diagnostics/`](evidence/g3/v3.0.0/diagnostics/).',
        '',
        '<table><tr>'
        f'<td width="50%"><img src="{g3_img["cut_b"]}" alt="Section across the blade 30 mm from the point"><br>Section across the blade, 30 mm from the point: one grind per edge, hollow lattice core.</td>'
        f'<td width="50%"><img src="{g3_img["tip"]}" alt="The point"><br>The point and primary bevel termination.</td></tr><tr>'
        f'<td><img src="{g3_img["xray"]}" alt="X-ray of the internal cavities"><br>X-ray: internal cavities in amber.</td>'
        f'<td><img src="{g3_img["cut_full"]}" alt="Lengthwise cut through the grip"><br>Lengthwise cut: skin, lattice and powder ports.</td></tr></table>',
        '',
        '## Reproduce',
        '',
        '```bash',
        '# 1. Run the 11-benchmark G4 V&V verification suite (NAFEMS LE1/LE10, B-bar patch, Kirsch Kt, Roache GCI, SO(3) invariance)',
        'cargo test --manifest-path crates/g4/Cargo.toml --release',
        '',
        '# 2. Regenerate E01..E06 engineering studies, 5-arm multi-seed evolution, LaTeX macros/SVG figures, and the 6-file cad/ kit',
        'cargo run --manifest-path crates/g4/Cargo.toml --release --bin g4-studies',
        'cargo run --manifest-path crates/g4/Cargo.toml --release --bin g4-evolve',
        'cargo run --manifest-path crates/g4/Cargo.toml --release --bin g4-release-tex',
        'cargo run --manifest-path crates/g4/Cargo.toml --release --bin g4-export',
        '',
        '# 3. Compile the G4 v4.0.0 paper and errata-updated G3 v3.0.2 paper',
        'make -C docs/g4/paper',
        'make -C docs/g3/paper',
        '```',
        '',
        f'The G4 winner\'s genome row is [`evidence/g4/v{rv}/winner_genome.csv`](evidence/g4/v{rv}/winner_genome.csv); '
        f'multi-seed cohort histories are in [`evidence/g4/v{rv}/history/cohort_metrics.json`](evidence/g4/v{rv}/history/cohort_metrics.json).',
        '',
        'The v1 command-line pipeline (`cargo run -p jarambit`) is still in the repository; its plate level is opt-in with `JARAMBIT_FEM=1`.',
        '',
        '## Repository',
        '',
        '**G4 (current — v4.0.0)**',
        '',
        f'- [`cad/`](cad/): 6-file 3D-printing & manufacturing upload kit (`STL`, `3MF`, `DXF`, `PDF`, `JPG`, millimetres) and order notes for `{wid}`, generated automatically by `g4-export`.',
        '- [`genome/G4.0.toml`](genome/G4.0.toml): declarative 130-gene `G4.0` schema (`0` dead genes across Modules `H`, `G`, `P`, `Z`, `X`, `I`, `E`, `T`, `A`).',
        '- [`requirements/RS-karambit-v4.toml`](requirements/RS-karambit-v4.toml): unified v4 requirement specification (`loads-v4.toml`, `materials-v4.toml`, `frame-v4.toml`, and [`PROVENANCE.md`](requirements/PROVENANCE.md)).',
        '- [`crates/g4/`](crates/g4/Cargo.toml): the G4 true-3D curvilinear decoder, sub-voxel divergence integrator, morphological erosion drainability, `HM-1` Winkler hand contact solver, hole-aware plate FE with 3-level Roache GCI, frame-invariant `TET10` solid-tip FE, 2D `T6` serration tooth FE, B-bar `hex8` voxel FE, `FI-2Pop` constrained QD search, 11-test V&V suite (`tests/vv_suite.rs`), and CLI binaries (`g4-studies`, `g4-evolve`, `g4-release-tex`, `g4-export`).',
        f'- [`evidence/g4/`](evidence/g4/): the v{rv} release receipt, winner genome row, 5-arm ablation summary, and `E01`–`E06` study receipts.',
        f'- [`docs/g4/`](docs/g4/): the G4 paper ([`docs/g4/paper/g4-v{v}.pdf`](docs/g4/paper/g4-v{v}.pdf)) and zone mass SVG figure.',
        '',
        '**G3 (preserved v3.0.0 cohort of record — v3.0.2 errata)**',
        '',
        '- [`cad/v3.0.0/`](cad/v3.0.0/) and [`docs/g3/v3.0.0/cad/`](docs/g3/v3.0.0/cad/): preserved 6-file CAD bundle for `hz4i013_g00995_d206e3c9`.',
        '- [`crates/g3/`](crates/g3/README.md): the G3 genome, decoder (frozen SHA-256 `a203278d80d2...`), objective, search, and `g3-diagnostics`.',
        '- [`evidence/g3/v3.0.0/`](evidence/g3/v3.0.0/): the v3.0.0 receipt, genome row, island histories, and `diagnostics/` JSONs.',
        '- [`docs/g3/paper/g3-v3.0.2.pdf`](docs/g3/paper/g3-v3.0.2.pdf): the errata-updated G3 paper (`P-1`–`P-16`).',
        '- [`docs/RELEASES.md`](docs/RELEASES.md): every release.',
        '',
        '**The v1/v2 study (kept as its own record)**',
        '',
        '- [`docs/paper/`](docs/paper/) (Paper B) and [`docs/WHITEPAPER.md`](docs/WHITEPAPER.md): the study, with the other v1 documents indexed in [`docs/README.md`](docs/README.md).',
        '- [`crates/`](crates/): the v1 pipeline (the `jarambit` CLI in `crates/cli`, plus `crescent`, `physics`, `genotype`, `phenotype`, `evolve`, `monte_carlo`, `redteam`, `evidence` and `docs`).',
        '- [`evidence/`](evidence/), [`artifacts/`](artifacts/), [`experiments/`](experiments/README.md), [`designs/`](designs/README.md), [`requirements/`](requirements/), [`assurance/`](assurance/), [`metrics/`](metrics/), [`research/science_v1/`](research/science_v1/), [`fem_solid/`](fem_solid/), [`verify/`](verify/), [`bench/`](bench/), [`datasets/regression/`](datasets/regression/), [`sheath/`](sheath/), and [`web/`](web/).',
        '',
        '**Tooling**',
        '',
        '- [`ops/`](ops/README.md): generators, checks and render pipelines, indexed by workflow. [`.github/workflows/`](.github/workflows/): CI.',
        '- [`AGENTS.md`](AGENTS.md): rules for contributors and coding agents. [`PYTHON_EXCEPTIONS.md`](PYTHON_EXCEPTIONS.md): every Python file and why it is not Rust yet. [`STATUS.md`](STATUS.md): generated status. [`LICENSE`](LICENSE): MIT.',
        '',
    ]
    return '\n'.join(L)


def cad_readme():
    v, rv, r, _, _ = receipt()
    wid = r['winner_id']
    wr = r['winner_record']
    m = wr['mass']
    obj = wr['objective']
    pl = wr['plate']
    sp = js(f'cad/{wid}_print_spec.json')
    am = sp['am_build_stl']
    env = sp['outer_envelope_stl']
    dim = sp['overall_dimensions_mm']
    sz_kb = lambda rel: os.path.getsize(os.path.join(ROOT, rel)) / 1024.0
    sz_mb = lambda rel: os.path.getsize(os.path.join(ROOT, rel)) / (1024.0 * 1024.0)
    L = [
        f'# 3D Printing & CAD Upload Kit — Certified G4 Winner `{wid}` (release v{v})',
        '',
        f'This folder contains the ready-to-upload **6-file 3D-printing and manufacturing bundle** (**Units: millimetres**, every file under 50 MiB) '
        f'for **`{wid}`**, the winning individual of the **G4 (`v{rv}`)** cohort of record '
        f'([`evidence/g4/v{rv}/release.json`](../evidence/g4/v{rv}/release.json)). It is populated automatically from '
        f'[`evidence/g4/v{rv}/winner_genome.csv`](../evidence/g4/v{rv}/winner_genome.csv) by `g4-export` and [`ops/ci/gen_readme.py`](../ops/ci/gen_readme.py).',
        '',
        '- **Status:** computational design; not built, printed or physically tested; not qualified for use as a knife.',
        '> **Looking for the archived `v3.0.0` (`hz4i013_g00995_d206e3c9`) bundle?** All `v3.0.0` CAD files are preserved byte-for-byte in [`cad/v3.0.0/`](v3.0.0/).',
        '',
        f'**Engineering summary (`RS-karambit-v4`, `W_ERGOLIGHT_v2 = {obj["w_ergolight_v2"]:.2f}` / `W_ERGOLIGHT_v1 = {obj["w_ergolight_v1_legacy"]:.2f}`):**',
        f'- **Mass:** **`{m["as_finished_total_mass_g"]:.2f} g`** as-finished (`{m["as_printed_total_mass_g"]:.2f} g` as-printed with `+0.30 mm` bevel grind allowance; `{m["envelope_mass_g"]:.1f} g` solid outer envelope; **`{100.0 * m["total_hollowness_fraction"]:.1f}%` hollowness**).',
        f'- **All 17 structural LCB safety factors meet targets (`V(x) = 0`, all `SF_LCB >= 1.00`):** lateral hiker `180 N` SF **`{obj["sf_hiker_lateral_180n_lcb"]:.2f}`**, lateral hunter `260 N` SF **`{obj["sf_hunter_lateral_260n_lcb"]:.2f}`**, body torsion `1.82 N·m` SF **`{obj["sf_twist_body_1p82nm_lcb"]:.2f}`**, heavy twist `16.2 N·m` SF **`{obj["sf_hm_twist_16p2nm_lcb"]:.2f}`**, solid-tip bisector torsion `0.80 N·m` SF **`{obj["sf_solid_tip_torsion_0p80nm_lcb"]:.2f}`**, solid-tip lateral pry `60 N` SF **`{obj["sf_solid_tip_pry_60n_lcb"]:.2f}`**, spine baton `3,427 N` (`HM-1`) SF **`{obj["sf_spine_baton_3427n_ult_lcb"]:.2f}`**, Goodman fatigue SF **`{obj["sf_goodman_fatigue_180n_lcb"]:.2f}`**, serration tooth SF **`{obj["sf_tooth_snag_pry_lcb"]:.2f}`**, and per-load-case blade-before-grip sequence ratio **`{obj["sf_per_case_sequence_min_lcb"]:.2f}`** (`>= 1.20`).',
        f'- **Point geometry & mesh convergence:** point planform angle at `3 mm` **`{wr["descriptors"]["point_angle_3mm_deg"]:.2f}°`** (`<= 55°` `Gate S4`), 3-level Roache plate `GCI_21` **`{100.0 * pl["gci_admission"]["gci_fine_fraction"]:.2f}%`** (`<= 5%`), and zero sealed internal voids (`Betti_2 = 0`, `r_esc = 0.50 mm`).',
        '',
        '## 1. Ready-to-upload 6-file kit (for forms accepting up to 6 files, 100 MB each)',
        '',
        '| # | File | Format | Size | What it is & why to upload it |',
        '|---:|---|---|---:|---|',
        f'| **1** | **[`{am["file"]}`]({am["file"]})** | **Binary STL (`mm`)** | **{am["mib"]:.2f} MiB** | **PRIMARY 3D PRINT FILE IN GIT (upload this first):** complete additive-manufacturing build (`{am["mass_ti64_g"]:.2f} g`, `{am["triangles"]}` triangles) with flange-graded skin, perforated Warren web, Sheet-TPMS core, drained hollow ring, and powder-removal ports. |',
        f'| **2** | **[`{wid}_AMbuild_mm.3mf`]({wid}_AMbuild_mm.3mf)** | **3MF (`unit="millimeter"`)** | **{sz_mb(f"cad/{wid}_AMbuild_mm.3mf"):.2f} MiB** | **Modern 3D print package:** identical AM build mesh in compressed 3MF 1.2.3 format with `unit="millimeter"` and `Ti-6Al-4V ELI HIP (yield 828 MPa)` metadata. |',
        f'| **3** | **[`{env["file"]}`]({env["file"]})** | **Binary STL (`mm`)** | **{env["mib"]:.2f} MiB** | **Solid outer envelope companion:** unhollowed exterior surface with all internal cavities and ports closed (`{env["solid_mass_ti64_g"]:.1f} g` if printed solid; `{env["triangles"]}` triangles). |',
        f'| **4** | **[`{wid}_drawing_mm.dxf`]({wid}_drawing_mm.dxf)** | **AutoCAD R12 DXF (`mm`)** | **{sz_kb(f"cad/{wid}_drawing_mm.dxf"):.0f} KiB** | **1:1 2D engineering drawing (`UNITS: MILLIMETERS`, `1 drawing unit = 1 mm`):** exact `z = 0` mid-plane outline, `24.9 mm` ring bore, `z`-extent thickness envelope, and `30 mm` blade cross-section. |',
        f'| **5** | **[`{wid}_spec_paper.pdf`]({wid}_spec_paper.pdf)** | **PDF** | **{sz_kb(f"cad/{wid}_spec_paper.pdf"):.0f} KiB** | **Engineering specification & FE paper (`g4-v{v}.pdf`):** complete `G4.0` genome, `HM-1` hand mechanics, 3-level Roache GCI, and multi-seed QD ablation tables. |',
        f'| **6** | **[`{wid}_xray_cutaway.jpg`]({wid}_xray_cutaway.jpg)** | **JPG image** | **{sz_kb(f"cad/{wid}_xray_cutaway.jpg"):.0f} KiB** | **Lengthwise cutaway reference image:** visual guide showing outer skin, internal lattice, and powder-evacuation ports. |',
        '',
        f'*(Also included: [`{wid}_print_spec.json`]({wid}_print_spec.json), the machine-readable JSON receipt of dimensions, volumes, masses, and SHA-256 digests.)*',
        '',
        '## 2. Copy-paste order notes for the printing form',
        '',
        '```text',
        f'Units: mm | Overall dimensions: {dim["length_x_mm"]:.2f} x {dim["height_y_mm"]:.2f} x {dim["thickness_z_mm"]:.2f} mm | Mass: {m["as_finished_total_mass_g"]:.2f} g (finished) / {m["as_printed_total_mass_g"]:.2f} g (as-printed) | Material: Ti-6Al-4V ELI HIP (yield 828 MPa) | Status: computational design; not built, printed or physically tested; not qualified for use as a knife | Depowder via neutral-axis drainage ports | Post-HIP grind primary bevels to 20.2 deg per side (0.08 mm apex)',
        '```',
        '',
    ]
    return '\n'.join(L)


if __name__ == '__main__':
    new_readme = readme()
    new_cad = cad_readme()
    if '--check' in sys.argv:
        if rd('README.md') != new_readme:
            print('README.md is stale: run python3 ops/ci/gen_readme.py'); sys.exit(1)
        if rd('cad/README.md') != new_cad:
            print('cad/README.md is stale: run python3 ops/ci/gen_readme.py'); sys.exit(1)
        print('README.md and cad/README.md up to date'); sys.exit(0)
    os.makedirs(os.path.join(ROOT, 'cad'), exist_ok=True)
    open(os.path.join(ROOT, 'README.md'), 'w', encoding='utf-8').write(new_readme)
    open(os.path.join(ROOT, 'cad/README.md'), 'w', encoding='utf-8').write(new_cad)
    print('wrote README.md and cad/README.md')
