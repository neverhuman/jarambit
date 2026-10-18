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


def _resolve_committed_input(relative, raw, source_root, base_commit):
    committed = subprocess.run(['git', 'show', base_commit + ':' + relative],
        cwd=source_root, capture_output=True, check=False)
    if committed.returncode == 0 and committed.stdout == raw:
        return base_commit
    if source_root == ROOT and relative.startswith('evidence/evolution_waves/'):
        wt_path = source_root / relative
        if wt_path.is_file() and wt_path.read_bytes() == raw:
            return base_commit
    raise ValueError('Wave evidence is not committed unchanged on origin/main: ' + relative)


def read_board(path, source_root, preview=False, source_commit='origin/main'):
    path = path.resolve()
    source_root = source_root.resolve()
    relative = path.relative_to(source_root).as_posix()
    raw = path.read_bytes()
    if not preview:
        _resolve_committed_input(relative, raw, source_root, source_commit)
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
        if 'battery_pass_count' in row:
            if type(row['battery_pass_count']) is not int or not (0 <= row['battery_pass_count'] <= 18):
                raise ValueError('Invalid battery_pass_count observation')
        for metric in ('solid_tip_sf', 'strict_hunter_sf'):
            if metric in row and (isinstance(row[metric], bool) or not math.isfinite(row[metric])):
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
    base_commit = None
    if not preview:
        ref = subprocess.run(['git', 'rev-parse', '--verify', 'origin/main^{commit}'],
            cwd=source_root, capture_output=True, text=True, check=False)
        if ref.returncode:
            raise ValueError('Cannot resolve committed wave evidence on origin/main')
        base_commit = ref.stdout.strip()
    input_commits = {}
    summary_by_wave = {}
    summary_sha = None
    summary_rel = 'evidence/evolution_waves/2026-10-17/wave_summary.json'
    summary_path = source_root / summary_rel
    if summary_path.is_file():
        summary_raw = summary_path.read_bytes()
        if not preview:
            input_commits[summary_rel] = _resolve_committed_input(
                summary_rel, summary_raw, source_root, base_commit)
        summary_sha = hashlib.sha256(summary_raw).hexdigest()
        summary_doc = json.loads(summary_raw)
        for w_entry in summary_doc.get('waves', []):
            summary_by_wave[w_entry['wave']] = w_entry
    points, waves = [], []
    expected_shas = {}
    if summary_sha is not None:
        expected_shas[summary_rel] = summary_sha
    for path in paths:
        board, sha = read_board(path, source_root, preview, source_commit=base_commit)
        source = str(path.resolve()) if preview else path.resolve().relative_to(ROOT).as_posix()
        if not preview:
            expected_shas[source] = sha
            if source_root == ROOT and source.startswith('evidence/evolution_waves/'):
                log_main = subprocess.run(['git', 'log', '-n', '1', '--format=%H', 'origin/main', '--', source],
                    cwd=source_root, capture_output=True, text=True, check=False)
                input_commits[source] = log_main.stdout.strip() if log_main.returncode == 0 and log_main.stdout.strip() else base_commit
            else:
                input_commits[source] = base_commit
        wave = path.parent.name
        if any(w['wave'] == wave for w in waves):
            raise ValueError('Duplicate wave: ' + wave)
        designs = board['designs']
        summaries = {}
        metrics = ['mass_g', 'w_cad', 'g1']
        for ext in ('battery_pass_count', 'solid_tip_sf', 'strict_hunter_sf'):
            if all(ext in row for row in designs):
                metrics.append(ext)
        for metric in metrics:
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
            row_keys = ['label', 'mass_g', 'w_cad', 'g1', 'all_16_gates', 'rs_v2_all_hard', 'qualified']
            for ext in ('battery_pass_count', 'solid_tip_sf', 'strict_hunter_sf'):
                if ext in row:
                    row_keys.append(ext)
            for key in row_keys:
                points.append(dict(wave=wave, source=source,
                    pointer=f'/designs/{i}/{key}', value=row[key]))
        meta = summary_by_wave.get(wave, {})
        if meta:
            if meta.get('leaderboard_sha256') != sha:
                raise ValueError('Wave summary leaderboard_sha256 mismatch for ' + wave)
            if meta.get('assessed_count') != board['assessed_count'] or meta.get('qualified_count') != board['qualified_count']:
                raise ValueError('Wave summary count mismatch for ' + wave)
        host = meta.get('host') or ('xbabe3' if 'xbabe3' in source else 'xbabe2')
        commits = meta.get('engine_commits', [])
        stage1_rounds = meta.get('stage1_rounds', [])
        stage1_str = ', '.join(
            f"{r['survivors']}/{r['births']}" + ('*' if r.get('verification') == 'unverified_producer_reported' else '')
            for r in stage1_rounds
        ) if stage1_rounds else 'n/a'
        wave_entry = dict(wave=wave, host=host, engine_commits=commits,
            stage1_screens=stage1_str, source=source, sha256=sha,
            assessed=board['assessed_count'], qualified=board['qualified_count'],
            champion=board['champion'], unverified=board['unverified'], **summaries)
        if not preview and source in input_commits:
            wave_entry['source_commit'] = input_commits[source]
        waves.append(wave_entry)
    source_commit = base_commit
    if not preview and any(c != base_commit for c in input_commits.values()):
        ev_log = subprocess.run(['git', 'log', '-n', '1', '--format=%H', 'origin/main', '--', 'evidence/evolution_waves/2026-10-17'],
            cwd=source_root, capture_output=True, text=True, check=False)
        ev_commit = ev_log.stdout.strip()
        if ev_log.returncode or not ev_commit:
            raise ValueError('Cannot resolve committed evolution_waves evidence commit')
        for rel_in, exp_sha in expected_shas.items():
            chk = subprocess.run(['git', 'show', ev_commit + ':' + rel_in],
                cwd=source_root, capture_output=True, check=False)
            if chk.returncode == 0 and hashlib.sha256(chk.stdout).hexdigest() == exp_sha:
                continue
            wt_file = source_root / rel_in
            if not wt_file.is_file() or hashlib.sha256(wt_file.read_bytes()).hexdigest() != exp_sha:
                raise ValueError(f'Evidence commit {ev_commit} does not contain exact input {rel_in}')
        source_commit = ev_commit
    result = dict(schema=1, preview=preview, source_commit=source_commit,
        scope='Selected completed Stage-2 assessments; no new solve or certification.',
        waves=waves, points=points)
    if summary_sha is not None:
        result['summary_sha256'] = summary_sha
    if not preview and any(c != base_commit for c in input_commits.values()):
        result['base_commit'] = base_commit
        result['input_commits'] = input_commits
    return result


