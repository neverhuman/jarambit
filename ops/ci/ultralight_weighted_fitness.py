#!/usr/bin/env python3
"""Ultralight Running Knife Global Weighted Fitness (W_ULTRALIGHT_v1).

Implements Jepson Ruling 2026-10-18T17:36Z Addendum:
1. Replaces the 26 hard pass/fail vetoes in selection with a 20-objective weighted
   global fitness (range [0.0, 100.0]) weighted for an ULTRALIGHT RUNNING KNIFE
   (30% Ultralight Mass & Running Carry, 25% Ergonomics/Ring/Collar Comfort,
   15% Cutting Edge/Point/B-Rep, 30% Structural/Safety non-veto normalized objectives).
2. No hard failures in selection: every candidate receives a continuous score in
   [0.0, 100.0] while logging every raw per-gate metric, its legacy threshold, and
   its signed margin/deficit across all 16 W_CAD_v2 gates (G16-01..16) and all 10
   RS-karambit-v2 hard-gate groups (RSV2-01..10).
3. Monotone non-decreasing Hall-of-Fame fitness curve across rounds/generations,
   plotted per run (fitness_curve.png / fitness_curve.json) with optional per-gen
   S3 sync of best-so-far design + 26-gate breakdown.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
from typing import Any


ULTRALIGHT_WEIGHTS_V1: dict[str, float] = {
    # A. Ultralight Mass & Running Carry (30.0%)
    'ultralight_mass': 22.0,
    'holder_and_compactness': 8.0,
    # B. Ergonomics, Ring Retention & Comfort (25.0%)
    'karambit_ring_and_claw': 7.0,
    'c6_aesthetic_and_ergo': 9.0,
    'bore_collar_comfort': 9.0,
    # C. Cutting Edge, Point & B-Rep Integrity (15.0%)
    'cad_watertight_brep': 5.0,
    'gate_s_sharpness': 5.0,
    'gate_s_point_profile': 5.0,
    # D. Structural & Safety Objectives — Non-Veto (30.0%)
    'hiker_lateral_180n': 3.5,
    'hunter_lateral_260n': 2.5,
    'strict_hunter_260n': 2.0,
    'ring_pull_1500n': 2.5,
    'twist_body_1p82nm': 2.5,
    'twist_tip_plate_1p82nm': 2.5,
    'solid_tip_3d_p2_and_xcheck': 3.0,
    'side_a4_buckling': 2.0,
    'failure_sequence_1p2': 3.5,
    'plate_gci_3level': 2.0,
    'human_max_overload': 2.0,
    'goodman_fatigue_l7': 2.0,
}


def _clamp01(x: float) -> float:
    return min(max(float(x), 0.0), 1.0)


def compute_ultralight_weighted_fitness(
    cad_eval_doc: dict[str, Any],
    rs_doc: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Compute W_ULTRALIGHT_v1 ([0.0, 100.0]) and 26-gate legacy threshold breakdown."""
    em = cad_eval_doc.get('engineering_metrics') or {}
    wc = cad_eval_doc.get('weighted_cad_score') or {}
    gates_16 = wc.get('gates') or {}
    cad = cad_eval_doc.get('cad') or {}
    ka = cad_eval_doc.get('karambit_archetype') or {}
    exp = cad_eval_doc.get('export_measurements') or (rs_doc or {}).get('export_measurements') or {}
    rs_gates = cad_eval_doc.get('rs_v2_gates') or (rs_doc or {}).get('gates') or {}

    mass_3d_g = float(em.get('mass_3d_g', em.get('mass_g', 95.0)))
    lat_cert_n = float(em.get('lat_rob_cert_n', 0.0))
    strict_lat_n = float(em.get('strict_lat_rob_0p0625_n', 0.0))
    ring_cert_n = float(em.get('ring_rob_cert_n', 0.0))
    tw_body_sf = float(em.get('twist_body_sf', 1.0 if gates_16.get('twist_body_1p82nm') else 0.0))
    tw_tip_sf = float(em.get('twist_tip_sf', 0.0))
    tw_strict_sf = float(em.get('twist_tipzone_sf_strict', 0.0))
    solid_tip_sf = float(em.get('solid_tip_sf_nodal_max', 0.0))
    solid_rel_err = float(em.get('solid_tip_dolfinx_rel_err', 1.0))
    xcheck_ok = bool(gates_16.get('dolfinx_cross_check_2pct', solid_rel_err <= 0.02))
    lambda_a4 = float(em.get('buckling_eigenvalue_a4', 0.0))
    seq_cert = float(em.get('seq_ratio_cert', 0.0))
    fat_sf = float(em.get('fat_sf_l7', 0.0))
    ring_bearing_w = float(em.get('ring_bearing_w_mm', 12.0))

    bore = exp.get('bore_radius') or {}
    ring_edge_r = float(bore.get('min_radius_mm', 1.5 if rs_gates.get('human_max') == 'pass' else 1.125))
    sharp = exp.get('sharpness') or {}
    apex_mm = float(sharp.get('apex_max_mm', 0.18 if rs_gates.get('gate_s_sharp_law') == 'pass' else 0.25))
    half_angle_deg = float(sharp.get('half_angle_max_deg', 20.0 if rs_gates.get('gate_s_sharp_law') == 'pass' else 28.0))
    pt = exp.get('point_profile') or {}
    t_025 = float(pt.get('t_0p25_mm', 0.25 if rs_gates.get('gate_s_tip_point_brep') == 'pass' else 0.45))
    t_100 = float(pt.get('t_1p0_mm', 0.50 if rs_gates.get('gate_s_tip_point_brep') == 'pass' else 0.85))
    t_200 = float(pt.get('t_2p0_mm', 0.85 if rs_gates.get('gate_s_tip_point_brep') == 'pass' else 1.40))
    pt_mono = bool(pt.get('monotone', rs_gates.get('gate_s_tip_point_brep') == 'pass'))

    hm_doc = (rs_doc or {}).get('human_max') or {}
    hm_twist_sf = float(hm_doc.get('hm_twist_sf', 1.0 if rs_gates.get('human_max') == 'pass' else 0.10))
    hm_spine_sf = float(hm_doc.get('hm_spine_sf', 1.0 if rs_gates.get('human_max') == 'pass' else 0.35))
    hm_ringback_sf = float(hm_doc.get('hm_ringback_sf', 1.5))

    # Normalized objective scores s_i in [0.0, 1.0]
    s_scores: dict[str, float] = {
        'ultralight_mass': _clamp01((130.0 - mass_3d_g) / (130.0 - 45.0)),
        'holder_and_compactness': 1.0 if gates_16.get('geometric_and_holder_gates', True) else 0.5,
        'karambit_ring_and_claw': 1.0 if gates_16.get('karambit_ring_and_curved_claw', bool(ka.get('karambit_valid', True))) else 0.5,
        'c6_aesthetic_and_ergo': 1.0 if gates_16.get('c6_aesthetic_floor', True) else 0.5,
        'bore_collar_comfort': 0.5 * _clamp01(ring_bearing_w / 10.0) + 0.5 * _clamp01(ring_edge_r / 1.5),
        'cad_watertight_brep': 1.0 if gates_16.get('cad_watertight_and_single_solid', bool(cad.get('cad_valid', True))) else 0.0,
        'gate_s_sharpness': 0.5 * _clamp01(0.20 / max(apex_mm, 1e-3)) + 0.5 * _clamp01(25.0 / max(half_angle_deg, 1.0)),
        'gate_s_point_profile': 0.25 * (
            _clamp01(0.30 / max(t_025, 1e-3))
            + _clamp01(0.60 / max(t_100, 1e-3))
            + _clamp01(1.00 / max(t_200, 1e-3))
            + (1.0 if pt_mono else 0.0)
        ),
        'hiker_lateral_180n': _clamp01(lat_cert_n / 180.0),
        'hunter_lateral_260n': _clamp01(lat_cert_n / 260.0),
        'strict_hunter_260n': _clamp01(strict_lat_n / 260.0),
        'ring_pull_1500n': _clamp01(ring_cert_n / 1500.0),
        'twist_body_1p82nm': _clamp01(tw_body_sf / 1.0),
        'twist_tip_plate_1p82nm': 0.5 * _clamp01(tw_tip_sf / 1.0) + 0.5 * _clamp01(tw_strict_sf / 1.0),
        'solid_tip_3d_p2_and_xcheck': _clamp01(solid_tip_sf / 1.0) * (0.8 + 0.2 * (1.0 if xcheck_ok else 0.0)),
        'side_a4_buckling': _clamp01(max(0.0, lambda_a4) / 1.0),
        'failure_sequence_1p2': _clamp01(seq_cert / 1.20),
        'plate_gci_3level': 1.0 if gates_16.get('plate_gci_valid_3level', True) else 0.5,
        'human_max_overload': (
            _clamp01(hm_twist_sf / 1.0)
            + _clamp01(hm_spine_sf / 1.0)
            + _clamp01(hm_ringback_sf / 1.0)
        ) / 3.0,
        'goodman_fatigue_l7': _clamp01(fat_sf / 1.0),
    }

    objectives_out: dict[str, Any] = {}
    total_score = 0.0
    for key, weight in ULTRALIGHT_WEIGHTS_V1.items():
        s_val = _clamp01(s_scores[key])
        pts = weight * s_val
        total_score += pts
        objectives_out[key] = {
            'weight_pct': weight,
            'normalized_score': round(s_val, 6),
            'weighted_points': round(pts, 6),
        }

    legacy_26 = {
        'G16-01:cad_watertight_and_single_solid': {'raw': bool(gates_16.get('cad_watertight_and_single_solid', False)), 'legacy_threshold': True, 'legacy_pass': bool(gates_16.get('cad_watertight_and_single_solid', False)), 'deficit': 0.0 if gates_16.get('cad_watertight_and_single_solid', False) else 1.0},
        'G16-02:geometric_and_holder_gates': {'raw': bool(gates_16.get('geometric_and_holder_gates', False)), 'legacy_threshold': True, 'legacy_pass': bool(gates_16.get('geometric_and_holder_gates', False)), 'deficit': 0.0 if gates_16.get('geometric_and_holder_gates', False) else 1.0},
        'G16-03:karambit_ring_and_curved_claw': {'raw': bool(gates_16.get('karambit_ring_and_curved_claw', False)), 'legacy_threshold': True, 'legacy_pass': bool(gates_16.get('karambit_ring_and_curved_claw', False)), 'deficit': 0.0 if gates_16.get('karambit_ring_and_curved_claw', False) else 1.0},
        'G16-04:c6_aesthetic_floor': {'raw': bool(gates_16.get('c6_aesthetic_floor', False)), 'legacy_threshold': True, 'legacy_pass': bool(gates_16.get('c6_aesthetic_floor', False)), 'deficit': 0.0 if gates_16.get('c6_aesthetic_floor', False) else 1.0},
        'G16-05:cert_hiker_180n': {'raw': round(lat_cert_n, 4), 'legacy_threshold': 180.0, 'legacy_pass': lat_cert_n >= 180.0, 'deficit': round(max(0.0, 180.0 - lat_cert_n), 4)},
        'G16-06:cert_hunter_260n': {'raw': round(lat_cert_n, 4), 'legacy_threshold': 260.0, 'legacy_pass': lat_cert_n >= 260.0, 'deficit': round(max(0.0, 260.0 - lat_cert_n), 4)},
        'G16-07:strict_hunter_260n': {'raw': round(strict_lat_n, 4), 'legacy_threshold': 260.0, 'legacy_pass': strict_lat_n >= 260.0, 'deficit': round(max(0.0, 260.0 - strict_lat_n), 4)},
        'G16-08:cert_ring_pull_1500n': {'raw': round(ring_cert_n, 4), 'legacy_threshold': 1500.0, 'legacy_pass': ring_cert_n >= 1500.0, 'deficit': round(max(0.0, 1500.0 - ring_cert_n), 4)},
        'G16-09:twist_body_1p82nm': {'raw': round(tw_body_sf, 4), 'legacy_threshold': 1.0, 'legacy_pass': tw_body_sf >= 1.0, 'deficit': round(max(0.0, 1.0 - tw_body_sf), 4)},
        'G16-10:twist_tip_only_1p82nm': {'raw': round(tw_tip_sf, 4), 'legacy_threshold': 1.0, 'legacy_pass': tw_tip_sf >= 1.0, 'deficit': round(max(0.0, 1.0 - tw_tip_sf), 4)},
        'G16-11:twist_tip_submodel_strict_1p82nm': {'raw': round(tw_strict_sf, 4), 'legacy_threshold': 1.0, 'legacy_pass': tw_strict_sf >= 1.0, 'deficit': round(max(0.0, 1.0 - tw_strict_sf), 4)},
        'G16-12:solid_tip_3d_p2_1p82nm': {'raw': round(solid_tip_sf, 4), 'legacy_threshold': 1.0, 'legacy_pass': solid_tip_sf >= 1.0, 'deficit': round(max(0.0, 1.0 - solid_tip_sf), 4)},
        'G16-13:side_a4_buckling_and_support': {'raw': round(lambda_a4, 4), 'legacy_threshold': 1.0, 'legacy_pass': lambda_a4 >= 1.0, 'deficit': round(max(0.0, 1.0 - lambda_a4), 4)},
        'G16-14:failure_sequence_1p2': {'raw': round(seq_cert, 4), 'legacy_threshold': 1.20, 'legacy_pass': seq_cert >= 1.20, 'deficit': round(max(0.0, 1.20 - seq_cert), 4)},
        'G16-15:plate_gci_valid_3level': {'raw': bool(gates_16.get('plate_gci_valid_3level', False)), 'legacy_threshold': True, 'legacy_pass': bool(gates_16.get('plate_gci_valid_3level', False)), 'deficit': 0.0 if gates_16.get('plate_gci_valid_3level', False) else 1.0},
        'G16-16:dolfinx_cross_check_2pct': {'raw': round(solid_rel_err, 6), 'legacy_threshold': 0.02, 'legacy_pass': xcheck_ok, 'deficit': round(max(0.0, solid_rel_err - 0.02), 6)},
        'RSV2-01:aesthetic_and_geometric': {'raw': rs_gates.get('aesthetic_and_geometric', 'unverified'), 'legacy_threshold': 'pass', 'legacy_pass': rs_gates.get('aesthetic_and_geometric', 'unverified') == 'pass', 'deficit': 0.0 if rs_gates.get('aesthetic_and_geometric', 'unverified') == 'pass' else 1.0},
        'RSV2-02:floors_828_joint': {'raw': round(lat_cert_n, 4), 'legacy_threshold': 260.0, 'legacy_pass': lat_cert_n >= 260.0 and ring_cert_n >= 1500.0, 'deficit': round(max(0.0, 260.0 - lat_cert_n), 4)},
        'RSV2-03:twist_floor_body_and_tipsub_strict': {'raw': round(tw_strict_sf, 4), 'legacy_threshold': 1.0, 'legacy_pass': tw_body_sf >= 1.0 and tw_strict_sf >= 1.0, 'deficit': round(max(0.0, 1.0 - tw_strict_sf), 4)},
        'RSV2-04:solid_tip_sf': {'raw': round(solid_tip_sf, 4), 'legacy_threshold': 1.0, 'legacy_pass': solid_tip_sf >= 1.0 and xcheck_ok, 'deficit': round(max(0.0, 1.0 - solid_tip_sf), 4)},
        'RSV2-05:strict_hunter_lateral': {'raw': round(strict_lat_n, 4), 'legacy_threshold': 260.0, 'legacy_pass': strict_lat_n >= 260.0, 'deficit': round(max(0.0, 260.0 - strict_lat_n), 4)},
        'RSV2-06:seq_hard': {'raw': round(seq_cert, 4), 'legacy_threshold': 1.20, 'legacy_pass': seq_cert >= 1.20, 'deficit': round(max(0.0, 1.20 - seq_cert), 4)},
        'RSV2-07:human_max': {'raw': {'hm_twist_sf': round(hm_twist_sf, 4), 'hm_spine_sf': round(hm_spine_sf, 4), 'ring_edge_r_mm': round(ring_edge_r, 4)}, 'legacy_threshold': {'hm_twist_sf': 1.0, 'hm_spine_sf': 1.0, 'ring_edge_r_mm': 1.5}, 'legacy_pass': hm_twist_sf >= 1.0 and hm_spine_sf >= 1.0 and ring_edge_r >= 1.5, 'deficit': round(max(0.0, 1.0 - min(hm_twist_sf, hm_spine_sf)), 4)},
        'RSV2-08:fatigue': {'raw': round(fat_sf, 4), 'legacy_threshold': 1.0, 'legacy_pass': fat_sf >= 1.0, 'deficit': round(max(0.0, 1.0 - fat_sf), 4)},
        'RSV2-09:gate_s_sharp_law': {'raw': {'apex_max_mm': round(apex_mm, 4), 'half_angle_max_deg': round(half_angle_deg, 4)}, 'legacy_threshold': {'apex_max_mm': 0.20, 'half_angle_max_deg': 25.0}, 'legacy_pass': apex_mm <= 0.20 and half_angle_deg <= 25.0, 'deficit': round(max(0.0, apex_mm - 0.20) + max(0.0, half_angle_deg - 25.0), 4)},
        'RSV2-10:gate_s_tip_point_brep': {'raw': {'t_0p25_mm': round(t_025, 4), 't_1p0_mm': round(t_100, 4), 't_2p0_mm': round(t_200, 4), 'monotone': pt_mono}, 'legacy_threshold': {'t_0p25_mm': 0.30, 't_1p0_mm': 0.60, 't_2p0_mm': 1.00, 'monotone': True}, 'legacy_pass': t_025 <= 0.30 and t_100 <= 0.60 and t_200 <= 1.00 and pt_mono, 'deficit': round(max(0.0, t_200 - 1.00), 4)},
    }

    failed_legacy = [k for k, v in legacy_26.items() if not v['legacy_pass']]
    score_100 = round(total_score, 6)
    return {
        'schema': 'jarambit.ultralight_weighted_fitness.v1',
        'metric': 'W_ULTRALIGHT_v1',
        'veto_free_selection': True,
        'score_100': score_100,
        'global_weighted_fitness': score_100,
        'weight_sum_pct': round(sum(ULTRALIGHT_WEIGHTS_V1.values()), 6),
        'all_26_legacy_hard_gates_pass': len(failed_legacy) == 0,
        'legacy_hard_gates_passed_count': 26 - len(failed_legacy),
        'legacy_hard_gates_failed': failed_legacy,
        'objectives': objectives_out,
        'legacy_26_gate_breakdown': legacy_26,
    }


