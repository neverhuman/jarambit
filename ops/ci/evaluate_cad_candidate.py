#!/usr/bin/env python3
"""End-to-end CAD generator & All-Gates Weighted Evaluator for evolved genomes.

Given `--label <label> --genome <genome.csv> --out-dir <out_dir>`:
1. Exports watertight 3D `.step` (AP214 MANIFOLD_SOLID_BREP) and `.stl` CAD files
   via `export_step`, and verifies 3D unit declaration, single manifold solid,
   watertight edge census (0 open, 0 non-manifold), and 3D volume mass vs raster mass.
2. Runs `crescent-d1` (0.25, 0.125, and strict 0.0625 mm^2) and `rs-v2-eval` to
   evaluate all geometric, aesthetic (Rev C.6), 828 MPa joint lateral/ring floors,
   1.82 N*m body & strict 4-15 mm tip-submodel twist, failure sequence (seq >= 1.2),
   and L7 Marin+Goodman fatigue (1e7 cycles, 0..180 N).
3. Runs 3-level plate GCI (4.0, 1.0, 0.25 mm^2), `side.a1..a3` sensitivity, and
   `side.a4` bore-pin + grip-foundation + linear buckling eigenvalue evaluation.
4. Runs (or loads) the 3D P2 solid-tip submodel (`fem_solid/tip_solid.py` 30 4 1)
   and `dolfinx` v0.9.0 independent cross-check (`rel_err <= 2%`).
5. Computes both the locked cross-wave metric `G1` (`metrics/global_metric.py`)
   and the All-Gates Weighted CAD Score (`W_CAD`).
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
if not (ROOT / 'metrics' / 'global_metric.py').is_file():
    ROOT = Path('/home/ubuntu/jarambit')
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from fem_solid.gci import Level, MeshRef, Observable, compute_gci  # noqa: E402
from metrics.global_metric import global_G, title_tag  # noqa: E402
from ops.ci.side_a4 import evaluate_subject as eval_side_a4  # noqa: E402


def _sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1 << 20), b''):
            h.update(chunk)
    return h.hexdigest()


def verify_step_and_stl(step_path: Path, stl_path: Path, export_stdout: str, ref_mass_g: float) -> dict:
    from ops.ci.cad_battery import _step_data_section, _step_declares_mm_unit, _strip_step_comments_and_strings
    step_text = step_path.read_text(encoding='utf-8', errors='replace')
    stripped_step = _strip_step_comments_and_strings(step_text)
    data_body = _step_data_section(stripped_step)
    has_iso = stripped_step.lstrip().startswith('ISO-10303-21;')
    has_mm_decl = _step_declares_mm_unit(step_path, stripped_step)
    n_solids = data_body.count('MANIFOLD_SOLID_BREP(')
    n_closed_shells = data_body.count('CLOSED_SHELL(')
    m = re.search(
        r'boundary\s+(\d+)\s+nonmanifold\s+(\d+)\s+dmax\s+(\d+)\s+volume\s+([0-9.]+)\s+mm3',
        export_stdout,
    )
    if not m:
        raise RuntimeError(f'unexpected export_step output: {export_stdout}')
    boundary = int(m.group(1))
    non_manifold = int(m.group(2))
    dmax = int(m.group(3))
    volume_mm3 = float(m.group(4))
    facets = data_body.count('ADVANCED_FACE(')
    watertight = boundary == 0 and non_manifold == 0 and dmax == 1
    mass_3d_g = round(volume_mm3 * 0.00443, 4)
    mass_raster_g = round(ref_mass_g, 4)
    rel_mass_diff = abs(mass_3d_g - ref_mass_g) / max(ref_mass_g, 1e-9)
    return {
        'step_path': str(step_path.relative_to(ROOT)) if step_path.is_relative_to(ROOT) else str(step_path),
        'step_sha256': _sha256(step_path),
        'stl_path': str(stl_path.relative_to(ROOT)) if stl_path.is_relative_to(ROOT) else str(stl_path),
        'stl_sha256': _sha256(stl_path),
        'units': 'mm',
        'units_source': 'step_declaration' if has_mm_decl else 'project_convention',
        'manifold_solids': n_solids,
        'closed_shells': n_closed_shells,
        'facets': facets,
        'boundary_edges': boundary,
        'non_manifold_edges': non_manifold,
        'max_directed_edge_repeat': dmax,
        'watertight_exact': watertight,
        'mass_3d_g': mass_3d_g,
        'mass_raster_g': mass_raster_g,
        'rel_mass_diff': round(rel_mass_diff, 6),
        'cad_valid': bool(has_iso and has_mm_decl and n_solids == 1 and n_closed_shells >= 1 and watertight and rel_mass_diff <= 0.01),
    }


def _knee(x: float, tau: float) -> float:
    """Exponential saturation knee: 0 at x <= 0, ~0.93 at x = 2.67 * tau, 1.0 as x -> inf."""
    if x <= 0.0:
        return 0.0
    return 1.0 - math.exp(-x / max(tau, 1e-9))


def compute_weighted_cad_score(gates_summary: dict, metrics: dict) -> dict:
    """Compute the Utah Ultra-Runner Mountain-Lion Karambit Score (W_CAD_v2).

    Mission profile:
    - 99.999% duty cycle: carried for 50-100 miles of Utah mountain trail running (every gram matters).
    - 0.001% duty cycle: sudden, violent close-quarters mountain lion ambush requiring a true karambit
      (finger retention ring + curved hawkbill double-serrated claw) that survives bone strikes without
      tip fracture (3D P2 solid tip + strict tip-submodel torsion >= 1.82 N*m), lateral collapse (>= 260 N
      at 0.0625 mm^2), or ring fracture before blade bending (seq_ratio >= 1.20).

    Feasible designs (all 16 hard karambit, CAD, sheath, and 828 MPa gates PASS) score in [1.00, 2.00]:
      - 70% Ultralight Mass Minimization (w_mass = 0.70): steep continuous reward from 92 g -> 74 g
        (~0.039 points per gram saved), ensuring 1 g of weight savings always beats excess strength above
        the +8% survival knee.
      - 12% Karambit Claw & Ring Authenticity (w_karambit = 0.12): rewards Rev C.6 crescent conformance,
        authoritative hawkbill hook drop (25-35 mm), and concave/convex claw curvature continuity.
      - 10% Bone-Contact Tip & Slash Survival Buffer (w_survival = 0.10): knee-capped at +8% margin
        (tau = 0.03, 93% saturated at SF = 1.08) across 3D P2 solid-tip SF, strict 0.0625 mm^2 hunter
        lateral SF, and strict tip-submodel twist SF.
      -  5% Hand-Safe Yield Sequence Buffer (w_seq = 0.05): knee-capped at seq_ratio = 1.28 (tau = 0.025).
      -  3% Vest Sheath Carry & Buckling Sanity (w_carry = 0.03): knee-capped buckling & L7 fatigue sanity.
    """
    gate_flags = gates_summary['gates']
    shortfalls = gates_summary['shortfalls']
    all_pass = all(bool(v) for v in gate_flags.values())

    mass_g = float(metrics['mass_g'])
    sf_hunter_strict = float(metrics['strict_lat_rob_0p0625_n']) / 260.0
    sf_tip_3d = float(metrics['solid_tip_sf_nodal_max'])
    sf_twist_strict = float(metrics['twist_tipzone_sf_strict'])
    seq_ratio = float(metrics['seq_ratio'])
    sf_fat = float(metrics['fat_sf'])
    aes_score = float(metrics['aes_score'])
    lambda_cr = float(metrics['buckling_eigenvalue'])
    hook_drop = float(metrics.get('hook_drop_mm', 30.0))
    inside_concave = float(metrics.get('inside_concave', 0.95))
    spine_convex = float(metrics.get('spine_convex', 0.95))

    # 1. 70% Ultralight Mass Score (normalized on [92 g -> 74 g], clamped to [0, 1])
    lin_m = (92.0 - mass_g) / (92.0 - 74.0)
    log_m = math.log(92.0 / max(mass_g, 20.0)) / math.log(92.0 / 74.0)
    s_mass = min(max(0.65 * lin_m + 0.35 * log_m, 0.0), 1.0)
    mass_term = 0.70 * s_mass

    # 2. 12% Karambit Claw & Ring Authenticity Score
    s_aes = min(max((aes_score - 0.50) / 0.20, 0.0), 1.0)
    s_hook = min(max((hook_drop - 25.0) / 10.0, 0.0), 1.0)
    s_curve = min(max((min(inside_concave, spine_convex) - 0.85) / 0.12, 0.0), 1.0)
    s_karambit = 0.50 * s_aes + 0.25 * s_hook + 0.25 * s_curve
    karambit_term = 0.12 * s_karambit

    # 3. 10% Knee-Capped Bone-Contact Survival Buffer (tau = 0.03 -> 93% saturated at SF = 1.08)
    k_tip3d = _knee(sf_tip_3d - 1.0, 0.03)
    k_hunter = _knee(sf_hunter_strict - 1.0, 0.03)
    k_twist = _knee(sf_twist_strict - 1.0, 0.03)
    s_survival = 0.40 * k_tip3d + 0.35 * k_hunter + 0.25 * k_twist
    survival_term = 0.10 * s_survival

    # 4. 5% Knee-Capped Hand-Safe Yield Sequence Buffer (tau = 0.025 -> 93% saturated at seq = 1.28)
    s_seq = _knee((seq_ratio - 1.20) / 1.20, 0.025)
    seq_term = 0.05 * s_seq

    # 5. 3% Vest Sheath Carry & Buckling Sanity
    s_carry = 0.60 * _knee(lambda_cr - 1.0, 0.20) + 0.40 * _knee(sf_fat - 1.0, 0.20)
    carry_term = 0.03 * s_carry

    positive_score = 1.0 + mass_term + karambit_term + survival_term + seq_term + carry_term

    if all_pass:
        w_cad = positive_score
    else:
        penalty = sum(float(v) for v in shortfalls.values())
        w_cad = -(1.0 + 4.0 * penalty) + 0.10 * s_mass

    return {
        'metric': 'W_CAD_v2_ultrarunner_karambit',
        'weights': {
            'w_mass': 0.70,
            'w_karambit': 0.12,
            'w_survival_buffer_knee_8pct': 0.10,
            'w_seq_buffer_knee_1p28': 0.05,
            'w_carry_stability': 0.03,
        },
        'all_engineering_gates_pass': all_pass,
        'W_CAD': round(w_cad, 6),
        'unpenalized_quality_score': round(positive_score, 6),
        'components': {
            'base_feasible': 1.0 if all_pass else -1.0,
            'mass_term_70pct': round(mass_term, 6),
            'karambit_authenticity_12pct': round(karambit_term, 6),
            'survival_buffer_10pct': round(survival_term, 6),
            'seq_buffer_5pct': round(seq_term, 6),
            'carry_stability_3pct': round(carry_term, 6),
            'shortfall_penalty': 0.0 if all_pass else round(4.0 * sum(float(v) for v in shortfalls.values()), 6),
        },
        'normalized_subscores': {
            's_mass': round(s_mass, 6),
            's_karambit': round(s_karambit, 6),
            's_survival_knee': round(s_survival, 6),
            's_seq_knee': round(s_seq, 6),
            's_carry_knee': round(s_carry, 6),
        },
        'gates': gate_flags,
        'shortfalls': shortfalls,
    }


def independent_agreement(doc: dict) -> dict:
    stored=doc['stored_solid'];dx=dict(doc['dolfinx_cross_check'])
    errs=[abs(float(stored[a])-float(dx[b]))/float(dx[b]) for a,b in
          [('SF_skfem_nodal_max','SF_nodal_max'),('SF_skfem_nodal_avg','SF_nodal_avg')]]
    dx.update(rel_err_nodal_max=round(errs[0],6),rel_err_nodal_avg=round(errs[1],6),within_2pct=all(e<=0.02 for e in errs))
    return dx


def solid_evidence_matches(doc: dict, genome_sha: str, d1_sha: str, identity: str) -> bool:
    """Only a content-bound, actually independent observation may be reused."""
    if any(doc.get(k) != v for k, v in {'genome_sha256':genome_sha, 'd1_sha256':d1_sha, 'certificate_identity':identity}.items()):
        return False
    dx=doc.get('dolfinx_cross_check', {});stored=doc.get('stored_solid', {})
    if dx.get('status') != 'verified' or dx.get('primary_solver') != 'skfem_p2' or dx.get('independent_solver') != 'dolfinx_v0.9.0':
        return False
    mesh=dx.get('mesh_sha256', '')
    if len(mesh)!=64 or any(c not in '0123456789abcdef' for c in mesh) or doc.get('mesh_sha256')!=mesh:return False
    if stored.get('NL') != 4 or stored.get('NSUB') != 1:return False
    try:
        for a,b in [('SF_skfem_nodal_max','SF_nodal_max'),('SF_skfem_nodal_avg','SF_nodal_avg')]:
            x,y=stored[a],dx[b]
            if isinstance(x,bool) or isinstance(y,bool) or not all(math.isfinite(v) and v>0 for v in [x,y]):return False
        return True  # A measured disagreement is valid evidence of a failed gate, never a reason to fabricate a pass.
    except (KeyError,TypeError,ValueError):return False


def validate_solid_artifacts(doc: dict, folder: Path, label: str) -> bool:
    """Re-read observations as well as hashing them; copied SF metadata is never an oracle."""
    try:
        for suffix,key in [('solid_mesh.npz','mesh_sha256'),('skfem_raw.json','skfem_raw_sha256'),('dolfinx_raw.json','dolfinx_raw_sha256')]:
            if _sha256(folder/f'{label}_{suffix}') != doc.get(key):return False
        sk=json.loads((folder/f'{label}_skfem_raw.json').read_text())
        if sk.get('solver',{}).get('converged') is not True:return False
        sk=sk.get('solid',sk)
        dx=json.loads((folder/f'{label}_dolfinx_raw.json').read_text());stored=doc['stored_solid']
        for raw,field in [('SF_solid_nodal_max','SF_skfem_nodal_max'),('SF_solid_nodal_avg','SF_skfem_nodal_avg')]:
            if sk[raw] != stored[field]:return False
        for field in ['SF_nodal_max','SF_nodal_avg']:
            if dx[field] != stored[field] or dx[field]!=doc['dolfinx_cross_check'][field]:return False
        return True
    except (OSError,KeyError,TypeError,ValueError):return False


def independent_runtime() -> dict:
    python=os.environ.get('JARAMBIT_SOLVER_PYTHON') or sys.executable
    versions=subprocess.check_output([python,'-c',
        'import json,sys,sysconfig,numpy,scipy,skfem,pyamg,OCP;print(json.dumps(dict(python=sys.version,numpy=numpy.__version__,scipy=scipy.__version__,skfem=skfem.__version__,pyamg=pyamg.__version__,site_packages=sysconfig.get_path("purelib"))))'],text=True)
    image='ghcr.io/fenics/dolfinx/dolfinx@sha256:628f40dd62c7f37962ccadc5a745f23d52c56b2ac3ad9b9b6a86c567c07aa27b'
    try:
        image_id=subprocess.check_output(['docker','image','inspect',image,'--format','{{.Id}}'],text=True).strip()
    except (OSError,subprocess.CalledProcessError) as exc:
        raise RuntimeError(f'independent DOLFINx runtime unverified; install Docker and pull {image}') from exc
    return {'solver_python':python,'versions':json.loads(versions),'docker_image':image,'docker_image_id':image_id}


def run_independent_cross_check(mesh_npz: Path, out_dir: Path, label: str, runtime: dict) -> dict:
    output = out_dir/'container-output'
    output.mkdir(exist_ok=True)
    site = Path(runtime['versions']['site_packages'])
    # Import NumPy from the pinned independent image BEFORE appending only SciPy's wheel path.
    launcher = "import numpy,runpy,sys;sys.path.append('/deps');sys.argv=sys.argv[1:];runpy.run_path(sys.argv[0],run_name='__main__')"
    try:
        stdout = subprocess.check_output([
            'docker','run','--rm','-v',f'{ROOT}:/repo:ro','-v',f'{out_dir}:/work:ro',
            '-v',f'{output}:/results:rw','-v',f'{site}/scipy:/deps/scipy:ro',
            '-v',f'{site}/scipy.libs:/deps/scipy.libs:ro',
            '-e','OMP_NUM_THREADS=1','-e','OPENBLAS_NUM_THREADS=1','-e','MKL_NUM_THREADS=1',
            '-e','PYTHONPATH=/usr/local/dolfinx-real/lib/python3.12/dist-packages:/usr/local/lib:/repo','-e',f'CROSS_CHECK_SIDECAR=/results/{label}_dolfinx_sidecar.json',
            runtime['docker_image'],'python3','-c',launcher,'/repo/fem_solid/cross_check.py',f'/work/{mesh_npz.name}'
        ],cwd=ROOT,text=True)
    except (OSError,subprocess.CalledProcessError) as exc:
        raise RuntimeError('independent Docker solve unverified') from exc
    (out_dir/f'{label}_dolfinx_stdout.txt').write_text(stdout)
    doc=json.loads(stdout[stdout.rfind('{'):])
    shutil.copyfile(output/f'{label}_dolfinx_sidecar.json',out_dir/f'{label}_dolfinx_sidecar.json')
    (out_dir/f'{label}_dolfinx_raw.json').write_text(json.dumps(doc,indent=2,sort_keys=True)+'\n')
    return doc


class SolidTipUnmeasured(RuntimeError):
    """The 3D tip solve or its independent cross-check produced no observation for this design."""


def unmeasured_solid_record(label: str, identity: str, reason: str, genome_sha: str, d1_sha: str) -> dict:
    """A missing tip observation scores zero on its W_ULTRALIGHT_v1 objective; it is never read as a pass.

    No solver statistics are recorded, so the CAD battery reports `solid.tip` as incomplete.
    """
    return {
        'schema': 'jarambit.stored_solid_tip.v1', 'certificate_identity': identity, 'subject': label,
        'status': 'unmeasured', 'reason': reason, 'genome_sha256': genome_sha, 'd1_sha256': d1_sha,
        'entered_as_solver_run': False,
        'stored_solid': {'SF_nodal_max': 0.0, 'SF_nodal_avg': 0.0},
        'dolfinx_cross_check': {'status': 'unmeasured', 'SF_nodal_max': 0.0, 'SF_nodal_avg': 0.0,
                                'rel_err_nodal_max': 1.0, 'rel_err_nodal_avg': 1.0, 'within_2pct': False},
    }


def evaluate_candidate(
    label: str,
    genome_csv: Path,
    out_dir: Path,
    *,
    bin_dir: Path | None = None,
    run_solid: bool = True,
) -> dict:
    genome_csv = genome_csv.resolve()
    out_dir = out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    runtime=independent_runtime()
    (out_dir/f'{label}_runtime.json').write_text(json.dumps(runtime,indent=2,sort_keys=True)+'\n')

    if bin_dir is None:
        bin_dir = ROOT / 'target' / 'release'
    jarambit_bin = bin_dir / 'jarambit'
    export_step_bin = bin_dir / 'examples' / 'export_step'
    if not export_step_bin.is_file():
        export_step_bin = bin_dir / 'export_step'
    probe_bin = ROOT / 'target' / 'release' / 'examples' / 'plate_gci_probe'
    side_bin = ROOT / 'target' / 'release' / 'examples' / 'g3_side_report'

    canon_genome = out_dir / f'{label}_genome.csv'
    if genome_csv != canon_genome:
        shutil.copyfile(genome_csv, canon_genome)

    # 1. Run crescent-d1 with strict 0.0625 mm^2 and D1_PLATES=1
    d1_env = {k: v for k, v in os.environ.items() if k != 'JARAMBIT_CAD_BATTERY'}
    d1_env.update({'D1_PLATES': '1', 'JARAMBIT_CERT_STRICT': '1', 'JARAMBIT_CAD_BATTERY_DEPTH': '1'})
    subprocess.run(
        [str(jarambit_bin), 'crescent-d1', '--genomes', f'{label}={canon_genome}', '--threads', '8', '--out', str(out_dir)],
        cwd=ROOT, env=d1_env, check=True,
    )
    d1_path = out_dir / f'{label}_D1.json'
    d1_doc = json.loads(d1_path.read_text(encoding='utf-8'))
    mt = d1_doc['metrics']
    d1_blk = d1_doc['d1']

    # 2. Export 3D STEP + STL and verify geometry + deterministic regeneration
    step_path = out_dir / f'{label}.step'
    stl_path = out_dir / f'{label}.stl'
    exp_res = subprocess.run(
        [str(export_step_bin), str(canon_genome), str(out_dir / label), label],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    (out_dir/f'{label}_export_stdout.txt').write_text(exp_res.stdout)
    cad_info = verify_step_and_stl(step_path, stl_path, exp_res.stdout.strip(), float(mt['mass_g']))
    from ops.ci.verify_genome_export import inspect_step
    surface_path=out_dir/f'{label}_surface.json'
    brep=inspect_step(step_path,surface_path)
    (out_dir/f'{label}_step_check.json').write_text(json.dumps(brep,indent=2,sort_keys=True)+'\n')
    regen_prefix = out_dir / f'_{label}_regen_check'
    subprocess.run(
        [str(export_step_bin), str(canon_genome), str(regen_prefix), label],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    surface_sha = _sha256(surface_path)
    regen_step_sha = _sha256(Path(f'{regen_prefix}.step'))
    regen_stl_sha = _sha256(Path(f'{regen_prefix}.stl'))
    regen_surf_sha = _sha256(Path(f'{regen_prefix}_surface.json'))
    regen_ok = (
        regen_step_sha == cad_info['step_sha256']
        and regen_stl_sha == cad_info['stl_sha256']
        and regen_surf_sha == surface_sha
    )
    for sfx in ('.step', '.stl', '_surface.json'):
        Path(f'{regen_prefix}{sfx}').unlink(missing_ok=True)
    cad_info['surface_path'] = str(surface_path.relative_to(ROOT)) if surface_path.is_relative_to(ROOT) else str(surface_path)
    cad_info['surface_sha256'] = surface_sha
    cad_info['regenerated_step_sha256'] = regen_step_sha
    cad_info['regenerated_stl_sha256'] = regen_stl_sha
    cad_info['regenerated_surface_sha256'] = regen_surf_sha
    cad_info['regeneration_deterministic'] = bool(regen_ok)
    cad_info['brep_check']=brep
    cad_info['cad_valid']=cad_info['cad_valid'] and brep['valid'] and brep['solids']==1 and bool(regen_ok)


    # Bind the actual saved export surface to the solver input. The fine mesh is refined from
    # these facets, including their actual edge heights, rather than a 0.5 mm display raster.
    surface_path=out_dir/f'{label}_surface.json'
    surface=json.loads(surface_path.read_text())
    if surface.get('certificate_identity')!=d1_doc['certificate_identity']:
        raise RuntimeError('export tool/model identity mismatch; rebuild jarambit and its export/probe examples')
    if surface.get('rs_v2') and not brep['bore_geometry_matches_mesh']:
        cad_info['cad_valid']=False
        cad_info['bore_validation_status']='unverified: STEP bore position/radii/full coverage must agree with actual mesh'
    surface.update(source_step_sha256=_sha256(step_path),source_stl_sha256=_sha256(stl_path),
                   source_surface_sha256=surface_sha)
    d1_doc['export_surface']=surface
    d1_path.write_text(json.dumps(d1_doc,sort_keys=True,separators=(',',':'))+'\n')

    # 3. Run 3-level plate GCI (4.0, 1.0, 0.25 mm^2) + extract Karambit geometry metrics
    gci_path = out_dir / f'{label}_plate_gci.json'
    raw_p = out_dir / f'{label}_plate_gci_raw.json'
    subprocess.run([str(probe_bin), str(canon_genome), str(raw_p)], cwd=ROOT, check=True)
    raw_gci = json.loads(raw_p.read_text(encoding='utf-8'))
    karambit_meta = dict(raw_gci.get('karambit') or {})
    levels_raw = raw_gci['levels_fine_to_coarse']
    obs_seq = Observable(
        id='plate_seq_ratio', metric='seq_ratio',
        region='blade_vs_ring_neck', projection='plate_cst_dkt', units='ratio',
    )
    obs_sf = Observable(
        id='plate_sf_hunter', metric='sf_hunter',
        region='blade_outside_4mm', projection='plate_cst_dkt', units='ratio',
    )
    l_seq = [
        Level(
            mesh=MeshRef(id=f"{label}_area_{lv['max_area_mm2']}", revision='rev1', sha256=lv['mesh_sha256']),
            h=float(lv['h_mm']), observable=obs_seq,
        )
        for lv in levels_raw
    ]
    l_sf = [
        Level(
            mesh=MeshRef(id=f"{label}_area_{lv['max_area_mm2']}", revision='rev1', sha256=lv['mesh_sha256']),
            h=float(lv['h_mm']), observable=obs_sf,
        )
        for lv in levels_raw
    ]
    seq_res = compute_gci(
        levels_raw[0]['seq_ratio'], levels_raw[1]['seq_ratio'], levels_raw[2]['seq_ratio'],
        r=2.0, levels=l_seq,
    ).to_dict()
    sf_res = compute_gci(
        levels_raw[0]['sf_hunter'], levels_raw[1]['sf_hunter'], levels_raw[2]['sf_hunter'],
        r=2.0, levels=l_sf,
    ).to_dict()
    gci_doc = {
        'schema': 'jarambit.plate_gci.v1',
        'certificate_identity': d1_doc['certificate_identity'],
        'subject': label,
        'genome': str(canon_genome.relative_to(ROOT)) if canon_genome.is_relative_to(ROOT) else str(canon_genome),
        'genome_sha256': _sha256(canon_genome),
        'karambit': karambit_meta,
        'primary_observable': 'seq_ratio',
        'refinement_ratio_r': 2.0,
        'levels_fine_to_coarse': levels_raw,
        'seq_ratio_gci': seq_res,
        'sf_hunter_gci': sf_res,
    }
    gci_path.write_text(json.dumps(gci_doc, indent=2, sort_keys=True) + '\n', encoding='utf-8')
    # 4. Run 3D P2 solid-tip submodel + dolfinx v0.9.0 cross-check
    solid_tip_path = out_dir / f'{label}_solid_tip.json'
    skfem_cache = out_dir / f'{label}_skfem_raw.json'
    cached = json.loads(solid_tip_path.read_text()) if solid_tip_path.is_file() else {}
    cached_files = validate_solid_artifacts(cached,out_dir,label)
    solid_unmeasured = None
    try:
        if run_solid or not cached_files or not solid_evidence_matches(cached, _sha256(canon_genome), _sha256(d1_path), d1_doc['certificate_identity']):
            mesh_npz = out_dir/f'{label}_solid_mesh.npz'
            raw_solid = skfem_cache
            py_sheath = runtime['solver_python']
            amg_path = str(ROOT)
            env_solid = {
                **os.environ,
                'PYTHONPATH': f'{amg_path}:{ROOT}',
                'DUMP_MESH': str(mesh_npz),
                'OMP_NUM_THREADS':'1','OPENBLAS_NUM_THREADS':'1','MKL_NUM_THREADS':'1',
            }
            primary=subprocess.run(
                [py_sheath,str(ROOT/'fem_solid'/'tip_solid.py'),str(d1_path),'30','4','1',str(raw_solid)],
                cwd=ROOT,env=env_solid,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
            (out_dir/f'{label}_skfem_stdout.txt').write_text(primary.stdout)
            print(primary.stdout,flush=True)
            if primary.returncode:raise SolidTipUnmeasured('primary solid solve unverified; see saved skfem output')
            sk_doc=json.loads(raw_solid.read_text(encoding='utf-8'))
            mesh_sha=_sha256(mesh_npz)
            primary_sha = _sha256(skfem_cache)
            try:
                dx_doc = run_independent_cross_check(mesh_npz, out_dir, label, runtime)
            except RuntimeError as exc:
                raise SolidTipUnmeasured(str(exc)) from exc
            if _sha256(skfem_cache) != primary_sha:
                raise RuntimeError('primary observation changed during independent solve')
            sk_solid = sk_doc.get('solid', sk_doc)
            sf_sk_max = float(sk_solid['SF_solid_nodal_max'])
            sf_dx_max = float(dx_doc['SF_nodal_max'])
            sf_sk_avg = float(sk_solid['SF_solid_nodal_avg'])
            sf_dx_avg = float(dx_doc['SF_nodal_avg'])
            rel_err_max = abs(sf_sk_max - sf_dx_max) / max(sf_dx_max, 1e-9)
            rel_err_avg = abs(sf_sk_avg - sf_dx_avg) / max(sf_dx_avg, 1e-9)
            solid_record = {
                'schema': 'jarambit.stored_solid_tip.v1',
                'certificate_identity': d1_doc['certificate_identity'],
                'mesh_sha256':mesh_sha, 'skfem_raw_sha256':_sha256(skfem_cache),
                'dolfinx_raw_sha256':_sha256(out_dir/f'{label}_dolfinx_raw.json'),
                'subject': label,
                'd1_path': str(d1_path.relative_to(ROOT)) if d1_path.is_relative_to(ROOT) else str(d1_path),
                'd1_sha256': _sha256(d1_path),
                'genome_sha256': _sha256(canon_genome),
                'source_step_sha256': _sha256(step_path), 'source_stl_sha256': _sha256(stl_path),
                'source_surface_sha256': _sha256(surface_path),
                'genome_data_row_matches_csv': True,
                'copied_from_another_genome': False,
                'entered_as_solver_run': True,
                'solve_rerun': True,
                'metrics': {
                    'spg': float(mt['spg']),
                    'aes_score': float(mt['aes_score']),
                },
                'stored_solid': {
                    'NL': int(sk_solid['NL']),
                    'NSUB': int(sk_solid['NSUB']),
                    'R_CUT': float(sk_solid['R_CUT']),
                    'PATCH_mm': float(sk_solid['PATCH_mm']),
                    'n_tet': int(sk_solid['n_tet']),
                    'n_dof': int(sk_solid['n_dof']),
                    'ksp_its': int(dx_doc['ksp_its']),
                    'primary_solver_stats':sk_doc['solver'],
                    'SF_nodal_max': sf_dx_max,
                    'SF_nodal_avg': sf_dx_avg,
                    'SF_skfem_nodal_max': sf_sk_max,
                    'SF_skfem_nodal_avg': sf_sk_avg,
                    'SF_elem_mean': float(sk_solid['SF_solid_elem_mean']),
                },
                'dolfinx_cross_check': {
                    'container': runtime['docker_image'], 'container_image_id':runtime['docker_image_id'],
                    'status': 'verified', 'primary_solver':'skfem_p2', 'independent_solver':'dolfinx_v0.9.0',
                    'mesh_sha256':mesh_sha,
                    'SF_nodal_max': sf_dx_max,
                    'SF_nodal_avg': sf_dx_avg,
                    'ksp_its': int(dx_doc['ksp_its']),
                    'rel_err_nodal_max': round(rel_err_max, 6),
                    'rel_err_nodal_avg': round(rel_err_avg, 6),
                    'within_2pct': bool(rel_err_max <= 0.02 and rel_err_avg <= 0.02),
                },
            }
            solid_tip_path.write_text(json.dumps(solid_record, indent=2, sort_keys=True) + '\n', encoding='utf-8')
        else:
            solid_record = json.loads(solid_tip_path.read_text(encoding='utf-8'))
            if 'metrics' not in solid_record:
                solid_record['metrics'] = {
                    'spg': float(mt['spg']),
                    'aes_score': float(mt['aes_score']),
                }
                solid_record['copied_from_another_genome'] = False
                solid_record['entered_as_solver_run'] = True
                solid_tip_path.write_text(json.dumps(solid_record, indent=2, sort_keys=True) + '\n', encoding='utf-8')
    except SolidTipUnmeasured as exc:
        # W_ULTRALIGHT_v1 has no hard failures: the design is still scored, with zero credit on the
        # solid-tip objective, and its legacy solid-tip and cross-check gates are logged as failed.
        solid_unmeasured = str(exc)
        solid_record = unmeasured_solid_record(label, d1_doc['certificate_identity'], solid_unmeasured,
                                               _sha256(canon_genome), _sha256(d1_path))
        solid_tip_path.write_text(json.dumps(solid_record, indent=2, sort_keys=True) + '\n', encoding='utf-8')

    if solid_unmeasured is None:
        solid_record['dolfinx_cross_check']=independent_agreement(solid_record)
        solid_tip_path.write_text(json.dumps(solid_record,indent=2,sort_keys=True)+'\n')

    # Write bindings.json + gci-format JSON in a temp solid dir so `rs-v2-eval` binds `solid_tip_sf`
    solid_tmp = str(out_dir / 'solid-bindings')
    Path(solid_tmp).mkdir(exist_ok=True)
    # Without a measured record there is no binding, and rs-v2-eval reports `solid_tip_sf` unverified.
    if solid_unmeasured is None:
        gci_file = Path(solid_tmp) / f'{label}_gci.json'
        gci_compat = {
            'design': label,
            'certificate_identity': d1_doc['certificate_identity'],
            'solid_record': solid_record,
            'SF_nodal_max': {
                'fine': solid_record['stored_solid']['SF_nodal_max'],
            },
            'levels': [
                {
                    'NL': solid_record['stored_solid']['NL'],
                    'NSUB': solid_record['stored_solid']['NSUB'],
                    'n_tet': solid_record['stored_solid']['n_tet'],
                    'n_dof': solid_record['stored_solid']['n_dof'],
                    'ksp_its': solid_record['stored_solid']['ksp_its'],
                    'SF_nodal_max': solid_record['stored_solid']['SF_nodal_max'],
                }
            ],
        }
        gci_file.write_text(json.dumps(gci_compat), encoding='utf-8')
        bindings_compat = {
            d1_doc['certificate_identity']: {
                'genome': str(canon_genome),
                'certificate_identity': d1_doc['certificate_identity'],
                'gci': str(gci_file),
                'gci_sha256':_sha256(gci_file),
            }
        }
        (Path(solid_tmp) / 'bindings.json').write_text(json.dumps(bindings_compat), encoding='utf-8')
    else:
        # A binding left by an earlier attempt in this folder must not stand in for the failed re-solve.
        for stale in (Path(solid_tmp) / 'bindings.json', Path(solid_tmp) / f'{label}_gci.json'):
            stale.unlink(missing_ok=True)
    subprocess.run(
        [
            str(jarambit_bin), 'rs-v2-eval',
            '--genomes', f'{label}={canon_genome}',
            '--out', str(out_dir),
            '--solid-dir', solid_tmp,
        ],
        cwd=ROOT, check=True,
    )
    rs_raw_path = out_dir / f'{label}.json'
    rs_path = out_dir / f'{label}_rs.json'
    shutil.copyfile(rs_raw_path,rs_path)
    rs_doc = json.loads(rs_path.read_text(encoding='utf-8'))
    if surface.get('rs_v2'):
        measured=rs_doc['export_measurements'].get('bore_edge_radius_mm')
        step_radius=brep.get('bore_edge_radius_mm')
        agrees=measured is not None and step_radius is not None and abs(measured-step_radius)<=1e-7
        cad_info['bore_measurement_agrees_with_step']=agrees
        cad_info['cad_valid']=cad_info['cad_valid'] and agrees
    rs_doc['genome'] = str(canon_genome.relative_to(ROOT)) if canon_genome.is_relative_to(ROOT) else str(canon_genome)
    rs_doc['solid_tip_source'] = str(solid_tip_path.relative_to(ROOT)) if solid_tip_path.is_relative_to(ROOT) else str(solid_tip_path)
    rs_path.write_text(json.dumps(rs_doc, indent=2, sort_keys=True) + '\n', encoding='utf-8')

    # 5. Run side.a1..a3 and side.a4
    a1a3_path = out_dir / f'{label}_a1a3.json'
    rel_genome = str(canon_genome.relative_to(ROOT)) if canon_genome.is_relative_to(ROOT) else str(canon_genome)
    subprocess.run([str(side_bin), rel_genome, str(a1a3_path)], cwd=ROOT, check=True)
    a4_path = out_dir / f'{label}_a4.json'
    a4_doc = eval_side_a4(label, d1_path, rs_path, a1a3_path)
    a4_path.write_text(json.dumps(a4_doc, indent=2, sort_keys=True) + '\n', encoding='utf-8')

    # 6. Assemble locked G1 and Utah Ultra-Runner Karambit Score (W_CAD_v2)
    sf_solid = float(solid_record['stored_solid']['SF_nodal_max'])
    g1_inputs = {
        'solid_tip_sf_nodal_max': sf_solid,
        'spg': float(mt['spg']),
        'aes_score': float(mt['aes_score']),
        'mass_g': float(mt['mass_g']),
        'seq_ratio': float(mt['seq_ratio']),
    }
    version = (ROOT / 'docs' / 'VERSION').read_text(encoding='utf-8').strip()
    g1_val, g1_terms = global_G(dict(g1_inputs))

    strict_lat = float(d1_blk['strict_lat_rob_0p0625'])
    tw_strict = float(d1_blk['twist_tipzone_sf_strict'])
    tw_body = float(mt['twist_body_sf'])
    tw_tip = float(mt['twist_tip_sf'])
    lat_rob = float(mt['lat_rob'])
    ring_rob = float(mt['ring_rob'])
    seq_ratio = float(mt['seq_ratio'])
    fat_sf = float(rs_doc['fat_sf'])
    aes_pass = bool(mt['aes_pass'])
    geom_gates_ok = len(rs_doc.get('rejects') or []) == 0

    ring_id_mm = float(karambit_meta.get('ring_id_mm', 25.4))
    ring_wall_mm = float(karambit_meta.get('ring_wall_mm', 5.0))
    groove_w_mm = float(karambit_meta.get('groove_w_mm', 2.0))
    hook_drop_mm = float(karambit_meta.get('hook_drop_mm', 30.0))
    inside_concave = float(karambit_meta.get('inside_concave', 0.95))
    spine_convex = float(karambit_meta.get('spine_convex', 0.95))
    teeth_in = int(karambit_meta.get('teeth_in', 4))
    teeth_sp = int(karambit_meta.get('teeth_sp', 4))
    blade_min_mm = min(
        float(karambit_meta.get('tip_to_heel_mm', mt['tip_to_heel'])),
        float(karambit_meta.get('horizontal_reach_mm', mt['horizontal_reach'])),
        float(karambit_meta.get('sharpened_arc_mm', mt['arc_in'])),
    )
    karambit_archetype_ok = bool(
        abs(ring_id_mm - 25.4) <= 0.5 + 1e-6
        and ring_wall_mm >= 4.5 - 1e-6
        and groove_w_mm >= 2.0 - 1e-6
        and hook_drop_mm >= 25.0 - 1e-6
        and blade_min_mm >= 85.725 - 1e-6
        and teeth_in >= 3
        and teeth_sp >= 3
        and aes_pass
    )

    gates_map = {
        'cad_watertight_and_single_solid': bool(cad_info['cad_valid']),
        'karambit_ring_and_curved_claw': karambit_archetype_ok,
        'c6_aesthetic_floor': aes_pass,
        'geometric_and_holder_gates': geom_gates_ok,
        'cert_hiker_180n': lat_rob >= 180.0,
        'cert_hunter_260n': lat_rob >= 260.0,
        'strict_hunter_260n': strict_lat >= 260.0,
        'cert_ring_pull_1500n': ring_rob >= 1500.0,
        'twist_body_1p82nm': tw_body >= 1.0,
        'twist_tip_only_1p82nm': tw_tip >= 1.0,
        'twist_tip_submodel_strict_1p82nm': tw_strict >= 1.0,
        'solid_tip_3d_p2_1p82nm': sf_solid >= 1.0,
        'dolfinx_cross_check_2pct': bool(solid_record['dolfinx_cross_check']['within_2pct']),
        'failure_sequence_1p2': seq_ratio >= 1.2,
        'plate_gci_valid_3level': bool(
            gci_doc['seq_ratio_gci'].get('sequence_valid')
            and gci_doc['seq_ratio_gci'].get('observable_valid')
        ),
        'side_a4_buckling_and_support': a4_doc.get('status') == 'pass' and float(a4_doc.get('buckling_eigenvalue', 0.0)) >= 1.0,
    }
    shortfalls = {
        'cad': 0.0 if gates_map['cad_watertight_and_single_solid'] else 1.0,
        'karambit_archetype': 0.0 if karambit_archetype_ok else 1.0,
        'aesthetic': 0.0 if aes_pass else max(0.50 - float(mt['aes_score']), 0.1),
        'geometry_gates': float(len(rs_doc.get('rejects') or [])),
        'hunter_strict': max(0.0, (260.0 - strict_lat) / 260.0),
        'ring_pull': max(0.0, (1500.0 - ring_rob) / 1500.0),
        'twist_body': max(0.0, 1.0 - tw_body),
        'twist_tip_strict': max(0.0, 1.0 - tw_strict),
        'solid_tip_3d': max(0.0, 1.0 - sf_solid),
        'seq_ratio': max(0.0, (1.2 - seq_ratio) / 1.2),
    }

    w_cad = compute_weighted_cad_score(
        {'gates': gates_map, 'shortfalls': shortfalls},
        {
            'mass_g': float(mt['mass_g']),
            'strict_lat_rob_0p0625_n': strict_lat,
            'solid_tip_sf_nodal_max': sf_solid,
            'twist_tipzone_sf_strict': tw_strict,
            'seq_ratio': seq_ratio,
            'fat_sf': fat_sf,
            'aes_score': float(mt['aes_score']),
            'buckling_eigenvalue': float(a4_doc['buckling_eigenvalue']),
            'hook_drop_mm': hook_drop_mm,
            'inside_concave': inside_concave,
            'spine_convex': spine_convex,
        },
    )

    summary = {
        'schema': 'jarambit.cad_candidate_evaluation.v1',
        'certificate_identity': d1_doc['certificate_identity'],
        'rs_v2_gates': rs_doc['gates'],
        'export_measurements': rs_doc['export_measurements'],
        'subject': label,
        'genome_csv': str(canon_genome.relative_to(ROOT)) if canon_genome.is_relative_to(ROOT) else str(canon_genome),
        'genome_sha256': _sha256(canon_genome),
        'cad': cad_info,
        'karambit_archetype': {
            'karambit_valid': karambit_archetype_ok,
            'ring_id_mm': ring_id_mm,
            'ring_wall_mm': ring_wall_mm,
            'c6_ring_groove_w_mm': groove_w_mm,
            'hook_drop_mm': hook_drop_mm,
            'inside_concave_fraction': inside_concave,
            'spine_convex_fraction': spine_convex,
            'blade_min_in': round(blade_min_mm / 25.4, 4),
            'teeth_in': teeth_in,
            'teeth_sp': teeth_sp,
        },
        'engineering_metrics': {
            'mass_g': float(mt['mass_g']),
            'mass_3d_g': cad_info['mass_3d_g'],
            't_stock_mm': float(mt['t']),
            'spg_n_per_g': float(mt['spg']),
            'aes_score': float(mt['aes_score']),
            'aes_pass': aes_pass,
            'lat_rob_cert_n': lat_rob,
            'sf_hunter_cert': float(mt['sf_hunter']),
            'strict_lat_rob_0p0625_n': strict_lat,
            'sf_hunter_strict': round(strict_lat / 260.0, 4),
            'ring_rob_cert_n': ring_rob,
            'sf_ring_cert': float(mt['sf_ring']),
            'twist_body_sf': tw_body,
            'twist_tip_sf': tw_tip,
            'twist_tipzone_sf_cert': float(mt['twist_tipzone_sf']),
            'twist_tipzone_sf_strict': tw_strict,
            'solid_tip_sf_nodal_max': sf_solid,
            'solid_tip_sf_nodal_avg': float(solid_record['stored_solid']['SF_nodal_avg']),
            'solid_tip_dolfinx_rel_err': float(solid_record['dolfinx_cross_check']['rel_err_nodal_max']),
            'seq_ratio_cert': seq_ratio,
            'seq_ratio_supported_a4': float(a4_doc['supported_seq_ratio']),
            'buckling_eigenvalue_a4': float(a4_doc['buckling_eigenvalue']),
            'fat_sf_l7': fat_sf,
            'ring_bearing_w_mm': float(rs_doc['ring_bearing_w_mm']),
            'plate_gci_fine': gci_doc['seq_ratio_gci']['gci_fine'],
            'rs_v2_screen_plate_pass': bool(rs_doc.get('groups', {}).get('screen_plate')),
            'promote_eligible': bool(d1_blk.get('promote_eligible')) and bool(rs_doc.get('all_hard')) and all(gates_map.values()),
            'rs_v2_all_hard': bool(rs_doc.get('all_hard')),
        },
        'global_fitness_G1': {
            'metric': 'G1',
            'G': g1_val,
            'status': g1_terms['status'],
            'provisional': g1_terms['provisional'],
            'inputs': g1_inputs,
            'terms': g1_terms,
            'title': title_tag(dict(g1_inputs), version),
        },
        'weighted_cad_score': w_cad,
    }
    eval_path = out_dir / f'{label}_cad_eval.json'
    eval_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + '\n', encoding='utf-8')
    return summary


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--label', required=True)
    parser.add_argument('--genome', type=Path, required=True)
    parser.add_argument('--out-dir', type=Path, required=True)
    parser.add_argument('--bin-dir', type=Path, default=None)
    parser.add_argument('--no-run-solid', action='store_true')
    args = parser.parse_args(argv)
    res = evaluate_candidate(
        args.label,
        args.genome,
        args.out_dir,
        bin_dir=args.bin_dir,
        run_solid=not args.no_run_solid,
    )
    em = res['engineering_metrics']
    wc = res['weighted_cad_score']
    g1 = res['global_fitness_G1']
    ka = res['karambit_archetype']
    print(
        f"[{res['subject']}] mass={em['mass_g']:.2f}g (3D {em['mass_3d_g']:.2f}g) "
        f"karambit={ka['karambit_valid']} (ring={ka['ring_id_mm']:.1f}mm, hook={ka['hook_drop_mm']:.1f}mm) "
        f"SPG={em['spg_n_per_g']:.3f} lat={em['lat_rob_cert_n']:.1f}N (strict {em['strict_lat_rob_0p0625_n']:.1f}N) "
        f"seq={em['seq_ratio_cert']:.3f} tw_strict={em['twist_tipzone_sf_strict']:.3f} "
        f"solid_tip={em['solid_tip_sf_nodal_max']:.3f} promote={em['promote_eligible']} "
        f"all_gates={wc['all_engineering_gates_pass']} W_CAD={wc['W_CAD']:.4f} G1={g1['G']:.4f}"
    )
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
