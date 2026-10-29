#!/usr/bin/env python3
"""Refresh every leader-design image used by README.md and the G5 paper from ONE manual command.

    python3 tools/refresh_leader_assets.py                     # leader of the newest CURRENT-pipeline (G5) run
    python3 tools/refresh_leader_assets.py --design kv4i1406_g00009_528e4365
    python3 tools/refresh_leader_assets.py --allow-legacy --evidence evidence/legacy_continuous_run_v6.1.0  # old v1 pipeline only
    python3 tools/refresh_leader_assets.py --plan-only         # resolve inputs, render nothing
    python3 tools/refresh_leader_assets.py --skip-render       # docs/paper only (reuse images)

Steps (idempotent; same inputs -> same file set and paths):
  1. Leader (current pipeline, default): newest ~/jarambit_runs/evo_g5*/cert/tier2_h020.jsonl (ranking grid
     h 0.2 mm, schema G5.*), max score_100; falls back to the committed evidence/g5/v<RECORD>/ranking_tier2_h020.jsonl.
     Cross-checked against evidence/g5/v<RECORD>/release.json. Legacy (ec* / v1 W_ULTRALIGHT) leaders are REFUSED
     unless --allow-legacy (v6.1.1 rendered the legacy ec2i089 by mistake).
  2. Geometry: the run's AM-build STL with its internal lattice voids, <run>/prep/cad/out/<id>_AMbuild_mm.stl
     (export it with ops/g5/export_best_stl.sh if absent). Void fraction 1 - V_AMbuild/V_envelope must be >= 5 %
     or the tool stops (a solid mesh means the wrong pipeline; --allow-solid overrides).
  3. Blender (ops/render/blender/leader_views.py, Cycles CPU, nice 19): hero, reverse, side, top,
     ring, grip, blade, tip, tip_edge, cut_full, cut_x, cut_b, xray (open cuts: voids visible) -> docs/assets/leader/<view>.jpg.
  4. Evolution plots: legacy mode only (ops/render/leader/aggregate_run.py -> docs/assets/legacy_run_v6.1.0/*.png).
     In G5 mode the run's plots are the receipt-bound docs/g5/v<RECORD>/img/evolution.* and fitness_*.
  5. docs/README_LATEST_RUN.md leader gallery block, docs/g5/paper/generated/leader.tex,
     ops/ci/gen_readme.py (README.md + cad/README.md), G5 paper PDF via latexmk.
  6. Writes docs/assets/leader/manifest.json and prints before/after SHA-256 of every output.
Nothing is re-scored, re-solved or certified; images are illustrations of the committed geometry.
"""
import argparse
import glob
import hashlib
import json
import os
import shutil
import struct
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VIEWS = ['hero', 'xray', 'reverse', 'side', 'top', 'ring', 'grip', 'blade', 'tip', 'tip_edge', 'cut_full', 'cut_x', 'cut_b']
CAPTIONS = {'hero': 'Oblique hero', 'reverse': 'Reverse oblique', 'side': 'Plan view', 'top': 'Edge-on',
            'ring': 'Ring', 'grip': 'Grip and pockets', 'blade': 'Blade', 'tip': 'The point',
            'tip_edge': 'Edge at the point', 'cut_full': 'Mid-plane cut (z = 0)',
            'cut_x': 'Section across the grip', 'cut_b': 'Section across the blade',
            'xray': 'X-ray cutaway: internal 3D-printed lattice voids'}
ASSETS = ROOT / 'docs/assets/leader'
PLOTS = ROOT / 'docs/assets/legacy_run_v6.1.0'
BEGIN, END = '<!-- leader-gallery:begin (tools/refresh_leader_assets.py) -->', '<!-- leader-gallery:end -->'


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest() if Path(p).is_file() else None


def snapshot():
    files = list(ASSETS.glob('*')) + list(PLOTS.glob('*.png')) + [ROOT / x for x in (
        'README.md', 'cad/README.md', 'docs/README_LATEST_RUN.md', 'docs/g5/paper/generated/leader.tex')]
    files += list((ROOT / 'docs/g5/paper').glob('g5-v*.pdf'))
    return {str(f.relative_to(ROOT)): sha(f) for f in files if f.is_file()}


