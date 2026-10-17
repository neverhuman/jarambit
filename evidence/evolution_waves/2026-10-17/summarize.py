#!/usr/bin/env python3
"""Verify committed automatic-wave leaderboards, RESULT.md files, and Stage-1 provenance against wave_summary.json."""
import hashlib
import json
import math
from pathlib import Path
import re
import statistics

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
HEX64 = re.compile(r'^[0-9a-f]{64}$')
EXPECTED_UNVERIFIED_STAGE1_DIGEST = 'db60e913d13e456bb8bf3629903c0dbc739c8b59a5d4a63477f92c9611a6be89'


def sha256_file(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_hex64(value, label, seen):
    if not isinstance(value, str) or not HEX64.fullmatch(value) or len(set(value)) < 4:
        raise ValueError(f'Invalid or degenerate SHA-256 digest for {label}: {value!r}')
    if value in seen:
        raise ValueError(f'Duplicate SHA-256 digest for {label}: {value}')
    seen.add(value)


def verify_and_summarize(summary_path=None, root=None):
    summary_path = Path(summary_path) if summary_path is not None else (HERE / 'wave_summary.json')
    root = Path(root) if root is not None else ROOT
    summary = json.loads(summary_path.read_text(encoding='utf-8'))
    if summary.get('schema') != 'jarambit.evolution_waves.summary.v1':
        raise ValueError('Unsupported wave summary schema')
    waves = summary.get('waves')
    if not isinstance(waves, list) or not waves:
        raise ValueError('Expected non-empty waves list')

    seen_stage1_digests = set()
    unverified_rounds = []
    plotted_waves_expected = []
    excluded_waves_expected = []

    for w in waves:
        wave_id = w['wave']
        lb_path = root / w['leaderboard_path']
        raw = lb_path.read_bytes()
        if len(raw) != w['leaderboard_bytes']:
            raise ValueError(f"Size mismatch for {w['leaderboard_path']}")
        if hashlib.sha256(raw).hexdigest() != w['leaderboard_sha256']:
            raise ValueError(f"SHA256 mismatch for {w['leaderboard_path']}")
        for opt_path_key, opt_sha_key in [
            ('leaderboard_md_path', 'leaderboard_md_sha256'),
            ('result_md_path', 'result_md_sha256'),
        ]:
            rel = w.get(opt_path_key)
            if rel is not None:
                if sha256_file(root / rel) != w[opt_sha_key]:
                    raise ValueError(f'SHA256 mismatch for {rel}')
            elif w.get(opt_sha_key) is not None:
                raise ValueError(f'Unexpected {opt_sha_key} when {opt_path_key} is null')
        lb = json.loads(raw)
        if lb['assessed_count'] != w['assessed_count'] or lb['assessed_count'] != len(lb['designs']):
            raise ValueError(f'assessed_count mismatch for {wave_id}')
        if lb['qualified_count'] != w['qualified_count'] or lb['qualified_count'] != sum(bool(d['qualified']) for d in lb['designs']):
            raise ValueError(f'qualified_count mismatch for {wave_id}')
        if lb['champion'] != w['champion']:
            raise ValueError(f'champion mismatch for {wave_id}')
        if len(lb['unverified']) != w['unverified_count'] or lb['unverified'] != w['unverified']:
            raise ValueError(f'unverified mismatch for {wave_id}')
        if w['unverified_count'] == 0 and w['assessed_count'] > 0:
            plotted_waves_expected.append(wave_id)
        else:
            excluded_waves_expected.append(wave_id)
        designs = lb['designs']
        commits = w.get('engine_commits')
        if not isinstance(commits, list) or not commits or any(not re.fullmatch(r'[0-9a-f]{40}', str(c)) for c in commits):
            raise ValueError(f'Invalid engine_commits for {wave_id}')
        if designs:
            for metric, expected in w['metrics'].items():
                vals = [d[metric] for d in designs]
                recomputed = {
                    'min': min(vals),
                    'median': statistics.median(vals),
                    'max': max(vals),
                }
                if recomputed != expected:
                    raise ValueError(f'Metric mismatch for {wave_id}.{metric}: {recomputed} != {expected}')
        elif w['metrics'] != {}:
            raise ValueError(f'Expected empty metrics for zero-assessed wave {wave_id}')

        rounds = w.get('stage1_rounds')
        if not isinstance(rounds, list) or not rounds:
            raise ValueError(f'Missing stage1_rounds for {wave_id}')
        v_count = 0
        u_count = 0
        for idx, r in enumerate(rounds):
            if r.get('round') != idx or type(r.get('seed')) is not int:
                raise ValueError(f'Invalid round/seed metadata for {wave_id} round {idx}')
            check_hex64(r.get('births_sha256'), f'{wave_id}.r{idx}.births_sha256', seen_stage1_digests)
            check_hex64(r.get('screen_sha256'), f'{wave_id}.r{idx}.screen_sha256', seen_stage1_digests)
            births = r.get('births')
            unique_births = r.get('unique_births')
            survivors = r.get('survivors')
            deficit = r.get('best_screen_deficit')
            if type(births) is not int or births <= 0:
                raise ValueError(f'Invalid births count for {wave_id} round {idx}')
            if type(unique_births) is not int or not (0 < unique_births <= births):
                raise ValueError(f'Invalid unique_births count for {wave_id} round {idx}')
            if type(survivors) is not int or not (0 <= survivors <= births):
                raise ValueError(f'Invalid survivors count for {wave_id} round {idx}')
            if isinstance(deficit, bool) or not isinstance(deficit, (int, float)) or not math.isfinite(deficit):
                raise ValueError(f'Invalid best_screen_deficit for {wave_id} round {idx}')
            b_fam = r.get('birth_families', {})
            s_fam = r.get('survivor_families', {})
            if sum(b_fam.values()) != births or any(type(v) is not int or v <= 0 for v in b_fam.values()):
                raise ValueError(f'birth_families sum mismatch for {wave_id} round {idx}')
            if sum(s_fam.values()) != survivors or any(type(v) is not int or v <= 0 for v in s_fam.values()):
                raise ValueError(f'survivor_families sum mismatch for {wave_id} round {idx}')

            ver = r.get('verification')
            if ver == 'committed_json_verified':
                b_rel = r.get('births_path')
                s_rel = r.get('screen_path')
                if not b_rel or not s_rel:
                    raise ValueError(f'Missing committed paths for verified Stage-1 round {wave_id} r{idx}')
                b_raw = (root / b_rel).read_bytes()
                s_raw = (root / s_rel).read_bytes()
                if hashlib.sha256(b_raw).hexdigest() != r['births_sha256']:
                    raise ValueError(f'Committed births JSON hash mismatch for {b_rel}')
                if hashlib.sha256(s_raw).hexdigest() != r['screen_sha256']:
                    raise ValueError(f'Committed screen JSON hash mismatch for {s_rel}')
                b_doc = json.loads(b_raw)
                s_doc = json.loads(s_raw)
                if b_doc['round'] != idx or b_doc['seed'] != r['seed']:
                    raise ValueError(f'Committed births round/seed mismatch for {b_rel}')
                if len(b_doc['genomes']) != births or len({g['row'] for g in b_doc['genomes']}) != unique_births:
                    raise ValueError(f'Committed births count mismatch for {b_rel}')
                if len(s_doc) != survivors or s_doc[0]['screen_deficit'] != deficit:
                    raise ValueError(f'Committed screen count/deficit mismatch for {s_rel}')
                recomputed_bfam = {}
                for g in b_doc['genomes']:
                    recomputed_bfam[g['family']] = recomputed_bfam.get(g['family'], 0) + 1
                recomputed_sfam = {}
                for g in s_doc:
                    recomputed_sfam[g['family']] = recomputed_sfam.get(g['family'], 0) + 1
                if recomputed_bfam != b_fam or recomputed_sfam != s_fam:
                    raise ValueError(f'Committed Stage-1 family distribution mismatch for {wave_id} r{idx}')
                v_count += 1
            elif ver == 'unverified_producer_reported':
                if r.get('births_path') is not None or r.get('screen_path') is not None:
                    raise ValueError(f'Unverified producer Stage-1 round must have null paths: {wave_id} r{idx}')
                u_count += 1
                unverified_rounds.append({'wave': wave_id, 'round': r})
            else:
                raise ValueError(f'Unknown Stage-1 verification status for {wave_id} r{idx}: {ver!r}')

        if w.get('stage1_births_total') != sum(r['births'] for r in rounds):
            raise ValueError(f'stage1_births_total mismatch for {wave_id}')
        if w.get('stage1_survivors_total') != sum(r['survivors'] for r in rounds):
            raise ValueError(f'stage1_survivors_total mismatch for {wave_id}')
        if w.get('stage1_committed_verified_rounds') != v_count:
            raise ValueError(f'stage1_committed_verified_rounds mismatch for {wave_id}')
        if w.get('stage1_unverified_producer_rounds') != u_count:
            raise ValueError(f'stage1_unverified_producer_rounds mismatch for {wave_id}')

    unverified_digest = hashlib.sha256(
        json.dumps(unverified_rounds, sort_keys=True, separators=(',', ':')).encode('utf-8')
    ).hexdigest()
    if summary.get('unverified_producer_stage1_digest_sha256') != unverified_digest or unverified_digest != EXPECTED_UNVERIFIED_STAGE1_DIGEST:
        raise ValueError('Substituted or corrupted unverified_producer_reported Stage-1 metadata')

    if summary.get('plotted_completed_waves') != plotted_waves_expected:
        raise ValueError('plotted_completed_waves mismatch')
    if summary.get('excluded_unverified_waves') != excluded_waves_expected:
        raise ValueError('excluded_unverified_waves mismatch')

    plotted_set = set(plotted_waves_expected)
    plotted_waves = [w for w in waves if w['wave'] in plotted_set]
    all_rounds = [r for w in waves for r in w['stage1_rounds']]
    plotted_rounds = [r for w in plotted_waves for r in w['stage1_rounds']]
    v_all = [r for r in all_rounds if r['verification'] == 'committed_json_verified']
    u_all = [r for r in all_rounds if r['verification'] == 'unverified_producer_reported']
    u_plotted = [r for r in plotted_rounds if r['verification'] == 'unverified_producer_reported']

    expected_totals = {
        'total_waves': len(waves),
        'total_assessed': sum(w['assessed_count'] for w in waves),
        'total_qualified': sum(w['qualified_count'] for w in waves),
        'total_unverified': sum(w['unverified_count'] for w in waves),
        'total_stage1_births': sum(w['stage1_births_total'] for w in waves),
        'total_stage1_survivors': sum(w['stage1_survivors_total'] for w in waves),
        'plotted_waves_count': len(plotted_waves),
        'plotted_assessed': sum(w['assessed_count'] for w in plotted_waves),
        'plotted_qualified': sum(w['qualified_count'] for w in plotted_waves),
        'plotted_unverified': sum(w['unverified_count'] for w in plotted_waves),
        'plotted_stage1_births': sum(w['stage1_births_total'] for w in plotted_waves),
        'plotted_stage1_survivors': sum(w['stage1_survivors_total'] for w in plotted_waves),
        'stage1_committed_verified_rounds': len(v_all),
        'stage1_committed_verified_files': len(v_all) * 2,
        'stage1_committed_verified_births': sum(r['births'] for r in v_all),
        'stage1_committed_verified_survivors': sum(r['survivors'] for r in v_all),
        'stage1_unverified_producer_rounds': len(u_all),
        'stage1_unverified_producer_files': len(u_all) * 2,
        'stage1_unverified_producer_births': sum(r['births'] for r in u_all),
        'stage1_unverified_producer_survivors': sum(r['survivors'] for r in u_all),
        'plotted_stage1_unverified_producer_births': sum(r['births'] for r in u_plotted),
        'plotted_stage1_unverified_producer_survivors': sum(r['survivors'] for r in u_plotted),
    }
    for key, exp in expected_totals.items():
        if summary.get(key) != exp:
            raise ValueError(f'Aggregate mismatch for {key}: {summary.get(key)!r} != {exp!r}')
    return summary


if __name__ == '__main__':
    verified = verify_and_summarize()
    print(f"Verified {verified['total_waves']} waves ({verified['total_assessed']} assessed, "
          f"{verified['total_qualified']} qualified, {verified['total_unverified']} unverified; "
          f"{verified['stage1_committed_verified_files']}/74 Stage-1 files committed-JSON-verified, "
          f"{verified['stage1_unverified_producer_files']}/74 producer-reported)")