def build_monotone_fitness_curve(
    wave_dir: Path,
    s3_prefix: str | None = None,
) -> dict[str, Any]:
    """Build monotone non-decreasing Hall-of-Fame global fitness curve across rounds/generations."""
    round_files = sorted(wave_dir.glob('round_*_screen.json'))
    generations: list[dict[str, Any]] = []
    best_so_far_score = -1.0
    best_so_far_entry: dict[str, Any] | None = None

    # Index any Stage-2 evaluated candidates by label so Stage-2 26-gate breakdowns are included.
    stage2_by_label: dict[str, dict[str, Any]] = {}
    for eval_file in sorted(wave_dir.glob('by-content/*/*/*_cad_eval.json')):
        try:
            eval_doc = json.loads(eval_file.read_text(encoding='utf-8'))
            subj = str(eval_doc.get('subject') or '')
            if subj:
                rs_file = eval_file.with_name(f'{subj}_rs.json')
                rs_doc = json.loads(rs_file.read_text(encoding='utf-8')) if rs_file.is_file() else None
                uwf = eval_doc.get('ultralight_weighted_fitness') or compute_ultralight_weighted_fitness(eval_doc, rs_doc)
                stage2_by_label[subj] = {
                    'label': subj,
                    'genome_sha256': eval_doc.get('genome_sha256'),
                    'certificate_identity': eval_doc.get('certificate_identity'),
                    'mass_g': (eval_doc.get('engineering_metrics') or {}).get('mass_g'),
                    'mass_3d_g': (eval_doc.get('engineering_metrics') or {}).get('mass_3d_g'),
                    'ultralight_weighted_fitness': float(uwf['global_weighted_fitness']),
                    'ultralight_fitness_breakdown': uwf,
                }
        except Exception:
            continue

    for idx, rf in enumerate(round_files):
        rows = json.loads(rf.read_text(encoding='utf-8'))
        gen_best_score = -1.0
        gen_best_row: dict[str, Any] | None = None
        for r in rows:
            lbl = str(r.get('label') or '')
            s2 = stage2_by_label.get(lbl)
            if s2 is not None:
                score = float(s2['ultralight_weighted_fitness'])
                cand_row = dict(r, **s2)
            else:
                score = float(r.get('ultralight_weighted_fitness', 0.0))
                if score == 0.0 and 'mass_g' in r:
                    # Recompute stage-1 proxy score if not yet populated
                    m = float(r['mass_g'])
                    seq = float(r.get('seq_ratio', 1.2))
                    fat = float(r.get('fat_sf', 0.5))
                    bw = float(r.get('ring_bearing_w_mm', 12.0))
                    deficit = float(r.get('screen_deficit', 1.0))
                    s_m = _clamp01((130.0 - m) / (130.0 - 45.0))
                    s_seq = _clamp01(seq / 1.20)
                    s_fat = _clamp01(fat / 1.0)
                    s_bw = _clamp01(bw / 10.0)
                    s_struct = _clamp01(1.0 / (1.0 + 0.35 * deficit))
                    score = round(22.0 * s_m + 31.0 + 9.0 * s_bw + 3.5 * s_seq + 2.0 * s_fat + 32.5 * s_struct, 6)
                cand_row = dict(r, ultralight_weighted_fitness=score)
            if score > gen_best_score:
                gen_best_score = score
                gen_best_row = cand_row
        if gen_best_score > best_so_far_score and gen_best_row is not None:
            best_so_far_score = gen_best_score
            best_so_far_entry = gen_best_row
        generations.append({
            'generation': idx,
            'round_file': rf.name,
            'round_best_fitness': round(max(gen_best_score, 0.0), 6),
            'hall_of_fame_best_so_far': round(max(best_so_far_score, 0.0), 6),
            'best_so_far_label': best_so_far_entry['label'] if best_so_far_entry else None,
            'best_so_far_mass_g': best_so_far_entry.get('mass_g') if best_so_far_entry else None,
        })

    best_stage2 = max(
        stage2_by_label.values(),
        key=lambda x: float(x.get('ultralight_weighted_fitness', 0.0)),
        default=None,
    )
    best_26_path = wave_dir / 'best_so_far_26_gates.json'
    best_26_doc = {
        'schema': 'jarambit.ultralight_best_so_far.v1',
        'metric': 'W_ULTRALIGHT_v1',
        'generations_completed': len(generations),
        'hall_of_fame_best_so_far': best_so_far_entry,
        'best_stage2_26_gate_design': best_stage2,
    }
    best_26_path.write_text(json.dumps(best_26_doc, indent=2, sort_keys=True) + '\n', encoding='utf-8')

    curve_doc = {
        'schema': 'jarambit.ultralight_fitness_curve.v1',
        'metric': 'W_ULTRALIGHT_v1',
        'monotone_non_decreasing': all(
            b['hall_of_fame_best_so_far'] >= a['hall_of_fame_best_so_far'] - 1e-12
            for a, b in zip(generations, generations[1:])
        ),
        'generations': generations,
        'best_so_far': best_so_far_entry,
        'best_stage2_26_gate_design': best_stage2,
    }
    curve_path = wave_dir / 'fitness_curve.json'
    curve_path.write_text(json.dumps(curve_doc, indent=2, sort_keys=True) + '\n', encoding='utf-8')

    # Render PNG curve if matplotlib is available
    png_path = wave_dir / 'fitness_curve.png'
    try:
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt

        xs = [g['generation'] + 1 for g in generations] or [1]
        ys_hof = [g['hall_of_fame_best_so_far'] for g in generations] or [0.0]
        ys_rnd = [g['round_best_fitness'] for g in generations] or [0.0]
        fig, ax = plt.subplots(figsize=(7.5, 4.2), dpi=160)
        ax.plot(xs, ys_hof, marker='o', linewidth=2.2, label='Hall-of-Fame Best-So-Far (Monotone)')
        ax.plot(xs, ys_rnd, marker='s', linestyle='--', linewidth=1.4, label='Generation Best')
        ax.set_xlabel('Generation / Round')
        ax.set_ylabel('W_ULTRALIGHT_v1 Global Fitness [0-100]')
        ax.set_title('Jarambit Ultralight Running Knife — Monotone Global Fitness Curve')
        ax.grid(True, alpha=0.3)
        ax.legend(loc='lower right')
        fig.tight_layout()
        fig.savefig(png_path)
        plt.close(fig)
    except Exception:
        pass

    prefix = s3_prefix or os.environ.get('JARAMBIT_S3_PREFIX')
    if prefix:
        prefix = prefix.rstrip('/')
        aws_env = dict(os.environ)
        aws_env.setdefault('AWS_PROFILE', 'veox-jepson')
        subprocess.run(['aws', 's3', 'cp', str(curve_path), f'{prefix}/fitness_curve.json'], env=aws_env, check=False)
        subprocess.run(['aws', 's3', 'cp', str(best_26_path), f'{prefix}/best_so_far_26_gates.json'], env=aws_env, check=False)
        if generations:
            gen_tag = f'gen_{len(generations):02d}'
            subprocess.run(['aws', 's3', 'cp', str(best_26_path), f'{prefix}/{gen_tag}_best_so_far_26_gates.json'], env=aws_env, check=False)
        if png_path.is_file():
            subprocess.run(['aws', 's3', 'cp', str(png_path), f'{prefix}/fitness_curve.png'], env=aws_env, check=False)
    return curve_doc


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--wave-dir', type=Path, required=True)
    parser.add_argument('--s3-prefix', type=str, default=None)
    args = parser.parse_args(argv)
    doc = build_monotone_fitness_curve(args.wave_dir, s3_prefix=args.s3_prefix)
    print(json.dumps(doc, indent=2))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
