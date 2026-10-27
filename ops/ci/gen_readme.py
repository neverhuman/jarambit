"""Generate README.md for the current release (v3.0.0+) from its committed release receipt. Deterministic: no clock,
no git, no network. Every computed number and image of the knife comes from evidence/g3/v<RECORD>/release.json,
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


REQUIRED = ('version', 'design_id', 'status', 'provenance', 'cohort', 'score', 'inputs', 'mass', 'blade', 'fe_certification',
            'evolution', 'images', 'figures', 'cad_in_git', 'release_assets', 'grid_check')


def record_declared(v, rv):
    """True when the release's own entry in docs/RELEASES.md names the cohort of record ("cohort of record v<rv>").
    A release keeps the cohort of record of its own major.minor line unless it says otherwise in that entry; v4.0.1
    does, because it withdraws v4.0.0 and returns to the G3 v3.0.0 cohort."""
    if v.split('.')[:2] == rv.split('.')[:2]:
        return True
    m = re.search(rf'^## v{re.escape(v)}\b.*?(?=^## v|\Z)', rd('docs/RELEASES.md'), re.M | re.S)
    return bool(m) and f'cohort of record v{rv}' in ' '.join(m.group(0).split())


def receipt():
    """(release version, cohort-of-record version, receipt). Fail closed on a missing field, a receipt for another
    version, or a cohort of record from another major.minor line that the release's RELEASES.md entry does not name."""
    v = rd('docs/VERSION').strip()
    rv = rd('evidence/g3/RECORD').strip()
    rec = js(f'evidence/g3/v{rv}/release.json')
    missing = [k for k in REQUIRED if k not in rec]
    if missing or rec['version'] != rv or not record_declared(v, rv):
        raise SystemExit(f'gen_readme: release.json v{rv} is incomplete or does not belong to release v{v} ({missing or rec["version"]})')
    return v, rv, rec


DIAGNOSTICS = ('seq_per_lc', 'tip_raster', 'zone_mass', 'plate_meshconv')


def diagnostics(rv):
    """The v3.0.3 diagnostics next to the receipt (written by g3-diagnostics; they change neither the receipt nor the
    ranking). Fail closed if one is missing."""
    return {k: js(f'evidence/g3/v{rv}/diagnostics/{k}.json') for k in DIAGNOSTICS}