def blender():
    for c in (os.environ.get('BLENDER'), os.path.expanduser('~/opt/blender-4.5.3-linux-x64/blender'), shutil.which('blender')):
        if c and os.path.isfile(c):
            return c
    sys.exit('no Blender found (set BLENDER)')


def to_binary_stl(src, dst, label):
    data = Path(src).read_bytes()
    if data[:5] != b'solid' or b'facet' not in data[:400]:
        shutil.copyfile(src, dst); return
    tris, cur = [], []
    for line in data.decode().splitlines():
        t = line.split()
        if t and t[0] == 'vertex':
            cur.append(tuple(map(float, t[1:4])))
            if len(cur) == 3:
                tris.append(cur); cur = []
    with open(dst, 'wb') as f:
        f.write(label.encode()[:80].ljust(80, b' ')); f.write(struct.pack('<I', len(tris)))
        for a, b, c in tris:
            f.write(struct.pack('<12fH', 0, 0, 0, *a, *b, *c, 0))


def stl_volume(p):
    import numpy as np
    d = Path(p).read_bytes(); n = struct.unpack('<I', d[80:84])[0]
    a = np.frombuffer(d[84:84 + 50 * n], dtype=np.dtype([('n', '<3f4'), ('v', '<9f4'), ('a', '<u2')]))['v'].reshape(-1, 3, 3).astype(float)
    return float(np.einsum('ij,ij->i', a[:, 0], np.cross(a[:, 1], a[:, 2])).sum() / 6)


def resolve_g5(args):
    rec = (ROOT / 'evidence/g5/RECORD').read_text().strip()
    release = json.loads((ROOT / f'evidence/g5/v{rec}/release.json').read_text())
    runs = sorted(glob.glob(os.path.join(os.path.expanduser(args.runs_root), 'evo_g5*/cert/tier2_h020.jsonl')), key=os.path.getmtime)
    src = Path(runs[-1]) if runs else ROOT / f'evidence/g5/v{rec}/ranking_tier2_h020.jsonl'
    rows = [json.loads(l) for l in src.read_text().splitlines() if l.strip()]
    rows = [r for r in rows if str(r.get('record', {}).get('schema', '')).startswith('G5')]
    if not rows:
        sys.exit(f'no G5 rows in {src}')
    by = {r['design_id']: r for r in rows}
    design = args.design or max(rows, key=lambda r: r['record']['score']['score_100'])['design_id']
    if design not in by:
        sys.exit(f'{design} is not in the current-pipeline ranking {src}')
    r = by[design]['record']
    run = src.parents[1] if runs else None
    print(f'pipeline G5 (schema {r["schema"]}, metric {r["score"]["metric"]}); ranking {src} (mtime {time.ctime(src.stat().st_mtime)})')
    print(f'  record of v{rec}: {release["design_id"]}' + ('' if release['design_id'] == design else '  <-- differs from the chosen leader'))
    stl = run / 'prep/cad/out' / f'{design}_AMbuild_mm.stl' if run else None
    if stl is None or not stl.is_file():
        sys.exit(f'no AM-build STL for {design}; export it with ops/g5/export_best_stl.sh first')
    env = stl.with_name(f'{design}_envelope_mm.stl')
    vf = 1 - stl_volume(stl) / stl_volume(env) if env.is_file() else None
    print(f'  AM-build STL {stl}; void fraction {vf if vf is None else round(vf, 3)}')
    if (vf is None or vf < 0.05) and not args.allow_solid:
        sys.exit('AM-build mesh shows no internal voids (<5 % or no envelope); wrong pipeline? (--allow-solid overrides)')
    meta = dict(pipeline='G5', run_schema=r['schema'], metric=r['score']['metric'], ranking=str(src), record_design=release['design_id'],
                score_100=r['score']['score_100'], mass_g=r['descriptors']['mass_g'], porosity_frac=r['descriptors'].get('porosity_frac'),
                void_fraction=vf)
    return design, round(r['score']['score_100'], 6), stl, None, meta


