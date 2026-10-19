import React, { useEffect, useMemo, useState } from 'react';
import { BatteryMatrixGrid } from './components/BatteryMatrixGrid';
import { CadViewer3D } from './components/CadViewer3D';
import { GateScorecardPanel } from './components/GateScorecardPanel';
import { WaveParetoChart } from './components/WaveParetoChart';
import { DEFAULT_STUDIO_BUNDLE } from './data/defaultDataset';
import type { MasterScorecard, ParsedMesh3D, WebStudioBundle } from './types';

export type FetchLike = (input: string) => Promise<{
  ok: boolean;
  json: () => Promise<unknown>;
}>;

function isPlainRecord(v: unknown): v is Record<string, unknown> {
  return v !== null && typeof v === 'object' && !Array.isArray(v);
}

function isFiniteNumber(v: unknown): v is number {
  return typeof v === 'number' && Number.isFinite(v);
}

function isAdmittedSubjectSummary(item: unknown): boolean {
  if (!isPlainRecord(item)) return false;
  return (
    typeof item.id === 'string' &&
    item.id.length > 0 &&
    (item.kind === 'native_step' || item.kind === 'crescent_genome') &&
    isFiniteNumber(item.pass_count) &&
    isFiniteNumber(item.fail_count) &&
    isFiniteNumber(item.blocked_count) &&
    isFiniteNumber(item.incomplete_count) &&
    isFiniteNumber(item.solver_run_count) &&
    (item.g1_score === null || isFiniteNumber(item.g1_score)) &&
    (item.mass_g === null || isFiniteNumber(item.mass_g)) &&
    (item.mass_3d_g === null || isFiniteNumber(item.mass_3d_g)) &&
    (item.w_cad === null || isFiniteNumber(item.w_cad)) &&
    typeof item.all_engineering_gates_pass === 'boolean'
  );
}

function isAdmittedMasterScorecard(sc: unknown): sc is MasterScorecard {
  if (!isPlainRecord(sc)) return false;
  if (typeof sc.subject !== 'string' || sc.subject.length === 0) return false;
  if (!isPlainRecord(sc.cad)) return false;
  if (
    !isFiniteNumber(sc.cad.mass_3d_g) ||
    !isFiniteNumber(sc.cad.mass_raster_g) ||
    !isFiniteNumber(sc.cad.facets) ||
    typeof sc.cad.watertight_exact !== 'boolean'
  ) {
    return false;
  }
  if (!isPlainRecord(sc.engineering_metrics)) return false;
  for (const key of [
    'mass_g',
    'mass_3d_g',
    'sf_hunter_cert',
    'sf_hunter_strict',
    'sf_ring_cert',
    'twist_body_sf',
    'twist_tip_sf',
    'twist_tipzone_sf_cert',
    'twist_tipzone_sf_strict',
    'solid_tip_sf_nodal_max',
    'seq_ratio_cert',
    'fat_sf_l7',
  ]) {
    if (!isFiniteNumber(sc.engineering_metrics[key])) return false;
  }
  if (!isPlainRecord(sc.weighted_cad_score)) return false;
  if (
    typeof sc.weighted_cad_score.all_engineering_gates_pass !== 'boolean' ||
    !isFiniteNumber(sc.weighted_cad_score.W_CAD) ||
    !isPlainRecord(sc.weighted_cad_score.gates)
  ) {
    return false;
  }
  if (!isPlainRecord(sc.global_fitness_G1)) return false;
  if (sc.global_fitness_G1.G !== null && !isFiniteNumber(sc.global_fitness_G1.G)) {
    return false;
  }
  return true;
}

function isAdmittedBatteryRecord(rec: unknown): boolean {
  if (!isPlainRecord(rec)) return false;
  return (
    typeof rec.subject === 'string' &&
    rec.subject.length > 0 &&
    isFiniteNumber(rec.pass_count) &&
    isFiniteNumber(rec.solver_run_count) &&
    isPlainRecord(rec.global_fitness) &&
    Array.isArray(rec.cases)
  );
}