def rs_v3_number(key, section=None):
    """A number from requirements/RS-karambit-v3.toml (no TOML dependency: the keys used here are plain `key = value`)."""
    text = rd('requirements/RS-karambit-v3.toml')
    if section:
        text = text.split(f'[{section}]', 1)[1].split('\n[', 1)[0]
    m = re.search(rf'^{re.escape(key)}\s*=\s*([-0-9.eE+]+)', text, re.M)
    if not m:
        raise SystemExit(f'gen_readme: RS-karambit-v3.toml lacks {key}')
    return float(m.group(1))


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
    v, rv, r = receipt()
    i, sc, pv, co, bl, fe, ev = r['inputs'], r['score'], r['provenance'], r['cohort'], r['blade'], r['fe_certification'], r['evolution']
    a, b = bl['edge_inner'], bl['edge_spine']
    img = r['images']
    wid = r['design_id']
    mass = r['mass']['mesh_g']
    q = lambda s: s.replace('-', '--').replace('_', '__').replace(' ', '%20')
    dg = diagnostics(rv)
    seq_fe = dg['seq_per_lc']['per_load_case_sequence_ratio_voxel_fe']
    fat_corr = dg['seq_per_lc']['goodman_fatigue_180n_audit']['corrected_fat_sf_from_true_plate_peak']
    tip_corr = dg['tip_raster']['half_cell_raster_shift_audit_at_0p15_mm2']['sf_cell_center_aligned']
    plate = dg['plate_meshconv']['subsampled_45_station_series']
    plate_lat = ' / '.join(f'{x["lateral_capacity_n"]:.0f}' for x in plate)
    plate_area = ' / '.join(f'{x["max_area_mm2"]:g}' for x in plate)
    on_mesh = lambda v, t: 'meets target on the search mesh only' if v >= t else '**below target**'
    L = []
    L += [f'# Jarambit {v}: an evolved titanium karambit for laser powder-bed fusion', '',
          f'[![release](https://img.shields.io/badge/release-v{v}-2f56d6)]({REPO}/releases/tag/v{v}) '
          f'[![paper](https://img.shields.io/badge/paper-G3%20v{v}%20PDF-2f56d6)](docs/g3/paper/g3-v{v}.pdf) '
          f'[![3D print CAD](https://img.shields.io/badge/3D%20print%20CAD-cad%2F%20(STL%20%7C%203MF%20%7C%20DXF%20%7C%20PDF)-2f56d6)](cad/) '
          f'![design](https://img.shields.io/badge/best%20individual-{q(wid)}-555) '
          f'![score](https://img.shields.io/badge/W__ERGOLIGHT__v1-{sc["score_100"]:.2f}-555) '
          f'![mass](https://img.shields.io/badge/mass-{mass:.1f}%20g-555) '
          f'![status](https://img.shields.io/badge/status-computational%2C%20not%20built-b4570f)', '',
          f'Every computed number and picture on this page describes one design, **`{wid}`**, the best individual of the v{rv} cohort of '
          f'record, and is generated from [`evidence/g3/v{rv}/release.json`](evidence/g3/v{rv}/release.json); the rule values (bounds, loads, targets) are the decoder\'s own. '
          f'Status: {r["status"]}.', '',
          f'**3D printing & CAD upload kit ([`cad/`](cad/)):** ready-to-upload 6-file bundle (each under 50 MiB) — '
          f'primary print mesh [`cad/{wid}_AMbuild_mm.stl`](cad/{wid}_AMbuild_mm.stl) (Binary STL, mm), '
          f'unit-embedded [`cad/{wid}_AMbuild_mm.3mf`](cad/{wid}_AMbuild_mm.3mf) (3MF, mm), '
          f'outer envelope [`cad/{wid}_envelope_mm.stl`](cad/{wid}_envelope_mm.stl), '
          f'1:1 2D drawing [`cad/{wid}_drawing_mm.dxf`](cad/{wid}_drawing_mm.dxf) (DXF, mm), '
          f'engineering spec [`cad/{wid}_spec_paper.pdf`](cad/{wid}_spec_paper.pdf) (PDF), and '
          f'internal cutaway [`cad/{wid}_xray_cutaway.jpg`](cad/{wid}_xray_cutaway.jpg) (JPG). '
          f'See [`cad/README.md`](cad/README.md) for copy-paste order notes.', '',
          f'**Paper:** [`docs/g3/paper/g3-v{v}.pdf`](docs/g3/paper/g3-v{v}.pdf), generated from the same receipt, sets out the '
          'design space, objective, search, measurement chain and every limit.', '',
          f'<img src="{img["hero"]}" alt="The best individual {wid}: anodized grip and ring, double-edged serrated claw blade" width="100%">', '']
    # ---- the best individual
    L += ['## The best individual', '',
          f'Ti-6Al-4V, laser powder-bed fusion: a hollow grip and blade core filled with a printed lattice, a {i["L_g_mm"]:.1f} mm '
          f'grip with a 24.9 mm finger ring, and a double-edged, double-serrated claw blade.', '',
          '| Measure | Value | Target | Reading |', '|---|---|---|---|',
          f'| W_ERGOLIGHT_v1 (draft d), Tier 2, Linux x86_64, h {co["ranking_h_mm"]} mm | {sc["score_100"]:.2f} | higher is better | ergonomic multiplier E {sc["E"]:.3f} |',
          f'| Mass of the print file (watertight 0.2 mm mesh × {r["mass"]["density_g_mm3"]} g/mm³) | {mass:.1f} g | under 50 g | {meets(mass, 50.0, "<=")} |',
          f'| Edge A (inner): per-side angle, apex / bevel | {a["per_side_angle_apex_deg"][0]:.1f}–{a["per_side_angle_apex_deg"][1]:.1f}° / '
          f'{a["per_side_angle_bevel_deg"][0]:.1f}–{a["per_side_angle_bevel_deg"][1]:.1f}° | 17.5–22.5° | ground from {a["ground_from_mm_from_point"]:.1f} to '
          f'{a["ground_to_mm_from_point"]:.1f} mm from the point |',
          f'| Edge B (spine): per-side angle, apex / bevel | {b["per_side_angle_apex_deg"][0]:.1f}–{b["per_side_angle_apex_deg"][1]:.1f}° / '
          f'{b["per_side_angle_bevel_deg"][0]:.1f}–{b["per_side_angle_bevel_deg"][1]:.1f}° | 17.5–22.5° | ground from {b["ground_from_mm_from_point"]:.1f} to '
          f'{b["ground_to_mm_from_point"]:.1f} mm from the point |',
          f'| Steepest slope within 2 mm of either edge | {max(a["steepest_within_2mm_deg"], b["steepest_within_2mm_deg"]):.1f}° | no shoulder | one grind per edge |',
          f'| Serrations, edge A / B: depth, scallops | {a["serration_depth_mm"]:.2f} mm, {a["teeth"]} / {b["serration_depth_mm"]:.2f} mm, {b["teeth"]} | both edges serrated | '
          f'mean flank {a["mean_flank_deg"]:.0f}° / {b["mean_flank_deg"]:.0f}° |',
          f'| Weakest tooth between scallops, edge A / B ({a["teeth_checked"]} / {b["teeth_checked"]} checked; {bl["tooth_loads_n"]["snag_along_edge"]:.0f} N snag, {bl["tooth_loads_n"]["pry_across_blade"]:.0f} N pry, beam formula) | '
          f'SF {a["tooth_root_sf_min"]:.2f} / {b["tooth_root_sf_min"]:.2f} | ≥ 1 | {meets(min(a["tooth_root_sf_min"], b["tooth_root_sf_min"]), 1.0)} |',
          f'| Solid tip, 1.82 N·m twist about the origin-to-point axis (TET10) | SF {i["solid_tip_sf"]:.2f} ({tip_corr:.2f} with the raster offset removed) | ≥ 1 | {meets(min(i["solid_tip_sf"], tip_corr), 1.0)} |',
          f'| Lateral load at the tip (search plate mesh) | {i["lat_rob_cert_n"]:.0f} N | ≥ 260 N | {on_mesh(i["lat_rob_cert_n"], 260.0)} |',
          f'| Grip, worst case (beam formula) / hex8 voxel FE, spine 3427 N | SF {i["grip_sf_min"]:.2f} / {fe["grip_sf"]["spine"]:.2f} | ≥ 1 | '
          f'{meets(min(i["grip_sf_min"], fe["grip_sf"]["spine"]), 1.0)} |',
          f'| Human-max twist 16.2 N·m (search plate mesh, blade body) / hex8 voxel FE, root | SF {i["hm_twist_sf"]:.2f} / {fe["root_sf"]["twist"]:.2f} | ≥ 1 | '
          f'{on_mesh(min(i["hm_twist_sf"], fe["root_sf"]["twist"]), 1.0)} |',
          f'| Spine load 3427 N (search plate mesh) | SF {i["hm_spine_sf"]:.2f} | ≥ 1 | {on_mesh(i["hm_spine_sf"], 1.0)} |',
          f'| Ring, 300 N bearing / 30 N back at SF 2 | {i["ring_bearing_cap_n"]:.0f} / {i["ring_back_cap_n"]:.0f} N | ≥ 600 / 60 N | '
          f'{meets(min(i["ring_bearing_cap_n"] / 600.0, i["ring_back_cap_n"] / 60.0), 1.0)} |',
          f'| Fatigue / blade-before-hand yield sequence, as coded | {i["fat_sf"]:.2f} / {i["seq_ratio"]:.2f} | ≥ 1 / ≥ 1.2 | '
          f'{meets(min(i["fat_sf"], i["seq_ratio"] / 1.2), 1.0)} |',
          f'| Yield sequence per load case (voxel-FE grip): lateral / twist / spine | {seq_fe["lateral_260n"]:.2f} / {seq_fe["twist_16p2nm"]:.2f} / {seq_fe["spine_3427n"]:.2f} | ≥ 1.2 | '
          f'{meets(min(seq_fe["lateral_260n"], seq_fe["twist_16p2nm"], seq_fe["spine_3427n"]), 1.2)} (spine) |',
          f'| Fatigue with the thin-wall knock-down applied once (search plate mesh) | {fat_corr:.2f} | ≥ 1 | {on_mesh(fat_corr, 1.0)} |',
          f'| Lateral-torsional buckling of the blade (analytic) | λ {i["buckling_lambda"]:.1f} | ≥ 1 | {meets(i["buckling_lambda"], 1.0)} |',
          '',
          '<table><tr>'
          f'<td width="50%"><img src="{img["cut_b"]}" alt="Section across the blade 30 mm from the point"><br>Section across the blade, 30 mm from the point: one grind per edge, hollow lattice core.</td>'
          f'<td width="50%"><img src="{img["tip"]}" alt="The point"><br>The point; the grinds end {a["ground_to_mm_from_point"]:.1f} mm (A) and {b["ground_to_mm_from_point"]:.1f} mm (B) from it.</td></tr><tr>'
          f'<td><img src="{img["edge_b"]}" alt="Spine edge with serrations"><br>Spine edge (B): one bevel, serration scallops.</td>'
          f'<td><img src="{img["edge_a"]}" alt="Inner edge"><br>Inner edge (A).</td></tr><tr>'
          f'<td><img src="{img["xray"]}" alt="X-ray of the internal cavities"><br>X-ray: internal cavities in amber.</td>'
          f'<td><img src="{img["cut_full"]}" alt="Lengthwise cut through the grip"><br>Lengthwise cut: skin, lattice and powder ports.</td></tr><tr>'
          f'<td><img src="{img["grip"]}" alt="Grip"><br>Grip.</td>'
          f'<td><img src="{img["ring"]}" alt="Ring"><br>Ring, 24.9 mm bore, 8 mm lip.</td></tr><tr>'
          f'<td><img src="{img["blade"]}" alt="The blade"><br>The blade: both serrated edges and the lightening slots.</td>'
          f'<td><img src="{img["cut_x"]}" alt="Section across the grip"><br>Section across the grip: skin and lattice core.</td></tr><tr>'
          f'<td><img src="{img["tip_edge"]}" alt="Edge at the point"><br>The edge at the point.</td>'
          f'<td><img src="{img["top"]}" alt="Edge-on view"><br>Edge-on: ring, grip and blade thickness.</td></tr><tr>'
          f'<td colspan="2"><img src="{img["side"]}" alt="Plan view" width="100%"><br>Plan view.</td></tr></table>', '',
          'Edge sections at four stations per edge, true scale, measured normal to the edge from the exact decoder law (dashed: a 20° per-side grind):', '',
          picture(r['figures']['edge_sections_light'], r['figures']['edge_sections_dark'], 'Blade cross-sections of both edges'), '']
    # ---- evolution
    s0, lin = ev['winner_island_after_sobol'], ev['lineage']
    gc = r['grid_check']
    gl = gc['search_leader']
    L += ['## How evolution produced it', '',
          f'{co["islands"]} independent islands ran {co["evaluations_per_island"]:,} evaluations each ({co["evaluations_total"]:,} in all) on {co["host"]}, '
          f'from {co["init"]}. Each island is a MAP-Elites archive of 1,024 behaviour niches with fixed Halton centres under W_ERGOLIGHT_v1 (draft d), with a three-tier '
          f'cascade: analytic Tier 0, voxel-SDF plus plate-FE Tier 1, and Tier 2 with a watertight mesh and a solid-tip TET10 model (gates '
          f'{co["tier_gates"][0]} and {co["tier_gates"][1]}, h {co["h_mm"]} mm). The run kept {co["elites"]:,} elites, {co["tier2_elites"]} of them at Tier 2. '
          f'The winner\'s island ({ev["winner_island"]}) started from a best score of {s0["best_score_100"]:.2f} after its Sobol initialisation.', '',
          picture(ev['figures']['evolution_best_light'], ev['figures']['evolution_best_dark'], 'Best score so far, every island'), '',
          picture(ev['figures']['evolution_archive_light'], ev['figures']['evolution_archive_dark'], 'Archive growth, every island'), '',
          f'The best design of {ev["winner_island"]} at each logged checkpoint (a checkpoint sequence, not a parent-child lineage), same camera and light:', '',
          '<table><tr>' + ''.join(f'<td align="center"><img src="{c["image"]}" alt="evaluation {c["evals"]}" width="100%"><br>'
                                  f'eval {c["evals"]:,}: {c["best_score_100"]:.2f}, {c["best_mass_g"]:.1f} g'
                                  + (f'<br><b>{c["role"]}</b>' if c.get('role') else '') + '</td>' for c in lin) + '</tr></table>', '',
          f'Scores and masses under each frame are the search\'s own (h {co["h_mm"]} mm).', '',
          f'**Choosing the winner on the print grid.** The search measures every design on a {co["h_mm"]} mm voxel grid, and that grid can '
          f'undercount mass. The search\'s own leader, `{gl["design_id"]}` (Tier 2 score {gl["tier2_score_100_h04"]:.2f} at h {co["h_mm"]} on Linux; the search host logged it a little higher), '
          f'weighs {gl["mass_g_h04"]:.1f} g at h {co["h_mm"]} but {gl["mass_g_h02"]:.1f} g at h 0.2 and {gl["mass_g_h015"]:.1f} g at h 0.15, and its Tier-1 score falls from '
          f'{gl["tier1_score_100_h04"]:.2f} to {gl["tier1_score_100_h02"]:.2f}. So the ranking and the mass of record are measured at h {co["ranking_h_mm"]} mm, '
          f'the grid of the print file (the voxel FE below runs at h {fe["hex8_voxel_fe_h_mm"]} mm): every elite ordered by its record from the run, the top 32 re-evaluated at Tier 2 at h {co["ranking_h_mm"]} on the search host, '
          f'then the top 8 again on Linux x86_64, the platform of record (the winner scored {r["arm64_score_100"]:.2f} on the search host).', '',
          f'Grid check: mass in grams at each voxel pitch h (mm). {gc["evaluation"]}; h 0.2 and h 0.15 agree within {gc["h02_vs_h015_max_rel_pct"]:.1f} % for every design.', '',
          '| Design | ' + ' | '.join(f'h {h}' for h in gc['h_mm']) + ' |', '|---|' + '---:|' * len(gc['h_mm'])]
    L += [f'| `{g["design_id"]}` | ' + ' | '.join(f'{m:.1f}' for m in g['mass_g']) + ' |' for g in gc['rows']]
    L += ['',
          'Final ranking:', '',
          f'| Rank | Design | Score (Linux Tier 2, h {co["ranking_h_mm"]}) | Mass at h {co["ranking_h_mm"]} |', '|---:|---|---:|---:|']
    L += [f'| {k + 1} | `{t["design_id"]}` | {t["score_100"]:.2f} | {t["mass_g"]:.1f} g |' for k, t in enumerate(co['top8_linux'])]
    L += ['', f'The top eight span {co["top8_linux"][0]["score_100"]:.2f} to {co["top8_linux"][-1]["score_100"]:.2f}, a spread the measurement chain does not '
          'resolve, so "best individual" means the first of these eight.', '']
    # ---- measurement
    tc = fe['tip_tet10_sf_by_mesh_area_mm2']
    L += ['## How the design is constrained and measured', '',
          '- **Edges (EDGE-1..5).** One primary grind per edge at 17.5–22.5° per side, measured normal to the actual serrated edge, at full height '
          'and unbroken to the point; only a short plunge by the guard is unsharpened.',
          '- **Serrations (SERR-1..4).** Both edges serrated, at least 3 full-depth scallops each, pitch 5–9 mm, depth at most a quarter of the '
          'pitch, and no serration within 18 mm of the point. Every tooth between two scallops is checked under a single-tooth snag and pry '
          '(a beam formula on the exact section law), on this design and on 60 Sobol designs in the decoder\'s tests; the check is not a term of the score.',
          '- **Evolution designs the knife.** Populations start from scrambled Sobol samples only. Nobody edits an individual; the rules above are '
          'constraints on what the decoder can express.',
          f'- **Voxel FE check of this design (hex8 voxel FE on the as-built voxels, h {fe["hex8_voxel_fe_h_mm"]} mm, {fe["stat"]}).** '
          f'Grip zone: lateral {fe["grip_sf"]["lat"]:.2f}, twist {fe["grip_sf"]["twist"]:.2f}, spine {fe["grip_sf"]["spine"]:.2f}. '
          f'Root: lateral {fe["root_sf"]["lat"]:.2f}, twist {fe["root_sf"]["twist"]:.2f}, spine {fe["root_sf"]["spine"]:.2f}. '
          f'Loads: {fe["loads"]}.',
          f'- **Tip mesh study.** Solid-tip SF {tc["area_0p4"]:.3f} / {tc["area_0p15_production"]:.3f} / {tc["area_0p08"]:.3f} at 0.4 / 0.15 '
          '(production) / 0.08 mm² planform meshes. The series is not monotone, so it shows the mesh sensitivity, not a converged value.',
          f'- **Mass.** {mass:.1f} g from the watertight 0.2 mm print mesh; the ranking counted {r["mass"]["ranking_voxel_g"]:.1f} g on its '
          f'{co["ranking_h_mm"]} mm voxel grid (the score above uses this mass), the search {r["mass"]["search_voxel_g"]:.1f} g on its {co["h_mm"]} mm grid, '
          f'and the coarser upload mesh in `cad/` {dg["zone_mass"]["am_stl_total_mass_g"]:.1f} g. Scored at its print-mesh mass, the design would score '
          f'{dg["seq_per_lc"]["mass_of_record_score_audit"]["score_100_at_print_mesh_mass"]:.2f}.',
          f'- **Diagnostics added in v3.0.3** ([`evidence/g3/v{rv}/diagnostics/`](evidence/g3/v{rv}/diagnostics/), written by `g3-diagnostics`; '
          'they change neither the receipt nor the ranking). The yield-sequence ratio above divides the grip\'s worst factor (spine) by the blade\'s '
          f'lateral factor, mixing two load cases; per case it is {seq_fe["lateral_260n"]:.2f} / {seq_fe["twist_16p2nm"]:.2f} / {seq_fe["spine_3427n"]:.2f}. '
          f'The coded fatigue stress applies the thin-wall knock-down twice; applied once, the factor is {fat_corr:.2f}. The tip model reads its '
          f'thickness raster half a cell off; corrected, the production tip mesh gives SF {tip_corr:.2f}. Refining the search\'s blade plate mesh '
          f'(maximum element area {plate_area} mm²) gives lateral capacities of {plate_lat} N, which do not converge, so the plate\'s lateral, '
          'twist and spine values hold for the search mesh only.', '']
    # ---- limits
    lim = []
    if mass > 50.0:
        lim.append(f'It weighs {mass:.1f} g, above the 50 g goal.')
    for name, val, tgt in (('solid-tip SF', i['solid_tip_sf'], 1.0), ('grip SF (formula)', i['grip_sf_min'], 1.0),
                           ('grip spine SF (FE)', fe['grip_sf']['spine'], 1.0), ('human-max twist SF', i['hm_twist_sf'], 1.0),
                           ('fatigue SF', i['fat_sf'], 1.0), ('yield-sequence ratio', i['seq_ratio'], 1.2)):
        if val < tgt:
            lim.append(f'Its {name} is {val:.2f}, below the {tgt:g} target.')
    w_mass, span = rs_v3_number('mass', 'weights'), rs_v3_number('mass_ceiling_g') - rs_v3_number('mass_floor_g')
    grams_per_pct_e = sc['score_100'] * 0.01 / (100.0 * sc['E'] * w_mass / span)
    lim.append('Its plate-model capacities (lateral, twist, spine) do not converge under mesh refinement, so their "meets target" readings '
               'hold on the search mesh only (see the diagnostics).')
    lim.append(f'The ergonomic multiplier does not forbid trading comfort for grams: 1 % of E is worth {grams_per_pct_e:.1f} g of mass credit.')
    lim.append('It is a computational result: nothing has been printed, heat-treated, sharpened or tested.')
    lim.append(f'The archive was selected on the {co["h_mm"]} mm search grid, which undercounts the mass of some designs (see the grid check); '
               f'only the final ranking is measured at {co["ranking_h_mm"]} mm, so designs the coarse grid scored low were not re-checked.')
    lim.append(f'The cohort is {co["evaluations_per_island"]:,} evaluations per island, a shortened run for this release.')
    L += ['## Limits of this result', ''] + [f'- {x}' for x in lim] + ['']
    # ---- cad
    spec_rel = next(p for p in r['cad_in_git'] if p.endswith('.json'))
    sp = json.loads(rd(spec_rel))
    am = sp['am_build_stl']
    env = sp['outer_envelope_stl']
    L += ['## Download the CAD & 3D-printing upload kit (`cad/`, millimetres)', '',
          'The top-level [`cad/`](cad/) folder contains a ready-to-upload **6-file manufacturing bundle** (every file under 50 MiB, '
          'designed for quote forms accepting up to 6 files of 100 MB each in `STL`, `3MF`, `DXF`, `PDF`, or `JPG`), '
          'generated automatically from the winning individual:', '',
          '| # | File in `cad/` | Format | Purpose on a 3D-printing / manufacturing quote form |',
          '|---:|---|---|---|',
          f'| 1 | [`cad/{wid}_AMbuild_mm.stl`](cad/{wid}_AMbuild_mm.stl) | Binary STL (`mm`) | **Primary 3D print file in git** (`{am["voxel_pitch_h_mm"]:g} mm` voxels, `{am["mass_ti64_g"]:.2f} g`, finest under 50 MiB; outer skin + TPMS lattice + powder ports; cutting edges require post-print grinding — see note below) |',
          f'| 2 | [`cad/{wid}_AMbuild_mm.3mf`](cad/{wid}_AMbuild_mm.3mf) | 3MF (`unit="millimeter"`) | **Modern 3D print package** with `millimeter` units and `Ti-6Al-4V (design assumption: yield 828 MPa)` material embedded |',
          f'| 3 | [`cad/{wid}_envelope_mm.stl`](cad/{wid}_envelope_mm.stl) | Binary STL (`mm`) | **Solid outer envelope companion** (`{env["voxel_pitch_h_mm"]:g} mm` voxels; all internal cavities and ports closed) |',
          f'| 4 | [`cad/{wid}_drawing_mm.dxf`](cad/{wid}_drawing_mm.dxf) | AutoCAD R12 DXF (`mm`) | **1:1 2D engineering drawing** (`UNITS: MILLIMETERS`, `1 drawing unit = 1 mm`: exact `z = 0` mid-plane outline, 24.9 mm ring bore at `x = -5.97 mm`, `z`-extent thickness envelope & 30 mm blade section) |',
          f'| 5 | [`cad/{wid}_spec_paper.pdf`](cad/{wid}_spec_paper.pdf) | PDF | **7-page engineering specification & FE paper** (`g3-v{v}.pdf`; reports all measured safety factors, including those below 1: solid tip {i["solid_tip_sf"]:.2f}, grip {i["grip_sf_min"]:.2f} / {fe["grip_sf"]["spine"]:.2f}, fatigue {i["fat_sf"]:.2f}, yield-sequence {i["seq_ratio"]:.2f} vs 1.2) |',
          f'| 6 | [`cad/{wid}_xray_cutaway.jpg`](cad/{wid}_xray_cutaway.jpg) | JPG image | **Lengthwise cutaway render** showing the internal TPMS lattice and powder-evacuation ports |',
          '',
          'Mesh assets and high-resolution release files:', '',
          '| File | Triangles | Size | SHA-256 | Contents | Where |', '|---|---:|---:|---|---|---|']
    for asset in r['release_assets']:
        where = (f'[`cad/{asset["file"]}`](cad/{asset["file"]}) · [archive](docs/g3/v{rv}/cad/{asset["file"]})'
                 if f'docs/g3/v{rv}/cad/{asset["file"]}' in r['cad_in_git']
                 else f'[release v{rv}]({REPO}/releases/tag/v{rv})')
        L.append(f'| `{asset["file"]}` | {asset["triangles"]:,} | {asset["bytes"] / 1048576:.0f} MB | `{asset["sha256"][:16]}…` | {asset["note"]} | {where} |')
    specs = [p for p in r['cad_in_git'] if p.endswith('.json')]
    L += ['', 'Units are millimetres. The print file is the AM build: the outer skin, the internal TPMS lattice and the powder-removal ports. '
          'The files in git are written by [`ops/g3/export_best_stl.sh`](ops/g3/export_best_stl.sh) from the committed genome row, each at the '
          f'finest voxel pitch that keeps it under 50 MiB (`{am["voxel_pitch_h_mm"]:g} mm` for `AMbuild_mm.stl` / `.3mf`, `{am["mass_ti64_g"]:.2f} g` vs `{mass:.1f} g` on the `0.2 mm` grid of record)'
          + (''.join(f'; [`cad/{os.path.basename(p)}`](cad/{os.path.basename(p)}) ([archive]({p})) lists their dimensions, volume, mass, pitch and watertightness' for p in specs))
          + f'. Because a `{am["voxel_pitch_h_mm"]:g} mm` voxel grid cannot resolve the `0.05–0.12 mm` apex flat or the `20°` primary bevel near the edge, '
          f'cutting edges on any print from this mesh would be finished by post-print grinding to the `EDGE-3` angle (`17.5–22.5°` per side); '
          f'the finer `0.2 mm` print mesh (`{wid}_AMbuild_h0.2_mm.stl`) and `0.05 mm` fine-blade outer envelope (`{wid}_envelope_fineblade_mm.stl`) are attached to [release v{rv}]({REPO}/releases/tag/v{rv}).', '']
    # ---- reproduce + repo
    L += ['## Reproduce', '', '```bash',
          f'# the cohort ran at commit {pv["git_commit"][:12]}; the decoder (hash {pv["decoder_sha256"][:12]}) and g3-evolve are unchanged since, so build from this release',
          'cargo build --release --manifest-path crates/g3/Cargo.toml --bins',
          f'crates/g3/target/release/g3-evolve --out out/<run>/island01/run --init 64 --evals {co["evaluations_per_island"]} \\',
          f'  --seed 4001 --host 4 --island 1 --commit {pv["git_commit"][:12]} --h {co["h_mm"]} --tier1-gate {co["tier_gates"][0]} --tier2-gate {co["tier_gates"][1]} --log-every 50',
          f'crates/g3/target/release/g3-pilot tier2 <root> <rank.jsonl> 32 tier2_top32.jsonl {co["ranking_h_mm"]} 4   # ranking of record',
          f'crates/g3/target/release/g3-pilot hconv <root> <id,id,...> 0.4,0.3,0.25,0.2,0.15 grid.jsonl 4   # grid check',
          f'crates/g3/target/release/g3-pilot export <root> {wid} <out_dir> 0.2   # watertight envelope + AM build',
          f'crates/g3/target/release/g3-gripfe run <root> {wid} 0.4 fe.jsonl 4             # hex8 voxel FE check',
          '```', '',
          f'The winner\'s genome row is [`evidence/g3/v{rv}/winner_genome.csv`](evidence/g3/v{rv}/winner_genome.csv); every island\'s logged history is in '
          f'[`evidence/g3/v{rv}/history/`](evidence/g3/v{rv}/history/).', '',
          'The v1 command-line pipeline (`cargo run -p jarambit`) is still in the repository; its plate level is opt-in with `JARAMBIT_FEM=1`.', '',
          '## Repository', '',
          '**G3 (current)**', '',
          f'- [`cad/`](cad/): 6-file 3D-printing & manufacturing upload kit (`STL`, `3MF`, `DXF`, `PDF`, `JPG`, millimetres) and order notes for `{wid}`, generated automatically from the winner.',
          '- [`crates/g3/`](crates/g3/README.md): the G3 genome, decoder, objective and search, and the release tools in `src/bin/`.',
          '- [`crates/g5/`](crates/g5/README.md): the G5 generation under RS-karambit-v5, in development (no results; not frozen).',
          f'- [`evidence/g3/`](evidence/g3/): the receipt, genome row, island histories and measurements of the v{rv} cohort of record.',
          f'- [`docs/g3/`](docs/g3/): the G3 paper (`make -C docs/g3/paper` rebuilds it from the receipt), the images, [`RELEASE_{rv}.md`](docs/g3/RELEASE_{rv}.md), [`RENDERS.md`](docs/g3/RENDERS.md) and the [provenance](docs/g3/provenance/README.md) of its requirement set.',
          '- [`docs/RELEASES.md`](docs/RELEASES.md): every release.', '',
          '**Withdrawn**', '',
          '- [`crates/g4/`](crates/g4/WITHDRAWN.md), [`evidence/g4/`](evidence/g4/WITHDRAWN.md) and [`docs/g4/`](docs/g4/WITHDRAWN.md): the v4.0.0 G4 prototype, withdrawn in v4.0.1 and kept unchanged as a record; not a result.', '',
          '**The v1/v2 study (kept as its own record)**', '',
          '- [`docs/paper/`](docs/paper/) (Paper B) and [`docs/WHITEPAPER.md`](docs/WHITEPAPER.md): the study, with the other v1 documents indexed in [`docs/README.md`](docs/README.md).',
          '- [`crates/`](crates/): the v1 pipeline (the `jarambit` CLI in `crates/cli`, plus `crescent`, `physics`, `genotype`, `phenotype`, `evolve`, `monte_carlo`, `redteam`, `evidence` and `docs`; `crescent` and `physics` are shared with G3), and two standalone tools, `paper-artifacts` (Paper B data and figures) and `runner-policy` (CI runner routing).',
          '- [`evidence/`](evidence/): the study\'s recorded runs, battery records and receipts. [`artifacts/`](artifacts/): the gen-800 population archive read by Paper B.',
          '- [`experiments/`](experiments/README.md) and [`designs/`](designs/README.md): self-contained studies and the hand-built SWITCHBACK packages.',
          '- [`requirements/`](requirements/): the requirement sets, including [`RS-karambit-v5.toml`](requirements/RS-karambit-v5.toml), the sourced set for the G5 generation (v5.0.0), with its [provenance](requirements/PROVENANCE-v5.md). [`assurance/`](assurance/), [`metrics/`](metrics/), [`research/science_v1/`](research/science_v1/), [`fem_solid/`](fem_solid/), [`verify/`](verify/), [`bench/`](bench/) and [`datasets/regression/`](datasets/regression/): evidence contracts, the v1 metric, the science suite, the solid-tip submodel, verification targets, the fixed benchmark and the regression datasets.',
          '- [`sheath/`](sheath/) and [`web/`](web/): the v1 sheath generator and the CAD scorecard studio. [`evolve.sh`](evolve.sh): the v1 auto-evolve driver.', '',
          '**Tooling**', '',
          '- [`ops/`](ops/README.md): generators, checks and render pipelines, indexed by workflow. [`.github/workflows/`](.github/workflows/): CI.',
          '- [`AGENTS.md`](AGENTS.md): rules for contributors and coding agents. [`PYTHON_EXCEPTIONS.md`](PYTHON_EXCEPTIONS.md): every Python file and why it is not Rust yet. [`STATUS.md`](STATUS.md): generated status. [`LICENSE`](LICENSE): MIT.', '']
    return '\n'.join(L)


