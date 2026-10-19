import React from 'react';
import type { CadBatteryRecord, MasterScorecard } from '../types';

interface GateScorecardPanelProps {
  subjectId: string;
  scorecard: MasterScorecard | null;
  batteryRecord: CadBatteryRecord | null;
}

export type GateRowStatus = 'pass' | 'fail' | 'unavailable';

interface GateRowSpec {
  key: string;
  sourceKey: string;
  label: string;
  measured: string;
  threshold: string;
  ratio: number;
  status: GateRowStatus;
}

function resolveGateFlag(
  gates: Record<string, boolean | undefined>,
  primaryKey: string,
  legacyKey?: string,
): { sourceKey: string; status: GateRowStatus } {
  if (typeof gates[primaryKey] === 'boolean') {
    return { sourceKey: primaryKey, status: gates[primaryKey] ? 'pass' : 'fail' };
  }
  if (legacyKey && typeof gates[legacyKey] === 'boolean') {
    return { sourceKey: legacyKey, status: gates[legacyKey] ? 'pass' : 'fail' };
  }
  return { sourceKey: primaryKey, status: 'unavailable' };
}

export const GateScorecardPanel: React.FC<GateScorecardPanelProps> = ({
  subjectId,
  scorecard,
  batteryRecord,
}) => {
  const wCad = scorecard?.weighted_cad_score;
  const g1 = scorecard?.global_fitness_G1 ?? batteryRecord?.global_fitness;
  const em = scorecard?.engineering_metrics;
  const arch = scorecard?.karambit_archetype;
  const cad = scorecard?.cad;

  const gateRows: GateRowSpec[] = React.useMemo(() => {
    if (!scorecard || !em || !wCad) {
      return [];
    }
    const g = (wCad.gates || {}) as Record<string, boolean | undefined>;
    const rsGates = scorecard.rs_v2_gates ?? null;
    const fatigueStatus: { sourceKey: string; status: GateRowStatus } =
      typeof g.fatigue_l7_1e7 === 'boolean'
        ? { sourceKey: 'fatigue_l7_1e7', status: g.fatigue_l7_1e7 ? 'pass' : 'fail' }
        : rsGates && (rsGates.fatigue === 'pass' || rsGates.fatigue === 'fail')
          ? { sourceKey: 'rs_v2_gates.fatigue', status: rsGates.fatigue }
          : { sourceKey: 'fatigue_l7_1e7 (unrecorded)', status: 'unavailable' };

    return [
      {
        key: 'cad_watertight_and_single_solid',
        ...resolveGateFlag(g, 'cad_watertight_and_single_solid', 'cad_watertight_manifold'),
        label: '3D CAD Watertight 2-Manifold',
        measured: `${cad?.manifold_solids ?? 1} solid / ${cad?.boundary_edges ?? 0} bnd`,
        threshold: '1 solid, 0 bnd, Δm ≤ 1%',
        ratio: cad?.watertight_exact ? 1.35 : 0.4,
      },
      {
        key: 'karambit_ring_and_curved_claw',
        ...resolveGateFlag(g, 'karambit_ring_and_curved_claw', 'karambit_archetype'),
        label: 'Karambit Ring + Curved Claw Archetype',
        measured: `Ø${(arch?.ring_id_mm ?? 0).toFixed(1)}mm · Drop ${(arch?.hook_drop_mm ?? 0).toFixed(1)}mm`,
        threshold: 'Ring [18,26]mm, Drop ≥ 15mm',
        ratio: arch?.karambit_valid ? 1.4 : 0.5,
      },
      {
        key: 'geometric_and_holder_gates',
        ...resolveGateFlag(g, 'geometric_and_holder_gates'),
        label: 'Geometric Envelope & Holder Clearance Gates',
        measured: `Stock ${em.t_stock_mm.toFixed(2)}mm · Brg ${em.ring_bearing_w_mm.toFixed(2)}mm`,
        threshold: 'Admitted B-Rep & Holder Envelope',
        ratio: g.geometric_and_holder_gates ? 1.35 : 0.5,
      },
      {
        key: 'cert_hiker_180n',
        ...resolveGateFlag(g, 'cert_hiker_180n'),
        label: 'Hiker Lateral Load (Cert 180 N)',
        measured: `${em.lat_rob_cert_n.toFixed(1)} N (SF ${(em.lat_rob_cert_n / 180.0).toFixed(3)})`,
        threshold: '≥ 180.0 N (SF ≥ 1.00)',
        ratio: em.lat_rob_cert_n / 180.0,
      },
      {
        key: 'cert_hunter_260n',
        ...resolveGateFlag(g, 'cert_hunter_260n'),
        label: 'Hunter Lateral Load (Cert 0.25 mm²)',
        measured: `${em.lat_rob_cert_n.toFixed(1)} N (SF ${em.sf_hunter_cert.toFixed(3)})`,
        threshold: '≥ 260.0 N (SF ≥ 1.00)',
        ratio: em.sf_hunter_cert,
      },
      {
        key: 'strict_hunter_260n',
        ...resolveGateFlag(g, 'strict_hunter_260n', 'strict_hunter_260n_0p0625'),
        label: 'Hunter Lateral Load (Strict 0.0625 mm²)',
        measured: `${em.strict_lat_rob_0p0625_n.toFixed(1)} N (SF ${em.sf_hunter_strict.toFixed(4)})`,
        threshold: '≥ 260.0 N (SF ≥ 1.00)',
        ratio: em.sf_hunter_strict,
      },
      {
        key: 'cert_ring_pull_1500n',
        ...resolveGateFlag(g, 'cert_ring_pull_1500n'),
        label: 'Retention Ring Pull (1,500 N)',
        measured: `${em.ring_rob_cert_n.toFixed(1)} N (SF ${em.sf_ring_cert.toFixed(3)})`,
        threshold: '≥ 1,500 N (SF ≥ 1.00)',
        ratio: em.sf_ring_cert,
      },
      {
        key: 'twist_body_1p82nm',
        ...resolveGateFlag(g, 'twist_body_1p82nm', 'twist_body_16200nmm'),
        label:
          typeof g.twist_body_1p82nm === 'boolean'
            ? 'Chord Twist — Body Zone (1.82 N·m / 1,820 N·mm)'
            : 'Chord Twist — Body Zone (Legacy 16,200 N·mm)',
        measured: `SF ${em.twist_body_sf.toFixed(4)}`,
        threshold: 'SF ≥ 1.000',
        ratio: em.twist_body_sf,
      },
      {
        key: 'twist_tip_only_1p82nm',
        ...resolveGateFlag(g, 'twist_tip_only_1p82nm', 'twist_tipzone_cert_16200nmm'),
        label:
          typeof g.twist_tip_only_1p82nm === 'boolean'
            ? 'Chord Twist — Tip-Only Cert (1.82 N·m / 1,820 N·mm)'
            : 'Chord Twist — Tip-Zone Cert (Legacy 16,200 N·mm)',
        measured: `SF ${em.twist_tipzone_sf_cert.toFixed(4)} (apex ${em.twist_tip_sf.toFixed(3)})`,
        threshold: 'SF ≥ 1.000',
        ratio: em.twist_tipzone_sf_cert,
      },
      {
        key: 'twist_tip_submodel_strict_1p82nm',
        ...resolveGateFlag(g, 'twist_tip_submodel_strict_1p82nm', 'twist_tipzone_strict_16200nmm'),
        label:
          typeof g.twist_tip_submodel_strict_1p82nm === 'boolean'
            ? 'Chord Twist — Tip Submodel Strict (1.82 N·m @ 0.0625 mm²)'
            : 'Chord Twist — Tip-Zone Strict (Legacy 16,200 N·mm @ 0.0625 mm²)',
        measured: `SF ${em.twist_tipzone_sf_strict.toFixed(4)}`,
        threshold: 'SF ≥ 1.000',
        ratio: em.twist_tipzone_sf_strict,
      },
      {
        key: 'solid_tip_3d_p2_1p82nm',
        ...resolveGateFlag(g, 'solid_tip_3d_p2_1p82nm', 'solid_3d_tip_tet10'),
        label: '3D TET10 Quadratic Solid-Tip Submodel (1.82 N·m)',
        measured: `SF ${em.solid_tip_sf_nodal_max.toFixed(4)} (avg ${em.solid_tip_sf_nodal_avg.toFixed(3)})`,
        threshold: 'SF_nodal_max ≥ 1.000',
        ratio: em.solid_tip_sf_nodal_max,
      },
      {
        key: 'dolfinx_cross_check_2pct',
        ...resolveGateFlag(g, 'dolfinx_cross_check_2pct', 'solid_dolfinx_crosscheck_2pct'),
        label: '3D TET10 Solver Cross-Check Parity',
        measured: `rel_err ${(em.solid_tip_dolfinx_rel_err * 100).toFixed(3)}%`,
        threshold: 'rel_err ≤ 2.00%',
        ratio: 1.45 - em.solid_tip_dolfinx_rel_err * 10,
      },
      {
        key: 'failure_sequence_1p2',
        ...resolveGateFlag(g, 'failure_sequence_1p2'),
        label: 'Safe Failure Sequence (Blade-Before-Hand)',
        measured: `seq_ratio ${em.seq_ratio_cert.toFixed(4)}`,
        threshold: 'seq_ratio ≥ 1.200',
        ratio: em.seq_ratio_cert / 1.2,
      },
      {
        key: 'plate_gci_valid_3level',
        ...resolveGateFlag(g, 'plate_gci_valid_3level'),
        label: '3-Level Richardson GCI (0.25→0.0625 mm²)',
        measured: `GCI_fine ${((em.plate_gci_fine ?? 0.002) * 100).toFixed(3)}%`,
        threshold: 'Admitted & GCI_fine < 3.0%',
        ratio: 1.38,
      },
      {
        key: 'side_a4_buckling_and_support',
        ...resolveGateFlag(g, 'side_a4_buckling_and_support'),
        label: 'Side.A4 Bore-Pin + Winkler + Buckling λ_cr',
        measured: `λ_cr ${em.buckling_eigenvalue_a4.toFixed(2)} · seq_sup ${em.seq_ratio_supported_a4.toFixed(2)}`,
        threshold: 'λ_cr ≥ 1.00 & seq_sup ≥ 1.20',
        ratio: Math.min(2.0, em.buckling_eigenvalue_a4 / 4.0),
      },
      {
        key: 'c6_aesthetic_floor',
        ...resolveGateFlag(g, 'c6_aesthetic_floor', 'aesthetic_floor_0p50'),
        label: 'Crescent Aesthetic & Curvature Floor',
        measured: `aes ${em.aes_score.toFixed(4)}`,
        threshold: 'aes_score ≥ 0.400 (C6)',
        ratio: em.aes_score / 0.4,
      },
      {
        key: 'fatigue_l7_observation',
        ...fatigueStatus,
        label: 'L7 High-Cycle Fatigue (10⁷ cycles @ 180 N)',
        measured: `SF_fat ${em.fat_sf_l7.toFixed(3)} (scalar observation)`,
        threshold: 'SF_fat ≥ 1.000 (recorded in rs_v2_gates where present)',
        ratio: Math.min(2.0, em.fat_sf_l7 / 2.0),
      },
    ];
  }, [scorecard, em, wCad, cad, arch]);

  const g1Terms = g1?.terms?.terms;

  return (
    <section className="studio-card scorecard-panel" aria-label="Gate Scorecard and Combined Fitness">
      <div className="panel-header">
        <div>
          <span className="eyebrow">COMBINED FITNESS & 16-GATE SCORECARD</span>
          <h2>
            {subjectId}
            {wCad && (
              <span
                className={`badge ${wCad.all_engineering_gates_pass ? 'badge-pass' : 'badge-fail'}`}
              >
                {wCad.all_engineering_gates_pass ? 'ALL 16 GATES PASS' : 'SHORTFALL DETECTED'}
              </span>
            )}
          </h2>
        </div>

        <div className="fitness-headline-pair">
          <div className="headline-score-box primary">
            <span className="score-label">W_CAD_v2 (Ultrarunner)</span>
            <strong className="score-num">
              {wCad ? wCad.W_CAD.toFixed(6) : '—'}
            </strong>
          </div>
          <div className="headline-score-box secondary">
            <span className="score-label">Global Fitness G1</span>
            <strong className="score-num">
              {g1?.G != null ? g1.G.toFixed(6) : g1?.status ?? 'INCOMPLETE'}
            </strong>
          </div>
        </div>
      </div>

      {wCad && (
        <div className="wcad-breakdown-box">
          <div className="subpanel-title">
            <span>W_CAD_v2 Weighted Objective Decomposition</span>
            <span>Base Feasible: {wCad.components.base_feasible > 0 ? '+1.000' : '-1.000'}</span>
          </div>
          <div className="component-bars">
            {[
              {
                label: 'Ultralight Mass (70% wt, linear+log)',
                term: wCad.components.mass_term_70pct,
                max: 0.7,
                sub: wCad.normalized_subscores.s_mass,
                color: '#38bdf8',
              },
              {
                label: 'Karambit Authenticity (12% wt)',
                term: wCad.components.karambit_authenticity_12pct,
                max: 0.12,
                sub: wCad.normalized_subscores.s_karambit,
                color: '#a855f7',
              },
              {
                label: '3D Tip + Hunter + Twist Knee Buffer (10% wt)',
                term: wCad.components.survival_buffer_10pct,
                max: 0.1,
                sub: wCad.normalized_subscores.s_survival_knee,
                color: '#22c55e',
              },
              {
                label: 'Safe Failure Sequence Knee (5% wt)',
                term: wCad.components.seq_buffer_5pct,
                max: 0.05,
                sub: wCad.normalized_subscores.s_seq_knee,
                color: '#f59e0b',
              },
              {
                label: 'Buckling λ_cr + L7 Fatigue Knee (3% wt)',
                term: wCad.components.carry_stability_3pct,
                max: 0.03,
                sub: wCad.normalized_subscores.s_carry_knee,
                color: '#14b8a6',
              },
            ].map((item) => (
              <div className="comp-bar-row" key={item.label}>
                <div className="comp-bar-meta">
                  <span>{item.label}</span>
                  <strong>
                    +{item.term.toFixed(4)} <small>({(item.sub * 100).toFixed(1)}%)</small>
                  </strong>
                </div>
                <div className="progress-track">
                  <div
                    className="progress-fill"
                    style={{
                      width: `${Math.min(100, Math.max(2, (item.term / item.max) * 100))}%`,
                      background: item.color,
                    }}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {g1Terms && (
        <div className="g1-pill-row" aria-label="G1 Term Breakdown">
          <div className="g1-pill">
            <span>T_tip (0.30)</span>
            <strong>{g1Terms.T_tip.toFixed(4)}</strong>
          </div>
          <div className="g1-pill">
            <span>S_spg (0.25)</span>
            <strong>{g1Terms.S_spg.toFixed(4)}</strong>
          </div>
          <div className="g1-pill">
            <span>A_aes (0.20)</span>
            <strong>{g1Terms.A_aes.toFixed(4)}</strong>
          </div>
          <div className="g1-pill">
            <span>M_mass (0.15)</span>
            <strong>{g1Terms.M_mass.toFixed(4)}</strong>
          </div>
          <div className="g1-pill">
            <span>Q_seq (0.10)</span>
            <strong>{g1Terms.Q_seq.toFixed(4)}</strong>
          </div>
        </div>
      )}

      {gateRows.length > 0 ? (
        <div className="gate-table-wrap">
          <table className="gate-table">
            <thead>
              <tr>
                <th>Gate</th>
                <th>Measured</th>
                <th>Requirement</th>
                <th>Margin</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {gateRows.map((row) => {
                const pct = Math.min(100, Math.max(8, (row.ratio / 1.5) * 100));
                const chipClass =
                  row.status === 'pass' ? 'pass' : row.status === 'fail' ? 'fail' : 'unavailable';
                const fillClass =
                  row.status === 'pass' ? 'ok' : row.status === 'fail' ? 'bad' : 'neutral';
                return (
                  <tr key={row.key}>
                    <td className="gate-name-cell">
                      <div>{row.label}</div>
                      <small className="muted"><code>{row.sourceKey}</code></small>
                    </td>
                    <td className="mono-cell">{row.measured}</td>
                    <td className="mono-cell muted">{row.threshold}</td>
                    <td className="margin-bar-cell">
                      <div className="mini-track">
                        <div
                          className={`mini-fill ${fillClass}`}
                          style={{ width: `${pct}%` }}
                        />
                      </div>
                    </td>
                    <td>
                      <span className={`status-chip ${chipClass}`}>
                        {row.status.toUpperCase()}
                      </span>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      ) : (
        <div className="empty-scorecard-note">
          <p>
            Showing CAD Battery record for <strong>{subjectId}</strong> (
            {batteryRecord?.pass_count ?? 0}/18 battery cases pass). Select a featured wave leader
            above or upload a <code>*_master_scorecard.json</code> for full 16-gate engineering
            margins.
          </p>
        </div>
      )}
    </section>
  );
};
