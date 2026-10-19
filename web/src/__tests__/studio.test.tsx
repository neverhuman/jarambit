import { renderToString } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { App, formatGateHeadlineStatus, loadStudioBundle } from '../App';
import { BatteryMatrixGrid } from '../components/BatteryMatrixGrid';
import { buildParametricKarambitMesh, CadViewer3D } from '../components/CadViewer3D';
import { GateScorecardPanel } from '../components/GateScorecardPanel';
import { WaveParetoChart } from '../components/WaveParetoChart';
import { DEFAULT_STUDIO_BUNDLE } from '../data/defaultDataset';
import type { MasterScorecard, WebStudioBundle } from '../types';
import { parseStepFacetedMesh, parseStlMesh } from '../utils/stlParser';

describe('Jarambit 3D CAD & Gate Scorecard Studio', () => {
  it('contains all 23 catalog subjects and 18 evaluator cases (414 cells) in DEFAULT_STUDIO_BUNDLE', () => {
    expect(DEFAULT_STUDIO_BUNDLE.schema).toBe('jarambit.web_studio_bundle.v1');
    expect(DEFAULT_STUDIO_BUNDLE.case_ids).toHaveLength(18);
    expect(DEFAULT_STUDIO_BUNDLE.subjects).toHaveLength(23);

    const w283 = DEFAULT_STUDIO_BUNDLE.featured_scorecards.w283_p14_c01;
    expect(w283).toBeDefined();
    expect(w283.weighted_cad_score.all_engineering_gates_pass).toBe(false);
    expect(w283.weighted_cad_score.W_CAD).toBeCloseTo(-3.878704, 6);
    expect(w283.global_fitness_G1.G).toBeCloseTo(0.6239029999999999, 12);
    expect(DEFAULT_STUDIO_BUNDLE.battery_records.w283_p14_c01.global_fitness.G).toBeCloseTo(
      0.623888308291947,
      12,
    );
    expect(w283.rs_v2_gates?.fatigue).toBe('fail');
    expect(w283.cad.watertight_exact).toBe(true);
    expect(w283.cad.mass_3d_g).toBeCloseTo(75.199, 2);

    const w283Peer = DEFAULT_STUDIO_BUNDLE.featured_scorecards.w283_p10_c02;
    expect(w283Peer).toBeDefined();
    expect(w283Peer.weighted_cad_score.all_engineering_gates_pass).toBe(false);
    expect(w283Peer.weighted_cad_score.W_CAD).toBeCloseTo(-3.964924, 6);
    expect(w283Peer.global_fitness_G1.G).toBeCloseTo(0.6216340416666666, 12);
    expect(DEFAULT_STUDIO_BUNDLE.battery_records.w283_p10_c02.global_fitness.G).toBeCloseTo(
      0.6216318603776162,
      12,
    );
    expect(w283Peer.rs_v2_gates?.fatigue).toBe('fail');

    const w282 = DEFAULT_STUDIO_BUNDLE.featured_scorecards.w282_p14_c01;
    expect(w282).toBeDefined();
    expect(w282.weighted_cad_score.all_engineering_gates_pass).toBe(true);
    expect(w282.weighted_cad_score.W_CAD).toBeCloseTo(1.779046, 5);
    expect(w282.global_fitness_G1.G).toBeCloseTo(0.87821, 5);
    expect(w282.cad.watertight_exact).toBe(true);
    expect(w282.cad.mass_3d_g).toBeCloseTo(75.323, 2);
  });

  it('parses ASCII STL and STEP faceted meshes and computes watertight 2-manifold census', () => {
    const asciiStl = `solid tet
  facet normal 0 0 -1
    outer loop
      vertex 0 0 0
      vertex 0 10 0
      vertex 10 0 0
    endloop
  endfacet
  facet normal 0 -1 0
    outer loop
      vertex 0 0 0
      vertex 10 0 0
      vertex 0 0 10
    endloop
  endfacet
  facet normal -1 0 0
    outer loop
      vertex 0 0 0
      vertex 0 0 10
      vertex 0 10 0
    endloop
  endfacet
  facet normal 0.577 0.577 0.577
    outer loop
      vertex 10 0 0
      vertex 0 10 0
      vertex 0 0 10
    endloop
  endfacet
endsolid tet`;
    const enc = new TextEncoder().encode(asciiStl);
    const mesh = parseStlMesh(enc.buffer, 'tet.stl');
    expect(mesh.facets).toBe(4);
    expect(mesh.boundaryEdges).toBe(0);
    expect(mesh.nonManifoldEdges).toBe(0);
    expect(mesh.watertightExact).toBe(true);
    expect(mesh.volumeMm3).toBeCloseTo(166.7, 1);

    const stepText = `ISO-10303-21;
DATA;
#1 = CARTESIAN_POINT('',(0.0,0.0,0.0));
#2 = CARTESIAN_POINT('',(0.0,10.0,0.0));
#3 = CARTESIAN_POINT('',(10.0,0.0,0.0));
#4 = CARTESIAN_POINT('',(0.0,0.0,10.0));
#10 = POLY_LOOP('',(#1,#2,#3));
#11 = POLY_LOOP('',(#1,#3,#4));
#12 = POLY_LOOP('',(#1,#4,#2));
#13 = POLY_LOOP('',(#3,#2,#4));
ENDSEC;
END-ISO-10303-21;`;
    const stepMesh = parseStepFacetedMesh(stepText, 'tet.step');
    expect(stepMesh.facets).toBe(4);
    expect(stepMesh.watertightExact).toBe(true);
  });

  it('builds a watertight parametric karambit preview mesh', () => {
    const mesh = buildParametricKarambitMesh('w282_p14_c01.stl');
    expect(mesh.facets).toBeGreaterThan(200);
    expect(mesh.watertightExact).toBe(true);
    expect(mesh.mass3dG).toBeCloseTo(75.323, 3);
  });

  it('renders App, GateScorecardPanel, BatteryMatrixGrid, WaveParetoChart, and CadViewer3D with corrected w283 fallback', () => {
    const appHtml = renderToString(<App />);
    expect(appHtml).toContain('3D CAD &amp; Multi-Solver Gate Scorecard Studio');
    expect(appHtml).toContain('SHORTFALL DETECTED');
    expect(appHtml).not.toContain('ALL 16 GATES PASS');
    expect(appHtml).toContain('-3.878704');
    expect(appHtml).toContain('0.623903');
    expect(appHtml).toContain(DEFAULT_STUDIO_BUNDLE.source_commit.slice(0, 12));
    expect(appHtml).not.toContain('+1.8166');
    expect(appHtml).not.toContain('1.795575');
    expect(appHtml).toContain('twist_tip_only_1p82nm');
    expect(appHtml).toContain('rs_v2_gates.fatigue');

    const peerSc = DEFAULT_STUDIO_BUNDLE.featured_scorecards.w283_p10_c02;
    const peerHtml = renderToString(
      <GateScorecardPanel
        subjectId="w283_p10_c02"
        scorecard={peerSc}
        batteryRecord={peerSc.cad_battery}
      />,
    );
    expect(peerHtml).toContain('-3.964924');
    expect(peerHtml).toContain('0.621634');
    expect(peerHtml).toContain('SHORTFALL DETECTED');
    expect(peerHtml).not.toContain('ALL 16 GATES PASS');

    const w282Sc = DEFAULT_STUDIO_BUNDLE.featured_scorecards.w282_p14_c01;
    const w282Html = renderToString(
      <GateScorecardPanel
        subjectId="w282_p14_c01"
        scorecard={w282Sc}
        batteryRecord={w282Sc.cad_battery}
      />,
    );
    // w282 has no recorded fatigue boolean in weighted_cad_score.gates or rs_v2_gates; must render UNAVAILABLE rather than synthesizing PASS from fat_sf_l7 >= 1.0
    expect(w282Html).toContain('fatigue_l7_1e7 (unrecorded)');
    expect(w282Html).toContain('UNAVAILABLE');

    const c10aSc = DEFAULT_STUDIO_BUNDLE.featured_scorecards.c10a;
    const c10aHtml = renderToString(
      <CadViewer3D
        subjectId="c10a"
        scorecard={c10aSc}
        customMesh={null}
        onUploadMesh={() => {}}
        onUploadScorecard={() => {}}
      />,
    );
    expect(c10aHtml).toContain('WATERTIGHT 2-MANIFOLD');

    const matrixHtml = renderToString(
      <BatteryMatrixGrid
        caseIds={DEFAULT_STUDIO_BUNDLE.case_ids}
        subjects={DEFAULT_STUDIO_BUNDLE.subjects}
        batteryRecords={DEFAULT_STUDIO_BUNDLE.battery_records}
        selectedSubject="w282_p14_c01"
        onSelectSubject={() => {}}
      />,
    );
    expect(matrixHtml).toContain('Full CAD Evaluator Battery Matrix');
    expect(matrixHtml).toContain('solid.tip');

    const chartHtml = renderToString(
      <WaveParetoChart
        trajectory={DEFAULT_STUDIO_BUNDLE.wave_trajectory}
        catalogIds={new Set(DEFAULT_STUDIO_BUNDLE.subjects.map((s) => s.id))}
        selectedSubject="w282_p14_c01"
        onSelectSubject={() => {}}
      />,
    );
    expect(chartHtml).toContain('Mass Reduction');
  });

  it('distinguishes true, false, and unavailable gate statuses without coercing missing flags to false', () => {
    expect(formatGateHeadlineStatus('w282_p14_c01', true)).toBe('All 16 Gates Pass');
    expect(formatGateHeadlineStatus('w283_p14_c01', false)).toBe('Withdrawn / Gates Fail');
    expect(formatGateHeadlineStatus('c10a', false)).toBe('Gates Fail');
    expect(formatGateHeadlineStatus('missing_subject', undefined)).toBe('Gates Unavailable');
    expect(formatGateHeadlineStatus('w283_p14_c01', null)).toBe('Gates Unavailable');

    const partialSc: MasterScorecard = {
      ...DEFAULT_STUDIO_BUNDLE.featured_scorecards.w282_p14_c01,
      rs_v2_gates: null,
      weighted_cad_score: {
        ...DEFAULT_STUDIO_BUNDLE.featured_scorecards.w282_p14_c01.weighted_cad_score,
        gates: {
          cad_watertight_and_single_solid: true,
          cert_hunter_260n: false,
        },
      },
    };
    const partialHtml = renderToString(
      <GateScorecardPanel
        subjectId="partial_subject"
        scorecard={partialSc}
        batteryRecord={null}
      />,
    );
    expect(partialHtml).toContain('PASS');
    expect(partialHtml).toContain('FAIL');
    expect(partialHtml).toContain('UNAVAILABLE');
  });

  it('A01 regression: loadStudioBundle exercises async fetch across 404, malformed JSON, network rejection, and valid bundle', async () => {
    let calls = 0;

    // 1. HTTP 404 / non-OK status
    const res404 = await loadStudioBundle(async (url) => {
      calls += 1;
      expect(url).toBe('/data/index.json');
      return {
        ok: false,
        json: async () => ({ schema: 'jarambit.web_studio_bundle.v1' }),
      };
    });
    expect(calls).toBe(1);
    expect(res404).toBe(DEFAULT_STUDIO_BUNDLE);
    expect(res404.featured_scorecards.w283_p10_c02.weighted_cad_score.W_CAD).toBeCloseTo(
      -3.964924,
      6,
    );
    expect(res404.featured_scorecards.w283_p14_c01.weighted_cad_score.W_CAD).toBeCloseTo(
      -3.878704,
      6,
    );

    // 2. Malformed JSON / schema-invalid payload
    const resInvalid = await loadStudioBundle(async () => {
      calls += 1;
      return {
        ok: true,
        json: async () => ({ schema: 'wrong.schema', subjects: 'not-an-array' }),
      };
    });
    expect(calls).toBe(2);
    expect(resInvalid).toBe(DEFAULT_STUDIO_BUNDLE);

    // 3. Network rejection (Promise.reject)
    const resRejected = await loadStudioBundle(async () => {
      calls += 1;
      throw new Error('network offline');
    });
    expect(calls).toBe(3);
    expect(resRejected).toBe(DEFAULT_STUDIO_BUNDLE);

    // 4. Valid source-bound bundle
    const validBundle: WebStudioBundle = {
      ...DEFAULT_STUDIO_BUNDLE,
      source_commit: 'e0ec88dd2dfbd6e4287cb718661a0793b9562183',
    };
    const resValid = await loadStudioBundle(async () => {
      calls += 1;
      return {
        ok: true,
        json: async () => validBundle,
      };
    });
    expect(calls).toBe(4);
    expect(resValid).toBe(validBundle);
    expect(resValid.source_commit).toBe('e0ec88dd2dfbd6e4287cb718661a0793b9562183');

    for (const sid of ['w283_p10_c02', 'w283_p14_c01'] as const) {
      const sc = DEFAULT_STUDIO_BUNDLE.featured_scorecards[sid];
      expect(sc.weighted_cad_score.all_engineering_gates_pass).toBe(false);
      expect(sc.weighted_cad_score.W_CAD).toBeLessThan(0);
      expect(sc.weighted_cad_score.gates.cert_hunter_260n).toBe(false);
      expect(sc.weighted_cad_score.gates.strict_hunter_260n).toBe(false);
      expect(sc.weighted_cad_score.gates.failure_sequence_1p2).toBe(false);
      expect(sc.weighted_cad_score.gates.solid_tip_3d_p2_1p82nm).toBe(false);
      expect(sc.weighted_cad_score.gates.twist_tip_only_1p82nm).toBe(false);
      expect(sc.rs_v2_gates?.fatigue).toBe('fail');
      expect(sc.rs_v2_gates?.strict_hunter_lateral).toBe('fail');
      expect(sc.rs_v2_gates?.solid_tip_sf).toBe('fail');
      expect(sc.rs_v2_gates?.seq_hard).toBe('fail');
      const summary = DEFAULT_STUDIO_BUNDLE.subjects.find((s) => s.id === sid);
      expect(summary?.all_engineering_gates_pass).toBe(false);
      expect(summary?.w_cad).toBeCloseTo(sc.weighted_cad_score.W_CAD, 6);
    }
  });
});