def cad_readme():
    v, rv, r = receipt()
    wid = r['design_id']
    mass = r['mass']['mesh_g']
    i = r['inputs']
    fe = r['fe_certification']
    spec_rel = next(p for p in r['cad_in_git'] if p.endswith('.json'))
    sp = json.loads(rd(spec_rel))
    am = sp['am_build_stl']
    env = sp['outer_envelope_stl']
    dim = sp['overall_dimensions_mm']
    sz_kb = lambda rel: os.path.getsize(os.path.join(ROOT, rel)) / 1024.0
    sz_mb = lambda rel: os.path.getsize(os.path.join(ROOT, rel)) / (1024.0 * 1024.0)
    order_notes = (
        f'Units: mm | Overall dimensions: {dim["length_x_mm"]:.2f} x {dim["height_y_mm"]:.2f} x {dim["thickness_z_mm"]:.2f} mm | '
        f'Material: Ti-6Al-4V (design assumption: yield 828 MPa) | Depowder via neutral-axis drainage ports | '
        f'Status: {r["status"]} — below-target safety factors (solid-tip SF {i["solid_tip_sf"]:.2f}, '
        f'grip SF {i["grip_sf_min"]:.2f} formula / {fe["grip_sf"]["spine"]:.2f} FE spine, '
        f'fatigue SF {i["fat_sf"]:.2f}, yield-sequence ratio {i["seq_ratio"]:.2f} vs 1.2 target; mass {mass:.1f} g vs 50 g goal); '
        f'not qualified for use as a knife'
    )
    L = [
        f'# 3D Printing & CAD Upload Kit — Winner `{wid}` (release v{v}, cohort of record v{rv})',
        '',
        f'This folder contains the ready-to-upload **6-file 3D-printing and manufacturing bundle** (**Units: millimetres**, every file under 50 MiB) '
        f'for **`{wid}`**, the winning individual of the v{rv} cohort of record ([`evidence/g3/v{rv}/release.json`](../evidence/g3/v{rv}/release.json)). '
        f'It is populated automatically from [`evidence/g3/v{rv}/winner_genome.csv`](../evidence/g3/v{rv}/winner_genome.csv) '
        'by [`ops/g3/export_best_stl.sh`](../ops/g3/export_best_stl.sh) and [`ops/ci/gen_readme.py`](../ops/ci/gen_readme.py).',
        '',
        f'**Status & limits ({r["status"]}):** this design is **not qualified for use as a knife**. '
        f'Its 0.20 mm print-mesh mass is **{mass:.1f} g** (above the 50 g goal; `{am["mass_ti64_g"]:.2f} g` on the `{am["voxel_pitch_h_mm"]:g} mm` in-git mesh), '
        f'and several structural safety factors are below target: solid-tip SF **{i["solid_tip_sf"]:.2f}** (< 1), '
        f'grip SF **{i["grip_sf_min"]:.2f}** (beam formula) / **{fe["grip_sf"]["spine"]:.2f}** (hex8 voxel FE spine, < 1), '
        f'fatigue SF **{i["fat_sf"]:.2f}** (< 1), and blade-before-hand yield-sequence ratio **{i["seq_ratio"]:.2f}** (< 1.2 target). '
        f'Because the in-git print mesh (`{am["file"]}` / `{wid}_AMbuild_mm.3mf`) is extracted on a **{am["voxel_pitch_h_mm"]:g} mm** voxel grid (the finest pitch under 50 MiB), '
        f'it cannot represent the `0.05–0.12 mm` apex flat or the `20°` primary bevel at the cutting edge; cutting edges would be finished by grinding after printing to the `EDGE-3` angle (`17.5–22.5°` per side), '
        f'or you can download the `0.2 mm` print mesh (`{wid}_AMbuild_h0.2_mm.stl`) and `0.05 mm` fine-blade mesh (`{wid}_envelope_fineblade_mm.stl`) from [release v{rv}]({REPO}/releases/tag/v{rv}).',
        '',
        '## 1. Ready-to-upload 6-file kit (for forms accepting up to 6 files, 100 MB each)',
        '',
        'If your printing or manufacturing quote form asks for **"Drawings and related files (STL, OBJ, 3MF, STEP, PDF, images, DXF/DWG, Word, or a zip — up to 6 files, 100 MB each)"**, you can upload all **6 files** below (or just File #1 / File #2 if you only want a single 3D print file):',
        '',
        '| # | File | Format | Size | What it is & why to upload it |',
        '|---:|---|---|---:|---|',
        f'| **1** | **[`{am["file"]}`]({am["file"]})** | **Binary STL (`mm`)** | **{am["mib"]:.2f} MiB** | **PRIMARY 3D PRINT FILE IN GIT (upload this first):** complete additive-manufacturing build at `{am["voxel_pitch_h_mm"]:g} mm` voxel pitch (finest under 50 MiB; `{am["mass_ti64_g"]:.2f} g`, {am["triangles"]:,} triangles) with outer skin, internal TPMS lattice, and powder-removal ports; cutting edges require post-print grinding. |',
        f'| **2** | **[`{wid}_AMbuild_mm.3mf`]({wid}_AMbuild_mm.3mf)** | **3MF (`unit="millimeter"`)** | **{sz_mb(f"cad/{wid}_AMbuild_mm.3mf"):.2f} MiB** | **Modern 3D print package:** identical `{am["voxel_pitch_h_mm"]:g} mm` AM build mesh in compressed 3MF 1.2.3 format with `unit="millimeter"` and `Ti-6Al-4V (design assumption: yield 828 MPa)` material metadata embedded in the header. |',
        f'| **3** | **[`{env["file"]}`]({env["file"]})** | **Binary STL (`mm`)** | **{env["mib"]:.2f} MiB** | **Solid outer envelope companion:** the unhollowed exterior surface at `{env["voxel_pitch_h_mm"]:g} mm` voxel pitch with all internal cavities and ports closed ({env["solid_mass_ti64_g"]:.1f} g if printed solid; {env["triangles"]:,} triangles). |',
        f'| **4** | **[`{wid}_drawing_mm.dxf`]({wid}_drawing_mm.dxf)** | **AutoCAD R12 DXF (`mm`)** | **{sz_kb(f"cad/{wid}_drawing_mm.dxf"):.0f} KiB** | **1:1 2D engineering drawing (`UNITS: MILLIMETERS`, `1 drawing unit = 1 mm`):** exact `z = 0` mid-plane outline, 24.9 mm ring bore at decoded `x = -5.97 mm`, `z`-extent thickness envelope (not a section), 30 mm blade cross-section, and overall dimension callouts. |',
        f'| **5** | **[`{wid}_spec_paper.pdf`]({wid}_spec_paper.pdf)** | **PDF** | **{sz_kb(f"cad/{wid}_spec_paper.pdf"):.0f} KiB** | **7-page engineering specification & FE paper (`g3-v{v}.pdf`):** complete geometry, bevel angles, CAD verification tables, and measured structural safety factors (including below-1 values: solid tip {i["solid_tip_sf"]:.2f}, grip {i["grip_sf_min"]:.2f} / {fe["grip_sf"]["spine"]:.2f}, fatigue {i["fat_sf"]:.2f}, yield-sequence {i["seq_ratio"]:.2f} vs 1.2). |',
        f'| **6** | **[`{wid}_xray_cutaway.jpg`]({wid}_xray_cutaway.jpg)** | **JPG image** | **{sz_kb(f"cad/{wid}_xray_cutaway.jpg"):.0f} KiB** | **Lengthwise cutaway reference image:** visual guide for the print technician showing the outer skin, internal TPMS lattice, and powder-evacuation ports. |',
        '',
        f'*(Also included in this folder: [`{os.path.basename(spec_rel)}`]({os.path.basename(spec_rel)}), the machine-readable JSON receipt of dimensions, volumes, masses and SHA-256 digests.)*',
        '',
        '## 2. Copy-paste order notes for the printing form',
        '',
        'STL files store raw triangle coordinates without a unit header. When uploading to a printing bureau (Protolabs, PCBWay, Craftcloud, Shapeways, Xometry, JLC3DP, or a local L-PBF shop), select **Millimetres (`mm`)** and paste:',
        '',
        '```text',
        order_notes,
        '```',
        '',
        '| Specification | Value |',
        '|---|---|',
        f'| Winning design ID | `{wid}` (genome `{sp["genome_sha256"][:12]}…`, decoder `{sp["decoder_sha256"][:12]}…`) |',
        f'| Units | `{sp["units"]}` (millimetres) |',
        f'| Overall bounding box ($L_x \\times H_y \\times T_z$) | `{dim["length_x_mm"]:.2f} × {dim["height_y_mm"]:.2f} × {dim["thickness_z_mm"]:.2f} mm` |',
        f'| Material | Ti-6Al-4V (design assumption: yield 828 MPa, `{sp["density_g_mm3"]} g/mm³`) |',
        f'| Mass (0.20 mm print mesh / {am["voxel_pitch_h_mm"]:g} mm git mesh) | `{mass:.1f} g` / `{am["mass_ti64_g"]:.2f} g` (`{am["volume_mm3"]:,.1f} mm³`) |',
        f'| Solid outer envelope mass (if unhollowed) | `{env["solid_mass_ti64_g"]:.1f} g` (`{env["volume_mm3"]:,.1f} mm³`) |',
        f'| Watertight manifold check (`AMbuild` / `envelope`) | `watertight = {str(am["watertight"]).lower()}` (`{am["boundary_edges"]}` boundary edges, `{am["non_manifold_edges"]}` non-manifold edges) |',
        '',
        '## 3. High-resolution release assets (> 50 MiB)',
        '',
        '| File | Triangles | Size | SHA-256 | Contents | Where |',
        '|---|---:|---:|---|---|---|',
    ]
    for asset in r['release_assets']:
        where = (f'[in `cad/`]({asset["file"]}) · [archive](../docs/g3/v{rv}/cad/{asset["file"]})'
                 if f'docs/g3/v{rv}/cad/{asset["file"]}' in r['cad_in_git']
                 else f'[release v{rv}]({REPO}/releases/tag/v{rv})')
        L.append(f'| `{asset["file"]}` | {asset["triangles"]:,} | {asset["bytes"] / 1048576:.0f} MB | `{asset["sha256"][:16]}…` | {asset["note"]} | {where} |')
    L += [
        '',
        '## 4. How to regenerate this folder automatically from the winner',
        '',
        '```bash',
        './ops/g3/export_best_stl.sh',
        '```',
        '',
        'This reads [`evidence/g3/RECORD`](../evidence/g3/RECORD), decodes the winning genome row '
        f'[`evidence/g3/v{rv}/winner_genome.csv`](../evidence/g3/v{rv}/winner_genome.csv), writes the watertight binary STL files '
        f'and print spec to `docs/g3/v{rv}/cad/`, generates the 6-file upload kit (`STL`, `3MF`, `DXF`, `PDF`, `JPG`) in `cad/`, '
        'and runs `python3 ops/ci/gen_readme.py` to update this guide and the root [`README.md`](../README.md).',
        '',
    ]
    return '\n'.join(L)