def resolve(args):
    legacy = args.evidence is not None or (args.design or '').startswith('ec')
    if not legacy:
        return resolve_g5(args)
    if not args.allow_legacy:
        sys.exit('refusing the legacy v1 pipeline (ec* designs / --evidence); pass --allow-legacy if you really mean it')
    args.evidence = args.evidence or 'evidence/legacy_continuous_run_v6.1.0'
    args.runs_root = os.path.join(os.path.expanduser(args.runs_root), 'evo_continuous')
    ev = ROOT / args.evidence
    fit = json.loads((ev / 'leaders/fitness.json').read_text())
    design = args.design or max(fit, key=fit.get).replace('_genome.csv', '')
    stl = ev / 'leaders' / f'{design}.stl'
    found = None
    if not stl.is_file():
        hits = sorted(glob.glob(os.path.join(os.path.expanduser(args.runs_root), '**', f'{design}.stl'), recursive=True))
        if not hits:
            sys.exit(f'no exported STL for {design} under {args.runs_root}; export it first')
        found = hits[0]
    return design, fit.get(design + '_genome.csv'), stl, found, dict(pipeline='legacy')


def gallery_md(design, fitness, mani):
    if mani.get('pipeline') == 'G5':
        return gallery_g5(design, fitness, mani)
    rows, cells = [], [v for v in VIEWS if v != 'hero']
    for i in range(0, len(cells), 2):
        pair = cells[i:i + 2]
        rows.append('<tr>' + ''.join(f'<td width="50%"><img src="docs/assets/leader/{v}.jpg" alt="{CAPTIONS[v]}"><br>{CAPTIONS[v]}.</td>' for v in pair) + '</tr>')
    return '\n'.join([BEGIN, '', f'### Leader renders: `{design}`', '',
        f'Blender renders of the leader\'s exported STL (fitness {fitness}, {mani["triangles"]:,} triangles), regenerated by '
        '`python3 tools/refresh_leader_assets.py` ([how](docs/LEADER_ASSETS.md)). Finishes are illustrative; section faces are the actual mesh cut by a plane.', '',
        f'<img src="docs/assets/leader/hero.jpg" alt="Leader {design}" width="100%">', '',
        '<table>' + ''.join(rows) + '</table>', '', END])


def gallery_g5(design, fitness, mani):
    rows, cells = [], [v for v in VIEWS if v not in ('hero', 'xray')]
    for i in range(0, len(cells), 2):
        pair = cells[i:i + 2]
        rows.append('<tr>' + ''.join(f'<td width="50%"><img src="docs/assets/leader/{v}.jpg" alt="{CAPTIONS[v]}"><br>{CAPTIONS[v]}.</td>' for v in pair) + '</tr>')
    vf = mani.get('void_fraction')
    return '\n'.join([BEGIN, '', f'## G5 leader renders: `{design}` (current pipeline)', '',
        f'Blender renders of the current G5 pipeline\'s leader `{design}` ({mani["metric"]} {fitness:.2f} on the h 0.2 mm ranking grid, '
        f'mass {mani["mass_g"]:.1f} g, lattice porosity {mani["porosity_frac"]:.2f}, internal void fraction of the AM-build mesh {vf:.2f}) '
        f'from its AM-build STL ({mani["triangles"]:,} triangles). Regenerate with `python3 tools/refresh_leader_assets.py` '
        '([how](docs/LEADER_ASSETS.md)). Finishes are illustrative; cuts are open so the printed internal lattice shows.', '',
        f'<img src="docs/assets/leader/xray.jpg" alt="X-ray cutaway of {design}" width="100%">', '',
        f'<img src="docs/assets/leader/hero.jpg" alt="Leader {design}" width="100%">', '',
        '<table>' + ''.join(rows) + '</table>', '', END])


