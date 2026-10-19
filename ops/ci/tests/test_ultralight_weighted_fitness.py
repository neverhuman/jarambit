#!/usr/bin/env python3
"""Unit and adverse boundary tests for W_ULTRALIGHT_v1 (ops.ci.ultralight_weighted_fitness)."""
from __future__ import annotations

import json
from pathlib import Path
import tempfile
import unittest

from ops.ci.ultralight_weighted_fitness import (
    ULTRALIGHT_WEIGHTS_V1,
    build_monotone_fitness_curve,
    compute_ultralight_weighted_fitness,
)


ROOT = Path(__file__).resolve().parents[3]


class TestUltralightWeightedFitness(unittest.TestCase):
    def test_weights_sum_to_100_and_cover_20_objectives(self):
        self.assertEqual(len(ULTRALIGHT_WEIGHTS_V1), 20)
        self.assertAlmostEqual(sum(ULTRALIGHT_WEIGHTS_V1.values()), 100.0, places=6)

    def test_committed_w283_candidate_scores_in_0_100_and_logs_all_26_legacy_gates(self):
        cad_eval = json.loads(
            (ROOT / 'evidence/rs_v2/w283_ultrarunner/w283_p14_c01_cad_eval.json').read_text(encoding='utf-8')
        )
        res = compute_ultralight_weighted_fitness(cad_eval)
        self.assertEqual(res['schema'], 'jarambit.ultralight_weighted_fitness.v1')
        self.assertTrue(res['veto_free_selection'])
        self.assertGreater(res['global_weighted_fitness'], 50.0)
        self.assertLessEqual(res['global_weighted_fitness'], 100.0)
        self.assertEqual(len(res['legacy_26_gate_breakdown']), 26)
        self.assertFalse(res['all_26_legacy_hard_gates_pass'])
        self.assertGreater(len(res['legacy_hard_gates_failed']), 0)

    def test_ultralight_mass_strictly_rewards_lighter_running_knife(self):
        base = {
            'engineering_metrics': {
                'mass_3d_g': 91.5,
                'lat_rob_cert_n': 178.0,
                'strict_lat_rob_0p0625_n': 180.4,
                'ring_rob_cert_n': 10500.0,
                'twist_body_sf': 1.45,
                'twist_tip_sf': 0.56,
                'twist_tipzone_sf_strict': 0.56,
                'solid_tip_sf_nodal_max': 0.71,
                'solid_tip_dolfinx_rel_err': 0.0005,
                'buckling_eigenvalue_a4': 2.1,
                'seq_ratio_cert': 1.32,
                'fat_sf_l7': 0.52,
                'ring_bearing_w_mm': 12.0,
            },
            'weighted_cad_score': {'gates': {'dolfinx_cross_check_2pct': True}},
        }
        lighter = json.loads(json.dumps(base))
        lighter['engineering_metrics']['mass_3d_g'] = 58.0

        r_heavy = compute_ultralight_weighted_fitness(base)
        r_light = compute_ultralight_weighted_fitness(lighter)
        self.assertGreater(
            r_light['objectives']['ultralight_mass']['weighted_points'],
            r_heavy['objectives']['ultralight_mass']['weighted_points'] + 7.5,
        )
        self.assertGreater(r_light['global_weighted_fitness'], r_heavy['global_weighted_fitness'])

    def test_12_deficit_gates_act_as_smooth_non_veto_objectives_with_logged_deficits(self):
        low = {
            'engineering_metrics': {
                'mass_3d_g': 75.0,
                'lat_rob_cert_n': 130.0,
                'strict_lat_rob_0p0625_n': 130.0,
                'ring_rob_cert_n': 1500.0,
                'twist_body_sf': 1.0,
                'twist_tip_sf': 0.50,
                'twist_tipzone_sf_strict': 0.50,
                'solid_tip_sf_nodal_max': 0.50,
                'solid_tip_dolfinx_rel_err': 0.001,
                'buckling_eigenvalue_a4': 1.2,
                'seq_ratio_cert': 1.25,
                'fat_sf_l7': 0.50,
                'ring_bearing_w_mm': 10.0,
            },
            'weighted_cad_score': {'gates': {'dolfinx_cross_check_2pct': True}},
        }
        high = json.loads(json.dumps(low))
        high['engineering_metrics'].update({
            'lat_rob_cert_n': 260.0,
            'strict_lat_rob_0p0625_n': 260.0,
            'twist_tip_sf': 1.0,
            'twist_tipzone_sf_strict': 1.0,
            'solid_tip_sf_nodal_max': 1.0,
            'fat_sf_l7': 1.0,
        })
        r_low = compute_ultralight_weighted_fitness(low)
        r_high = compute_ultralight_weighted_fitness(high)
        self.assertGreater(r_low['global_weighted_fitness'], 0.0)
        self.assertGreater(r_high['global_weighted_fitness'], r_low['global_weighted_fitness'])
        self.assertAlmostEqual(
            r_low['legacy_26_gate_breakdown']['G16-05:cert_hiker_180n']['deficit'],
            50.0,
            places=3,
        )
        self.assertAlmostEqual(
            r_low['legacy_26_gate_breakdown']['G16-07:strict_hunter_260n']['deficit'],
            130.0,
            places=3,
        )
        self.assertAlmostEqual(
            r_low['legacy_26_gate_breakdown']['G16-12:solid_tip_3d_p2_1p82nm']['deficit'],
            0.50,
            places=3,
        )
        self.assertAlmostEqual(
            r_low['legacy_26_gate_breakdown']['RSV2-08:fatigue']['deficit'],
            0.50,
            places=3,
        )

    def test_hall_of_fame_fitness_curve_is_monotone_non_decreasing(self):
        with tempfile.TemporaryDirectory() as tmp:
            wdir = Path(tmp)
            (wdir / 'round_00_screen.json').write_text(
                json.dumps([{'label': 'r0_c01', 'mass_g': 91.0, 'ultralight_weighted_fitness': 74.2}]),
                encoding='utf-8',
            )
            (wdir / 'round_01_screen.json').write_text(
                json.dumps([{'label': 'r1_c01', 'mass_g': 93.0, 'ultralight_weighted_fitness': 71.8}]),
                encoding='utf-8',
            )
            (wdir / 'round_02_screen.json').write_text(
                json.dumps([{'label': 'r2_c01', 'mass_g': 78.5, 'ultralight_weighted_fitness': 81.6}]),
                encoding='utf-8',
            )
            curve = build_monotone_fitness_curve(wdir)
            self.assertTrue(curve['monotone_non_decreasing'])
            hof = [g['hall_of_fame_best_so_far'] for g in curve['generations']]
            self.assertEqual(hof, [74.2, 74.2, 81.6])
            self.assertEqual(curve['best_so_far']['label'], 'r2_c01')
            self.assertTrue((wdir / 'fitness_curve.json').is_file())

    def test_missing_gate_evidence_fails_closed(self):
        res = compute_ultralight_weighted_fitness({})
        self.assertFalse(res['all_26_legacy_hard_gates_pass'])
        self.assertEqual(res['legacy_hard_gates_passed_count'], 0)
        self.assertEqual(len(res['legacy_hard_gates_failed']), 26)

    def test_sharp_apex_within_limit_scores_without_error_and_checks_half_angle(self):
        # apex <= 0.20 mm used to evaluate an undefined name in the RSV2-09 half-angle check.
        def rsv2_09(half_angle_deg):
            doc = {'export_measurements': {'sharpness': {'apex_max_mm': 0.18, 'half_angle_max_deg': half_angle_deg}}}
            return compute_ultralight_weighted_fitness(doc)['legacy_26_gate_breakdown']['RSV2-09:gate_s_sharp_law']

        self.assertTrue(rsv2_09(20.0)['legacy_pass'])
        self.assertFalse(rsv2_09(28.0)['legacy_pass'])
        self.assertEqual(rsv2_09(28.0)['deficit'], 3.0)


if __name__ == '__main__':
    unittest.main()