# ---------------------------------------------------------------- G5 (v5.0.0 on)
# Sources, all committed: the receipt evidence/g5/v<RECORD>/release.json (g5-release), its vv/ studies and history/,
# the render manifest docs/g5/v<RECORD>/img/manifest.json, the CAD folder docs/g5/v<RECORD>/cad/ (print spec) and
# requirements/RS-karambit-v5.toml. Every lookup is strict: a missing key stops generation (no default numbers).
REQUIRED_G5 = (
    'version', 'design_id', 'status', 'provenance', 'cohort', 'baseline', 'ranking', 'grid_check', 'rank_of_record',
    'score', 'loads', 'margins', 'worst_margin', 'meets_every_load_case', 'mass', 'am', 'blade', 'point', 'ergonomics',
    'decoded', 'evolution', 'release_assets',
)
GIT_FILE_MAX_MIB = 50  # repository rule (v3.0.2): no committed file above 50 MiB; larger meshes are release assets


def receipt_g5():
    """(release version, G5 cohort-of-record version, receipt). Fail closed on any missing field or version mismatch."""
    v = rd('docs/VERSION').strip()
    rv = rd('evidence/g5/RECORD').strip()
    rec = js(f'evidence/g5/v{rv}/release.json')
    missing = [k for k in REQUIRED_G5 if k not in rec]
    if missing or rec['version'] != rv or not record_declared(v, rv):
        raise SystemExit(f'gen_readme: G5 release.json v{rv} is incomplete or does not belong to release v{v} ({missing or rec["version"]})')
    return v, rv, rec


