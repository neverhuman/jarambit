#!/usr/bin/env python3
"""Verify committed automatic-wave leaderboards and RESULT.md files against wave_summary.json."""
import hashlib
import json
from pathlib import Path
import statistics

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha256_file(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify_and_summarize():
    summary_path = HERE / 'wave_summary.json'
    summary = json.loads(summary_path.read_text(encoding='utf-8'))
    if summary.get('schema') != 'jarambit.evolution_waves.summary.v1':
        raise ValueError('Unsupported wave summary schema')
    waves = summary['waves']
    for w in waves:
        lb_path = ROOT / w['leaderboard_path']
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
                if sha256_file(ROOT / rel) != w[opt_sha_key]:
                    raise ValueError(f"SHA256 mismatch for {rel}")
        lb = json.loads(raw)
        if lb['assessed_count'] != w['assessed_count']:
            raise ValueError(f"assessed_count mismatch for {w['wave']}")
        if lb['qualified_count'] != w['qualified_count']:
            raise ValueError(f"qualified_count mismatch for {w['wave']}")
        if len(lb['unverified']) != w['unverified_count']:
            raise ValueError(f"unverified_count mismatch for {w['wave']}")
        designs = lb['designs']
        if designs:
            for metric, expected in w['metrics'].items():
                vals = [d[metric] for d in designs]
                recomputed = {
                    'min': min(vals),
                    'median': statistics.median(vals),
                    'max': max(vals),
                }
                if recomputed != expected:
                    raise ValueError(f"Metric mismatch for {w['wave']}.{metric}: {recomputed} != {expected}")
    return summary


if __name__ == '__main__':
    verified = verify_and_summarize()
    print(f"Verified {verified['total_waves']} waves ({verified['total_assessed']} assessed, "
          f"{verified['total_qualified']} qualified, {verified['total_unverified']} unverified)")
