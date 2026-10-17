"""Check published observations against evidence, without reusing plot reductions."""
import json
from pathlib import Path
import statistics
import unittest

ROOT = Path(__file__).resolve().parents[3]


def load(name):
    return json.loads((ROOT / name).read_text())


def at(document, pointer):
    for key in pointer.split('/')[1:]:
        key = key.replace('~1', '/').replace('~0', '~')
        document = document[int(key)] if isinstance(document, list) else document[key]
    return document


class PaperObservations(unittest.TestCase):
    def test_every_new_plotted_value_reduces_actual_evidence(self):
        ledger = load('docs/figures/engineering_gates/certifiable_points.json')
        manifest = load('docs/figures/engineering_gates/fig_certifiable_pipeline.png.manifest.json')
        for point in ledger['points']:
            with self.subTest(point=point):
                self.assertIn(point['source'], manifest['inputs'])
                self.assertTrue(point['source'].startswith('evidence/'))
                document = load(point['source'])
                if 'pointers' in point:
                    values = [at(document, p) for p in point['pointers']]
                    expected = len(set(values)) if point['reduction']=='distinct content identities' else statistics.median(values)
                else:
                    expected = at(document, point['pointer'])
                    if point.get('reduction')=='array length':expected = len(expected)
                self.assertIs(type(point['value']), type(expected))
                self.assertEqual(point['value'], expected)
                if 'plotted_ratio' in point:
                    self.assertEqual(point['plotted_ratio'], expected/point['divisor'])

    def test_atlas_is_the_assessed_failure_with_exact_gate_verdicts(self):
        proof = load('evidence/evolve_certifiable/manifest.json')['automatic']
        artifacts = proof['artifacts']
        board = load(artifacts['leaderboard'])
        score = load(artifacts['scorecard'])
        gates = score['weighted_cad_score']['gates']
        self.assertIsNone(board['champion'])
        self.assertEqual(board['qualified_count'], 0)
        self.assertFalse(load(artifacts['rs'])['all_hard'])
        for scene in list(gates)+['hero_oblique', 'hero_reverse', 'hero_section']:
            manifest = load('docs/figures/whitepaper/blender/'+scene+'.webp.manifest.json')
            extra = manifest['extra']
            with self.subTest(scene=scene):
                self.assertEqual(extra['subject'], proof['label'])
                self.assertEqual(extra['certificate_identity'], proof['certificate_identity'])
                self.assertEqual(extra['gate_verdict'], gates.get(scene))
                self.assertEqual(extra['qualified_count'], 0)
                self.assertFalse(extra['rs_v2_all_hard'])
                self.assertIn('not qualified', extra['status'])
                self.assertIn('champion=null', extra['status'])
                text=' '.join(c['text'] for c in extra['captions'])
                self.assertIn('ASSESSED EXAMPLE', text)
                self.assertIn('CHAMPION = NULL', text)
                if scene in gates:
                    self.assertIn('W_CAD_v2 gate: '+('PASS' if gates[scene] else 'FAIL'), text)
                for key in ['stl', 'd1', 'scorecard', 'rs', 'leaderboard']:
                    self.assertIn(artifacts[key], manifest['inputs'])

    def test_wave_history_points_and_paper_gate_coverage(self):
        ledger = load('docs/figures/engineering_gates/wave_history_points.json')
        manifest = load('docs/figures/engineering_gates/fig_wave_history.png.manifest.json')
        self.assertFalse(ledger['preview'])
        self.assertNotIn('/home/', json.dumps(manifest))
        for point in ledger['points']:
            with self.subTest(point=point):
                self.assertIn(point['source'], manifest['inputs'])
                document = load(point['source'])
                if 'pointers' in point:
                    values = [at(document, p) for p in point['pointers']]
                    expected = {'min': min, 'median': statistics.median, 'max': max}[point['reduction']](values)
                else:
                    expected = at(document, point['pointer'])
                self.assertIs(type(point['value']), type(expected))
                self.assertEqual(point['value'], expected)
        wp = (ROOT / 'docs/WHITEPAPER.md').read_text(encoding='utf-8')
        eg = (ROOT / 'docs/ENGINEERING_GATES_AND_EVOLUTION.md').read_text(encoding='utf-8')
        proof = load('evidence/evolve_certifiable/manifest.json')['automatic']
        gates = [
            'cad_watertight_and_single_solid', 'geometric_and_holder_gates',
            'karambit_ring_and_curved_claw', 'c6_aesthetic_floor',
            'cert_hiker_180n', 'cert_hunter_260n', 'strict_hunter_260n',
            'cert_ring_pull_1500n', 'twist_body_1p82nm', 'twist_tip_only_1p82nm',
            'twist_tip_submodel_strict_1p82nm', 'solid_tip_3d_p2_1p82nm',
            'side_a4_buckling_and_support', 'failure_sequence_1p2',
            'plate_gci_valid_3level', 'dolfinx_cross_check_2pct',
        ]
        self.assertEqual(set(gates), set(load(proof['artifacts']['scorecard'])['weighted_cad_score']['gates']))
        for idx, gate in enumerate(gates, 1):
            self.assertIn(f'##### {idx}. `{gate}`', wp, msg=gate)
            self.assertIn(f'#### {idx}. `{gate}`', eg, msg=gate)
        for group in ('aesthetic_and_geometric', 'floors_828_joint', 'twist_floor_body_and_tipsub_strict',
                      'solid_tip_sf', 'strict_hunter_lateral', 'seq_hard', 'human_max',
                      'fatigue', 'gate_s_sharp_law', 'gate_s_tip_point_brep'):
            self.assertIn(f'`{group}`', wp, msg=group)
            self.assertIn(f'`{group}`', eg, msg=group)
        for scene in gates + ['hero_oblique', 'hero_reverse', 'hero_section']:
            self.assertIn(f'](figures/whitepaper/blender/{scene}.webp)', wp, msg=scene)
            self.assertIn(f'](figures/whitepaper/blender/{scene}.webp)', eg, msg=scene)
        summary = load('evidence/evolution_waves/2026-10-17/wave_summary.json')
        self.assertIn('evidence/evolution_waves/2026-10-17/wave_summary.json', manifest['inputs'])
        self.assertEqual(summary['total_waves'], 17)
        self.assertEqual(summary['total_assessed'], 69)
        self.assertEqual(summary['total_qualified'], 0)
        self.assertEqual(summary['total_unverified'], 5)
        self.assertEqual(summary['total_stage1_births'], 2936)
        self.assertEqual(summary['total_stage1_survivors'], 2606)
        for doc_name, doc in (('WHITEPAPER.md', wp), ('ENGINEERING_GATES_AND_EVOLUTION.md', eg)):
            self.assertIn('![Selected Stage-2 wave history](figures/engineering_gates/fig_wave_history.png)', doc, msg=doc_name)
            for w in ledger['waves']:
                commits = '/'.join(c[:7] for c in w['engine_commits'])
                label = f"`{w['wave']}` (`{w['host']}` / `{commits}`)"
                self.assertIn(
                    f"| {label} | {w['stage1_screens']} | {w['assessed']} | {w['qualified']} | "
                    f"{w['mass_g_min']:.3f} / {w['mass_g_median']:.3f} | "
                    f"{w['w_cad_max']:.6f} / {w['w_cad_median']:.6f} | "
                    f"{w['battery_pass_count_max']} / {w['battery_pass_count_median']:g} | "
                    f"{w['solid_tip_sf_max']:.4f} / {w['solid_tip_sf_median']:.4f} |",
                    doc,
                    msg=f"{doc_name}:{w['wave']}",
                )


if __name__ == '__main__':unittest.main()