def g5_files(rv):
    """The render manifest, the CAD folder listing (committed files, each within GIT_FILE_MAX_MIB) and its print spec."""
    man = js(f'docs/g5/v{rv}/img/manifest.json')
    for p in list(man['images'].values()) + [c['image'] for c in man['lineage']]:
        if not os.path.isfile(os.path.join(ROOT, p)):
            raise SystemExit(f'gen_readme: manifest names a missing image {p}')
    cad_dir = f'docs/g5/v{rv}/cad'
    cad = sorted(f'{cad_dir}/{n}' for n in os.listdir(os.path.join(ROOT, cad_dir)))
    big = [p for p in cad if os.path.getsize(os.path.join(ROOT, p)) > GIT_FILE_MAX_MIB * 1048576]
    if big:
        raise SystemExit(f'gen_readme: committed CAD files above {GIT_FILE_MAX_MIB} MiB: {big}')
    spec = js(next(p for p in cad if p.endswith('_print_spec.json')))
    return man, cad, spec


def g5_load_cases(rs, ld, mg):
    """The 16 RS-karambit-v5 structural cases: label from RS-karambit-v5.toml, safety factor from the receipt's loads,
    requirement from [margins], margin v = SF / SF_req from the receipt (objective::margins)."""
    L, T = rs['loads'], rs['loads']['tip']
    lb, bt, gr, gd, rg, pl, th = (L[k] for k in ('lateral_bend', 'body_twist', 'grip', 'guard', 'ring', 'pull', 'teeth'))
    mf, y, u, ring = rs['material']['fatigue'], rs['margins']['sf_yield'], rs['margins']['sf_ultimate'], rs['margins']['sf_ring']
    return [
        (f'Lateral bending, limit: {lb["limit_root_moment_nm"]:g} N·m root moment, load {lb["apply_mm_from_point"]:g} mm from the point (blade plate, grip)',
         min(ld['lateral_limit_sf'], ld['grip_lateral_sf']), y, mg['lateral_limit']),
        (f'Lateral bending, ultimate: {lb["ultimate_root_moment_nm"]:g} N·m (blade plate)', ld['lateral_ultimate_sf'], u, mg['lateral_ultimate']),
        (f'Body twist, limit: {bt["limit_nm"]:g} N·m (blade plate, grip)', min(ld['twist_limit_sf'], ld['grip_twist_sf']), y, mg['twist_limit']),
        (f'Tip, axial: {T["axial_n"]:g} N along the tip bisector (TET10 solid)', ld['tip_axial_sf'], y, mg['tip_axial']),
        (f'Tip, side load: {T["lateral_n"]:g} N at the point (TET10 solid)', ld['tip_lateral_sf'], y, mg['tip_lateral']),
        (f'Tip, twist: {T["twist_nm"]:g} N·m, {T["twist_apply_mm_from_point"]:g} mm from the point (TET10 solid)', ld['tip_twist_sf'], y, mg['tip_twist']),
        (f'Grip squeeze: {gr["limit_squeeze_n"]:g} N', ld['grip_squeeze_sf'], y, mg['grip_squeeze']),
        (f'Guard push: {gd["limit_push_n"]:g} N', ld['guard_push_sf'], y, mg['guard_push']),
        (f'Ring bearing: {rg["bearing_n"]:g} N on the handle side of the bore (T6 ring)', ld['ring_bearing_sf'], ring, mg['ring_bearing']),
        (f'Ring back pull: {rg["back_n"]:g} N (T6 ring)', ld['ring_back_sf'], ring, mg['ring_back']),
        (f'Ring, any in-plane direction: {rg["any_direction_n"]:g} N (T6 ring sweep)', ld['ring_any_direction_sf'], y, mg['ring_any_direction']),
        (f'Axial pull: {pl["limit_n"]:g} N, held over the {pl["held_length_from_point_mm"]:g} mm nearest the point', min(ld['pull_sf'], ld['grip_pull_sf']), y, mg['pull']),
        (f'Serration teeth: {th["snag_n"]:g} N snag, {th["pry_n"]:g} N pry shared by {th["pry_teeth_sharing"]} teeth (every tooth)', ld['teeth_sf'], y, mg['teeth']),
        (f'Fatigue: {rs["fatigue"]["cycles"]:,.0f} cycles, {mf["mean_stress_method"]} mean stress (blade and hand zone)',
         min(ld['fatigue_sf'], ld['hand_fatigue_sf']), rs['fatigue']['sf_fatigue'], mg['fatigue']),
        ('Yield sequence: the blade yields before the hand zone, per load case (lateral / twist / pull)',
         min(ld['seq_lateral'], ld['seq_twist'], ld['seq_pull']), rs['sequence']['ratio_min'], mg['sequence']),
        (f'Tip buckling under the {T["axial_n"]:g} N axial load (Rayleigh–Ritz eigenvalue λ)', ld['buckling_lambda'], rs['buckling']['lambda_min'], mg['buckling']),
    ]


CASE_KEYS = ('lateral_limit', 'lateral_ultimate', 'twist_limit', 'tip_axial', 'tip_lateral', 'tip_twist', 'grip_squeeze', 'guard_push',
             'ring_bearing', 'ring_back', 'ring_any_direction', 'pull', 'teeth', 'fatigue', 'sequence', 'buckling')


def g5v6_fitness(r):
    """v6 on: the 100-point budget of the winner's score (receipt fitness_budget, figures by g5-release)."""
    fb, sc = r['fitness_budget'], r['score']
    gr = fb['groups']
    low = sorted(((k, x) for k, x in sc['hand_factors'].items() if x['e'] < 1), key=lambda kv: kv[1]['e'])
    hand = f'; the lowest hand factor is {low[0][0].replace("_", " ")}, e = {low[0][1]["e"]:.3f}' if low else ''
    img = r['evolution']['figure'].rsplit('/', 1)[0]
    return ['## Where the score comes from', '',
            f'The score is 100 · E · W · base: the weighted credit of {len(sc["terms"]) - 1} terms (base, at most 1), times the hand '
            f'multiplier E and the worst-margin multiplier W. Of its 100 points the winner earns **{fb["earned"]:.1f}**: structural safety '
            f'{gr["structural"]:.1f} (the {r["cases"]} load-case margins), mass {gr["mass"]:.1f}, cutting geometry {gr["cutting"]:.1f} (edge, point '
            f'and serrations) and build and form {gr["build"]:.1f}. It loses {fb["lost_to_multipliers"]:.1f} to the multipliers '
            f'(E = {fb["E"]:.3f}{hand}; W = {fb["worst_margin_W"]:.3f}), and {fb["unmet_credit"]:.1f} are unmet credit: terms short of full '
            f'credit (base = {fb["base"]:.3f}).', '',
            f'<img src="{img}/fitness_budget.svg" alt="The 100 points of the score: earned by group, lost to the multipliers, unmet credit" width="100%">', '',
            f'<img src="{img}/fitness_terms.svg" alt="Points earned by each score term against the most it could earn" width="100%">', '']


def g5v6_evolution(r, rk, gc, man):
    """v6 on: the continuation (warm start, pool lineage, rounds per host) against the fresh-start control."""
    co, bs, ws, ev, lin = r['cohort'], r['baseline'], r['warm_start'], r['evolution'], r['lineage']
    rounds = co['rounds']
    hosts = sorted({x['host'] for x in rounds})
    per_host = {h: sum(1 for x in rounds if x['host'] == h) for h in hosts}
    cbest = max(bs['final_best_score_100'])
    sl = rk['search_leader']
    L = ['## How evolution produced it', '',
         f'v{r["version"]} continues the v5.0.0 cohort with the same frozen decoder (G5.0, `{r["provenance"]["decoder_sha256"][:12]}`) and objective, '
         f'on one platform of record ({r["provenance"]["platform_of_record"]}). The {ws["v5_elites"]:,} final elites of v5.0.0 were re-evaluated on it '
         f'and merged to the best record per niche: {ws["pool"]["niches"]} niches, best {ws["pool"]["best_raw_score_100"]:.1f} on the search grid '
         f'(Tier 1, h {co["h_mm"]:g} mm).', '',
         f'From that pool, {co["islands"]} warm-started islands ran {len(rounds)} rounds on {len(hosts)} hosts '
         f'({", ".join(f"{h}: {per_host[h]}" for h in hosts)}), {int(co["evaluations_total"]):,} evaluations in all. Whenever a host finished a '
         f'round, the hub merged its islands into the latest pool and the host\'s next round started from that pool; every merge is recorded with '
         f'its sources and recomputed by the receipt. Beside them, {bs["islands"]} fresh-start control islands ran {int(bs["evaluations_per_island"]):,} '
         'evaluations each from scrambled Sobol samples, with no pool and no migration.', '',
         f'**The lineage\'s best rose from {lin[0]["best_raw_score_100"]:.1f} to {lin[-1]["best_raw_score_100"]:.1f}; the best control island reached '
         f'{cbest:.1f}.**', '',
         f'<img src="{ev["figure"]}" alt="Best score found against evaluations: the pool lineage and the fresh-start control" width="100%">', '']
    pools = {x['pool']: x for x in lin}
    if man['lineage']:
        L += ['The best design of the pool at points of the lineage, same camera and light (scores on the search grid):', '',
              '<table><tr>' + ''.join(
                  f'<td align="center"><img src="{c["image"]}" alt="best of {c["pool"]}" width="100%"><br>`{c["pool"]}`<br>'
                  f'{pools[c["pool"]]["best_raw_score_100"]:.1f}, {pools[c["pool"]]["best_mass_g"]:.0f} g'
                  + (f'<br><b>{c["role"]}</b>' if c['role'] else '') + '</td>' for c in man['lineage']) + '</tr></table>', '']
    L += ['| Host | Round | Islands × evaluations | Started from (best) | Merged into (best) |', '|---|---:|---:|---|---|']
    for x in rounds:
        L.append(f'| {x["host"]} | {x["round"]} | {x["islands"]} × {int(x["evaluations_per_island"]):,} | `{x["pool_in"]}` ({x["pool_in_best"]:.2f}) | '
                 f'`{x["merged_into"]}` ({x["merged_best"]:.2f}) |')
    lead = (f'**The search leader is the winner.** ' if sl['design_id'] == r['design_id'] else
            f'**Why the search leader is not the winner.** The best design on the search grid, `{sl["design_id"]}` ({sl["tier1_h04_score_100"]:.1f}), '
            f'{"ranks " + str(sl["tier2_rank"]) if sl["tier2_rank"] else "falls outside the shortlist"} once re-scored at Tier 2, h {rk["h_mm"]:g} mm. ')
    L += ['', lead + f'Every finalist is judged at Tier 2 (solid tip at {rk["tip_mesh_max_area_mm2"]:g} mm²); the top {len(gc["rows"])} also on five grids '
          f'(partial-volume mass in grams at h = {" / ".join(f"{h:g}" for h in gc["h_mm"])} mm; the h {rk["h_mm"]:g} and h {gc["h_mm"][-1]:g} mm masses '
          f'must agree within {100 * gc["requirement_max_frac"]:g} %):', '',
          '| Rank | Design | Score | E | W | Mass (g) | Mass at the five grids (g) | h 0.2 vs 0.15 |', '|---:|---|---:|---:|---:|---:|---|---:|']
    grid = {x['design_id']: x for x in gc['rows']}
    for t in rk['rows'][:len(gc['rows'])]:
        x = grid[t['design_id']]
        L.append(f'| {t["rank"]} | `{t["design_id"]}` | {t["score_100"]:.2f} | {t["E"]:.3f} | {t["worst_margin_W"]:.3f} | {t["mass_g"]:.1f} | '
                 f'{" / ".join(f"{m:.1f}" for m in x["mass_g"])} | {100 * x["mass_rel_h020_vs_h015"]:.2f} % |')
    return L + ['']


