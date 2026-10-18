#!/usr/bin/env python3
"""Generate the data-bearing blocks of README.md (issue #35). Deterministic: no clock, no git, no network.
Every number comes from a committed, CI-validated source:
  docs/VERSION · evidence/rs_v2/w282_ultrarunner/w282_p14_c01_cad_eval.json ·
  evidence/cad_battery/w282_p14_c01.json · docs/wave27_diversity/plates/wave27_D1_index.json ·
  requirements/RS-karambit-v{1,2}.toml · crates/crescent/src/qd.rs + fem.rs constants ·
  docs/v1/science/tests/TEST_CASE_CATALOG.json
Blocks: <!-- GEN:HEAD:BEGIN --> ... <!-- GEN:HEAD:END -->, <!-- GEN:FIGS:BEGIN --> ... <!-- GEN:FIGS:END -->,
        and <!-- GEN:RELEASE:BEGIN --> ... <!-- GEN:RELEASE:END -->.
usage: gen_readme.py          rewrite the blocks in README.md
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


def head(context=None):
    v = rd('docs/VERSION').strip()
    rec_subj, rec_eval, rec_bat = current_record()
    em = rec_eval['engineering_metrics']
    wsc = rec_eval['weighted_cad_score']
    gates = wsc['gates']
    n_pass = sum(1 for ok in gates.values() if ok)
    n_tot = len(gates)
    m = em['mass_g']
    m3d = em['mass_3d_g']
    g1 = rec_bat['global_fitness']['G']
    w_cad = wsc['W_CAD']
    tip = em['solid_tip_sf_nodal_max']
    spg = em['spg_n_per_g']
    lat = em['strict_lat_rob_0p0625_n']
    sf_lat = em['sf_hunter_strict']
    tw_strict = em['twist_tipzone_sf_strict']
    seq_cert = em['seq_ratio_cert']
    t_stock = em['t_stock_mm']
    w27 = next(r for r in js(IDX) if r.get('label') == LABEL)
    g_w27, _ = global_metric.global_G(w27)
    rel = f'{REPO}/releases/tag/v{v}'
    L = [
        badge('version', f'v{v}', 'ff6f00', rel),
        badge('historical record', rec_subj, '2e7d32', 'docs/ENGINEERING_GATES_AND_EVOLUTION.md'),
        badge('mass', f'{m:.3f} g ({m3d:.3f} g 3D)', '00897b', 'docs/WHITEPAPER.md'),
        badge('G1', f'{g1:.6f}', '7b1fa2', 'docs/GLOBAL_METRIC.md'),
        badge('W_CAD', f'+{w_cad:.4f} ({n_pass}/{n_tot} gates)', '1565c0', 'docs/ENGINEERING_GATES_AND_EVOLUTION.md'),
        badge('tip×SPG', f(tip * spg), '0277bd', 'docs/WHITEPAPER.md'),
        badge('status', 'engineering provisional · not promoted', 'c62828', 'docs/PROVISIONAL.md'),
        '',
        f'**Historical 16-gate screen record `{rec_subj}` — {m:.3f} g ({m3d:.3f} g 3D solid) · G1 {g1:.6f} (`COMPLETE`) · '
        f'W_CAD +{w_cad:.4f} ({n_pass}/{n_tot} engineering gates pass) · strict lateral {lat:.1f} N (SF {sf_lat:.3f}) · '
        f'strict twist SF {tw_strict:.3f} · cert seq {seq_cert:.3f} · 3D P2 solid-tip SF {tip:.4f} × SPG {f(spg)} = {f(tip * spg)} · '
        f'Ti-6Al-4V {t_stock:.1f} mm** *(historical Wave-27 reference `{DESIGN}`: {w27["mass_g"]:.2f} g, G1 {g_w27:.4f}, seq {w27["seq_ratio"]:.3f} < 1.20)*',
    ]
    return '\n'.join(L) + '\n\n' + integrity_status()


def figs(context=None):
    v = rd('docs/VERSION').strip()
    rec_subj, rec_eval, rec_bat = current_record()
    em = rec_eval['engineering_metrics']
    wsc = rec_eval['weighted_cad_score']
    g1 = rec_bat['global_fitness']['G']
    rel = f'{REPO}/releases/tag/v{v}'
    rec_wave = rec_subj.split('_')[0]
    L = [
        f'📄 [White paper](docs/WHITEPAPER.md) · 📐 [Engineering gates & evolution paper](docs/ENGINEERING_GATES_AND_EVOLUTION.md) · '
        f'📋 [Work plan](docs/WORKPLAN.md) · 📊 [Live status](STATUS.md) · ⚖️ [Patent draft (not filed)](docs/PATENT_DRAFT.md) · 📦 [Release v{v}]({rel}) · 🗺️ [Technology map](TECHNOLOGY_MAP.md)',
        '',
        f'<img src="{FIG}/fig_readme_mass_vs_wave.png" width="96%" alt="Historical pre-#82 model: selected-candidate mass vs wave w20 through {rec_wave} from committed evidence JSON, with historical screen record {rec_subj} at {em["mass_g"]:.3f} g ({em["mass_3d_g"]:.3f} g 3D) labelled"/>',
        '',
        f'*Historical (pre-#82 model) selected-candidate mass vs. wave (`w20` through `{rec_wave}`, generated by `ops/render/whitepaper_hero_figs.py` and verified by `ops/ci/verify_figures.py`): '
        f'historical 16-gate screen record **`{rec_subj}` at `{em["mass_g"]:.3f} g` (`{em["mass_3d_g"]:.3f} g` 3D solid), `G1 = {g1:.6f}`, `W_CAD = +{wsc["W_CAD"]:.4f}`**.*',
        '',
        f'<img src="{FIG}/fig_web_studio.png" width="96%" alt="Corrected Wave 283 certificate scorecard"/>',
        '',
        f'*Interactive 3D CAD & Gate Scorecard Web Studio (`web/` — Vite + TypeScript + React + Three.js, exported by `./target/release/jarambit export-web-data --out-dir web/public/data`): '
        f'3D STL/STEP inspection, measured 16-gate `W_CAD_v2` breakdown, failed `RS-karambit-v2` scorecard, and 23-subject × 18-evaluator CAD battery matrix. Wave 283 is rejected.*',
    ]
    return '\n'.join(L)


def release(context=None):
    v = rd('docs/VERSION').strip()
    rec_subj, rec_eval, rec_bat = current_record()
    rec_wave = rec_subj.split('_')[0]
    em = rec_eval['engineering_metrics']
    wsc = rec_eval['weighted_cad_score']
    g1 = rec_bat['global_fitness']['G']
    tot_p = tot_f = tot_b = tot_i = 0
    for sid in BATTERY_SUBJECTS:
        b = js(f'evidence/cad_battery/{sid}.json')
        tot_p += sum(1 for c in b['cases'] if c['status'] == 'pass')
        tot_f += sum(1 for c in b['cases'] if c['status'] == 'fail')
        tot_b += sum(1 for c in b['cases'] if c['status'] == 'blocked')
        tot_i += sum(1 for c in b['cases'] if c['status'] == 'incomplete')
    inc_str = f', `{tot_i} incomplete`' if tot_i else ''
    pareto_chain = []
    for subj, rel in CAD_EVAL_PATHS:
        ev = js(rel)
        wem = ev['engineering_metrics']
        if not ev['weighted_cad_score']['all_engineering_gates_pass']:
            continue
        pareto_chain.append(f'`{subj}` (`{wem["mass_g"]:.3f} g` / `{wem["mass_3d_g"]:.3f} g` 3D, `W_CAD = +{ev["weighted_cad_score"]["W_CAD"]:.4f}`)')
    tcat = js('docs/v1/science/tests/TEST_CASE_CATALOG.json')
    n_groups = tcat['case_group_count']
    L = [
        f'## Release v{v} & Documentation',
        '',
        f'- **Historical 16-gate screen record (`{rec_subj}`):** `{em["mass_g"]:.3f} g` 2.5D raster / `{em["mass_3d_g"]:.3f} g` 3D watertight solid, '
        f'locked `G1 = {g1:.6f}` (`COMPLETE`), `W_CAD = +{wsc["W_CAD"]:.4f}` (`16/16` engineering gates pass: strict 0.0625 mm² lateral '
        f'`{em["strict_lat_rob_0p0625_n"]:.1f} N` / `SF = {em["sf_hunter_strict"]:.3f}`, strict 4–15 mm tip-submodel twist `SF = {em["twist_tipzone_sf_strict"]:.3f}`, '
        f'certified failure sequence `seq_ratio = {em["seq_ratio_cert"]:.3f} >= 1.20`, 3D P2 tetrahedral solid-tip `SF = {em["solid_tip_sf_nodal_max"]:.4f}` '
        f'with `{100.0 * em["solid_tip_dolfinx_rel_err"]:.3f}%` `dolfinx` v0.9.0 vs. `skfem` agreement, and `side.a4` Winkler+bore supported `seq = {em["seq_ratio_supported_a4"]:.3f}`).',
        f'- **Historical 16-gate `W_CAD_v2` mass progression (`w274` → `{rec_wave}`):** {" → ".join(pareto_chain)}.',
        f'- **Battery total across {len(BATTERY_SUBJECTS)} subjects × 18 evaluators ({len(BATTERY_SUBJECTS) * 18} cells):** `{tot_p} pass`, `{tot_f} fail`, `{tot_b} blocked`{inc_str}.',
        f'- **Scientific assurance catalog (`T00`–`T19`):** **{n_groups} paired positive/negative control groups ({2 * n_groups} controls)** defined in the committed catalog; run the software controls via `python3 -B ops/ci/science_suite.py`.',
        '- **Full technical papers, tables, and evidence:**',
        f'  - [White paper (`docs/WHITEPAPER.md`)](docs/WHITEPAPER.md) — page-1 summary, `w20`–`{rec_wave}` mass curve, market comparison, `{rec_subj}` 16-gate margin breakdown, and T1→T2 calibration.',
        f'  - [Engineering gates & evolution paper (`docs/ENGINEERING_GATES_AND_EVOLUTION.md`)](docs/ENGINEERING_GATES_AND_EVOLUTION.md) — complete {len(BATTERY_SUBJECTS)}-subject × 18-evaluator CAD battery matrix, Margin-vs-Mass Pareto table, `RS-karambit-v2` gate tables, and 3D CAD/FEM gallery.',
        f'  - [Master work plan (`docs/WORKPLAN.md`)](docs/WORKPLAN.md) · [Live status (`STATUS.md`)](STATUS.md) · [Wave log (`docs/WAVE_LOG.md`)](docs/WAVE_LOG.md) · [Release notes (`docs/RELEASES.md`)](docs/RELEASES.md) · [Global metric `G1` (`docs/GLOBAL_METRIC.md`)](docs/GLOBAL_METRIC.md) · [Provisional record (`docs/PROVISIONAL.md`)](docs/PROVISIONAL.md) · [Technology map (`TECHNOLOGY_MAP.md`)](TECHNOLOGY_MAP.md).',
    ]
    return '\n'.join(L)


def render(text, context=None):
    for name, fn in (('HEAD', head), ('FIGS', figs), ('RELEASE', release)):
        pat = re.compile(rf'(<!-- GEN:{name}:BEGIN -->\n)(?:.*?\n)?(<!-- GEN:{name}:END -->)', re.S)
        if not pat.search(text):
            raise SystemExit(f'gen_readme: README.md lacks the GEN:{name} block')
        text = pat.sub(lambda m: m.group(1) + fn(context) + '\n' + m.group(2), text)
    if context is not None:
        text = rs_verdicts.replace_claim_block(text, context)
    return text


if __name__ == '__main__':
    cur = rd('README.md'); new = render(cur)
    if '--check' in sys.argv:
        if cur != new:
            print('README.md generated blocks are stale: run python3 ops/ci/gen_readme.py'); sys.exit(1)
        print('README.md up to date'); sys.exit(0)
    open(os.path.join(ROOT, 'README.md'), 'w', encoding='utf-8').write(new); print('wrote README.md')
