"""Generate README.md for the current release (v3.0.0+) from its committed release receipt. Deterministic: no clock,
no git, no network. Every number, image and claim about the knife comes from evidence/g3/v<VERSION>/release.json,
written by the release pipeline from the cohort of record (owner, 2026-10-26: "the README is 100 % accurate with the
best individual (no older data), also it should show the evolution").
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


def receipt():
    v = rd('docs/VERSION').strip()
    rec = js(f'evidence/g3/v{v}/release.json')
    missing = [k for k in REQUIRED if k not in rec]
    if missing or rec['version'] != v:
        raise SystemExit(f'gen_readme: release.json for v{v} is incomplete or for another version ({missing or rec["version"]})')
    return v, rec


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
    v, r = receipt()
    i, sc, pv, co, bl, fe, ev = r['inputs'], r['score'], r['provenance'], r['cohort'], r['blade'], r['fe_certification'], r['evolution']
    a, b = bl['edge_inner'], bl['edge_spine']
    img = r['images']
    wid = r['design_id']
    mass = r['mass']['mesh_g']
    q = lambda s: s.replace('-', '--').replace('_', '__').replace(' ', '%20')
    L = []
    L += [f'# Jarambit {v}: an evolved, 3D-printable titanium karambit', '',
          f'[![release](https://img.shields.io/badge/release-v{v}-2f56d6)]({REPO}/releases/tag/v{v}) '
          f'![design](https://img.shields.io/badge/best%20individual-{q(wid)}-555) '
          f'![score](https://img.shields.io/badge/W__ERGOLIGHT__v1-{sc["score_100"]:.2f}-555) '
          f'![mass](https://img.shields.io/badge/mass-{mass:.1f}%20g-555) '
          f'![status](https://img.shields.io/badge/status-computational%2C%20not%20built-b4570f)', '',
          f'Every number and picture on this page describes one design, **`{wid}`**, the best individual of the v{v} cohort of '
          f'record, and is generated from [`evidence/g3/v{v}/release.json`](evidence/g3/v{v}/release.json). '
          f'Status: {r["status"]}.', '',
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
          f'| Serrations, edge A / B: depth, teeth | {a["serration_depth_mm"]:.2f} mm, {a["teeth"]} / {b["serration_depth_mm"]:.2f} mm, {b["teeth"]} | both edges serrated | '
          f'mean flank {a["mean_flank_deg"]:.0f}° / {b["mean_flank_deg"]:.0f}° |',
          f'| Weakest tooth, edge A / B ({bl["tooth_loads_n"]["snag_along_edge"]:.0f} N snag, {bl["tooth_loads_n"]["pry_across_blade"]:.0f} N pry) | '
          f'SF {a["tooth_root_sf_min"]:.2f} / {b["tooth_root_sf_min"]:.2f} | ≥ 1 | {meets(min(a["tooth_root_sf_min"], b["tooth_root_sf_min"]), 1.0)} |',
          f'| Solid tip, 1.82 N·m pry (TET10) | SF {i["solid_tip_sf"]:.2f} | ≥ 1 | {meets(i["solid_tip_sf"], 1.0)} |',
          f'| Lateral load at the tip (plate FE) | {i["lat_rob_cert_n"]:.0f} N | ≥ 260 N | {meets(i["lat_rob_cert_n"], 260.0)} |',
          f'| Grip, worst case (beam formula) / hex8 voxel FE, spine 3427 N | SF {i["grip_sf_min"]:.2f} / {fe["grip_sf"]["spine"]:.2f} | ≥ 1 | '
          f'{meets(min(i["grip_sf_min"], fe["grip_sf"]["spine"]), 1.0)} |',
          f'| Human-max twist 16.2 N·m (plate FE, blade body) / hex8 voxel FE, root | SF {i["hm_twist_sf"]:.2f} / {fe["root_sf"]["twist"]:.2f} | ≥ 1 | '
          f'{meets(min(i["hm_twist_sf"], fe["root_sf"]["twist"]), 1.0)} |',
          f'| Spine load 3427 N (plate FE) | SF {i["hm_spine_sf"]:.2f} | ≥ 1 | {meets(i["hm_spine_sf"], 1.0)} |',
          f'| Ring, 300 N bearing / 30 N back at SF 2 | {i["ring_bearing_cap_n"]:.0f} / {i["ring_back_cap_n"]:.0f} N | ≥ 600 / 60 N | '
          f'{meets(min(i["ring_bearing_cap_n"] / 600.0, i["ring_back_cap_n"] / 60.0), 1.0)} |',
          f'| Fatigue / blade-before-hand yield sequence | {i["fat_sf"]:.2f} / {i["seq_ratio"]:.2f} | ≥ 1 / ≥ 1.2 | '
          f'{meets(min(i["fat_sf"], i["seq_ratio"] / 1.2), 1.0)} |',
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
          f'from {co["init"]}. Each island is a CVT-MAP-Elites archive of 1,024 behaviour niches under W_ERGOLIGHT_v1 (draft d), with a three-tier '
          f'cascade: analytic Tier 0, voxel-SDF plus plate-FE Tier 1, and Tier 2 with a watertight mesh and a solid-tip TET10 model (gates '
          f'{co["tier_gates"][0]} and {co["tier_gates"][1]}, h {co["h_mm"]} mm). The run kept {co["elites"]:,} elites, {co["tier2_elites"]} of them at Tier 2. '
          f'The winner\'s island ({ev["winner_island"]}) started from a best measured score of {s0["best_score_100"]:.2f} after its Sobol initialisation.', '',
          picture(ev['figures']['evolution_best_light'], ev['figures']['evolution_best_dark'], 'Best measured score so far, every island'), '',
          picture(ev['figures']['evolution_archive_light'], ev['figures']['evolution_archive_dark'], 'Archive growth, every island'), '',
          f'The best design of {ev["winner_island"]} at each logged checkpoint, same camera and light:', '',
          '<table><tr>' + ''.join(f'<td align="center"><img src="{c["image"]}" alt="evaluation {c["evals"]}" width="100%"><br>'
                                  f'eval {c["evals"]:,}: {c["best_score_100"]:.2f}, {c["best_mass_g"]:.1f} g'
                                  + (f'<br><b>{c["role"]}</b>' if c.get('role') else '') + '</td>' for c in lin) + '</tr></table>', '',
          f'Scores and masses under each frame are the search\'s own (h {co["h_mm"]} mm).', '',
          f'**Choosing the winner on the print grid.** The search measures every design on a {co["h_mm"]} mm voxel grid, and that grid can '
          f'undercount mass. The search\'s own leader, `{gl["design_id"]}` (Tier 2 score {gl["tier2_score_100_h04"]:.2f} at h {co["h_mm"]}), '
          f'weighs {gl["mass_g_h04"]:.1f} g at h {co["h_mm"]} but {gl["mass_g_h02"]:.1f} g at h 0.2 and {gl["mass_g_h015"]:.1f} g at h 0.15, and its Tier-1 score falls from '
          f'{gl["tier1_score_100_h04"]:.2f} to {gl["tier1_score_100_h02"]:.2f}. So every number of record is measured at h {co["ranking_h_mm"]} mm, '
          f'the grid of the print file: every elite ordered by its record from the run, the top 32 re-evaluated at Tier 2 at h {co["ranking_h_mm"]} on the search host, '
          f'then the top 8 again on Linux x86_64, the platform of record (the winner scored {r["arm64_score_100"]:.2f} on the search host).', '',
          f'Grid check: mass in grams at each voxel pitch h (mm). {gc["evaluation"]}; h 0.2 and h 0.15 agree within {gc["h02_vs_h015_max_rel_pct"]:.1f} % for every design.', '',
          '| Design | ' + ' | '.join(f'h {h}' for h in gc['h_mm']) + ' |', '|---|' + '---:|' * len(gc['h_mm'])]
    L += [f'| `{g["design_id"]}` | ' + ' | '.join(f'{m:.1f}' for m in g['mass_g']) + ' |' for g in gc['rows']]
    L += ['',
          'Final ranking:', '',
          f'| Rank | Design | Score (Linux Tier 2, h {co["ranking_h_mm"]}) | Mass at h {co["ranking_h_mm"]} |', '|---:|---|---:|---:|']
    L += [f'| {k + 1} | `{t["design_id"]}` | {t["score_100"]:.2f} | {t["mass_g"]:.1f} g |' for k, t in enumerate(co['top8_linux'])]
    L += ['']
    # ---- measurement
    tc = fe['tip_tet10_sf_by_mesh_area_mm2']
    L += ['## How the design is constrained and measured', '',
          '- **Edges (EDGE-1..5).** One primary grind per edge at 17.5–22.5° per side, measured normal to the actual serrated edge, at full height '
          'and unbroken to the point; only a short plunge by the guard is unsharpened.',
          '- **Serrations (SERR-1..4).** Both edges serrated, at least 3 full-depth teeth each, pitch 5–9 mm, depth at most a quarter of the '
          'pitch, and no serration within 18 mm of the point. Each tooth root is checked under a single-tooth snag and pry.',
          '- **Evolution designs the knife.** Populations start from scrambled Sobol samples only. Nobody edits an individual; the rules above are '
          'constraints on what the decoder can express.',
          f'- **FE certification of this design (hex8 voxel FE on the as-built voxels, h {fe["hex8_voxel_fe_h_mm"]} mm, {fe["stat"]}).** '
          f'Grip zone: lateral {fe["grip_sf"]["lat"]:.2f}, twist {fe["grip_sf"]["twist"]:.2f}, spine {fe["grip_sf"]["spine"]:.2f}. '
          f'Root: lateral {fe["root_sf"]["lat"]:.2f}, twist {fe["root_sf"]["twist"]:.2f}, spine {fe["root_sf"]["spine"]:.2f}. '
          f'Loads: {fe["loads"]}.',
          f'- **Tip mesh convergence.** Solid-tip SF {tc["area_0p4"]:.3f} / {tc["area_0p15_production"]:.3f} / {tc["area_0p08"]:.3f} at 0.4 / 0.15 '
          '(production) / 0.08 mm² planform meshes.',
          f'- **Mass.** {mass:.1f} g from the watertight 0.2 mm print mesh; the ranking counted {r["mass"]["ranking_voxel_g"]:.1f} g on its '
          f'{co["ranking_h_mm"]} mm voxel grid and the search {r["mass"]["search_voxel_g"]:.1f} g on its {co["h_mm"]} mm grid.', '']
    # ---- limits
    lim = []
    if mass > 50.0:
        lim.append(f'It weighs {mass:.1f} g, above the 50 g goal.')
    for name, val, tgt in (('solid-tip SF', i['solid_tip_sf'], 1.0), ('grip SF (formula)', i['grip_sf_min'], 1.0),
                           ('grip spine SF (FE)', fe['grip_sf']['spine'], 1.0), ('human-max twist SF', i['hm_twist_sf'], 1.0),
                           ('fatigue SF', i['fat_sf'], 1.0), ('yield-sequence ratio', i['seq_ratio'], 1.2)):
        if val < tgt:
            lim.append(f'Its {name} is {val:.2f}, below the {tgt:g} target.')
    lim.append('It is a computational result: nothing has been printed, heat-treated, sharpened or tested.')
    lim.append(f'The archive was selected on the {co["h_mm"]} mm search grid, which undercounts the mass of some designs (see the grid check); '
               f'only the final ranking is measured at {co["ranking_h_mm"]} mm, so designs the coarse grid scored low were not re-checked.')
    lim.append(f'The cohort is {co["evaluations_per_island"]:,} evaluations per island, a shortened run for this release.')
    L += ['## Limits of this result', ''] + [f'- {x}' for x in lim] + ['']
    # ---- cad
    L += ['## Download the CAD (binary STL, millimetres)', '',
          '| File | Triangles | Size | SHA-256 | Contents | Where |', '|---|---:|---:|---|---|---|']
    for asset in r['release_assets']:
        where = (f'[in git](docs/g3/v{v}/cad/{asset["file"]})' if f'docs/g3/v{v}/cad/{asset["file"]}' in r['cad_in_git']
                 else f'[release v{v}]({REPO}/releases/tag/v{v})')
        L.append(f'| `{asset["file"]}` | {asset["triangles"]:,} | {asset["bytes"] / 1048576:.0f} MB | `{asset["sha256"][:16]}…` | {asset["note"]} | {where} |')
    specs = [p for p in r['cad_in_git'] if p.endswith('.json')]
    L += ['', 'Units are millimetres. The print file is the AM build: the outer skin, the internal TPMS lattice and the powder-removal ports. '
          'The files in git are written by [`ops/g3/export_best_stl.sh`](ops/g3/export_best_stl.sh) from the committed genome row, each at the '
          'finest voxel pitch that keeps it under 50 MiB'
          + (''.join(f'; [`{os.path.basename(p)}`]({p}) lists their dimensions, volume, mass, pitch and watertightness' for p in specs)) + '.', '']
    # ---- reproduce + repo
    L += ['## Reproduce', '', '```bash',
          f'git checkout {pv["git_commit"][:12]}   # decoder {pv["decoder_sha256"][:12]}',
          'cargo build --release --manifest-path crates/g3/Cargo.toml --bin g3-evolve --example pilot_tools --example gripfe',
          f'crates/g3/target/release/g3-evolve --out ~/jarambit_runs/evo_g3/<run>/island01/run --init 64 --evals {co["evaluations_per_island"]} \\',
          f'  --seed 4001 --host 4 --island 1 --commit {pv["git_commit"][:12]} --h {co["h_mm"]} --tier1-gate {co["tier_gates"][0]} --tier2-gate {co["tier_gates"][1]} --log-every 50',
          f'crates/g3/target/release/examples/pilot_tools tier2 <root> <rank.jsonl> 32 tier2_top32.jsonl {co["ranking_h_mm"]} 4   # ranking of record',
          f'crates/g3/target/release/examples/pilot_tools hconv <root> <id,id,...> 0.4,0.3,0.25,0.2,0.15 grid.jsonl 4   # grid check',
          f'crates/g3/target/release/examples/pilot_tools export <root> {wid} <out_dir> 0.2   # watertight envelope + AM build',
          f'crates/g3/target/release/examples/gripfe run <root> {wid} 0.4 fe.jsonl 4             # hex8 voxel FE certification',
          '```', '',
          f'The winner\'s genome row is [`evidence/g3/v{v}/winner_genome.csv`](evidence/g3/v{v}/winner_genome.csv); every island\'s logged history is in '
          f'[`evidence/g3/v{v}/history/`](evidence/g3/v{v}/history/).', '',
          'The v1 command-line pipeline (`cargo run -p jarambit`) is still in the repository; its plate level is opt-in with `JARAMBIT_FEM=1`.', '',
          '## Repository', '',
          f'- [`docs/g3/RELEASE_{v}.md`](docs/g3/RELEASE_{v}.md): what changed in this release and why.',
          '- [`crates/g3/README.md`](crates/g3/README.md): the G3 genome, decoder, objective and their known limitations.',
          '- [`docs/RELEASES.md`](docs/RELEASES.md): every release. [`docs/WHITEPAPER.md`](docs/WHITEPAPER.md): the earlier v1 study, kept as a historical record.', '']
    return '\n'.join(L)


if __name__ == '__main__':
    new = readme()
    if '--check' in sys.argv:
        if rd('README.md') != new:
            print('README.md is stale: run python3 ops/ci/gen_readme.py'); sys.exit(1)
        print('README.md up to date'); sys.exit(0)
    open(os.path.join(ROOT, 'README.md'), 'w', encoding='utf-8').write(new); print('wrote README.md')
