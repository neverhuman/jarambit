#!/usr/bin/env python3
"""Generate selected Stage-2 wave history, its ledger, and an IV-D insertion.

Publication inputs must match origin/main. --preview permits runtime inputs but
writes only outside docs/ and labels the draft/figure as unpublished.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path
import statistics
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'ops/wave_figures'))
import figlib


def read_board(path, source_root, preview=False, source_commit='origin/main'):
    path = path.resolve()
    relative = path.relative_to(source_root.resolve()).as_posix()
    raw = path.read_bytes()
    if not preview:
        committed = subprocess.run(['git', 'show', source_commit + ':' + relative],
            cwd=source_root, capture_output=True, check=False)
        if committed.returncode or committed.stdout != raw:
            raise ValueError('Wave evidence is not committed unchanged on origin/main: ' + relative)
    board = json.loads(raw)
    if board['schema'] != 'jarambit.auto_evolve.leaderboard.v2':
        raise ValueError('Unsupported leaderboard schema')
    designs = board['designs']
    for key in ('assessed_count', 'qualified_count'):
        if type(board[key]) is not int or board[key] < 0:
            raise ValueError('Expected non-negative JSON integer: ' + key)
    for row in designs:
        for key in ('all_16_gates', 'rs_v2_all_hard', 'qualified'):
            if type(row[key]) is not bool:
                raise ValueError('Expected JSON boolean verdict: ' + key)
    if not designs or board['assessed_count'] != len(designs):
        raise ValueError('Leaderboard does not enumerate its completed assessments')
    if board['unverified']:
        raise ValueError('History of completed assessments cannot include unverified entries')
    if len({d['label'] for d in designs}) != len(designs):
        raise ValueError('Repeated assessment label')
    for row in designs:
        if row['qualified'] is not (row['all_16_gates'] and row['rs_v2_all_hard']):
            raise ValueError('Contradictory qualification flags')
        for metric in ('mass_g', 'w_cad', 'g1'):
            if isinstance(row[metric], bool) or not math.isfinite(row[metric]):
                raise ValueError('Non-finite observation: ' + metric)
    if board['qualified_count'] != sum(d['qualified'] for d in designs):
        raise ValueError('Qualification count mismatch')
    champion = board['champion']
    if (champion is None) != (board['qualified_count'] == 0):
        raise ValueError('Champion/count mismatch')
    if champion is not None and (champion not in designs or not champion['qualified']):
        raise ValueError('Champion is not a qualified assessed row')
    return board, hashlib.sha256(raw).hexdigest()


def collect(paths, source_root, preview=False):
    """Admit and reduce observations without importing any plotting package."""
    source_root = source_root.resolve()
    if not preview and source_root != ROOT:
        raise ValueError('Publish from this worktree so manifest inputs stay repository-relative')
    source_commit = None
    if not preview:
        ref = subprocess.run(['git', 'rev-parse', '--verify', 'origin/main^{commit}'],
            cwd=source_root, capture_output=True, text=True, check=False)
        if ref.returncode:
            raise ValueError('Cannot resolve committed wave evidence on origin/main')
        source_commit = ref.stdout.strip()
    points, waves = [], []
    for path in paths:
        board, sha = read_board(path, source_root, preview, source_commit=source_commit)
        source = str(path.resolve()) if preview else path.resolve().relative_to(ROOT).as_posix()
        wave = path.parent.name
        if any(w['wave'] == wave for w in waves):
            raise ValueError('Duplicate wave: ' + wave)
        designs = board['designs']
        summaries = {}
        for metric in ('mass_g', 'w_cad', 'g1'):
            values = [row[metric] for row in designs]
            for operation, reduce in [('min', min), ('median', statistics.median), ('max', max)]:
                value = reduce(values)
                summaries[metric + '_' + operation] = value
                points.append(dict(wave=wave, source=source,
                    pointers=[f'/designs/{i}/{metric}' for i in range(len(designs))],
                    reduction=operation, value=value, metric=metric))
        for key in ('assessed_count', 'qualified_count', 'champion', 'unverified'):
            points.append(dict(wave=wave, source=source, pointer='/' + key, value=board[key]))
        for i, row in enumerate(designs):
            for key in ('label', 'mass_g', 'w_cad', 'g1', 'all_16_gates', 'rs_v2_all_hard', 'qualified'):
                points.append(dict(wave=wave, source=source,
                    pointer=f'/designs/{i}/{key}', value=row[key]))
        waves.append(dict(wave=wave, source=source, sha256=sha,
            assessed=board['assessed_count'], qualified=board['qualified_count'],
            champion=board['champion'], unverified=board['unverified'], **summaries))
    return dict(schema=1, preview=preview, source_commit=source_commit,
        scope='Selected completed Stage-2 assessments; no new solve or certification.',
        waves=waves, points=points)


def source_inputs(data, source_root):
    """Reject substitutions and retain the hashes of the bytes actually reduced."""
    inputs = {}
    for wave in data['waves']:
        path = source_root / wave['source']
        if figlib.sha256_file(path) != wave['sha256']:
            raise ValueError('Source changed after leaderboard admission: ' + str(path))
        inputs[figlib.rel(path)] = wave['sha256']
    return inputs


def write_history_manifest(figure, ledger, snippet, data, source_root, paths):
    # Unlike re-hashing all inputs at publication, these source entries bind the
    # same bytes as the point ledger, even if a file changes after this check.
    inputs = source_inputs(data, source_root)
    script = Path(__file__).resolve()
    inputs.update({figlib.rel(p):figlib.sha256_file(p) for p in (ledger, script)})
    rel_or_str = str if data['preview'] else figlib.rel
    cmd = (['python3', figlib.rel(script)]
           + ([] if not data['preview'] and source_root.resolve() == ROOT else ['--source-root', rel_or_str(source_root)])
           + ['--output-dir', rel_or_str(figure.parent)]
           + (['--preview'] if data['preview'] else [])
           + [rel_or_str(p) for p in paths])
    manifest = dict(schema=1, figure=figlib.rel(figure),
        figure_sha256=figlib.sha256_file(figure), script=figlib.rel(script),
        inputs=inputs, norms={}, extra={
            'preview':data['preview'], 'source_commit':data['source_commit'],
            'scope':'Selected Stage-2 observations, not population estimates or new certification',
            'point_ledger':rel_or_str(ledger), 'snippet_sha256':figlib.sha256_file(snippet),
            'command':' '.join(cmd)})
    Path(str(figure)+'.manifest.json').write_text(json.dumps(manifest, indent=1, sort_keys=True)+'\n')
    return manifest


def build_insertion(waves, preview=False):
    lines = ['#### Completed automatic-wave observations', '',
        '**LOCAL DRAFT — runtime inputs are not yet committed on main. Do not insert into the paper.**' if preview else
        'The following observations come from committed automatic-wave leaderboards.', '',
        '| Wave | Assessed | Qualified | Raster mass min / median (g) | W_CAD best / median |',
        '|---|---:|---:|---:|---:|']
    for w in waves:
        lines.append(f"| {w['wave']} | {w['assessed']} | {w['qualified']} | {w['mass_g_min']:.3f} / {w['mass_g_median']:.3f} | {w['w_cad_max']:.6f} / {w['w_cad_median']:.6f} |")
    lines += ['', '![Selected Stage-2 wave history](figures/engineering_gates/fig_wave_history.png)', '',
        '*Script-generated from the named leaderboards: dots show selected-assessment medians, lines their ranges, and diamonds the lowest mass or highest W_CAD. These are not full-population statistics. Different wave source/model identities are not a controlled comparison of the search rule.*', '',
        'Completed independent solves do not imply physical qualification. ' +
        ('All listed leaderboards retain `qualified_count=0` and `champion=null`. No certified winner is established.' if not any(w['qualified'] for w in waves) else
         'Qualification flags retain their recorded scope; promotion requires the separate committed export/certificate checks.'), '',
        'Later waves enter only after an actual leaderboard exists and the cited evidence is committed on main. No missing wave or unfinished assessment is imputed. The catalog remains 23 subjects ×18 evaluators (414 cells); wave assessments are a separate denominator.', '',
        'Sources: ' + '; '.join(f"`{w['source']}` (SHA256 `{w['sha256']}`)" for w in waves) + '.', '']
    return '\n'.join(lines)


def generate(paths, source_root, output, preview=False):
    source_root, output = source_root.resolve(), output.resolve()
    if preview and any(output.is_relative_to(p/'docs') for p in (ROOT, source_root)):
        raise ValueError('Runtime previews must stay outside the paper')
    data = collect(paths, source_root, preview)
    source_inputs(data, source_root)
    waves, points = data['waves'], data['points']

    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    import numpy as np
    plt.rcParams.update({'font.family':'DejaVu Sans', 'font.size':10,
        'axes.spines.top':False, 'axes.spines.right':False,
        'figure.facecolor':'#faf9f5', 'axes.facecolor':'#faf9f5', 'savefig.facecolor':'#faf9f5'})
    fig, axes = plt.subplots(1, 3, figsize=(12, 4.4))
    x = np.arange(len(waves))
    for ax, metric, title in [(axes[0], 'mass_g', '(a) Selected Stage-2 mass (g)'),
                              (axes[1], 'w_cad', '(b) Selected Stage-2 W_CAD')]:
        for i, wave in enumerate(waves):
            lo, med, hi = (wave[metric + '_' + k] for k in ('min', 'median', 'max'))
            ax.plot([i, i], [lo, hi], color='#007f86', lw=3)
            ax.scatter([i], [med], color='#007f86', s=42, zorder=3)
            ax.scatter([i], [lo if metric == 'mass_g' else hi], marker='D',
                color='#ad4e43', s=28, zorder=4)
        ax.set_title(title); ax.grid(axis='y', alpha=.2)
    axes[0].plot([], [], 'o', color='#007f86', label='Median; line = min–max')
    axes[0].plot([], [], 'D', color='#ad4e43', label='Lowest mass / highest W_CAD')
    axes[0].legend(fontsize=8)
    axes[2].bar(x-.16, [w['assessed'] for w in waves], .32, color='#007f86', label='Assessed')
    axes[2].bar(x+.16, [w['qualified'] for w in waves], .32, color='#ad4e43', label='Qualified')
    for i, w in enumerate(waves):
        axes[2].text(i, w['assessed']+.08, f"{w['qualified']}/{w['assessed']}", ha='center')
    axes[2].set_title('(c) Qualification among assessed rows'); axes[2].legend(fontsize=8)
    for ax in axes:
        ax.set_xticks(x, [w['wave'] + f"\nn={w['assessed']}" for w in waves]); ax.set_xlim(-.6, len(waves)-.4)
    scope = 'LOCAL PREVIEW — UNCOMMITTED INPUTS; NOT PAPER EVIDENCE' if preview else 'Committed automatic-wave observations'
    fig.suptitle(scope, fontsize=12, color='#9b4c36' if preview else '#243641')
    fig.text(.02, .025, 'Selected Stage-2 candidates only; not full-population medians, pass rates, or a controlled comparison of search versions.', fontsize=9)
    fig.tight_layout(rect=(0, .08, 1, .95))
    output.mkdir(parents=True, exist_ok=True)
    figure = output / 'fig_wave_history.png'; fig.savefig(figure, dpi=145); plt.close(fig)
    ledger = output / 'wave_history_points.json'
    ledger.write_text(json.dumps(data, indent=1, sort_keys=True)+'\n')
    snippet = output / 'iv_d_wave_history.md'; snippet.write_text(build_insertion(waves, preview))
    write_history_manifest(figure, ledger, snippet, data, source_root, paths)
    print(f'{len(waves)} waves; {sum(w["assessed"] for w in waves)} assessments; {len(points)} traced observations; {snippet}')


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('leaderboards', nargs='+', type=Path)
    p.add_argument('--source-root', type=Path, default=ROOT)
    p.add_argument('--output-dir', type=Path, default=ROOT/'target/paper/wave-history')
    p.add_argument('--preview', action='store_true')
    args = p.parse_args()
    paths = [path if path.is_absolute() else args.source_root/path for path in args.leaderboards]
    try:
        generate(paths, args.source_root, args.output_dir, args.preview)
    except (ValueError, KeyError) as error:
        p.exit(1, str(error)+'\n')


if __name__ == '__main__':main()
