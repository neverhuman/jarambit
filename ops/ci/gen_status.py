#!/usr/bin/env python3
"""Generate STATUS.md from repo state (PR-0.28.5). Deterministic: no clock, no git calls.
Sources: docs/VERSION, docs/RELEASES.md, the G3 release receipt named by evidence/g3/RECORD (the current design),
docs/v1/jarambit_v1_roadmap.json and the v1 provisional design's D1 index record (requirements verdict via
ops/ci/rs_verdicts.py, G1 via metrics/global_metric.py).
usage: gen_status.py          write STATUS.md
       gen_status.py --check  exit 1 if STATUS.md differs from what would be generated"""
import json, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(ROOT, 'ops', 'ci')); sys.path.insert(0, os.path.join(ROOT, 'metrics'))
sys.path.insert(0, ROOT)
from ops.ci import rs_verdicts  # noqa: E402
import global_metric  # noqa: E402

from gen_readme import BATTERY_SUBJECTS, integrity_status  # noqa: E402

PROVISIONAL = ('docs/wave27_diversity/plates/wave27_D1_index.json', 'g3_tip+00_t60_tt3_D1')


def rd(rel):
    with open(os.path.join(ROOT, rel), encoding='utf-8') as fh:
        return fh.read()


def vkey(v):
    return tuple(int(x) for x in re.match(r'v?(\d+)\.(\d+)\.(\d+)', v).groups())


def latest_release_section(rel):
    matches = list(re.finditer(r'^## (.+)$', rel, re.M))
    for i, m in enumerate(matches):
        title = m.group(1).strip()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(rel)
        body = rel[m.end():end]
        if title.lower() == 'unreleased' and not re.search(r'^- ', body, re.M):
            continue
        return title, re.findall(r'^- (PR-[0-9.]+[a-z0-9]*)', body, re.M)
    return ('(none)', [])


def g4_status(version):
    """The current design: the certified G4 v4.0.0 winner from its release receipt."""
    rv = rd('evidence/g4/RECORD').strip()
    r = json.loads(rd(f'evidence/g4/v{rv}/release.json'))
    wr = r['winner_record']
    obj = wr['objective']
    m = wr['mass']
    ser = wr['serration']
    return [f'## Current release: G4 v{version}', '',
            f'- **Best individual:** `{r["winner_id"]}`, certified winner of the v{rv} multi-seed cohort '
            f'([`evidence/g4/v{rv}/release.json`](evidence/g4/v{rv}/release.json)); '
            f'feasible across all 29 `RS-karambit-v4` hard gates (`V(x) = {obj["violation_sum_sq"]:.4f}`, min margin `+{obj["min_safety_margin"]:.4f}`, computational — not physically built).',
            f'- **Score & mass:** `W_ERGOLIGHT_v2 = {obj["w_ergolight_v2"]:.2f}` (`W_ERGOLIGHT_v1 = {obj["w_ergolight_v1_legacy"]:.2f}`); '
            f'mass **{m["as_finished_total_mass_g"]:.2f} g as-finished** / **{m["as_printed_total_mass_g"]:.2f} g as-printed** '
            f'({100.0 * m["total_hollowness_fraction"]:.1f} % hollow vs {m["envelope_mass_g"]:.2f} g solid envelope).',
            f'- **Blade & structural gates:** both edges ground and serrated ({ser["teeth_checked_in"]}/{ser["teeth_total_in"]} inner and '
            f'{ser["teeth_checked_sp"]}/{ser["teeth_total_sp"]} spine teeth checked, min tooth SF {ser["min_tooth_sf_overall"]:.2f}); '
            f'solid-tip torsion SF {wr["tip"]["tip_torsion_0p80nm_sf"]:.2f} (LCB {obj["sf_solid_tip_torsion_0p80nm_lcb"]:.2f}), '
            f'HM-1 grip governing SF {wr["hm1"]["grip_sf_min_governing"]:.2f} (LCB {obj["sf_grip_squeeze_250n_lcb"]:.2f}), '
            f'Goodman fatigue SF {wr["plate"]["goodman_fatigue_180n_sf"]:.2f} (LCB {obj["sf_goodman_fatigue_180n_lcb"]:.2f}), '
            f'per-case yield sequence {obj["sequence_ledger"]["governing_blade_seq_ratio"]:.2f} lateral / {obj["sequence_ledger"]["spine_seq_ratio"]:.2f} spine (LCB {obj["sf_per_case_sequence_min_lcb"]:.2f} vs 1.20 floor).',
            f'- **Full table, CAD kit and paper:** [`README.md`](README.md), [`cad/`](cad/) and [`docs/g4/paper/g4-v{version}.pdf`](docs/g4/paper/g4-v{version}.pdf).',
            '']


def g3_status():
    """The G3 v3.0.0 cohort of record, preserved byte-for-byte."""
    rv = rd('evidence/g3/RECORD').strip()
    r = json.loads(rd(f'evidence/g3/v{rv}/release.json'))
    a, b = r['blade']['edge_inner'], r['blade']['edge_spine']
    return ['## Previous generation: G3 v3.0.2 (v3.0.0 cohort of record)', '',
            f'- **Best individual:** `{r["design_id"]}`, the best of the v{rv} cohort of record '
            f'([`evidence/g3/v{rv}/release.json`](evidence/g3/v{rv}/release.json)); status: {r["status"]}.',
            f'- **Score:** {r["provenance"]["metric"]} {r["score"]["score_100"]:.2f} '
            f'({r["provenance"]["platform_of_record"]}); **mass** {r["mass"]["mesh_g"]:.1f} g as printed.',
            f'- **Blade:** both edges ground and serrated ({a["teeth"]} and {b["teeth"]} scallops).',
            '- **Archive & paper:** [`cad/v3.0.0/`](cad/v3.0.0/) and [`docs/g3/paper/g3-v3.0.2.pdf`](docs/g3/paper/g3-v3.0.2.pdf).',
            '']


