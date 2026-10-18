#!/usr/bin/env python3
"""Publish retained native observations after the complete deterministic run."""
import hashlib,json
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def line(values):return '| '+' | '.join(f'{x:.9f}' if isinstance(x,float) else str(x) for x in values)+' |'
def publish():
    receipt=HERE/'phase9_receipt.json';report=HERE/'PHASE9.md'
    r=json.loads(receipt.read_text());assert r['state']=='complete' and not r['errors']
    assert '## Detailed observations' not in report.read_text(),'Already finalized; preserve original report'
    rows=r['results']+r['early_taper_controls'];controls={v['family']:v for v in r['early_taper_controls']}
    lines=['','## Detailed observations','',
      'All38 inputs are canonical; all retain the production geometry and material. The44 stations are axial sections clipped to the requested radial annulus, not a continuous search over all possible section orientations. Reported extrema and moduli use sampled production thickness. The unrounded receipt retains all three quadrature estimates and native export/FEM observations.','',
      'Positive margins mean the measured value is below the unchanged upper limit. Native point monotonicity over0..10mm and taper length remain independent requirements; positive margins at the three point stations alone do not pass Gate S. Direct-law centreline samples and triangulated export measurements can differ.','',
      '| Case | Native apex margin mm | Native angle margin deg | Native point margins .25/1/2 mm | Native taper margin mm | Monotonic | Gate S | Direct thickness .25/1/2 mm | Direct margins .25/1/2 mm |',
      '| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in rows:
        g=v['gate_s_margins'];point=v['gate_s']['measurement']['point']
        triple=lambda vs:', '.join(f'{x:.9f}' for x in vs) if vs is not None else 'unavailable'
        lines.append(line([v['name'],g['apex_mm'],g['half_angle_deg'],triple(g['point_mm']),g['point_taper_mm'],point['monotonic'] if point else None,v['gate_s_pass'],
            triple([p['thickness_mm'] for p in v['exact_law_point_samples']]),triple([p['margin_mm'] for p in v['exact_law_point_samples']])]))
    lines+=['','Every FEM observation below uses the unchanged production thickness law and native supports/loads. The358N column retains the prior study floor; native hiker180N/hunter260N ratios are also shown. Fatigue is the production Goodman factor. No mesh result is waived.','',
        '| Case | Mesh area mm² | Lateral N | SF /358N | Hiker SF | Hunter SF | Fatigue SF | Sequence ratio |',
        '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for v in rows:
        for level in v['levels']:
            m=level['result']['metrics'] if level['result'] else {}
            lines.append(line([v['name'],level['area_mm2']]+[m.get(k) for k in ['lat_rob_n','lateral_sf_358','native_hiker_sf','native_hunter_sf','fatigue_sf','sequence_ratio']]))
    keys=['area_mm2','Z_out_of_plane_mm3','Z_in_plane_mm3']
    convergence={}
    lines+=['','Quadrature changes below are maximum absolute relative changes from.01 to.005mm over the44 stations. Control ratios are minimum/maximum matched-station fine values divided by the same-family116/2.62/18 control. They describe this sampled study without imposing a new acceptance tolerance.','',
        '| Case | Max relative area change | Max relative Z_out change | Max relative Z_in change | Area/control range | Z_out/control range | Z_in/control range |',
        '| --- | --- | --- | --- | --- | --- | --- |']
    comparisons={}
    for v in rows:
        convergence[v['name']]={k:max(abs(s['estimates'][-1][k]-s['estimates'][-2][k])/abs(s['estimates'][-1][k]) for s in v['sections']) for k in keys}
        base=controls[v['family']]
        ratios={k:[s['estimates'][-1][k]/b['estimates'][-1][k] for s,b in zip(v['sections'],base['sections'])] for k in keys}
        comparisons[v['name']]={k:[min(a),max(a)] for k,a in ratios.items()}
        lines.append(line([v['name']]+[convergence[v['name']][k] for k in keys]+[', '.join(f'{a:.9f}' for a in comparisons[v['name']][k]) for k in keys]))
    r['section_convergence']=convergence;r['matched_control_ratios']=comparisons
    best=max(r['results'],key=lambda v:v['strict']['lat_rob_n'])
    counts=r['outcome_counts']
    lines+=['',f'Grid outcome: {counts}. Best strict lateral observation is {best["name"]}: {best["strict"]["lat_rob_n"]:.9f}N, SF/358={best["strict"]["lateral_sf_358"]:.9f}, fatigue SF={best["strict"]["fatigue_sf"]:.9f}. This is a descriptive mesh observation, not an all-gate winner. Full qualification=false.',
      '','Reproduce numerical evidence with the pinned/niced native build and runner; existing outputs intentionally prevent accidental overwrite. The portable audit performs no native replay and requires no target directory, Cargo settings or CPU affinity. Any present raw evidence must match its recorded hash. Native source fingerprints, tracked genomes, measurements, thresholds and report tables are checked.','',
      'Validation commands: `python3 ops/ci/g16_06_rsv2_08/audit_phase9.py` and `python3 -m unittest ops/ci/tests/test_portable_phase7.py -v`. The latter retains the existing Phase7/8 portability regressions.','',
      '| Case | s_mm | Area mm² | Z_out mm³ | Z_in mm³ | Min sampled thickness mm | Max sampled thickness mm |',
      '| --- | --- | --- | --- | --- | --- | --- |']
    for v in rows:
        for s in v['sections']:
            e=s['estimates'][-1]
            lines.append('| '+' | '.join([v['name'],str(s['s_mm'])]+[f'{e[k]:.9f}' for k in keys+['min_thickness_mm','max_thickness_mm']])+' |')
    report.write_text(report.read_text()+'\n'.join(lines)+'\n')
    for p in [report,HERE/'finalize_phase9.py',ROOT/'ops/ci/g16_06_rsv2_08/__init__.py',ROOT/'ops/ci/g16_06_rsv2_08/audit_phase9.py',ROOT/'ops/ci/tests/test_portable_phase7.py']:
        r['sha256'][str(p.relative_to(ROOT))]=digest(p)
    receipt.write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':publish()