def g5v6_verification(r, rv):
    """v6 on: the first bullets of Verification and limits (platform, reproduction, lineage, GPU spike)."""
    co, rp = r['cohort'], r['cohort']['reproduced']
    sp = js(f'evidence/g5/v{rv}/spike/spike.json')
    return [f'- **One platform of record.** {r["provenance"]["platform_of_record"]}. Every row of every archive re-scores bit for bit; '
            f'every pool of the lineage is recomputed from its recorded sources byte for byte; re-evaluating all {co["elites"]:,} elites of the final '
            f'pool reproduces every Tier-1 row ({co["tier1_rows_reproduced"]}); and `{rp["island"]}` re-run on {rp["rerun"]["rerun_host"]} reproduces its '
            f'history bit for bit ({rp["history_entries_identical"]} entries over {rp["rerun"]["evaluations"]} evaluations).',
            f'- **GPUs: renders, not search.** A profile of the Tier-1 evaluation ([`spike/spike.json`](evidence/g5/v{rv}/spike/spike.json)) puts '
            f'{sp["profile"]["euclidean_distance_transform_pct"] + sp["profile"]["am_connectivity_pct"]:.0f} % of it in voxel kernels and '
            f'{sp["profile"]["plate_skyline_solve_pct"]:.0f} % in the plate solve. Records must stay exact CPU arithmetic, so a GPU could only screen children '
            f'before Tier 1: a perfect, free screen would give at most {sp["bounds"]["perfect_free_screen_top10_speedup"]:.1f}× more evaluations per hour, '
            f'and the Tier-0 screen already keeps {sp["screen"]["tier1_top10_kept_by_tier0_top40"]} of the {sp["screen"]["tier1_top10"]} best children '
            f'(rank correlation {sp["screen"]["spearman_tier0_tier1"]:.2f}). The 3× bar for adopting it is out of reach, so the search stays on the CPUs.']


def g5v6_fem(r, rv):
    """v6 on: the FEM renderings of every load case (g5-fem-fields, docs/g5/v<rv>/img/fem/figures.json). Fail closed:
    the figures must be of this receipt's design and decoder, and every scored value they re-solve must match."""
    fx = js(f'docs/g5/v{rv}/img/fem/figures.json')
    if fx['design_id'] != r['design_id'] or fx['decoder_sha256'] != r['provenance']['decoder_sha256']:
        raise SystemExit('gen_readme: the FEM figures are not of the winner of record')
    gated = [x for x in fx['figures'] if x['gated']]
    if fx['mismatches'] or not all(x['match'] for x in gated) or not all(x['match'] for x in fx['reproduced']):
        raise SystemExit('gen_readme: a FEM rendering does not reproduce its scored value')
    d = f'docs/g5/v{rv}/img/fem'
    cell = lambda x: (f'<td width="50%"><img src="{d}/{x["svg"]}" alt="{x["title"]}" width="100%"><br>{x["title"]}: SF {x["sf"]:.2f}</td>'
                      if x['gated'] else
                      f'<td width="50%"><img src="{d}/{x["svg"]}" alt="{x["title"]}" width="100%"><br>{x["title"]}: hex8 FE SF {x["sf"]:.2f} '
                      f'(score model {x["record_sf"]:.2f})</td>')
    figs = fx['figures']
    L = ['## FEM renderings of every load case', '',
         f'Each load case of `{r["design_id"]}` re-solved by `g5-fem-fields` on the ranking grid (h {fx["h"]:g} mm), with the field the scored '
         f'solvers discard: von Mises stress, the governing point and the deformed shape. The {len(fx["reproduced"])} scored values these solves '
         'produce match the receipt bit for bit.', '', '<table>']
    for k in range(0, len(figs), 2):
        L.append('<tr>' + ''.join(cell(x) for x in figs[k:k + 2]) + '</tr>')
    L += ['</table>', '']
    xc = [x for x in figs if not x['gated']]
    if xc:
        L += ['The score models grip squeeze, guard push and teeth analytically. The hex8 voxel FE of the same design is an independent '
              'check, not used by the score:', '', '| Case | Score model SF | Hex8 FE SF |', '|---|---:|---:|']
        L += [f'| {x["title"]} | {x["record_sf"]:.2f} | {x["sf"]:.2f} |' for x in xc]
        L.append('')
    return L


def g5v6_reproduce(r, rv, rk):
    """v6 on: how to re-run an island, a merge, the certification and the receipt."""
    bins = r['provenance']['binaries']
    return ['', '## Reproduce', '', '```bash',
            f'# static x86_64 binaries, as on the platform of record ({bins["build"]})',
            f'RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --locked --target x86_64-unknown-linux-gnu --manifest-path crates/g5/Cargo.toml --bins',
            'B=crates/g5/target/x86_64-unknown-linux-gnu/release',
            '# one island: the arguments of any evidence/g5/v' + rv + '/launch/<round>_<host>_<island>.json',
            '$B/g5-continue run --out ~/jarambit_runs/evo_g5_v6/<round>/<host>/<island> --pool <pool> --evals <evals> --seed <raw_seed> \\',
            '  --host <host> --island <island> --commit bf3e37fa --h 0.4 --tier1-top 0.4 --tier2-top 0 --log-every 25',
            '# a pool of the lineage: the merge of its recorded sources, in order (evidence/g5/v' + rv + '/pools/*.json)',
            '$B/g5-continue merge --out <pool> --from <previous pool>,<island>,<island>,...',
            f'$B/g5-pilot eval <final_root> eval_h040.jsonl 0.4 4 && $B/g5-pilot tier2 <final_root> eval_h040.jsonl {rk["shortlist"]} tier2_h020.jsonl {rk["h_mm"]:g} 4',
            '$B/g5-release receipt-v6 . <run_root> <cert_dir>', '```', '',
            f'Every island\'s launch record and history, every pool\'s sources and the final pool are in [`evidence/g5/v{rv}/`](evidence/g5/v{rv}/); '
            'the islands\' archives are a release asset.', '']


