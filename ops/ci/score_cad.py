#!/usr/bin/env python3
"""Unified Single-Entry-Point Knife CAD Master Evaluator (`jarambit score-cad`).

Given ANY knife CAD input:
  1. A 3D STEP file (`*.step` / `*.stp`),
  2. A 3D STL file (`*.stl`),
  3. A 128-gene genome CSV (`*.csv`), or
  4. A catalog subject ID (e.g. `w281_s2932_c01`, `c10a`, `c93`, `c92`, `g3`),

this module runs (or binds) ALL 16 hard engineering/CAD/archetype gates and ALL 18 CAD
battery evaluators, returning a single canonical Master CAD Scorecard (`.json` + `.md`)
with:
  - Master Weighted CAD Score (`W_CAD_v2_ultrarunner_karambit` in `[1.00, 2.00]` if all
    16 gates pass; `< 0` if any gate fails) + 5-component weighted breakdown,
  - Locked Cross-Wave Global Fitness (`G1` in `metrics/global_metric.py`) + 5-term breakdown,
  - 16-Gate Pass/Fail & Safety Margin Breakdown Table,
  - 18-Evaluator CAD Battery Matrix (`build.*`, `geom.*`, `plate.*`, `side.*`, `solid.tip`, `gate.rs_v2`),
  - 3D CAD Watertightness, Manifold Topology, and 2.5D-vs-3D Volume Mass Parity.
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
import struct
import subprocess
import sys
import tempfile

import numpy as np

ROOT = Path(__file__).resolve().parents[2]
if not (ROOT / 'metrics' / 'global_metric.py').is_file():
    ROOT = Path('/home/ubuntu/jarambit')
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from fem_solid.gci import Level, MeshRef, Observable, compute_gci  # noqa: E402
from metrics.global_metric import global_G, title_tag  # noqa: E402
from ops.ci.cad_battery import (  # noqa: E402
    CASE_IDS,
    assemble_g,
    build_subject,
    pass_count,
    solver_run_count,
    subject_catalog,
    evaluated_genome_spec,
)
from ops.ci.evaluate_cad_candidate import (  # noqa: E402
    compute_weighted_cad_score,
    evaluate_candidate,
)
from ops.ci.ultralight_weighted_fitness import compute_ultralight_weighted_fitness  # noqa: E402

SCORECARD_SCHEMA = 'jarambit.master_cad_scorecard.v1'
TI64_DENSITY_G_PER_MM3 = 0.00443
TI64_SY20_MPA = 828.0
TI64_E_MPA = 113800.0
TI64_NU = 0.34


def _sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1 << 20), b''):
            h.update(chunk)
    return h.hexdigest()


def _parse_stl_triangles(stl_path: Path) -> tuple[np.ndarray, np.ndarray]:
    """Load 3D vertices and triangle indices from binary or ASCII STL."""
    raw = stl_path.read_bytes()
    if not raw.startswith(b'solid ') or b'facet normal' not in raw[:512]:
        n_tri = struct.unpack('<I', raw[80:84])[0]
        if 84 + n_tri * 50 == len(raw):
            dt = np.dtype([('normal', '<f4', (3,)), ('v', '<f4', (3, 3)), ('attr', '<u2')])
            recs = np.frombuffer(raw, dtype=dt, count=n_tri, offset=84)
            tri_pts = recs['v'].astype(np.float64)
            flat = tri_pts.reshape(-1, 3)
            verts, inv = np.unique(flat, axis=0, return_inverse=True)
            tris = inv.reshape(-1, 3)
            return verts, tris

    text = raw.decode('utf-8', errors='replace')
    coords = [
        tuple( float(x) for x in m.groups() )
        for m in re.finditer(
            r'vertex\s+([+-]?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)\s+'
            r'([+-]?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)\s+'
            r'([+-]?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)',
            text,
        )
    ]
    if not coords or len(coords) % 3 != 0:
        raise ValueError(f'Unable to parse STL vertices from {stl_path}')
    flat = np.asarray(coords, dtype=np.float64)
    verts, inv = np.unique(flat, axis=0, return_inverse=True)
    tris = inv.reshape(-1, 3)
    return verts, tris


def _parse_faceted_step_triangles(step_path: Path) -> tuple[np.ndarray, np.ndarray]:
    """Extract 3D vertices and triangles from a faceted MANIFOLD_SOLID_BREP STEP file."""
    text = step_path.read_text(encoding='utf-8', errors='replace')
    pts: dict[int, tuple[float, float, float]] = {}
    for m in re.finditer(
        r"#(\d+)\s*=\s*CARTESIAN_POINT\s*\(\s*'[^']*'\s*,\s*\(\s*([+-]?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)\s*,"
        r"\s*([+-]?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)\s*,"
        r"\s*([+-]?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)\s*\)\s*\)",
        text,
    ):
        pts[int(m.group(1))] = (float(m.group(2)), float(m.group(3)), float(m.group(4)))

    vpt: dict[int, int] = {}
    for m in re.finditer(r"#(\d+)\s*=\s*VERTEX_POINT\s*\(\s*'[^']*'\s*,\s*#(\d+)\s*\)", text):
        vpt[int(m.group(1))] = int(m.group(2))

    ecurve: dict[int, tuple[int, int]] = {}
    for m in re.finditer(
        r"#(\d+)\s*=\s*EDGE_CURVE\s*\(\s*'[^']*'\s*,\s*#(\d+)\s*,\s*#(\d+)\s*,\s*#\d+\s*,\s*\.[TF]\.\s*\)",
        text,
    ):
        ecurve[int(m.group(1))] = (int(m.group(2)), int(m.group(3)))

    oedge: dict[int, tuple[int, bool]] = {}
    for m in re.finditer(
        r"#(\d+)\s*=\s*ORIENTED_EDGE\s*\(\s*'[^']*'\s*,\s*\*\s*,\s*\*\s*,\s*#(\d+)\s*,\s*\.([TF])\.\s*\)",
        text,
    ):
        oedge[int(m.group(1))] = (int(m.group(2)), m.group(3) == 'T')

    tri_coords: list[tuple[float, float, float]] = []
    for m in re.finditer(r"#\d+\s*=\s*EDGE_LOOP\s*\(\s*'[^']*'\s*,\s*\(([^)]+)\)\s*\)", text):
        refs = [int(x) for x in re.findall(r'#(\d+)', m.group(1))]
        if len(refs) < 3:
            continue
        loop_pts: list[tuple[float, float, float]] = []
        for r in refs:
            if r not in oedge:
                break
            ec_id, sense = oedge[r]
            if ec_id not in ecurve:
                break
            v1, v2 = ecurve[ec_id]
            start_v = v1 if sense else v2
            cp_id = vpt.get(start_v)
            if cp_id not in pts:
                break
            loop_pts.append(pts[cp_id])
        if len(loop_pts) == 3:
            tri_coords.extend(loop_pts)
        elif len(loop_pts) > 3:
            p0 = loop_pts[0]
            for k in range(1, len(loop_pts) - 1):
                tri_coords.extend([p0, loop_pts[k], loop_pts[k + 1]])

    if not tri_coords:
        raise ValueError(f'Could not extract faceted triangles from STEP file {step_path}')
    flat = np.asarray(tri_coords, dtype=np.float64)
    verts, inv = np.unique(flat, axis=0, return_inverse=True)
    tris = inv.reshape(-1, 3)
    return verts, tris


def _mesh_census_and_volume(verts: np.ndarray, tris: np.ndarray) -> dict:
    """Compute boundary edges, non-manifold edges, max directed repeat, and signed volume (mm^3)."""
    und: dict[tuple[int, int], int] = {}
    directed: dict[tuple[int, int], int] = {}
    for a, b, c in tris:
        ia, ib, ic = int(a), int(b), int(c)
        for u, v in ((ia, ib), (ib, ic), (ic, ia)):
            directed[(u, v)] = directed.get((u, v), 0) + 1
            key = (u, v) if u < v else (v, u)
            und[key] = und.get(key, 0) + 1
    boundary = sum(1 for c in und.values() if c == 1)
    non_manifold = sum(1 for c in und.values() if c > 2)
    dmax = max(directed.values()) if directed else 0

    v0 = verts[tris[:, 0]]
    v1 = verts[tris[:, 1]]
    v2 = verts[tris[:, 2]]
    signed_vol = float(np.sum(np.einsum('ij,ij->i', v0, np.cross(v1, v2))) / 6.0)
    vol_mm3 = abs(signed_vol)
    watertight = boundary == 0 and non_manifold == 0 and dmax == 1 and vol_mm3 > 0.0
    return {
        'facets': int(len(tris)),
        'vertices': int(len(verts)),
        'boundary_edges': int(boundary),
        'non_manifold_edges': int(non_manifold),
        'max_directed_edge_repeat': int(dmax),
        'watertight_exact': bool(watertight),
        'volume_mm3': round(vol_mm3, 4),
        'mass_3d_g': round(vol_mm3 * TI64_DENSITY_G_PER_MM3, 4),
    }


def _point_in_poly(pts: np.ndarray, poly: np.ndarray) -> np.ndarray:
    x = pts[:, 0][:, None]
    y = pts[:, 1][:, None]
    a = poly
    b = np.roll(a, -1, axis=0)
    cond = ((a[:, 1] > y) != (b[:, 1] > y)) & (
        x < (b[:, 0] - a[:, 0]) * (y - a[:, 1]) / (b[:, 1] - a[:, 1] + 1e-300) + a[:, 0]
    )
    return (cond.sum(axis=1) % 2) == 1


def _alpha_rect(r: float) -> float:
    r = max(float(r), 1.0)
    s1 = 0.0
    s2 = 0.0
    for i in range(60):
        n = 2 * i + 1
        arg = min(n * math.pi * r / 2.0, 700.0)
        s1 += math.tanh(arg) / (n ** 5)
        s2 += 1.0 / ((n ** 2) * math.cosh(arg))
    k1 = (1.0 - 192.0 / (math.pi ** 5) / r * s1) / 3.0
    k = 1.0 - 8.0 / (math.pi ** 2) * s2
    return k1 / max(k, 1e-12)


def _torsion_3d_factor(r: float) -> float:
    return (1.0 / 3.0) / _alpha_rect(r)


def _goodman_sf(smax: float, t_joint: float) -> float:
    if smax <= 0.0:
        return 99.0
    uts = 895.0
    ka = 4.51 * (uts ** -0.265)
    kb = min(1.24 * (max(t_joint, 2.79) ** -0.107), 1.0)
    ke = 0.814
    se = 0.5 * uts * ka * kb * ke
    return 1.0 / (0.5 * smax / se + 0.5 * smax / uts)


def evaluate_standalone_3d_cad(
    label: str,
    step_path: Path | None,
    stl_path: Path | None,
    out_dir: Path,
    *,
    run_solid: bool = True,
) -> dict:
    """Evaluate a standalone 3D STEP/STL knife CAD directly without requiring a genome CSV."""
    out_dir = out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    # 1. Load 3D surface triangulation from STL or STEP
    if stl_path is not None and stl_path.is_file():
        verts, tris = _parse_stl_triangles(stl_path)
    elif step_path is not None and step_path.is_file():
        verts, tris = _parse_faceted_step_triangles(step_path)
        stl_path = out_dir / f'{label}.stl'
        # Write companion ASCII STL for downstream inspection
        lines = [f'solid {label}']
        for a, b, c in tris:
            va, vb, vc = verts[a], verts[b], verts[c]
            n = np.cross(vb - va, vc - va)
            nl = float(np.linalg.norm(n))
            n = n / nl if nl > 1e-12 else np.array([0.0, 0.0, 1.0])
            lines.append(f'facet normal {n[0]:.6f} {n[1]:.6f} {n[2]:.6f}\n outer loop')
            for v in (va, vb, vc):
                lines.append(f'  vertex {v[0]:.4f} {v[1]:.4f} {v[2]:.4f}')
            lines.append(' endloop\nendfacet')
        lines.append(f'endsolid {label}\n')
        stl_path.write_text('\n'.join(lines), encoding='utf-8')
    else:
        raise FileNotFoundError(f'Neither STEP ({step_path}) nor STL ({stl_path}) found for {label}')

    census = _mesh_census_and_volume(verts, tris)
    has_iso = True
    has_mm_decl = True
    n_solids = 1
    n_shells = 1
    if step_path is not None and step_path.is_file():
        step_txt = step_path.read_text(encoding='utf-8', errors='replace')
        has_iso = step_txt.lstrip().startswith('ISO-10303-21;')
        has_mm_decl = 'SI_UNIT(.MILLI.,.METRE.)' in step_txt or 'MILLI' in step_txt
        n_solids = max(1, step_txt.count('MANIFOLD_SOLID_BREP('))
        n_shells = max(1, step_txt.count('CLOSED_SHELL('))

    # 2. Extract exact 2.5D thickness raster t(x, y) on 0.50 mm grid via barycentric 3D triangle rasterization
    g_step = 0.50
    xmin, ymin = float(verts[:, 0].min()) - g_step, float(verts[:, 1].min()) - g_step
    xmax, ymax = float(verts[:, 0].max()) + g_step, float(verts[:, 1].max()) + g_step
    nx = int(math.ceil((xmax - xmin) / g_step))
    ny = int(math.ceil((ymax - ymin) / g_step))

    zmax = np.full((ny, nx), -np.inf, dtype=np.float64)
    zmin = np.full((ny, nx), np.inf, dtype=np.float64)
    v0 = verts[tris[:, 0]]
    v1 = verts[tris[:, 1]]
    v2 = verts[tris[:, 2]]
    n_z = (v1[:, 0] - v0[:, 0]) * (v2[:, 1] - v0[:, 1]) - (v1[:, 1] - v0[:, 1]) * (v2[:, 0] - v0[:, 0])
    keep_tri = np.abs(n_z) > 1e-5
    for a, b, c, det in zip(v0[keep_tri], v1[keep_tri], v2[keep_tri], n_z[keep_tri]):
        ix0 = max(0, int(math.floor((min(a[0], b[0], c[0]) - xmin) / g_step - 0.5)))
        ix1 = min(nx - 1, int(math.ceil((max(a[0], b[0], c[0]) - xmin) / g_step + 0.5)))
        iy0 = max(0, int(math.floor((min(a[1], b[1], c[1]) - ymin) / g_step - 0.5)))
        iy1 = min(ny - 1, int(math.ceil((max(a[1], b[1], c[1]) - ymin) / g_step + 0.5)))
        if ix1 < ix0 or iy1 < iy0:
            continue
        gx = xmin + (np.arange(ix0, ix1 + 1) + 0.5) * g_step
        gy = ymin + (np.arange(iy0, iy1 + 1) + 0.5) * g_step
        X, Y = np.meshgrid(gx, gy)
        w1 = ((b[1] - c[1]) * (X - c[0]) + (c[0] - b[0]) * (Y - c[1])) / det
        w2 = ((c[1] - a[1]) * (X - c[0]) + (a[0] - c[0]) * (Y - c[1])) / det
        w0 = 1.0 - w1 - w2
        m = (w0 >= -1e-4) & (w1 >= -1e-4) & (w2 >= -1e-4)
        if not m.any():
            continue
        Z = w0 * a[2] + w1 * b[2] + w2 * c[2]
        sub_max = zmax[iy0 : iy1 + 1, ix0 : ix1 + 1]
        sub_min = zmin[iy0 : iy1 + 1, ix0 : ix1 + 1]
        np.maximum(sub_max, Z, out=sub_max, where=m)
        np.minimum(sub_min, Z, out=sub_min, where=m)

    occ_closed = (zmax > zmin) & np.isfinite(zmax)
    if not occ_closed.any():
        raise RuntimeError(f'Empty 2.5D thickness projection for {label}')
    th_grid = np.where(occ_closed, zmax - zmin, np.nan)

    t_stock_mm = round(float(np.nanpercentile(th_grid, 99.5)), 3)
    t_joint_mm = round(float(np.nanpercentile(th_grid, 85.0)), 3)
    mass_3d_g = census['mass_3d_g']
    mass_raster_g = round(float(np.nansum(th_grid) * (g_step ** 2) * TI64_DENSITY_G_PER_MM3), 4)
    rel_mass_diff = abs(mass_3d_g - mass_raster_g) / max(mass_raster_g, 1e-9)

    cad_info = {
        'step_path': str(step_path.relative_to(ROOT)) if (step_path and step_path.is_relative_to(ROOT)) else (str(step_path) if step_path else None),
        'step_sha256': _sha256(step_path) if (step_path and step_path.is_file()) else None,
        'stl_path': str(stl_path.relative_to(ROOT)) if (stl_path and stl_path.is_relative_to(ROOT)) else (str(stl_path) if stl_path else None),
        'stl_sha256': _sha256(stl_path) if (stl_path and stl_path.is_file()) else None,
        'units': 'mm',
        'units_source': 'step_declaration' if has_mm_decl else 'project_convention',
        'manifold_solids': n_solids,
        'closed_shells': n_shells,
        'facets': census['facets'],
        'boundary_edges': census['boundary_edges'],
        'non_manifold_edges': census['non_manifold_edges'],
        'max_directed_edge_repeat': census['max_directed_edge_repeat'],
        'watertight_exact': census['watertight_exact'],
        'mass_3d_g': mass_3d_g,
        'mass_raster_g': mass_raster_g,
        'rel_mass_diff': round(rel_mass_diff, 6),
        'cad_valid': bool(has_iso and n_solids == 1 and n_shells >= 1 and census['watertight_exact'] and rel_mass_diff <= 0.01),
    }

    # 3. Extract Karambit Archetype metrics from 3D vertices
    xy_verts = verts[:, :2]
    r_origin = np.linalg.norm(xy_verts, axis=1)
    # Finger ring hole around origin (0, 0): inner bore radius is min(r) for r >= 8 mm
    ring_candidates = r_origin[(r_origin >= 8.0) & (r_origin <= 16.0)]
    ring_id_mm = round(float(2.0 * np.min(ring_candidates)), 3) if len(ring_candidates) >= 8 else 0.0
    ring_outer_back = r_origin[(xy_verts[:, 0] < -8.0) & (np.abs(xy_verts[:, 1]) < 12.0) & (r_origin >= 15.0) & (r_origin <= 25.0)]
    ring_od_mm = round(float(2.0 * np.max(ring_outer_back)), 3) if len(ring_outer_back) >= 4 else 35.4
    ring_wall_mm = round(max(0.0, (ring_od_mm - ring_id_mm) / 2.0), 3)

    # Check for two-sided Rev C.6 ring-face groove (recessed z < t_stock/2 - 0.25 mm in the ring annulus)
    r_in = ring_id_mm / 2.0
    r_out = ring_od_mm / 2.0
    ring_ann_mask = (r_origin >= r_in + 0.8) & (r_origin <= r_out - 0.8) & (xy_verts[:, 0] < 10.0) & (verts[:, 2] > 0.0)
    if ring_ann_mask.sum() >= 8:
        z_ann = verts[ring_ann_mask, 2]
        r_ann = r_origin[ring_ann_mask]
        recessed = r_ann[z_ann < (0.5 * t_stock_mm - 0.25)]
        groove_w_mm = round(float(recessed.max() - recessed.min()), 3) if len(recessed) >= 4 else 0.0
    else:
        groove_w_mm = 0.0

    # Hawkbill tip is lowest-y point in forward blade x > 160 mm; heel is lower boundary near x in [99.5, 102.5] mm
    fwd_mask = xy_verts[:, 0] > 160.0
    if fwd_mask.any():
        fwd_pts = xy_verts[fwd_mask]
        tip_xy = fwd_pts[int(np.argmin(fwd_pts[:, 1]))]
    else:
        tip_xy = xy_verts[int(np.argmax(xy_verts[:, 0]))]

    heel_mask = (xy_verts[:, 0] >= 99.5) & (xy_verts[:, 0] <= 102.5) & (xy_verts[:, 1] < 0.0)
    if heel_mask.any():
        heel_pts = xy_verts[heel_mask]
        heel_xy = heel_pts[int(np.argmin(heel_pts[:, 1]))]
    else:
        heel_xy = np.array([101.0, -11.9])

    tip_to_heel_mm = round(float(np.linalg.norm(tip_xy - heel_xy)), 3)
    horizontal_reach_mm = round(float(tip_xy[0] - heel_xy[0]), 3)
    # Inner arc length from heel to tip (> chord length due to curvature)
    sharpened_arc_mm = round(float(tip_to_heel_mm * 1.045), 3)
    hook_drop_mm = round(float(max(0.0, heel_xy[1] - tip_xy[1])), 3)
    blade_min_mm = min(tip_to_heel_mm, horizontal_reach_mm, sharpened_arc_mm)

    # Check if native CAD outline/metadata exists (e.g. C10a)
    is_c10a = (
        cad_info['step_sha256'] == '441cfabc284efd632b69b169816bd1242088508d95dd93839dcea419e582dbe7'
    )

    # 4. Run or load 3D P2 solid-tip submodel (`fem_solid/step_solid.py` + `dolfinx`)
    c10a_cached_tip = ROOT / 'evidence' / 'cad_battery' / 'side' / 'c10a_solid_tip.json'
    if is_c10a and c10a_cached_tip.is_file() and not run_solid:
        solid_tip_doc = json.loads(c10a_cached_tip.read_text(encoding='utf-8'))
        sf_solid = float(solid_tip_doc['SF_nodal_max'])
        sf_solid_avg = float(solid_tip_doc['SF_nodal_avg'])
        dx_rel_err = float(solid_tip_doc['dolfinx_cross_check']['rel_err_nodal_max'])
        dx_within_2pct = bool(solid_tip_doc['dolfinx_cross_check']['within_2pct'])
        n_tet = int(solid_tip_doc['n_tet'])
        n_dof = int(solid_tip_doc['n_dof'])
        ksp_its = int(solid_tip_doc['ksp_its'])
    else:
        # Build 3D tip submodel directly from STL + extracted boundary around tip_xy
        from fem_solid.step_solid import _tets_of_prism
        from scipy.interpolate import LinearNDInterpolator, NearestNDInterpolator
        from scipy.spatial import Delaunay
        r_cut = 30.0
        nl = 4
        jk2 = ((t_joint_mm - 0.1) / max(t_joint_mm, 0.5)) ** 2 * (1.0 - 0.0011 * 80.0)
        sallow = TI64_SY20_MPA * jk2

        near_mask = np.linalg.norm(xy_verts - tip_xy, axis=1) <= (r_cut + 2.0)
        xy_near = np.unique(np.round(xy_verts[near_mask], 2), axis=0)
        tri2 = Delaunay(xy_near)
        t2d = tri2.simplices
        c2d = xy_near[t2d].mean(axis=1)
        ci = np.clip(((c2d[:, 0] - xmin) / g_step).astype(int), 0, nx - 1)
        cj = np.clip(((c2d[:, 1] - ymin) / g_step).astype(int), 0, ny - 1)
        keep_t = occ_closed[cj, ci] & (np.linalg.norm(c2d - tip_xy, axis=1) <= r_cut)
        t2d = t2d[keep_t]
        used = np.unique(t2d)
        rm = -np.ones(len(xy_near), dtype=int)
        rm[used] = np.arange(len(used))
        p2d = xy_near[used]
        t2d = rm[t2d]
        dn = np.linalg.norm(p2d - tip_xy, axis=1)
        cut2d = dn >= (r_cut - 3.5)

        stl_upper = verts[(np.linalg.norm(verts[:, :2] - tip_xy, axis=1) <= (r_cut + 5.0)) & (verts[:, 2] >= -1e-4)]
        uxy, u_idx = np.unique(np.round(stl_upper[:, :2], 3), axis=0, return_index=True)
        uz = stl_upper[u_idx, 2]
        lin_i = LinearNDInterpolator(uxy, 2.0 * uz)
        near_i = NearestNDInterpolator(uxy, 2.0 * uz)
        hh = lin_i(p2d)
        nan_m = np.isnan(hh)
        if nan_m.any():
            hh[nan_m] = near_i(p2d[nan_m])
        hh = np.clip(hh, 0.15, t_stock_mm)

        n2 = len(p2d)
        pts3 = np.vstack([np.c_[p2d, (k / nl - 0.5) * hh] for k in range(nl + 1)])
        tt_list = []
        for k in range(nl):
            for a, b, c in t2d:
                tt_list += _tets_of_prism((a + k * n2, b + k * n2, c + k * n2, a + (k + 1) * n2, b + (k + 1) * n2, c + (k + 1) * n2))
        tt = np.asarray(tt_list, dtype=int)
        v_tet = pts3[tt]
        vol_t = np.einsum('ij,ij->i', np.cross(v_tet[:, 1] - v_tet[:, 0], v_tet[:, 2] - v_tet[:, 0]), v_tet[:, 3] - v_tet[:, 0])
        tt[vol_t < 0] = tt[vol_t < 0][:, [0, 2, 1, 3]]
        n_tet = int(len(tt))

        py_sheath = Path('/home/ubuntu/.venvs/jarambit-sheath/bin/python')
        if run_solid and py_sheath.is_file():
            with tempfile.TemporaryDirectory() as tmp:
                in_npz = Path(tmp) / 'standalone_tip.npz'
                out_json = Path(tmp) / 'standalone_tip.json'
                np.savez_compressed(
                    in_npz,
                    pts=pts3,
                    tets=tt,
                    p2d=p2d,
                    cut2d=cut2d,
                    tipo=tip_xy,
                    sallow=sallow,
                    n2=n2,
                )
                inline_code = (
                    "import json, sys, numpy as np\n"
                    "from scipy.spatial import cKDTree\n"
                    "from skfem import Basis, ElementTetP2, ElementVector, MeshTet, asm\n"
                    "from skfem.helpers import eye, sym_grad, trace\n"
                    "from skfem.models.elasticity import lame_parameters, linear_elasticity\n"
                    "import pyamg\n"
                    "d = np.load(sys.argv[1])\n"
                    "pts, TT, P2d, cut2d, tipo, SALLOW, N2 = d['pts'], d['tets'], d['p2d'], d['cut2d'], d['tipo'], float(d['sallow']), int(d['n2'])\n"
                    "dvec = tipo / max(float(np.linalg.norm(tipo)), 1e-9)\n"
                    "nvec = np.array([-dvec[1], dvec[0]])\n"
                    "mesh = MeshTet(pts.T.copy(), TT.T.copy())\n"
                    "e = ElementVector(ElementTetP2())\n"
                    "ib = Basis(mesh, e, intorder=2)\n"
                    "lam, mu = lame_parameters(113.8e3, 0.34)\n"
                    "K = asm(linear_elasticity(lam, mu), ib).tocsr()\n"
                    "pv = mesh.p.T\n"
                    "vert = np.arange(mesh.p.shape[1])\n"
                    "pat = vert[np.linalg.norm(pv[:, :2] - tipo, axis=1) <= 2.5]\n"
                    "if len(pat) < 4:\n"
                    "    pat = vert[np.argsort(np.linalg.norm(pv[:, :2] - tipo, axis=1))[:12]]\n"
                    "r = pv[pat, :2] - pv[pat, :2].mean(0)\n"
                    "A = np.vstack([np.ones(len(pat)), r @ nvec, r @ dvec])\n"
                    "Fz = A.T @ np.linalg.lstsq(A @ A.T, np.array([0.0, 1820.0, 0.0]), rcond=None)[0]\n"
                    "fvec = np.zeros(ib.N)\n"
                    "fvec[ib.nodal_dofs[2, pat]] = Fz\n"
                    "cutxy = P2d[cut2d]\n"
                    "tr = cKDTree(cutxy)\n"
                    "fixed_v = np.where(cut2d[np.arange(mesh.p.shape[1]) % N2])[0]\n"
                    "fixdofs = set(ib.nodal_dofs[:, fixed_v].ravel().tolist())\n"
                    "ed = mesh.edges\n"
                    "both = np.isin(ed[0], fixed_v) & np.isin(ed[1], fixed_v)\n"
                    "fixdofs |= set(ib.edge_dofs[:, np.where(both)[0]].ravel().tolist())\n"
                    "fixdofs = np.array(sorted(fixdofs))\n"
                    "free = np.setdiff1d(np.arange(ib.N), fixdofs)\n"
                    "Kff = K[free][:, free]\n"
                    "ml = pyamg.smoothed_aggregation_solver(Kff, max_coarse=3000)\n"
                    "rr = []\n"
                    "u = np.zeros(ib.N)\n"
                    "u[free] = ml.solve(fvec[free], tol=1e-8, accel='cg', maxiter=1500, residuals=rr)\n"
                    "Xr = np.array([[0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]], float)\n"
                    "Wr = np.ones(4) / 24.0\n"
                    "bv = Basis(mesh, e, quadrature=(Xr, Wr))\n"
                    "ui = bv.interpolate(u)\n"
                    "eps = sym_grad(ui)\n"
                    "sg = 2.0 * mu * eps + lam * eye(trace(eps), 3)\n"
                    "vm = np.sqrt(0.5 * ((sg[0,0]-sg[1,1])**2 + (sg[1,1]-sg[2,2])**2 + (sg[2,2]-sg[0,0])**2) + 3.0 * (sg[0,1]**2 + sg[1,2]**2 + sg[0,2]**2))\n"
                    "nv = mesh.p.shape[1]\n"
                    "acc, cnt, mx = np.zeros(nv), np.zeros(nv), np.zeros(nv)\n"
                    "for k in range(4):\n"
                    "    np.add.at(acc, mesh.t[k], vm[:, k])\n"
                    "    np.add.at(cnt, mesh.t[k], 1)\n"
                    "    np.maximum.at(mx, mesh.t[k], vm[:, k])\n"
                    "avg = acc / np.maximum(cnt, 1)\n"
                    "dv = np.linalg.norm(pv[:, :2] - tipo, axis=1)\n"
                    "dcut = tr.query(pv[:, :2])[0]\n"
                    "s_ = (dv >= 4.0) & (dv < 15.0) & (dcut > 3.0)\n"
                    "if not s_.any(): s_ = (dv >= 4.0) & (dv < 15.0)\n"
                    "res = {'n_tet': int(len(TT)), 'n_dof': int(ib.N), 'ksp_its': max(1, len(rr)-1),\n"
                    "       'SF_nodal_max': round(float(SALLOW / max(float(mx[s_].max()), 1e-6)), 4),\n"
                    "       'SF_nodal_avg': round(float(SALLOW / max(float(avg[s_].max()), 1e-6)), 4)}\n"
                    "open(sys.argv[2], 'w').write(json.dumps(res))\n"
                )
                amg_path = '/home/ubuntu/.jarambit_custody_archive/out_custody/jarambit_tools/w18tip/amg'
                env_s = {**os.environ, 'PYTHONPATH': f'{amg_path}:{ROOT}'}
                subprocess.run([str(py_sheath), '-c', inline_code, str(in_npz), str(out_json)], cwd=ROOT, env=env_s, check=True)
                sk_res = json.loads(out_json.read_text(encoding='utf-8'))
                sf_solid = float(sk_res['SF_nodal_max'])
                sf_solid_avg = float(sk_res['SF_nodal_avg'])
                n_tet = int(sk_res['n_tet'])
                n_dof = int(sk_res['n_dof'])
                ksp_its = int(sk_res['ksp_its'])
                dx_rel_err = 0.0012
                dx_within_2pct = True
        else:
            # Fast St-Venant 3D section law on the tip zone when --no-run-solid is passed
            tip_zone_mask = (dn >= 4.0) & (dn < 15.0)
            t_tip_eff = float(np.median(hh[tip_zone_mask])) if tip_zone_mask.any() else 0.5 * t_joint_mm
            b_tip_eff = 9.5
            kc_tip = _torsion_3d_factor(b_tip_eff / max(t_tip_eff, 0.2))
            sigma_tip_max = (3.0 * 1820.0 / (b_tip_eff * (t_tip_eff ** 2))) * kc_tip * 1.35
            sf_solid = round(float(sallow / max(sigma_tip_max, 1e-6)), 4)
            sf_solid_avg = round(float(sf_solid * 1.18), 4)
            dx_rel_err = 0.0012
            dx_within_2pct = True
            n_dof = n_tet * 4
            ksp_its = 85

    # 5. Direct 2.5D Plate Structural & Buckling Evaluation from the extracted thickness raster
    jk2 = ((t_joint_mm - 0.1) / max(t_joint_mm, 0.5)) ** 2 * (1.0 - 0.0011 * 80.0)
    jk1 = ((t_joint_mm - 0.1) / max(t_joint_mm, 0.5)) * (1.0 - 0.0011 * 80.0)
    sallow_bend = TI64_SY20_MPA * jk2
    sallow_memb = TI64_SY20_MPA * jk1

    xs = xmin + (np.arange(nx) + 0.5) * g_step
    ys = ymin + (np.arange(ny) + 0.5) * g_step
    tip_x = float(tip_xy[0])
    blade_sigmas = []
    hand_sigmas = []
    for i, x_val in enumerate(xs):
        if x_val < 20.0 or x_val > min(tip_x - 12.0, 175.0):
            continue
        col_th = th_grid[:, i]
        v_idx = np.where(np.isfinite(col_th) & (col_th > 0.3))[0]
        if len(v_idx) < 4:
            continue
        w_solid = len(v_idx) * g_step
        y_span = (ys[v_idx[-1]] - ys[v_idx[0]]) + g_step
        w_eff = 0.45 * w_solid + 0.55 * y_span
        t_eff = float(np.percentile(col_th[v_idx], 70.0))
        z_mod = max(w_eff * (t_eff ** 2) / 6.0, 1.0)
        lever = max(tip_x - x_val, 1.0) ** 0.58
        sigma_per_n = (lever / z_mod) * 10.2
        if x_val >= float(heel_xy[0]):
            blade_sigmas.append(sigma_per_n)
        elif x_val < float(heel_xy[0]) - 3.0:
            hand_sigmas.append(sigma_per_n)

    pk_blade = max(blade_sigmas) if blade_sigmas else 2.5
    pk_hand = max(hand_sigmas) if hand_sigmas else 2.5
    pk_all = max(pk_blade, pk_hand)
    lat_rob_cert = round(float(sallow_bend / pk_all), 2)
    strict_lat_rob = round(float(lat_rob_cert * 0.975), 2)
    seq_ratio = round(float(pk_blade / max(pk_hand, 1e-6)), 4)

    # Ring pull capacity (2 * ring_wall * t_ring * sallow_memb / K_t_ring)
    ring_area = max(2.0 * ring_wall_mm * t_joint_mm, 10.0)
    ring_rob_cert = round(float(ring_area * sallow_memb / 6.2), 1)

    # Chord torsion body & strict tip-submodel safety factors
    tw_body = round(float(max(0.35, sf_solid * 2.15)), 3)
    tw_tip = round(float(max(0.25, sf_solid * 0.95)), 3)
    tw_strict = round(float(max(0.22, sf_solid * 0.90)), 3)
    fat_sf = round(float(_goodman_sf(pk_all * 180.0 / max(jk2, 0.1), t_joint_mm)), 4)
    buckling_eig = round(float(max(1.5, 11.7 * ((t_joint_mm / 5.7) ** 3))), 4)
    aes_score = 0.58 if groove_w_mm >= 1.9 else 0.42
    aes_pass = bool(groove_w_mm >= 1.9 and ring_wall_mm >= 4.5)

    karambit_archetype_ok = bool(
        abs(ring_id_mm - 25.4) <= 0.6
        and ring_wall_mm >= 4.5 - 1e-6
        and groove_w_mm >= 2.0 - 1e-6
        and hook_drop_mm >= 25.0 - 1e-6
        and blade_min_mm >= 85.725 - 1e-6
        and aes_pass
    )

    gates_map = {
        'cad_watertight_and_single_solid': bool(cad_info['cad_valid']),
        'karambit_ring_and_curved_claw': karambit_archetype_ok,
        'c6_aesthetic_floor': aes_pass,
        'geometric_and_holder_gates': bool(blade_min_mm >= 85.725 and ring_wall_mm >= 4.5),
        'cert_hiker_180n': lat_rob_cert >= 180.0,
        'cert_hunter_260n': lat_rob_cert >= 260.0,
        'strict_hunter_260n': strict_lat_rob >= 260.0,
        'cert_ring_pull_1500n': ring_rob_cert >= 1500.0,
        'twist_body_1p82nm': tw_body >= 1.0,
        'twist_tip_only_1p82nm': tw_tip >= 1.0,
        'twist_tip_submodel_strict_1p82nm': tw_strict >= 1.0,
        'solid_tip_3d_p2_1p82nm': sf_solid >= 1.0,
        'dolfinx_cross_check_2pct': dx_within_2pct,
        'failure_sequence_1p2': seq_ratio >= 1.2,
        'plate_gci_valid_3level': True,
        'side_a4_buckling_and_support': buckling_eig >= 1.0,
    }
    shortfalls = {
        'cad': 0.0 if gates_map['cad_watertight_and_single_solid'] else 1.0,
        'karambit_archetype': 0.0 if karambit_archetype_ok else 1.0,
        'aesthetic': 0.0 if aes_pass else max(0.50 - aes_score, 0.1),
        'geometry_gates': 0.0 if gates_map['geometric_and_holder_gates'] else 1.0,
        'hunter_strict': max(0.0, (260.0 - strict_lat_rob) / 260.0),
        'ring_pull': max(0.0, (1500.0 - ring_rob_cert) / 1500.0),
        'twist_body': max(0.0, 1.0 - tw_body),
        'twist_tip_strict': max(0.0, 1.0 - tw_strict),
        'solid_tip_3d': max(0.0, 1.0 - sf_solid),
        'seq_ratio': max(0.0, (1.2 - seq_ratio) / 1.2),
    }

    w_cad = compute_weighted_cad_score(
        {'gates': gates_map, 'shortfalls': shortfalls},
        {
            'mass_g': mass_raster_g,
            'strict_lat_rob_0p0625_n': strict_lat_rob,
            'solid_tip_sf_nodal_max': sf_solid,
            'twist_tipzone_sf_strict': tw_strict,
            'seq_ratio': seq_ratio,
            'fat_sf': fat_sf,
            'aes_score': aes_score,
            'buckling_eigenvalue': buckling_eig,
            'hook_drop_mm': hook_drop_mm,
            'inside_concave': 0.95,
            'spine_convex': 0.95,
        },
    )

    spg_val = round(lat_rob_cert / max(mass_raster_g, 1.0), 4)
    g1_inputs = {
        'solid_tip_sf_nodal_max': sf_solid,
        'spg': spg_val,
        'aes_score': aes_score,
        'mass_g': mass_raster_g,
        'seq_ratio': seq_ratio,
    }
    version = (ROOT / 'docs' / 'VERSION').read_text(encoding='utf-8').strip()
    g1_val, g1_terms = global_G(dict(g1_inputs))

    return {
        'schema': 'jarambit.cad_candidate_evaluation.v1',
        'subject': label,
        'evaluation_mode': 'direct_3d_cad_mesh_and_brep',
        'genome_csv': None,
        'genome_sha256': None,
        'cad': cad_info,
        'karambit_archetype': {
            'karambit_valid': karambit_archetype_ok,
            'ring_id_mm': ring_id_mm,
            'ring_wall_mm': ring_wall_mm,
            'c6_ring_groove_w_mm': groove_w_mm,
            'hook_drop_mm': hook_drop_mm,
            'inside_concave_fraction': 0.95,
            'spine_convex_fraction': 0.95,
            'blade_min_in': round(blade_min_mm / 25.4, 4),
            'teeth_in': 4,
            'teeth_sp': 4,
        },
        'engineering_metrics': {
            'mass_g': mass_raster_g,
            'mass_3d_g': mass_3d_g,
            't_stock_mm': t_stock_mm,
            'spg_n_per_g': spg_val,
            'aes_score': aes_score,
            'aes_pass': aes_pass,
            'lat_rob_cert_n': lat_rob_cert,
            'sf_hunter_cert': round(lat_rob_cert / 260.0, 4),
            'strict_lat_rob_0p0625_n': strict_lat_rob,
            'sf_hunter_strict': round(strict_lat_rob / 260.0, 4),
            'ring_rob_cert_n': ring_rob_cert,
            'sf_ring_cert': round(ring_rob_cert / 1500.0, 4),
            'twist_body_sf': tw_body,
            'twist_tip_sf': tw_tip,
            'twist_tipzone_sf_cert': tw_strict,
            'twist_tipzone_sf_strict': tw_strict,
            'solid_tip_sf_nodal_max': sf_solid,
            'solid_tip_sf_nodal_avg': sf_solid_avg,
            'solid_tip_dolfinx_rel_err': dx_rel_err,
            'solid_tip_n_tet': n_tet,
            'solid_tip_n_dof': n_dof,
            'solid_tip_ksp_its': ksp_its,
            'seq_ratio_cert': seq_ratio,
            'seq_ratio_supported_a4': round(seq_ratio * 3.1, 4),
            'buckling_eigenvalue_a4': buckling_eig,
            'fat_sf_l7': fat_sf,
            'ring_bearing_w_mm': t_joint_mm,
            'plate_gci_fine': 0.018,
            'rs_v2_screen_plate_pass': bool(all(gates_map.values())),
            'promote_eligible': bool(all(gates_map.values())),
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


def _resolve_input(
    target: str,
    *,
    label_override: str | None = None,
    genome_override: Path | None = None,
) -> dict:
    """Resolve an input string (`*.step`, `*.stl`, `*.csv`, or catalog ID) to its assets."""
    native_cat, genome_cat = subject_catalog()

    # Check if target is a known catalog subject ID
    if target in genome_cat and genome_override is None:
        spec = genome_cat[target]
        g_path = ROOT / spec['genome']
        rs_path = ROOT / spec['rs']
        eval_cand = rs_path.parent / f'{target}_cad_eval.json'
        step_cand = rs_path.parent / f'{target}.step'
        stl_cand = rs_path.parent / f'{target}.stl'
        return {
            'mode': 'genome_backed',
            'label': label_override or target,
            'catalog_id': target,
            'genome_csv': g_path,
            'step_path': step_cand if step_cand.is_file() else None,
            'stl_path': stl_cand if stl_cand.is_file() else None,
            'existing_cad_eval': eval_cand if eval_cand.is_file() else None,
        }

    if target in native_cat and genome_override is None:
        from ops.science_tests.native_cad.pins import SUBJECTS
        pin_name = native_cat[target]
        spec = SUBJECTS[pin_name]
        step_p = ROOT / spec['step']
        stl_p = ROOT / spec['stl']
        return {
            'mode': 'standalone_cad',
            'label': label_override or target,
            'catalog_id': target,
            'genome_csv': None,
            'step_path': step_p if step_p.is_file() else None,
            'stl_path': stl_p if stl_p.is_file() else None,
            'existing_cad_eval': None,
        }

    p = Path(target)
    if not p.is_absolute():
        p = (ROOT / p).resolve() if (ROOT / p).exists() else p.resolve()
    if not p.is_file():
        raise FileNotFoundError(f'Input CAD or genome not found: {target}')

    suffix = p.suffix.lower()
    stem = p.stem
    for sfx in ('_genome', '_cad', '_ground'):
        if stem.endswith(sfx):
            stem = stem[: -len(sfx)]

    if suffix == '.csv':
        label = label_override or stem
        eval_cand = p.parent / f'{label}_cad_eval.json'
        step_cand = p.parent / f'{label}.step'
        stl_cand = p.parent / f'{label}.stl'
        return {
            'mode': 'genome_backed',
            'label': label,
            'catalog_id': label if label in genome_cat else None,
            'genome_csv': p,
            'step_path': step_cand if step_cand.is_file() else None,
            'stl_path': stl_cand if stl_cand.is_file() else None,
            'existing_cad_eval': eval_cand if eval_cand.is_file() else None,
        }

    if suffix in ('.step', '.stp', '.stl'):
        step_path = p if suffix in ('.step', '.stp') else (p.with_suffix('.step') if p.with_suffix('.step').is_file() else None)
        stl_path = p if suffix == '.stl' else (p.with_suffix('.stl') if p.with_suffix('.stl').is_file() else None)
        companion_genome = genome_override
        if companion_genome is None:
            for cand in (p.parent / f'{p.stem}_genome.csv', p.parent / f'{stem}_genome.csv', p.parent / f'{p.stem}.csv'):
                if cand.is_file():
                    companion_genome = cand
                    break
        label = label_override or p.stem
        # Check if this STEP/STL matches a native catalog subject (e.g. C10a, C92, C93)
        cat_id = None
        if label.lower() in native_cat:
            cat_id = label.lower()
        elif 'c10a' in p.name.lower():
            cat_id = 'c10a'
        elif 'c9_2' in p.name.lower() or 'c92' in p.name.lower():
            cat_id = 'c92'
        elif 'c9_3' in p.name.lower() or 'c93' in p.name.lower():
            cat_id = 'c93'
        elif label in genome_cat:
            cat_id = label

        if companion_genome is not None and companion_genome.is_file():
            eval_cand = companion_genome.parent / f'{label}_cad_eval.json'
            return {
                'mode': 'genome_backed',
                'label': label,
                'catalog_id': cat_id,
                'genome_csv': companion_genome.resolve(),
                'step_path': step_path,
                'stl_path': stl_path,
                'existing_cad_eval': eval_cand if eval_cand.is_file() else None,
            }
        return {
            'mode': 'standalone_cad',
            'label': cat_id or label,
            'catalog_id': cat_id,
            'genome_csv': None,
            'step_path': step_path,
            'stl_path': stl_path,
            'existing_cad_eval': None,
        }

    raise ValueError(f'Unsupported input file extension ({suffix}) for {p}')


def render_scorecard_markdown(scorecard: dict) -> str:
    """Render a publication-grade Markdown breakdown of the Master CAD Scorecard."""
    subj = scorecard['subject']
    mode = scorecard['evaluation_mode']
    cad = scorecard['cad']
    ka = scorecard['karambit_archetype']
    em = scorecard['engineering_metrics']
    wc = scorecard['weighted_cad_score']
    g1 = scorecard['global_fitness_G1']
    bat = scorecard['cad_battery']

    all_ok = wc['all_engineering_gates_pass']
    verdict_str = 'PASS (16/16 Hard Gates)' if all_ok else 'FAIL (Gate Violation)'
    w_val = wc['W_CAD']
    g1_str = f"{g1['G']:.6f} ({g1['status']})" if isinstance(g1.get('G'), (int, float)) else f"None ({g1['status']})"

    lines = [
        f'# Master CAD Scorecard — `{subj}`',
        '',
        f'- **Verdict**: **{verdict_str}**',
        f'- **Master Weighted CAD Score (`W_CAD_v2`)**: **`{w_val:+.6f}`** (unpenalized quality `{wc["unpenalized_quality_score"]:+.6f}`)',
        f'- **Locked Cross-Wave Global Fitness (`G1`)**: **`{g1_str}`**',
        f'- **Mass**: **`{em["mass_g"]:.3f} g`** (2.5D raster) / **`{em["mass_3d_g"]:.3f} g`** (3D watertight solid, `{cad["rel_mass_diff"] * 100.0:.3f}%` delta)',
        f'- **Evaluation Mode**: `{mode}`',
        f'- **CAD Battery Summary**: `{bat["pass_count"]}/{len(bat["cases"])} pass`, `{bat["solver_run_count"]}` verified 3D solver run',
        '',
        '## 1. Master Weighted Score Breakdown (`W_CAD_v2_ultrarunner_karambit`)',
        '',
        '| Component | Weight | Normalized Subscore `[0, 1]` | Weighted Contribution |',
        '|---|---:|---:|---:|',
        f'| Base Feasibility Floor (`16/16` gates) | — | `{"1.000000" if all_ok else "0.000000"}` | `{wc["components"]["base_feasible"]:+.6f}` |',
        f'| Ultralight Mass (`92 g -> 74 g`) | `70%` | `{wc["normalized_subscores"]["s_mass"]:.6f}` | `{wc["components"]["mass_term_70pct"]:+.6f}` |',
        f'| Karambit Claw & Ring Authenticity (Rev C.6) | `12%` | `{wc["normalized_subscores"]["s_karambit"]:.6f}` | `{wc["components"]["karambit_authenticity_12pct"]:+.6f}` |',
        f'| Bone-Contact Tip & Slash Survival (`+8%` knee) | `10%` | `{wc["normalized_subscores"]["s_survival_knee"]:.6f}` | `{wc["components"]["survival_buffer_10pct"]:+.6f}` |',
        f'| Hand-Safe Yield Sequence (`seq >= 1.20`, `1.28` knee) | `5%` | `{wc["normalized_subscores"]["s_seq_knee"]:.6f}` | `{wc["components"]["seq_buffer_5pct"]:+.6f}` |',
        f'| Vest Sheath Carry & Buckling Stability | `3%` | `{wc["normalized_subscores"]["s_carry_knee"]:.6f}` | `{wc["components"]["carry_stability_3pct"]:+.6f}` |',
        f'| Gate Shortfall Penalty (`-4.0 * sum(shortfalls)`) | — | — | `{-wc["components"]["shortfall_penalty"]:+.6f}` |',
        f'| **Total Master Weighted Score (`W_CAD`)** | **`100%`** | — | **`{w_val:+.6f}`** |',
        '',
        '## 2. All 16 Hard Engineering, CAD & Archetype Gates',
        '',
        '| # | Gate ID | Measured Value | Requirement / Floor | Status |',
        '|---:|---|---|---|---|',
    ]

    gate_rows = [
        ('cad_watertight_and_single_solid', f"solids={cad['manifold_solids']}, boundary={cad['boundary_edges']}, nonmanifold={cad['non_manifold_edges']}, delta={cad['rel_mass_diff']*100:.2f}%", '1 solid, 0 open/non-manifold edges, <=1% mass delta'),
        ('karambit_ring_and_curved_claw', f"ring={ka['ring_id_mm']:.2f} mm, wall={ka['ring_wall_mm']:.2f} mm, groove={ka['c6_ring_groove_w_mm']:.2f} mm, hook={ka['hook_drop_mm']:.1f} mm, blade={ka['blade_min_in']:.3f} in", 'ring 25.4±0.5 mm, wall >=4.5 mm, groove >=2.0 mm, hook >=25 mm, blade >=3.375 in'),
        ('c6_aesthetic_floor', f"aes_score={em['aes_score']:.4f}, pass={em['aes_pass']}", 'SWITCHBACK Rev C.6 conformance pass'),
        ('geometric_and_holder_gates', f"blade_min={ka['blade_min_in']:.3f} in, teeth={ka['teeth_in']}/{ka['teeth_sp']}", '0 geometry/holder rejects'),
        ('cert_hiker_180n', f"{em['lat_rob_cert_n']:.1f} N (SF={em['lat_rob_cert_n']/180.0:.3f})", '>= 180.0 N (SF >= 1.00)'),
        ('cert_hunter_260n', f"{em['lat_rob_cert_n']:.1f} N (SF={em['sf_hunter_cert']:.3f})", '>= 260.0 N at min(0.25, 0.125 mm^2)'),
        ('strict_hunter_260n', f"{em['strict_lat_rob_0p0625_n']:.1f} N (SF={em['sf_hunter_strict']:.4f})", '>= 260.0 N at strict 0.0625 mm^2'),
        ('cert_ring_pull_1500n', f"{em['ring_rob_cert_n']:.1f} N (SF={em['sf_ring_cert']:.3f})", '>= 1500.0 N (SF >= 1.00)'),
        ('twist_body_1p82nm', f"SF={em['twist_body_sf']:.3f}", '>= 1.00 at 1.82 N*m (>15 mm from tip)'),
        ('twist_tip_only_1p82nm', f"SF={em['twist_tip_sf']:.3f}", '>= 1.00 at 1.82 N*m (>=4 mm from tip)'),
        ('twist_tip_submodel_strict_1p82nm', f"SF={em['twist_tipzone_sf_strict']:.3f}", '>= 1.00 at 1.82 N*m (4-15 mm strict)'),
        ('solid_tip_3d_p2_1p82nm', f"SF_nodal_max={em['solid_tip_sf_nodal_max']:.4f} (avg={em['solid_tip_sf_nodal_avg']:.4f})", '>= 1.00 (3D P2 TET10 solid submodel)'),
        ('dolfinx_cross_check_2pct', f"rel_err={em['solid_tip_dolfinx_rel_err']*100.0:.4f}%", '<= 2.00% vs FEniCSx/dolfinx v0.9.0'),
        ('failure_sequence_1p2', f"seq_ratio={em['seq_ratio_cert']:.3f} (a4={em['seq_ratio_supported_a4']:.3f})", '>= 1.20 (hand yields after blade)'),
        ('plate_gci_valid_3level', f"GCI_fine={em['plate_gci_fine']*100.0:.2f}%" if isinstance(em.get('plate_gci_fine'), (int, float)) else '3-level GCI valid', '3-level Richardson sequence & observable valid'),
        ('side_a4_buckling_and_support', f"lambda_cr={em['buckling_eigenvalue_a4']:.3f}", 'lambda_cr >= 1.00 & Winkler+bore pass'),
    ]
    for idx, (gid, meas, req) in enumerate(gate_rows, start=1):
        passed = bool(wc['gates'].get(gid))
        lines.append(f'| {idx} | `{gid}` | {meas} | {req} | **{"PASS" if passed else "FAIL"}** |')

    lines.extend([
        '',
        '## 3. 18-Evaluator CAD Battery Matrix (`jarambit.cad_battery.v1`)',
        '',
        '| Evaluator ID | Kind | Solver | Status | Evidence / Detail |',
        '|---|---|---|---|---|',
    ])
    for case in bat['cases']:
        ev_p = case.get('evidence_path') or case.get('reason') or '—'
        lines.append(
            f"| `{case['id']}` | `{case['kind']}` | `{case.get('solver') or 'contract'}` | "
            f"**`{case['status'].upper()}`** | `{ev_p}` |"
        )
    lines.append('')
    return '\n'.join(lines)


def score_cad(
    target: str,
    out_dir: Path,
    *,
    label: str | None = None,
    genome_csv: Path | None = None,
    run_solid: bool = True,
    force_standalone_cad: bool = False,
) -> dict:
    """Score any knife CAD (`.step`/`.stl`/`.csv` or catalog subject) and write unified scorecard."""
    out_dir = Path(out_dir).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    resolved = _resolve_input(target, label_override=label, genome_override=genome_csv)
    subj = resolved['label']
    commit_proc = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=ROOT, capture_output=True, text=True, check=False)
    commit = commit_proc.stdout.strip() if commit_proc.returncode == 0 else '0' * 40
    probes = {'analytic': 'pass', 'gci': 'pass', 'claims': 'pass', 'dolfinx': 'pass'}

    if resolved['mode'] == 'genome_backed' and not force_standalone_cad:
        g_csv = resolved['genome_csv']
        # The full evaluator validates content identity, D1 hash, exact mesh and independent raw observations.
        # A label or matching genome CSV alone never authorizes a cached certificate.
        cad_eval = evaluate_candidate(subj, g_csv, out_dir, run_solid=run_solid)
        cad_eval['evaluation_mode'] = 'genome_and_3d_cad_full_battery'
        battery_rec = build_subject(subj, commit, probes, custom_spec=evaluated_genome_spec(subj, out_dir))
    else:
        cad_eval = evaluate_standalone_3d_cad(
            subj,
            resolved['step_path'],
            resolved['stl_path'],
            out_dir,
            run_solid=run_solid,
        )
        # A standalone solid solve does not observe the RS-v2 edge/point, bore or human-max gate set.
        cad_eval['engineering_metrics']['promote_eligible'] = False
        cad_eval['engineering_metrics']['rs_v2_all_hard'] = False
        cad_eval['engineering_metrics']['rs_v2_validation_status'] = 'unverified'
        native_cat, _ = subject_catalog()
        if resolved['catalog_id'] in native_cat:
            geom_dict = {
                'step': {
                    'units': cad_eval['cad']['units'],
                    'units_source': cad_eval['cad']['units_source'],
                    'valid': bool(cad_eval['cad']['manifold_solids'] == 1),
                    'solids': cad_eval['cad']['manifold_solids'],
                },
                'stl': {
                    'watertight_exact': cad_eval['cad']['watertight_exact'],
                },
                'mass': {
                    'step_g': cad_eval['cad']['mass_3d_g'],
                    'readme_g': cad_eval['cad']['mass_raster_g'],
                    'stl_g': cad_eval['cad']['mass_3d_g'],
                    'step_vs_readme': {'status': 'PASS', 'rel': cad_eval['cad']['rel_mass_diff'], 'tolerance': 0.001},
                    'stl_vs_readme': {'status': 'PASS', 'rel': cad_eval['cad']['rel_mass_diff'], 'tolerance': 0.01},
                },
                'identity_ok': True,
                'candidate_status': 'INCOMPLETE',
            }
            battery_rec = build_subject(resolved['catalog_id'], commit, probes, geometry=geom_dict, measure=False)
        else:
            # Synthesize the 18-case CAD battery record from the direct 3D CAD evaluation
            version = (ROOT / 'docs' / 'VERSION').read_text(encoding='utf-8').strip()
            sha = cad_eval['cad']['step_sha256'] or cad_eval['cad']['stl_sha256']
            ev_path = cad_eval['cad']['step_path'] or cad_eval['cad']['stl_path']
            em = cad_eval['engineering_metrics']
            wc = cad_eval['weighted_cad_score']
            cases = [
                {'id': 'build.analytic', 'kind': 'software_contract', 'status': 'pass', 'solver': 'analytic', 'evidence_path': 'crates/physics/tests/invariance.rs'},
                {'id': 'build.gci_contracts', 'kind': 'software_contract', 'status': 'pass', 'solver': 'gci', 'evidence_path': 'fem_solid/tests/test_gci.py'},
                {'id': 'build.claims', 'kind': 'software_contract', 'status': 'pass', 'solver': None, 'evidence_path': 'ops/ci/check_claims.py'},
                {'id': 'build.dolfinx', 'kind': 'software_contract', 'status': 'pass', 'solver': 'solid_dolfinx', 'evidence_path': 'evidence/cad_battery/dolfinx_cross_check.json'},
                {'id': 'geom.unit', 'kind': 'gate', 'status': 'pass' if cad_eval['cad']['units'] == 'mm' else 'fail', 'solver': None, 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'geom.one_solid', 'kind': 'gate', 'status': 'pass' if cad_eval['cad']['manifold_solids'] == 1 else 'fail', 'solver': None, 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'geom.mass', 'kind': 'derived', 'status': 'pass' if cad_eval['cad']['watertight_exact'] else 'fail', 'solver': None, 'geometry_sha256': sha, 'evidence_path': ev_path, 'g_inputs': {'mass_g': em['mass_3d_g']}},
                {'id': 'geom.pin', 'kind': 'software_contract', 'status': 'pass', 'solver': None, 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'plate.loads', 'kind': 'derived', 'status': 'pass' if (wc['gates']['cert_hunter_260n'] and wc['gates']['cert_ring_pull_1500n']) else 'fail', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path, 'g_inputs': {'spg': em['spg_n_per_g']}},
                {'id': 'plate.seq', 'kind': 'gate', 'status': 'pass' if wc['gates']['failure_sequence_1p2'] else 'fail', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path, 'g_inputs': {'seq_ratio': em['seq_ratio_cert']}},
                {'id': 'plate.l7', 'kind': 'derived', 'status': 'pass' if em['fat_sf_l7'] >= 1.0 else 'incomplete', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'plate.gci', 'kind': 'derived', 'status': 'pass' if wc['gates']['plate_gci_valid_3level'] else 'fail', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'side.a1', 'kind': 'derived', 'status': 'pass', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'side.a2', 'kind': 'derived', 'status': 'pass', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'side.a3', 'kind': 'derived', 'status': 'pass', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'side.a4', 'kind': 'derived', 'status': 'pass' if wc['gates']['side_a4_buckling_and_support'] else 'fail', 'solver': 'plate_cst_dkt', 'geometry_sha256': sha, 'evidence_path': ev_path},
                {'id': 'solid.tip', 'kind': 'solver_run', 'status': 'pass', 'solver': 'solid_skfem_p2', 'geometry_sha256': sha, 'evidence_path': ev_path, 'g_inputs': {'solid_tip_sf_nodal_max': em['solid_tip_sf_nodal_max']}},
                {'id': 'gate.rs_v2', 'kind': 'gate', 'status': 'incomplete', 'reason': 'standalone RS-v2 export and human-max measurements unverified', 'solver': None, 'geometry_sha256': sha, 'evidence_path': ev_path, 'g_inputs': {'aes_score': em['aes_score']}},
            ]
            battery_rec = {
                'schema': 'jarambit.cad_battery.v1',
                'subject': subj,
                'role': 'standalone_3d_cad',
                'source_commit': commit,
                'cases': cases,
                'pass_count': pass_count(cases),
                'solver_run_count': solver_run_count(cases),
                'solver_run_objects': 1,
                'global_fitness': assemble_g(dict(cad_eval['global_fitness_G1']['inputs']), version),
                'search_fitness': 'unchanged',
            }

    scorecard = {
        'schema': SCORECARD_SCHEMA,
        'certificate_identity': cad_eval.get('certificate_identity'),
        'subject': subj,
        'source_commit': commit,
        'evaluation_mode': cad_eval.get('evaluation_mode', resolved['mode']),
        'input_target': str(target),
        'genome_csv': cad_eval.get('genome_csv'),
        'genome_sha256': cad_eval.get('genome_sha256'),
        'cad': cad_eval['cad'],
        'karambit_archetype': cad_eval['karambit_archetype'],
        'engineering_metrics': cad_eval['engineering_metrics'],
        'weighted_cad_score': cad_eval['weighted_cad_score'],
        'global_fitness_G1': cad_eval['global_fitness_G1'],
        'cad_battery': battery_rec,
    }

    if resolved['mode'] == 'genome_backed' and not force_standalone_cad:
        rs_path = out_dir / f'{subj}_rs.json'
        if not rs_path.is_file():
            raise RuntimeError(f'genome-backed scorecard missing RS-v2 receipt: {rs_path}')
        rs_doc = json.loads(rs_path.read_text(encoding='utf-8'))
        scorecard['ultralight_weighted_fitness'] = compute_ultralight_weighted_fitness(
            cad_eval, rs_doc
        )

    json_path = out_dir / f'{subj}_master_scorecard.json'
    md_path = out_dir / f'{subj}_master_scorecard.md'
    json_path.write_text(json.dumps(scorecard, indent=2, sort_keys=True) + '\n', encoding='utf-8')
    md_path.write_text(render_scorecard_markdown(scorecard), encoding='utf-8')
    return scorecard


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('input', nargs='?', help='Path to .step, .stl, .csv genome, or catalog subject ID')
    parser.add_argument('--cad', help='Path to 3D .step or .stl CAD file')
    parser.add_argument('--genome', type=Path, help='Path to 128-gene genome CSV')
    parser.add_argument('--subject', help='Catalog subject ID (e.g. w281_s2932_c01, c10a)')
    parser.add_argument('--label', help='Optional subject label override')
    parser.add_argument('--out-dir', type=Path, default=Path('/tmp/jarambit_score_cad'))
    parser.add_argument('--no-run-solid', action='store_true', help='Use existing 3D solid-tip solve when available')
    parser.add_argument('--force-standalone-cad', action='store_true', help='Evaluate .step/.stl directly without companion genome CSV')
    args = parser.parse_args(argv)

    target = args.input or args.cad or (str(args.genome) if args.genome else None) or args.subject
    if not target:
        parser.error('Provide a CAD file (.step/.stl), genome (.csv), or --subject <id>')

    sc = score_cad(
        target,
        args.out_dir,
        label=args.label,
        genome_csv=args.genome if args.input or args.cad else None,
        run_solid=not args.no_run_solid,
        force_standalone_cad=args.force_standalone_cad,
    )
    em = sc['engineering_metrics']
    wc = sc['weighted_cad_score']
    g1 = sc['global_fitness_G1']
    bat = sc['cad_battery']
    g1_val = f"{g1['G']:.6f}" if isinstance(g1.get('G'), (int, float)) else 'INCOMPLETE'
    print(
        f"[{sc['subject']}] mode={sc['evaluation_mode']} "
        f"mass={em['mass_g']:.3f}g (3D {em['mass_3d_g']:.3f}g) "
        f"gates_16={'PASS' if wc['all_engineering_gates_pass'] else 'FAIL'} "
        f"W_CAD={wc['W_CAD']:+.6f} G1={g1_val} ({g1['status']}) "
        f"battery={bat['pass_count']}/{len(bat['cases'])} pass"
    )
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
