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

export function isAdmittedStudioBundle(candidate: unknown): candidate is WebStudioBundle {
  if (!candidate || typeof candidate !== 'object') return false;
  const b = candidate as Partial<WebStudioBundle>;
  return (
    b.schema === 'jarambit.web_studio_bundle.v1' &&
    typeof b.source_commit === 'string' &&
    b.source_commit.length >= 7 &&
    Array.isArray(b.case_ids) &&
    Array.isArray(b.subjects) &&
    b.featured_scorecards != null &&
    typeof b.featured_scorecards === 'object' &&
    b.battery_records != null &&
    typeof b.battery_records === 'object'
  );
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