def readme_g5():
    v, rv, r = receipt_g5()
    rs = toml('requirements/RS-karambit-v5.toml')
    man, cad, spec = g5_files(rv)
    img = man['images']
    sc, pv, co, bs, rk, gc = r['score'], r['provenance'], r['cohort'], r['baseline'], r['ranking'], r['grid_check']
    ld, mg, ms, am, bl, pt, eg, dc, ev = (r[k] for k in ('loads', 'margins', 'mass', 'am', 'blade', 'point', 'ergonomics', 'decoded', 'evolution'))
    v6 = 'lineage' in r  # v6.0.0 on: a continuation with a pool lineage, on one platform of record
    la, lc, sl = (None, None, rk['search_leader']) if v6 else (co['launch'], rk['linux_check'], rk['search_leader'])
    vvd = r['provenance']['vv'] if v6 else f'evidence/g5/v{rv}/vv'
    wid = r['design_id']
    mat, am_rs, ed_rs, sr_rs, pt_rs, ms_rs = (rs[k] for k in ('material', 'am', 'edge', 'serration', 'point', 'mass'))
    eg_rs = rs['ergonomics']
    ein, esp = bl['edge_inner'], bl['edge_spine']
    hist = {} if v6 else {h['evals']: h for h in (json.loads(l) for l in rd(f'{ev["history_dir"]}/{ev["winner_island"]}.jsonl').splitlines() if l.strip())}
    q = lambda s: s.replace('-', '--').replace('_', '__').replace(' ', '%20')
    mgm = r['margins_min_over_plate_meshes']  # B1: each case's margin of record is its minimum over the plate meshes
    pmc = r['plate_mesh_check']
    pick = ('It is the top-ranked design' if r['rank_of_record'] == 1
            else f'The higher-ranked designs fail a mesh, so the winner is rank {r["rank_of_record"]}')
    worst = min((x for x in mgm.items() if isinstance(x[1], float)), key=lambda kv: kv[1])
    paper = f'docs/g5/paper/g5-v{v}.pdf'
    am_file, env_file = spec['am_build_stl']['file'], spec['outer_envelope_stl']['file']
    kit = {n: f'cad/{wid}_{n}' for n in ('AMbuild_mm.stl', 'AMbuild_mm.3mf', 'envelope_mm.stl', 'drawing_mm.dxf', 'spec_paper.pdf', 'xray_cutaway.jpg')}
    L = [f'# Jarambit {v}: an evolved titanium karambit for laser powder-bed fusion', '',
         f'[![release](https://img.shields.io/badge/release-v{v}-2f56d6)]({REPO}/releases/tag/v{v}) '
         f'[![paper](https://img.shields.io/badge/paper-G5%20v{v}%20PDF-2f56d6)]({paper}) '
         f'[![3D print CAD](https://img.shields.io/badge/3D%20print%20CAD-cad%2F%20(STL%20%7C%203MF%20%7C%20DXF%20%7C%20PDF)-2f56d6)](cad/) '
         f'![design](https://img.shields.io/badge/best%20individual-{q(wid)}-555) '
         f'![score](https://img.shields.io/badge/W__ERGOLIGHT__v2-{sc["score_100"]:.2f}-555) '
         f'![mass](https://img.shields.io/badge/mass-{ms["of_record_g"]:.1f}%20g-555) '
         f'![status](https://img.shields.io/badge/status-computational%2C%20not%20built-b4570f)', '',
         f'Every computed number and picture on this page describes one design, **`{wid}`**, the best individual of the G5 '
         f'v{rv} cohort of record. They are generated from [`evidence/g5/v{rv}/release.json`](evidence/g5/v{rv}/release.json), '
         f'its verification studies and the sourced requirement set [`requirements/RS-karambit-v5.toml`](requirements/RS-karambit-v5.toml) '
         f'([provenance](requirements/PROVENANCE-v5.md)). Status: {r["status"]}.', '',
         f'**Paper:** [`{paper}`]({paper}), generated from the same receipt. **3D printing:** the upload kit in [`cad/`](cad/).', '',
         f'<img src="{img["hero"]}" alt="The best individual {wid}" width="100%">', '',
         '## The best individual', '',
         f'Titanium ({mat["name"]}): a {eg["grip_length_mm"]:.1f} mm grip of {eg["grip_perimeter_mm"]:.1f} mm '
         f'perimeter with a {eg_rs["ring"]["bore_d_mm"]:g} mm finger ring, a hollow core of sheet-TPMS lattice with powder ports, and a '
         f'double-edged claw blade with serrations on both edges ({ein["teeth"]} and {esp["teeth"]} teeth). '
         f'It meets {"every" if r["meets_every_load_case"] else "not every"} RS-karambit-v5 load case: the lowest margin is '
         f'{worst[0].replace("_", " ")}, {worst[1]:.3f} (the minimum over the plate meshes {" / ".join(f"{a:g}" for a in pmc["max_area_mm2"])} mm²).', '',
         '| Measure | Value | Requirement (RS-karambit-v5) | Reading |', '|---|---|---|---|',
         f'| W_ERGOLIGHT_v2, Tier 2 at h {rk["h_mm"]:g} mm (rank {r["rank_of_record"]} of {rk["shortlist"]}) | **{sc["score_100"]:.2f}** | higher is better | '
         f'hand multiplier E {sc["E"]:.3f} (raised to the {rs["objective"]["e_exponent"]:g}th power), worst-margin multiplier W {sc["worst_margin_W"]:.3f} |',
         f'| Mass of record (partial volume at h {rk["h_mm"]:g} mm, × {ms["density_g_mm3"]} g/mm³) | **{ms["of_record_g"]:.1f} g** | ceiling {ms_rs["ceiling_g"]:g} g; '
         f'{ms_rs["goal_g"]:g} g goal (a frontier, not a requirement) | {meets(ms["of_record_g"], ms_rs["ceiling_g"], "<=")}; watertight print mesh {ms["print_mesh_g"]:.1f} g |',
         f'| Grip length / perimeter / height-to-thickness | {eg["grip_length_mm"]:.1f} mm / {eg["grip_perimeter_mm"]:.1f} mm / {eg["grip_h_over_t"]:.2f} | '
         f'≥ {eg_rs["grip"]["length_min_mm"]:g} mm / {eg_rs["grip"]["perimeter_min_mm"]:g}–{eg_rs["grip"]["perimeter_max_mm"]:g} mm / '
         f'{eg_rs["grip"]["height_over_thickness_min"]:g}–{eg_rs["grip"]["height_over_thickness_max"]:g} | '
         f'{meets(eg["grip_length_mm"], eg_rs["grip"]["length_min_mm"])} |',
         f'| Guard height / centre of mass to the index finger | {eg["guard_height_mm"]:.1f} mm / {eg["com_to_index_mm"]:.1f} mm | '
         f'≥ {eg_rs["guard"]["height_target_mm"]:g} mm / ≤ {eg_rs["balance"]["com_to_index_max_mm"]:g} mm | '
         f'{meets(eg["guard_height_mm"], eg_rs["guard"]["height_target_mm"])} / {meets(eg["com_to_index_mm"], eg_rs["balance"]["com_to_index_max_mm"], "<=")} |',
         f'| Point: included angle at 1 mm / at 3 mm; width at 2 mm | {pt["angle_1mm_deg"]:.1f}° / {pt["angle_3mm_deg"]:.1f}°; {pt["width_2mm"]:.2f} mm | '
         f'≤ {pt_rs["angle_max_at_1mm_deg"]:g}° / ≤ {pt_rs["angle_max_at_3mm_deg"]:g}°; ≤ {pt_rs["width_max_at_2mm"]:g} mm | '
         f'{meets(pt["angle_1mm_deg"], pt_rs["angle_max_at_1mm_deg"], "<=")} / {meets(pt["angle_3mm_deg"], pt_rs["angle_max_at_3mm_deg"], "<=")}; '
         f'{meets(pt["width_2mm"], pt_rs["width_max_at_2mm"], "<=")} (a continuous penalty, not a gate) |',
         f'| Edges A (inner) / B (spine): per-side grind angle | {ein["per_side_angle_gene_deg"]:.1f}° / {esp["per_side_angle_gene_deg"]:.1f}° | '
         f'{ed_rs["per_side_angle_min_deg"]:g}–{ed_rs["per_side_angle_max_deg"]:g}° | largest measured half-angle {bl["max_half_angle_deg"]:.1f}°, apex at most {bl["max_apex_mm"]:.3f} mm thick |',
         f'| Serrations A / B: depth, teeth | {ein["serration_depth_mm"]:.2f} mm, {ein["teeth"]} / {esp["serration_depth_mm"]:.2f} mm, {esp["teeth"]} | '
         f'{sr_rs["depth_min_mm"]:g}–{sr_rs["depth_max_mm"]:g} mm; gullet radius ≥ {sr_rs["gullet_radius_min_mm"]:g} mm by construction | '
         f'weakest tooth SF {ein["tooth_root_sf_min"]:.2f} / {esp["tooth_root_sf_min"]:.2f} |',
         f'| Hand comfort factors (1 = within every limit; E is their weighted product) | '
         + ', '.join(f'{k.replace("_", " ")} {x["e"]:.2f}' for k, x in sorted(eg['hand_factors'].items(), key=lambda kv: kv[1]['e']) if x['e'] < 1)
         + ' | NASA/MIL hand limits (contact pressure, edge radii, grip section) | the other factors are 1.00 |',
         f'| Powder: sealed cavity / thin crevices (printed solid) | {am["sealed_void_mm3"]:.0f} mm³ / {am["crevice_mm3"]:.0f} mm³ | '
         f'no closed voids; drainage passages ≥ {am_rs["drain_channel_min_mm"]:g} mm | sealed volume is penalised by the score; '
         f'{dc["port_n"]:.0f} ports of {dc["port_d"]:.1f} mm (≥ {am_rs["port_d_min_mm"]:g} mm) |',
         '', f'Every RS-karambit-v5 load case, as a safety factor (SF) against its requirement. The margin is v = SF / requirement; '
         f'the smallest margin sets the worst-margin multiplier W once it falls below 1. SF is on the production plate mesh '
         f'({pmc["production_max_area_mm2"]:g} mm²), as scored; the margin of record is the minimum over the plate meshes '
         f'{" / ".join(f"{a:g}" for a in pmc["max_area_mm2"])} mm² (cases not solved on the plate are mesh-independent):', '',
         '| Load case | SF | Requirement | Margin v (score mesh) | Margin v of record (min over meshes) |', '|---|---:|---:|---:|---:|']
    for (label, sf, req, m), key in zip(g5_load_cases(rs, ld, mg), ('lateral_limit', 'lateral_ultimate', 'twist_limit', 'tip_axial', 'tip_lateral', 'tip_twist',
                                                                  'grip_squeeze', 'guard_push', 'ring_bearing', 'ring_back', 'ring_any_direction',
                                                                  'pull', 'teeth', 'fatigue', 'sequence', 'buckling')):
        L.append(f'| {label} | {sf:.2f} | ≥ {req:g} | {m:.3f} | {mgm[key]:.3f} |')
    g = lambda k, alt: f'<img src="{img[k]}" alt="{alt}">'
    L += ['', '<table><tr>'
          f'<td width="50%">{g("side", "Plan view")}<br>Plan view.</td><td width="50%">{g("top", "Edge-on view")}<br>Edge-on: ring, grip and blade thickness.</td></tr><tr>'
          f'<td>{g("blade", "The blade")}<br>The blade, reach {bl["reach_chord_mm"]:.1f} mm.</td><td>{g("tip", "The point")}<br>The point: {pt["angle_1mm_deg"]:.0f}° at 1 mm.</td></tr><tr>'
          f'<td>{g("edge_a", "Inner edge")}<br>Inner edge (A), {ein["teeth"]} teeth, as decoded (before grinding).</td><td>{g("edge_b", "Spine edge")}<br>Spine edge (B), {esp["teeth"]} teeth, as decoded (before grinding).</td></tr><tr>'
          f'<td>{g("grip", "Grip")}<br>Grip, {eg["grip_perimeter_mm"]:.1f} mm perimeter.</td><td>{g("ring", "Ring")}<br>Ring, {eg_rs["ring"]["bore_d_mm"]:g} mm bore.</td></tr><tr>'
          f'<td>{g("xray", "X-ray of the internal cavities")}<br>X-ray of the hollow core.</td><td>{g("cut_full", "Lengthwise cut")}<br>Lengthwise cut: skin, lattice and ports.</td></tr><tr>'
          f'<td>{g("cut_x", "Section across the grip")}<br>Section across the grip.</td><td>{g("cut_b", "Section across the blade")}<br>Section across the blade.</td></tr><tr>'
          f'<td>{g("tip_edge", "Edge at the point")}<br>The edge at the point, as decoded (before grinding).</td><td></td></tr></table>', '',
          f'Edge sections at four stations per edge, true scale, from the exact decoder law (dashed: {ed_rs["per_side_angle_min_deg"]:g}° per side, '
          'the smallest angle the requirement allows):', '',
          picture(img['edge_sections_light'], img['edge_sections_dark'], 'Blade cross-sections of both edges'), '']
    # ---- v6: where the score comes from, the FEM renderings, the evolution (the comparison with the v5.0.0 winner is in
    # docs/RELEASES.md and the paper: the README describes only this release's best individual)
    if v6:
        L += g5v6_fitness(r) + g5v6_fem(r, rv) + g5v6_evolution(r, rk, gc, man)
    # ---- evolution (v5.0.0)
    base_best = None if v6 else max(ev['baseline_final_best_score_100'])
    L += [] if v6 else ['## How evolution produced it', '',
          f'{co["islands"]} independent islands each ran {int(co["evaluations_per_island"]):,} evaluations ({int(co["evaluations_total"]):,} in all) of a '
          f'MAP-Elites search over {la["archive_niches"]:,} Lloyd-relaxed niches, starting only from scrambled Sobol samples '
          f'({la["cohort"]["init"]} per island; no seeded or hand-made individuals). Alongside them, {bs["islands"]} control islands drew '
          f'{int(bs["evaluations_per_island"]):,} Sobol samples each with the same decoder, objective and promotion, and no evolution.', '',
          f'**All {ev["cohort_islands_beating_best_baseline"]} of {co["islands"]} evolving islands beat the best random-search island** '
          f'({base_best:.1f}). The island median rose from {ev["cohort_after_sobol_init_median_score_100"]:.1f} after the Sobol start to '
          f'{ev["cohort_final_best_score_100"]["median"]:.1f} (range {ev["cohort_final_best_score_100"]["min"]:.1f}–{ev["cohort_final_best_score_100"]["max"]:.1f}). '
          f'Evolution found the winner in {ev["winner_island"]} at {ev["winner_first_island_best_at_evals"]:,} evaluations.', '',
          f'<img src="{ev["figure"]}" alt="Best score found against evaluations, every island of the cohort of record" width="100%">', '',
          f'The best design of {ev["winner_island"]} at checkpoints of its run, same camera and light (scores on the h {co["h_mm"]:g} mm search grid):', '',
          '<table><tr>' + ''.join(
              f'<td align="center"><img src="{c["image"]}" alt="{ev["winner_island"]} at {c["evals"]} evaluations" width="100%"><br>'
              f'{c["evals"]:,} evaluations<br>{hist[c["evals"]]["best_score_100"]:.1f}, {hist[c["evals"]]["best_mass_g"]:.0f} g'
              + (f'<br><b>{c["role"]}</b>' if c['role'] else '') + '</td>' for c in man['lineage']) + '</tr></table>', '',
          f'**Why the search leader is not the winner.** The best design on the search grid, `{sl["design_id"]}` '
          f'({sl["tier1_h04_score_100"]:.1f}), ranks {sl["tier2_rank"]} once re-scored at Tier 2. On the TET10 solid its tip carries the '
          f'{rs["loads"]["tip"]["axial_n"]:g} N axial case at SF {sl["tip_sf_solid_h02"]["axial"]:.2f}, below the {rs["margins"]["sf_yield"]:g} required, '
          f'where the section model of the search had given {sl["tip_sf_sections_h04"]["axial"]:.2f}. Every finalist is judged on the solid.', '',
          f'Tier-2 ranking (h {rk["h_mm"]:g} mm, solid tip at {rk["tip_mesh_max_area_mm2"]:g} mm²) and the grid check of the top {lc["designs"]} '
          f'(partial-volume mass in grams at h = {" / ".join(f"{h:g}" for h in gc["h_mm"])} mm; the h {rk["h_mm"]:g} and h {gc["h_mm"][-1]:g} mm '
          f'masses must agree within {100 * gc["requirement_max_frac"]:g} %):', '',
          '| Rank | Design | Island | Score | E | W | Mass (g) | Mass at the five grids (g) | h 0.2 vs 0.15 |', '|---:|---|---|---:|---:|---:|---:|---|---:|']
    grid = {x['design_id']: x for x in gc['rows']}
    for t in ([] if v6 else rk['rows'][:lc['designs']]):
        x = grid[t['design_id']]
        L.append(f'| {t["rank"]} | `{t["design_id"]}` | {t["island"]} | {t["score_100"]:.2f} | {t["E"]:.3f} | {t["worst_margin_W"]:.3f} | '
                 f'{t["mass_g"]:.1f} | {" / ".join(f"{m:.1f}" for m in x["mass_g"])} | {100 * x["mass_rel_h020_vs_h015"]:.2f} % |')
    # ---- verification and limits
    vv = lambda n: js(f'{vvd}/{n}.json')
    tg, ck, pm = vv('decoder_targets'), vv('checks'), vv('plate_mesh')
    spread = lambda key: max(100 * (max(x[key] for x in row['plate']) / min(x[key] for x in row['plate']) - 1) for row in pm['rows'])
    areas = ' / '.join(f'{x["max_area_mm2"]:g}' for x in pm['rows'][0]['plate'])
    rp = co['reproduced']
    L += ['', '## Verification and limits', ''] + (g5v6_verification(r, rv) if v6 else [
          f'- **Reproducible on the platform of record.** Re-evaluating all {co["elites"]:,} final elites reproduces every Tier-1 archive row bit for bit '
          f'({co["tier1_rows_reproduced"]} rows), and {rp["island"]} re-run from the launch record reproduces its history bit for bit '
          f'({rp["history_entries_identical"]} entries over {rp["evaluations"]} evaluations, {rp["platform"]}).',
          f'- **Not bit-identical across architectures.** On Linux x86_64 the platform maths library differs: the top {lc["designs"]} Tier-2 scores agree within '
          f'{100 * lc["max_rel_diff"]:.2f} % (bit-identical: {"yes" if lc["bit_identical"] else "no"}), the winner ranks {lc["winner_rank_linux"]} there as well, '
          f'and {lc["evolution"]["island"]} keeps the same best design through all {lc["evolution"]["same_best_genome_through_evals"]} evaluations compared.']) + [
          f'- **Plate mesh sensitivity** ([`vv/plate_mesh.json`]({vvd}/plate_mesh.json)). Across plate meshes of {areas} mm² the '
          f'lateral safety factor of the study designs varies by up to {spread("lateral_sf"):.0f} % (not monotone) and the twist safety factor by up to '
          f'{spread("twist_sf"):.0f} %; the search and ranking use {pm["production_max_area_mm2"]:g} mm². So the winner was chosen on the minimum over '
          f'{" / ".join(f"{a:g}" for a in pmc["max_area_mm2"])} mm² ([`plate_meshes.jsonl`](evidence/g5/v{rv}/plate_meshes.jsonl), solved on the platform of '
          f'record, whose {pmc["production_max_area_mm2"]:g} mm² rows reproduce the ranking bit for bit): in rank order, the first design meeting every load case '
          f'on every mesh. {pick}; its lowest margin over the meshes is '
          f'{worst[0].replace("_", " ")} {worst[1]:.3f}.',
          f'- **Numerical studies** ([`{vvd}/`]({vvd}/)): grid agreement of mass, blade-plate and solid-tip mesh studies, '
          f'the grip perimeter within {100 * tg["max_perimeter_rel_err"]:.2f} % of its gene, Rayleigh–Ritz against Euler within {abs(ck["ritz_column"]["rel_err"]):.0e}, '
          f'and Lloyd niches with {100 * (1 - ck["cvt_1024"]["lloyd_quantisation_error"] / ck["cvt_1024"]["halton_quantisation_error"]):.1f} % less quantisation error than Halton.',
          f'- **Texture on the blade.** The frozen G5.0 decoder lets the evolved surface texture reach the blade (here texture depth '
          f'{dc["tex_amp"]:.2f} mm, blade scope {dc["scope_blade"]:.2f}), bevels included, and the renders and print meshes show it. The edge '
          f'figures and measurements use the exact bevel law, which grinding after printing restores: the requirement\'s finishing allowance '
          f'({ed_rs["finishing_allowance_mm"]:g} mm per ground face) exceeds the texture depth. Keeping texture off the bevels is a decoder change for a later generation.',
          f'- **Status: {r["status"]}.** Nothing here is a qualification for use. The safety factors are computed from models with the '
          'limits stated in the paper; the material values are specification minima, not measured coupons.',
          '- **Evidence gaps** listed in [`requirements/PROVENANCE-v5.md`](requirements/PROVENANCE-v5.md) need physical tests before any build: '
          'tooth loads, tip load shares, edge retention, thin-wall roughness, lattice homogenisation, the temperature derate and the drop test.', '']
    # ---- CAD
    L += ['## 3D printing and CAD (`cad/`, millimetres)', '',
          f'The upload kit in [`cad/`](cad/) holds six files for `{wid}`, each under {GIT_FILE_MAX_MIB} MiB, generated from the winner\'s genome row '
          f'([`evidence/g5/v{rv}/winner_genome.csv`](evidence/g5/v{rv}/winner_genome.csv)). Order notes are in [`cad/README.md`](cad/README.md).', '',
          '| File | What it is |', '|---|---|',
          f'| [`{kit["AMbuild_mm.stl"]}`]({kit["AMbuild_mm.stl"]}) | print file: skin, sheet-TPMS lattice and powder ports, {spec["am_build_stl"]["voxel_pitch_h_mm"]:g} mm voxels, {spec["am_build_stl"]["triangles"]:,} triangles |',
          f'| [`{kit["AMbuild_mm.3mf"]}`]({kit["AMbuild_mm.3mf"]}) | the same print mesh as 3MF, with millimetre units |',
          f'| [`{kit["envelope_mm.stl"]}`]({kit["envelope_mm.stl"]}) | outer envelope, {spec["outer_envelope_stl"]["voxel_pitch_h_mm"]:g} mm voxels |',
          f'| [`{kit["drawing_mm.dxf"]}`]({kit["drawing_mm.dxf"]}) | 1:1 plan drawing (DXF, mm) |',
          f'| [`{kit["spec_paper.pdf"]}`]({kit["spec_paper.pdf"]}) | the G5 paper |',
          f'| [`{kit["xray_cutaway.jpg"]}`]({kit["xray_cutaway.jpg"]}) | cutaway of the hollow core |', '',
          f'Finer meshes and the full evaluation records are release assets of [v{v}]({REPO}/releases/tag/v{v}), each with its SHA-256 in the receipt:', '',
          '| Asset | Size | SHA-256 |', '|---|---:|---|']
    for a in r['release_assets']:
        L.append(f'| `{a["file"]}` | {a["bytes"] / 1048576:.1f} MiB | `{a["sha256"][:16]}…` |')
    # ---- reproduce
    if v6:
        L += g5v6_reproduce(r, rv, rk)
    lr = None if v6 else la['cohort']
    L += [] if v6 else ['', '## Reproduce', '', '```bash',
          'cargo build --release --locked --manifest-path crates/g5/Cargo.toml --bins',
          f'# one island of the cohort of record (decoder {pv["decoder_sha256"][:12]}, freeze label {lr["commit"]}); seed = host × 1000 + island',
          f'crates/g5/target/release/g5-evolve --out ~/jarambit_runs/evo_g5/<run>/island01/run --init {lr["init"]} --evals {lr["evals"]} \\',
          f'  --seed {int(lr["host"]) * 1000 + 1} --host {lr["host"]} --island 1 --commit {lr["commit"]} --h {lr["h"]:g} '
          f'--tier1-top {lr["tier1_top"]:g} --tier2-top {lr["tier2_top"]:g} --log-every {lr["log_every"]}',
          f'crates/g5/target/release/g5-pilot eval <cohort_root> eval_h040.jsonl {co["h_mm"]:g} 4',
          f'crates/g5/target/release/g5-pilot tier2 <cohort_root> eval_h040.jsonl {rk["shortlist"]} tier2_h020.jsonl {rk["h_mm"]:g} 4',
          f'crates/g5/target/release/g5-release receipt . <cohort_root> <control_root> <cert_dir> {rk["shortlist"]} {lc["designs"]}',
          f'crates/g5/target/release/g5-vv evidence/g5/v{rv}/vv 4', '```', '',
          f'Every island\'s genomes and history are in [`evidence/g5/v{rv}/`](evidence/g5/v{rv}/), so any elite can be re-evaluated.', '']
    L += ['## Repository', '',
          '- [`crates/g5/`](crates/g5/README.md): the G5 genome (G5.0), decoder, materials, load models, objective, search and release tools.',
          '- [`requirements/`](requirements/RS-karambit-v5.toml): RS-karambit-v5 and its [provenance](requirements/PROVENANCE-v5.md).',
          f'- [`evidence/g5/v{rv}/`](evidence/g5/v{rv}/): the receipt, the winner\'s genome, every island\'s history, the ranking, the grid check and `vv/`.',
          f'- [`docs/g5/`](docs/g5/): the paper, the images and the CAD archive. [`docs/RELEASES.md`](docs/RELEASES.md): every release.',
          '- Earlier generations, kept as their own record: G3 ([`docs/g3/`](docs/g3/), [`crates/g3/`](crates/g3/README.md)), the withdrawn G4 '
          'prototype ([`crates/g4/WITHDRAWN.md`](crates/g4/WITHDRAWN.md)) and the v1/v2 study ([`docs/paper/`](docs/paper/), [`docs/WHITEPAPER.md`](docs/WHITEPAPER.md)). '
          'The v1 command-line pipeline (`cargo run -p jarambit`) is still in the repository; its plate level is opt-in with `JARAMBIT_FEM=1`.', '']
    return '\n'.join(L)