function isAdmittedWaveTrajectoryPoint(pt: unknown): boolean {
  if (!isPlainRecord(pt)) return false;
  return (
    typeof pt.wave === 'string' &&
    pt.wave.length > 0 &&
    typeof pt.subject === 'string' &&
    pt.subject.length > 0 &&
    isFiniteNumber(pt.mass_g) &&
    isFiniteNumber(pt.mass_3d_g) &&
    isFiniteNumber(pt.sf_hunter_strict) &&
    isFiniteNumber(pt.seq_ratio) &&
    isFiniteNumber(pt.solid_tip_sf) &&
    isFiniteNumber(pt.w_cad) &&
    isFiniteNumber(pt.g1)
  );
}

export function isAdmittedStudioBundle(candidate: unknown): candidate is WebStudioBundle {
  if (!isPlainRecord(candidate)) return false;
  const b = candidate as Partial<WebStudioBundle>;
  if (
    b.schema !== 'jarambit.web_studio_bundle.v1' ||
    typeof b.source_commit !== 'string' ||
    b.source_commit.length < 7 ||
    !Array.isArray(b.case_ids) ||
    b.case_ids.length === 0 ||
    !b.case_ids.every((id) => typeof id === 'string' && id.length > 0) ||
    !Array.isArray(b.subjects) ||
    b.subjects.length === 0 ||
    !b.subjects.every(isAdmittedSubjectSummary) ||
    !isPlainRecord(b.featured_scorecards) ||
    !isPlainRecord(b.battery_records) ||
    !Array.isArray(b.wave_trajectory) ||
    !b.wave_trajectory.every(isAdmittedWaveTrajectoryPoint)
  ) {
    return false;
  }
  const featuredEntries = Object.entries(b.featured_scorecards);
  if (
    featuredEntries.length === 0 ||
    !featuredEntries.every(([k, v]) => k.length > 0 && isAdmittedMasterScorecard(v))
  ) {
    return false;
  }
  const batteryEntries = Object.entries(b.battery_records);
  if (
    batteryEntries.length === 0 ||
    !batteryEntries.every(([k, v]) => k.length > 0 && isAdmittedBatteryRecord(v))
  ) {
    return false;
  }
  return true;
}

export async function loadStudioBundle(
  fetchImpl: FetchLike | undefined = typeof fetch !== 'undefined' ? (fetch as FetchLike) : undefined,
  fallback: WebStudioBundle = DEFAULT_STUDIO_BUNDLE,
): Promise<WebStudioBundle> {
  if (!fetchImpl) return fallback;
  try {
    const resp = await fetchImpl('/data/index.json');
    if (!resp.ok) return fallback;
    const json = await resp.json();
    if (isAdmittedStudioBundle(json)) {
      return json;
    }
    return fallback;
  } catch {
    return fallback;
  }
}

export function formatGateHeadlineStatus(
  id: string,
  allPass: boolean | null | undefined,
): 'All 16 Gates Pass' | 'Withdrawn / Gates Fail' | 'Gates Fail' | 'Gates Unavailable' {
  if (allPass === true) return 'All 16 Gates Pass';
  if (allPass === false) {
    return id.startsWith('w283_') ? 'Withdrawn / Gates Fail' : 'Gates Fail';
  }
  return 'Gates Unavailable';
}