def leader_tex(design, fitness, mani=None):
    if (mani or {}).get('pipeline') == 'G5':
        d = design.replace('_', chr(92) + '_')
        figs = [v for v in VIEWS if v not in ('hero', 'xray')]
        lines = ['% generated by tools/refresh_leader_assets.py -- do not edit',
                 '\\section*{G5 leader renders (current pipeline)}',
                 f'The current G5 pipeline\'s leader \\texttt{{{d}}} (score {fitness:.2f}, mass {mani["mass_g"]:.1f}\\,g, '
                 f'internal void fraction {mani["void_fraction"]:.2f}) rendered in Blender from its AM-build STL. Cuts are open so the '
                 'printed internal lattice is visible; finishes are illustrative.',
                 '\\begin{figure*}[t]\\centering',
                 '\\includegraphics[width=\\textwidth]{../../assets/leader/xray.jpg}',
                 f'\\caption{{X-ray cutaway of \\texttt{{{d}}}: internal 3D-printed lattice voids.}}\\end{{figure*}}',
                 '\\begin{figure*}[t]\\centering',
                 '\\includegraphics[width=\\textwidth]{../../assets/leader/hero.jpg}',
                 f'\\caption{{G5 leader \\texttt{{{d}}}, oblique.}}\\end{{figure*}}',
                 '\\begin{figure*}[t]\\centering']
        for i, v in enumerate(figs):
            lines.append(f'\\begin{{minipage}}{{0.32\\textwidth}}\\centering\\includegraphics[width=\\linewidth]{{../../assets/leader/{v}.jpg}}\\\\{{\\footnotesize {CAPTIONS[v]}}}\\end{{minipage}}' + ('\\\\[4pt]' if i % 3 == 2 else '\\hfill'))
        lines.append('\\caption{All Blender views of the G5 leader.}\\end{figure*}')
        return '\n'.join(lines) + '\n'
    figs = [v for v in VIEWS if v != 'hero']
    lines = ['% generated by tools/refresh_leader_assets.py -- do not edit',
             '\\section*{Latest-run leader (legacy pipeline, not the cohort of record)}',
             f'The latest continuous island run\'s leader \\texttt{{{design.replace("_", chr(92) + "_")}}} (fitness {fitness}) '
             'rendered in Blender from its exported STL. Finishes are illustrative; section faces are the actual mesh cut by a plane.',
             '\\begin{figure*}[t]\\centering',
             '\\includegraphics[width=\\textwidth]{../../assets/leader/hero.jpg}',
             f'\\caption{{Leader \\texttt{{{design.replace("_", chr(92) + "_")}}}, oblique.}}\\end{{figure*}}',
             '\\begin{figure*}[t]\\centering']
    for i, v in enumerate(figs):
        lines.append(f'\\begin{{minipage}}{{0.32\\textwidth}}\\centering\\includegraphics[width=\\linewidth]{{../../assets/leader/{v}.jpg}}\\\\{{\\footnotesize {CAPTIONS[v]}}}\\end{{minipage}}' + ('\\\\[4pt]' if i % 3 == 2 else '\\hfill'))
    lines.append(f'\\caption{{All Blender views of the latest-run leader.}}\\end{{figure*}}')
    return '\n'.join(lines) + '\n'


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument('--design'); p.add_argument('--evidence', default=None, help='legacy evidence dir (needs --allow-legacy)')
    p.add_argument('--runs-root', default='~/jarambit_runs')
    p.add_argument('--allow-legacy', action='store_true'); p.add_argument('--allow-solid', action='store_true')
    p.add_argument('--split', default=None, help='hand-zone/blade finish split as a fraction of length')
    p.add_argument('--run-root', default='~/jarambit_runs/evo_continuous/xbabe2', help='run whose plots to regenerate')
    p.add_argument('--samples', type=int, default=96); p.add_argument('--res', default='1600x1000')
    p.add_argument('--threads', type=int, default=8); p.add_argument('--views', default=','.join(VIEWS))
    p.add_argument('--plan-only', action='store_true'); p.add_argument('--skip-render', action='store_true')
    p.add_argument('--skip-plots', action='store_true'); p.add_argument('--skip-paper', action='store_true')
    a = p.parse_args()
    design, fitness, stl, found, meta = resolve(a)
    g5 = meta.get('pipeline') == 'G5'
    print(f'leader {design} fitness {fitness}\n  stl {stl if not found else found + " -> " + str(stl)}')
    if a.plan_only:
        return
    before = snapshot()
    if found:
        to_binary_stl(found, stl, f'{design} (exported by the run; binary copy)')
    ASSETS.mkdir(parents=True, exist_ok=True)
    if not a.skip_render:
        tmp = ROOT / 'target/leader-render'
        shutil.rmtree(tmp, ignore_errors=True)
        subprocess.run(['nice', '-n', '19', blender(), '-b', '--factory-startup', '-P', str(ROOT / 'ops/render/blender/leader_views.py'),
                        '--', str(stl), str(tmp), '--views', a.views, '--samples', str(a.samples), '--res', a.res,
                        '--threads', str(a.threads), '--open_cuts', '1' if g5 else '0',
                        '--split', a.split or ('0.56' if g5 else '0.47')], check=True)
        from PIL import Image
        for v in a.views.split(','):
            Image.open(tmp / f'{v}.png').convert('RGB').save(ASSETS / f'{v}.jpg', quality=90, optimize=True)
    if g5 and not a.skip_plots:
        print('  plots: G5 run plots are receipt-bound (docs/g5/v<RECORD>/img/evolution.*); left as committed')
    if not g5 and not a.skip_plots:
        rr = Path(os.path.expanduser(a.run_root))
        if (rr / 'islands').is_dir():
            out = ROOT / 'target/leader-plots'; shutil.rmtree(out, ignore_errors=True)
            subprocess.run(['nice', '-n', '19', sys.executable, str(ROOT / 'ops/render/leader/aggregate_run.py'), str(rr), rr.name, str(out)], check=True)
            for png in out.glob('*.png'):
                if (PLOTS / png.name).is_file():
                    shutil.copyfile(png, PLOTS / png.name)
        else:
            print(f'  plots: run root {rr} absent; keeping committed plots')
    tris = struct.unpack('<I', stl.read_bytes()[80:84])[0]
    mani = dict(schema='jarambit.leader_assets.v1', design=design, fitness=fitness, stl=str(stl.relative_to(ROOT)) if stl.is_relative_to(ROOT) else str(stl), **meta,
                stl_sha256=sha(stl), triangles=tris, blender=blender(), samples=a.samples, res=a.res,
                renderer='ops/render/blender/leader_views.py', renderer_sha256=sha(ROOT / 'ops/render/blender/leader_views.py'),
                images={v: dict(path=f'docs/assets/leader/{v}.jpg', sha256=sha(ASSETS / f'{v}.jpg')) for v in VIEWS if (ASSETS / f'{v}.jpg').is_file()})
    (ASSETS / 'manifest.json').write_text(json.dumps(mani, indent=1, sort_keys=True) + '\n')
    md = ROOT / 'docs/README_LATEST_RUN.md'
    text = md.read_text()
    block = gallery_md(design, fitness, mani)
    if BEGIN in text:
        text = text[:text.index(BEGIN)] + block + text[text.index(END) + len(END):]
    else:
        text = text.rstrip('\n') + '\n\n' + block + '\n'
    md.write_text(text)
    (ROOT / 'docs/g5/paper/generated/leader.tex').write_text(leader_tex(design, fitness, mani))
    subprocess.run([sys.executable, str(ROOT / 'ops/ci/gen_readme.py')], cwd=ROOT, check=True)
    if not a.skip_paper:
        pd = ROOT / 'docs/g5/paper'
        ver = (ROOT / 'docs/VERSION').read_text().strip()
        r = subprocess.run(['nice', '-n', '19', 'latexmk', '-lualatex', '-interaction=nonstopmode', '-halt-on-error', '-outdir=build', 'main.tex'], cwd=pd)
        if r.returncode == 0:
            shutil.copyfile(pd / 'build/main.pdf', pd / f'g5-v{ver}.pdf')
        else:
            print('  paper: latexmk failed; see docs/g5/paper/build/main.log')
    after = snapshot()
    print('\nchanged files (sha256 before -> after):')
    for k in sorted(set(before) | set(after)):
        if before.get(k) != after.get(k):
            print(f'  {k}: {(before.get(k) or "new")[:12]} -> {(after.get(k) or "deleted")[:12]}')


if __name__ == '__main__':
    main()