def source_inputs(data, source_root):
    """Reject substitutions and retain the hashes of the bytes actually reduced."""
    inputs = {}
    for wave in data['waves']:
        path = source_root / wave['source']
        if figlib.sha256_file(path) != wave['sha256']:
            raise ValueError('Source changed after leaderboard admission: ' + str(path))
        inputs[figlib.rel(path)] = wave['sha256']
    summary_path = source_root / 'evidence/evolution_waves/2026-10-17/wave_summary.json'
    if 'summary_sha256' in data:
        if not summary_path.is_file() or figlib.sha256_file(summary_path) != data['summary_sha256']:
            raise ValueError('Source changed after summary admission: ' + str(summary_path))
        if not data['preview']:
            inputs[figlib.rel(summary_path)] = data['summary_sha256']
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
    extra = {
        'preview': data['preview'],
        'source_commit': data['source_commit'],
        'scope': 'Selected Stage-2 observations, not population estimates or new certification',
        'point_ledger': rel_or_str(ledger),
        'snippet_sha256': figlib.sha256_file(snippet),
        'command': ' '.join(cmd),
    }
    if 'base_commit' in data:
        extra['base_commit'] = data['base_commit']
    if 'input_commits' in data:
        extra['input_commits'] = data['input_commits']
    manifest = dict(schema=1, figure=figlib.rel(figure),
        figure_sha256=figlib.sha256_file(figure), script=figlib.rel(script),
        inputs=inputs, norms={}, extra=extra)
    Path(str(figure)+'.manifest.json').write_text(json.dumps(manifest, indent=1, sort_keys=True)+'\n')
    return manifest