export const App: React.FC = () => {
  const [bundle, setBundle] = useState<WebStudioBundle>(DEFAULT_STUDIO_BUNDLE);
  const [selectedSubject, setSelectedSubject] = useState<string>('w283_p14_c01');
  const [customMesh, setCustomMesh] = useState<ParsedMesh3D | null>(null);
  const [customScorecards, setCustomScorecards] = useState<Record<string, MasterScorecard>>({});

  useEffect(() => {
    let cancelled = false;
    loadStudioBundle().then((loaded) => {
      if (!cancelled) {
        setBundle(loaded);
      }
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const catalogIds = useMemo(
    () => new Set(bundle.subjects.map((s) => s.id)),
    [bundle.subjects],
  );

  const activeScorecard: MasterScorecard | null =
    customScorecards[selectedSubject] ??
    bundle.featured_scorecards[selectedSubject] ??
    null;

  const activeBatteryRecord =
    activeScorecard?.cad_battery ??
    bundle.battery_records[selectedSubject] ??
    null;

  const handleSelectSubject = (subjId: string) => {
    setCustomMesh(null);
    setSelectedSubject(subjId);
  };

  const handleUploadScorecard = (sc: MasterScorecard) => {
    const id = sc.subject || 'uploaded_cad';
    setCustomScorecards((prev) => ({ ...prev, [id]: sc }));
    setSelectedSubject(id);
  };

  const featuredQuickLinks = useMemo(() => {
    const order = ['w282_p14_c01', 'w282_p10_c02', 'w283_p14_c01', 'w283_p10_c02', 'c10a'];
    return order.map((id) => {
      const sc = customScorecards[id] ?? bundle.featured_scorecards[id];
      const wCad = sc?.weighted_cad_score?.W_CAD;
      const allPass = sc?.weighted_cad_score?.all_engineering_gates_pass;
      const mass3d = sc?.cad?.mass_3d_g;
      const scoreText =
        wCad != null ? `W_CAD ${wCad >= 0 ? '+' : ''}${wCad.toFixed(4)}` : 'W_CAD unavailable';
      const statusText = formatGateHeadlineStatus(id, allPass);
      const massPrefix =
        id === 'c10a'
          ? 'Switchback C10a STEP'
          : mass3d != null
            ? `${mass3d.toFixed(2)}g`
            : id;
      return {
        id,
        tag: `${massPrefix} · ${scoreText} (${statusText})`,
      };
    });
  }, [bundle.featured_scorecards, customScorecards]);

  return (
    <div className="studio-shell">
      <header className="studio-topbar">
        <div className="brand-block">
          <div className="brand-badge">JARAMBIT</div>
          <div>
            <h1>3D CAD & Multi-Solver Gate Scorecard Studio</h1>
            <p className="subtitle">
              Pure-Rust B-Rep STEP/STL Manifold · CST+DKT Plate · Bore-Pin/Winkler Buckling (side.a4) · 3D TET10 Tip Submodel · Snapshot <code>{bundle.source_commit.slice(0, 12)}</code>
            </p>
          </div>
        </div>

        <div className="topbar-actions">
          <div className="quick-pills" role="group" aria-label="Featured CAD Candidates">
            {featuredQuickLinks.map((item) => (
              <button
                key={item.id}
                type="button"
                className={`quick-pill ${selectedSubject === item.id ? 'active' : ''}`}
                onClick={() => handleSelectSubject(item.id)}
              >
                <strong>{item.id}</strong>
                <span>{item.tag}</span>
              </button>
            ))}
          </div>

          <label className="subject-select-label">
            <span>All {bundle.subjects.length} Catalog Subjects:</span>
            <select
              aria-label="Select Catalog Subject"
              value={selectedSubject}
              onChange={(e) => handleSelectSubject(e.target.value)}
            >
              {bundle.subjects.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.id} ({s.pass_count}/18 pass · G1={s.g1_score != null ? s.g1_score.toFixed(4) : 'INC'})
                </option>
              ))}
              {Object.keys(customScorecards)
                .filter((k) => !catalogIds.has(k))
                .map((k) => (
                  <option key={k} value={k}>
                    {k} (Uploaded)
                  </option>
                ))}
            </select>
          </label>
        </div>
      </header>

      <main className="studio-main-grid">
        <div className="top-split-grid">
          <CadViewer3D
            subjectId={selectedSubject}
            scorecard={activeScorecard}
            customMesh={customMesh}
            onUploadMesh={(mesh) => setCustomMesh(mesh)}
            onUploadScorecard={handleUploadScorecard}
          />
          <GateScorecardPanel
            subjectId={selectedSubject}
            scorecard={activeScorecard}
            batteryRecord={activeBatteryRecord}
          />
        </div>

        <WaveParetoChart
          trajectory={bundle.wave_trajectory}
          catalogIds={catalogIds}
          selectedSubject={selectedSubject}
          onSelectSubject={handleSelectSubject}
        />

        <BatteryMatrixGrid
          caseIds={bundle.case_ids}
          subjects={bundle.subjects}
          batteryRecords={bundle.battery_records}
          selectedSubject={selectedSubject}
          onSelectSubject={handleSelectSubject}
        />
      </main>

      <footer className="studio-footer">
        <code>cargo run --release -p jarambit -- score-cad w282_p14_c01</code>
        <span>·</span>
        <code>cargo run --release -p jarambit -- cad-battery</code>
        <span>·</span>
        <code>cargo run --release -p jarambit -- export-web-data --out-dir web/public/data</code>
      </footer>
    </div>
  );
};

export default App;