def generate(context=None):
    version = rd('docs/VERSION').strip()
    idx, label = PROVISIONAL
    rec = next(r for r in json.loads(rd(idx)) if r.get('label') == label)
    verdict = rs_verdicts.verdict(rec, rs_verdicts.load_rs())
    g, t = global_metric.global_G(rec)
    g_s = f"{g:.4f}" if g is not None else f"INCOMPLETE ({', '.join(t['missing'])})"
    rel = rd('docs/RELEASES.md')
    sec_title, prs = latest_release_section(rel)
    bat_records = [json.loads(rd(f'evidence/cad_battery/{sid}.json')) for sid in BATTERY_SUBJECTS]
    n_rsv2_pass = sum(
        1 for b in bat_records
        if next((c['status'] for c in b['cases'] if c['id'] == 'gate.rs_v2'), 'fail') == 'pass'
    )
    feasible_wcad = []
    for b in bat_records:
        cmap = {c['id']: c for c in b['cases']}
        mg = cmap['geom.mass'].get('g_inputs', {}).get('mass_g')
        if mg is not None and cmap['plate.seq']['status'] == 'pass' and cmap['solid.tip']['status'] == 'pass':
            feasible_wcad.append((mg, b, cmap))
    best_mg, best_b, best_cmap = min(feasible_wcad, key=lambda x: x[0])
    gf = best_b['global_fitness']
    best_g = gf['G']
    best_seq = best_cmap['plate.seq']['detail']['seq_ratio']
    best_tip = gf['inputs']['solid_tip_sf_nodal_max']
    wsum = json.loads(rd('evidence/evolution_waves/2026-10-17/wave_summary.json'))
    road = json.loads(rd('docs/v1/jarambit_v1_roadmap.json'))
    L = ['# STATUS (generated; do not edit)', '',
         f'Generated by `ops/ci/gen_status.py` from repo state; CI fails if this file is stale (`gen_status.py --check`). Author: Jepson Taylor.', '',
         f'- **Version:** {version} (`docs/VERSION`)',
         f'- **Latest release section:** {sec_title}',
         f'- **PRs in that section:** {", ".join(prs) if prs else "none"}',
         f'- **Engineering provisional:** `{label}` ({idx})',
         f'  - {verdict}',
         f'  - G1 {g_s} · mass {rec.get("mass_g")} g · seq {rec.get("seq_ratio")}',
         f'- **Historical CAD battery & calibrated `W_CAD_v2` search (`evidence/cad_battery/*.json`):** {len(BATTERY_SUBJECTS)} subjects × 18 evaluators ({n_rsv2_pass}/{len(BATTERY_SUBJECTS)} pass all `RS-karambit-v2` hard gates)',
         f'  - Lightest historical screen passing certified `seq >= 1.20` + 3D P2 solid-tip `SF >= 1.0`: `{best_b["subject"]}` · G1 {best_g:.4f} ({gf["status"]}) · mass {best_mg:.3f} g · seq {best_seq:.3f} · solid-tip SF {best_tip:.4f}',
         f'- **Post-#84 automatic evolution ledger (`evidence/evolution_waves/2026-10-17/wave_summary.json`):** {wsum["total_waves"]} waves ({wsum["plotted_waves_count"]} plotted completed waves: `{wsum["plotted_completed_waves"][0]}`, `w300`–`w319`, `w321`–`w322`, `w401c3`, `w402c3`, `w404c3`–`w406t3`, `w407t3`–`w408t3`, `w411c3`–`w411t3`), {wsum["total_stage1_births"]:,} Stage-1 births → {wsum["total_stage1_survivors"]:,} survivors → {wsum["total_assessed"]} Stage-2 assessments (`{wsum["plotted_assessed"]}` plotted, `{wsum["total_unverified"]}` unverified), `{wsum["total_qualified"]}` qualified (`champion = null`)',
         '', '## Roadmap milestones (`docs/v1/jarambit_v1_roadmap.json`)', '',
         '| version | title | state | PRs |', '|---|---|---|---|']
    nxt = None
    for ms in road['milestones']:
        v = ms['version']
        if vkey(v) <= vkey(version):
            state = 'released'
        elif nxt is None:
            nxt = v; state = 'next'
        else:
            state = 'planned'
        L.append(f"| {v} | {ms['title']} | {state} | {len(ms.get('work_items', []))} |")
    L += ['', 'History: `docs/history/` (archived root STATUS.md / GAPS.md snapshots). Release notes: `docs/RELEASES.md`.', '']
    L[4:4] = g4_status(version) + g3_status() + ['## v1 study (historical record)', '', integrity_status(), '']
    output = '\n'.join(L)
    return rs_verdicts.replace_claim_block(output, context) if context is not None else output


def main(context=None):
    out = generate(context)
    path = os.path.join(ROOT, 'STATUS.md')
    if '--check' in sys.argv:
        cur = open(path, encoding='utf-8').read() if os.path.exists(path) else ''
        if cur != out:
            print('STATUS.md is stale: run python3 ops/ci/gen_status.py'); sys.exit(1)
        print('STATUS.md up to date'); return
    with open(path, 'w', encoding='utf-8') as fh:
        fh.write(out)
    print('wrote STATUS.md')


if __name__ == '__main__':
    main()