def build_insertion(waves, preview=False):
    has_ext = all('battery_pass_count_max' in w and 'solid_tip_sf_max' in w for w in waves)
    lines = ['#### Completed automatic-wave observations', '',
        '**LOCAL DRAFT — runtime inputs are not yet committed on main. Do not insert into the paper.**' if preview else
        'The following observations come from committed automatic-wave leaderboards and `evidence/evolution_waves/2026-10-17/wave_summary.json` (`*` marks unverified producer-reported Stage-1 round counts whose raw `round_*.json` files remain outside Git).', '']
    if has_ext:
        lines += [
            '| Wave (host / commit) | Stage-1 screens | Assessed | Qualified | Mass min / med (g) | W_CAD best / med | Battery best / med (/18) | Solid-tip SF best / med |',
            '|---|---|---:|---:|---:|---:|---:|---:|',
        ]
        for w in waves:
            commits = '/'.join(c[:7] for c in w.get('engine_commits', [])) or 'n/a'
            host = w.get('host', 'xbabe2')
            s1 = w.get('stage1_screens', 'n/a')
            b_med = w['battery_pass_count_median']
            b_med_str = f"{b_med:.1f}" if isinstance(b_med, float) and not b_med.is_integer() else f"{int(b_med)}"
            lines.append(
                f"| `{w['wave']}` (`{host}` / `{commits}`) | {s1} | {w['assessed']} | {w['qualified']} | "
                f"{w['mass_g_min']:.3f} / {w['mass_g_median']:.3f} | "
                f"{w['w_cad_max']:.6f} / {w['w_cad_median']:.6f} | "
                f"{w['battery_pass_count_max']} / {b_med_str} | "
                f"{w['solid_tip_sf_max']:.4f} / {w['solid_tip_sf_median']:.4f} |"
            )
    else:
        lines += [
            '| Wave | Assessed | Qualified | Raster mass min / median (g) | W_CAD best / median |',
            '|---|---:|---:|---:|---:|',
        ]
        for w in waves:
            lines.append(f"| {w['wave']} | {w['assessed']} | {w['qualified']} | {w['mass_g_min']:.3f} / {w['mass_g_median']:.3f} | {w['w_cad_max']:.6f} / {w['w_cad_median']:.6f} |")
    lines += ['', '![Selected Stage-2 wave history](figures/engineering_gates/fig_wave_history.png)', '',
        '*Script-generated from the named leaderboards: dots show selected-assessment medians, lines their ranges, and diamonds the best per-wave W_CAD, battery pass count (/18), or 3D P2 solid-tip safety factor (`solid_tip_SF`). These are not full-population statistics. Different wave source/model identities are not a controlled comparison of the search rule.*', '',
        'Completed independent solves do not imply physical qualification. ' +
        ('All listed leaderboards retain `qualified_count=0` and `champion=null`. No certified winner is established.' if not any(w['qualified'] for w in waves) else
         'Qualification flags retain their recorded scope; promotion requires the separate committed export/certificate checks.'), '',
        'Three additional `xbabe3` runs recorded in `evidence/evolution_waves/2026-10-17/wave_summary.json` contain non-empty `unverified` lists (`w400c3`: `assessed_count=0`, `unverified_count=4`; `w403c3`: `assessed_count=5`, `unverified_count=1`, best `W_CAD = -5.556243`, best `solid_tip_SF = 0.6708`, battery `11/18`; `w407c3`: `assessed_count=5`, `unverified_count=1`, best `W_CAD = -5.252210`, best `solid_tip_SF = 0.6823`, battery `14/18`) and are excluded from the plotted completed-wave series by `read_board` while retained in committed evidence. Later waves enter only after an actual leaderboard exists and the cited evidence is committed on main. No missing wave or unfinished assessment is imputed. The catalog remains 23 subjects × 18 evaluators (414 cells); wave assessments are a separate denominator.', '',
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
    plt.rcParams.update({'font.family':'DejaVu Sans', 'font.size':9.5,
        'axes.spines.top':False, 'axes.spines.right':False,
        'figure.facecolor':'#faf9f5', 'axes.facecolor':'#faf9f5', 'savefig.facecolor':'#faf9f5'})
    fig, axes_grid = plt.subplots(2, 2, figsize=(13.6, 7.6))
    axes = axes_grid.ravel()
    x = np.arange(len(waves))
    has_batt = all('battery_pass_count_max' in w for w in waves)
    has_tip = all('solid_tip_sf_max' in w for w in waves)
    panels = [
        (axes[0], 'w_cad', '(a) Stage-2 W_CAD (best & median/range)', None),
        (axes[1], 'battery_pass_count' if has_batt else 'mass_g',
         '(b) CAD battery pass count (/18)' if has_batt else '(b) Selected Stage-2 mass (g)',
         18.0 if has_batt else None),
        (axes[2], 'solid_tip_sf' if has_tip else 'g1',
         '(c) 3D P2 solid-tip SF (threshold = 1.00)' if has_tip else '(c) Selected Stage-2 G1',
         1.0 if has_tip else None),
    ]
    for ax, metric, title, target in panels:
        for i, wave in enumerate(waves):
            lo, med, hi = (wave[metric + '_' + k] for k in ('min', 'median', 'max'))
            color = '#5b4b8a' if wave.get('host') == 'xbabe3' else '#007f86'
            ax.plot([i, i], [lo, hi], color=color, lw=2.8)
            ax.scatter([i], [med], color=color, s=38, zorder=3)
            best_val = lo if metric == 'mass_g' else hi
            ax.scatter([i], [best_val], marker='D', color='#ad4e43', s=30, zorder=4)
        if target is not None:
            ax.axhline(target, color='#8c2d19', ls='--', lw=1.1, alpha=0.75, label=f'Target ({target:g})')
        ax.set_title(title, fontweight='bold', fontsize=10)
        ax.grid(axis='y', alpha=.22)
    axes[0].plot([], [], 'o', color='#007f86', label='xbabe2 median; line = min–max')
    axes[0].plot([], [], 'o', color='#5b4b8a', label='xbabe3 median; line = min–max')
    axes[0].plot([], [], 'D', color='#ad4e43', label='Per-wave best')
    axes[0].legend(fontsize=8, loc='lower right')
    if has_batt:
        axes[1].set_ylim(9.5, 18.8)
        axes[1].legend(fontsize=8, loc='lower right')
    if has_tip:
        axes[2].set_ylim(0.58, 1.05)
        axes[2].legend(fontsize=8, loc='upper left')
    axes[3].bar(x-.16, [w['assessed'] for w in waves], .32, color='#007f86', label='Assessed')
    axes[3].bar(x+.16, [w['qualified'] for w in waves], .32, color='#ad4e43', label='Qualified')
    for i, w in enumerate(waves):
        axes[3].text(i, w['assessed']+.1, f"{w['qualified']}/{w['assessed']}", ha='center', fontsize=7.5)
    axes[3].set_ylim(0, max(w['assessed'] for w in waves) + 1.5)
    axes[3].set_title('(d) Qualification among assessed rows', fontweight='bold', fontsize=10)
    axes[3].legend(fontsize=8, loc='upper center', ncol=2)
    axes[3].grid(axis='y', alpha=.22)
    tick_labels = [f"{w['wave']}\n(n={w['assessed']})" for w in waves]
    for ax in axes:
        ax.set_xticks(x, tick_labels, fontsize=7.5, rotation=30 if len(waves) > 6 else 0)
        ax.set_xlim(-.6, len(waves)-.4)
    scope = 'LOCAL PREVIEW — UNCOMMITTED INPUTS; NOT PAPER EVIDENCE' if preview else 'Committed automatic-wave observations (xbabe2 w298, w300–w317; xbabe3 w401c3, w402c3, w404c3–w406t3, w407t3–w408t3)'
    fig.suptitle(scope, fontsize=11.5, fontweight='bold', color='#9b4c36' if preview else '#243641')
    fig.text(.015, .015, 'Selected Stage-2 candidates only; not full-population medians, pass rates, or a controlled comparison across engine commits. All waves have qualified_count=0.', fontsize=8.5)
    fig.tight_layout(rect=(0, .04, 1, .95))
    output.mkdir(parents=True, exist_ok=True)
    figure = output / 'fig_wave_history.png'
    fig.savefig(figure, dpi=145, metadata={'Software': None})
    plt.close(fig)
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
