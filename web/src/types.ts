export type CaseStatus = 'pass' | 'fail' | 'blocked' | 'incomplete';

export interface BatteryCase {
  id: string;
  kind: 'gate' | 'derived' | 'verifier' | 'software_contract' | 'solver_run';
  status: CaseStatus;
  solver: string | null;
  subject_sha256: string | null;
  source_commit: string;
  evidence_path: string | null;
  reason?: string | null;
  detail?: Record<string, unknown> | null;
  g_inputs?: Record<string, number> | null;
}

export interface G1Terms {
  metric: string;
  status: 'COMPLETE' | 'INCOMPLETE';
  provisional: boolean;
  terms?: {
    T_tip: number;
    S_spg: number;
    A_aes: number;
    M_mass: number;
    Q_seq: number;
  };
  missing?: string[];
  reason?: string;
}

export interface GlobalFitnessG1 {
  metric: string;
  G: number | null;
  status: 'COMPLETE' | 'INCOMPLETE';
  provisional: boolean;
  inputs: Record<string, number | null>;
  terms: G1Terms;
  title: string;
}

export interface CadBatteryRecord {
  schema: string;
  subject: string;
  role?: string;
  kind?: 'native_step' | 'crescent_genome';
  source_commit: string;
  pass_count: number;
  fail_count?: number;
  blocked_count?: number;
  incomplete_count?: number;
  solver_run_count: number;
  solver_run_objects?: number;
  global_fitness: GlobalFitnessG1;
  cases: BatteryCase[];
}

export interface WeightedCadScore {
  metric: string;
  weights: {
    w_mass: number;
    w_karambit: number;
    w_survival_buffer_knee_8pct: number;
    w_seq_buffer_knee_1p28: number;
    w_carry_stability: number;
  };
  all_engineering_gates_pass: boolean;
  W_CAD: number;
  unpenalized_quality_score: number;
  components: {
    base_feasible: number;
    mass_term_70pct: number;
    karambit_authenticity_12pct: number;
    survival_buffer_10pct: number;
    seq_buffer_5pct: number;
    carry_stability_3pct: number;
    shortfall_penalty: number;
  };
  normalized_subscores: {
    s_mass: number;
    s_karambit: number;
    s_survival_knee: number;
    s_seq_knee: number;
    s_carry_knee: number;
  };
  gates: Record<string, boolean>;
  shortfalls: Record<string, number>;
}

export interface MasterScorecard {
  schema: string;
  subject: string;
  evaluation_mode: string;
  source_commit: string;
  genome_csv: string | null;
  genome_sha256: string | null;
  cad: {
    step_path: string | null;
    step_sha256: string | null;
    stl_path: string | null;
    stl_sha256: string | null;
    units: string;
    units_source: string;
    manifold_solids: number;
    closed_shells: number;
    facets: number;
    boundary_edges: number;
    non_manifold_edges: number;
    max_directed_edge_repeat: number;
    watertight_exact: boolean;
    mass_3d_g: number;
    mass_raster_g: number;
    rel_mass_diff: number;
    cad_valid: boolean;
  };
  karambit_archetype: {
    karambit_valid: boolean;
    ring_id_mm: number;
    ring_wall_mm: number;
    c6_ring_groove_w_mm: number;
    hook_drop_mm: number;
    inside_concave_fraction: number;
    spine_convex_fraction: number;
    blade_min_in: number;
    teeth_in: number;
    teeth_sp: number;
  };
  engineering_metrics: {
    mass_g: number;
    mass_3d_g: number;
    t_stock_mm: number;
    spg_n_per_g: number;
    aes_score: number;
    aes_pass: boolean;
    lat_rob_cert_n: number;
    sf_hunter_cert: number;
    strict_lat_rob_0p0625_n: number;
    sf_hunter_strict: number;
    ring_rob_cert_n: number;
    sf_ring_cert: number;
    twist_body_sf: number;
    twist_tip_sf: number;
    twist_tipzone_sf_cert: number;
    twist_tipzone_sf_strict: number;
    solid_tip_sf_nodal_max: number;
    solid_tip_sf_nodal_avg: number;
    solid_tip_dolfinx_rel_err: number;
    seq_ratio_cert: number;
    seq_ratio_supported_a4: number;
    buckling_eigenvalue_a4: number;
    fat_sf_l7: number;
    ring_bearing_w_mm: number;
    plate_gci_fine: number | null;
    rs_v2_screen_plate_pass: boolean;
    promote_eligible: boolean;
  };
  weighted_cad_score: WeightedCadScore;
  global_fitness_G1: GlobalFitnessG1;
  cad_battery: CadBatteryRecord;
  rs_v2_gates?: Record<string, 'pass' | 'fail'> | null;
}

export interface SubjectSummary {
  id: string;
  kind: 'native_step' | 'crescent_genome';
  pass_count: number;
  fail_count: number;
  blocked_count: number;
  incomplete_count: number;
  solver_run_count: number;
  g1_score: number | null;
  g1_status: 'COMPLETE' | 'INCOMPLETE';
  mass_g: number | null;
  mass_3d_g: number | null;
  w_cad: number | null;
  all_engineering_gates_pass: boolean;
  has_scorecard: boolean;
  has_stl: boolean;
}

export interface WaveTrajectoryPoint {
  wave: string;
  subject: string;
  mass_g: number;
  mass_3d_g: number;
  sf_hunter_strict: number;
  seq_ratio: number;
  solid_tip_sf: number;
  w_cad: number;
  g1: number;
}

export interface WebStudioBundle {
  schema: string;
  source_commit: string;
  case_ids: string[];
  subjects: SubjectSummary[];
  featured_scorecards: Record<string, MasterScorecard>;
  battery_records: Record<string, CadBatteryRecord>;
  wave_trajectory: WaveTrajectoryPoint[];
}

export interface ParsedMesh3D {
  name: string;
  vertices: [number, number, number][];
  triangles: [number, number, number][];
  normals: [number, number, number][];
  facets: number;
  boundaryEdges: number;
  nonManifoldEdges: number;
  maxDirectedRepeat: number;
  watertightExact: boolean;
  volumeMm3: number;
  mass3dG: number;
  bboxMin: [number, number, number];
  bboxMax: [number, number, number];
  tipPoint: [number, number, number];
}