def cad_readme_g5():
    v, rv, r = receipt_g5()
    rs = toml('requirements/RS-karambit-v5.toml')
    _, _, spec = g5_files(rv)
    wid, ms, mg = r['design_id'], r['mass'], r['margins']
    mat, ed_rs = rs['material'], rs['edge']
    dim, am_s, env_s = spec['overall_dimensions_mm'], spec['am_build_stl'], spec['outer_envelope_stl']
    worst = sorted(((k, x) for k, x in mg.items() if isinstance(x, float)), key=lambda kv: kv[1])[:3]
    notes = (f'Units: mm | Overall {dim["length_x_mm"]:.1f} x {dim["height_y_mm"]:.1f} x {dim["thickness_z_mm"]:.1f} mm | '
             f'Material: {mat["name"]} | Remove powder through the ports in every hollow zone before stress relief | '
             f'Grind both cutting edges after printing ({ed_rs["per_side_angle_min_deg"]:g}-{ed_rs["per_side_angle_max_deg"]:g} deg per side, '
             f'{ed_rs["finishing_allowance_mm"]:g} mm allowance per ground face) | Machine the ring bore round | '
             f'Status: {r["status"]}; not qualified for use as a knife')
    size = lambda n: os.path.getsize(os.path.join(ROOT, 'cad', n)) / 1048576
    files = [(am_s['file'], 'print file: skin, sheet-TPMS lattice, powder ports'), (f'{wid}_AMbuild_mm.3mf', 'the print mesh as 3MF (millimetres)'),
             (env_s['file'], 'outer envelope'), (f'{wid}_drawing_mm.dxf', '1:1 plan drawing'), (f'{wid}_spec_paper.pdf', f'the G5 paper (v{v})'),
             (f'{wid}_xray_cutaway.jpg', 'cutaway of the hollow core')]
    L = [f'# 3D printing and CAD upload kit: `{wid}` (release v{v})', '',
         f'Six files for the best individual of the G5 v{rv} cohort of record ([`evidence/g5/v{rv}/release.json`](../evidence/g5/v{rv}/release.json)), '
         f'in millimetres, each under {GIT_FILE_MAX_MIB} MiB. Generated from [`evidence/g5/v{rv}/winner_genome.csv`](../evidence/g5/v{rv}/winner_genome.csv).', '',
         f'**Status: {r["status"]}.** This design is not qualified for use as a knife. Its mass of record is {ms["of_record_g"]:.1f} g '
         f'(this print mesh, at {am_s["voxel_pitch_h_mm"]:g} mm voxels: {am_s["mass_ti64_g"]:.1f} g). Its lowest computed margins are '
         + ', '.join(f'{k.replace("_", " ")} {x:.2f}' for k, x in worst) + '.', '',
         '| File | Size | What it is |', '|---|---:|---|']
    for n, what in files:
        L.append(f'| [`{n}`]({n}) | {size(n):.1f} MiB | {what} |')
    L += ['', '## Order notes (copy into the quote form)', '', '```text', notes, '```', '',
          f'Release assets of v{v} (too large for git), each with its SHA-256 in the receipt:', '',
          '| Asset | Size | SHA-256 |', '|---|---:|---|']
    L += [f'| `{a["file"]}` | {a["bytes"] / 1048576:.1f} MiB | `{a["sha256"][:16]}…` |' for a in r['release_assets']] + ['']
    return '\n'.join(L)


if __name__ == '__main__':
    v_cur = rd('docs/VERSION').strip()
    is_g5 = int(v_cur.split('.')[0]) >= 5 and os.path.isfile(os.path.join(ROOT, 'evidence/g5/RECORD'))
    new_readme = readme_g5() if is_g5 else readme()
    new_cad = cad_readme_g5() if is_g5 else cad_readme()
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
