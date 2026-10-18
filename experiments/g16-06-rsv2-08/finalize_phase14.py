#!/usr/bin/env python3
"""Publish native observations and refresh only dependent historical digests."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
sys.path.insert(0,str(ROOT))
import json,os,shutil
from ops.ci.g16_06_rsv2_08 import audit_phase14 as audit
OUT=ROOT/'target/g16-06-rsv2-08/phase14'
def finalize():
    assert os.sched_getaffinity(0)=={12,13} and os.environ['CARGO_BUILD_JOBS']=='2' and os.getpriority(os.PRIO_PROCESS,0)>=10
    observations=audit.read(OUT/'observations.json');rows=observations['results'];assert len(rows)==16 and not observations['errors']
    seed_dir=HERE/'phase14_candidates';seed=audit.read(seed_dir/'seed_provenance.json')
    for key,name in [('original','c4_seed'),('fixed','fixed_seed')]:
        p=seed_dir/f'{name}.csv';seed[key+'_genome']=str(p.relative_to(ROOT));seed[key+'_sha256']=audit.digest(p)
    seed['provenance']=str((seed_dir/'seed_provenance.json').relative_to(ROOT))
    for row in rows:
        row['geometry_legal']=not row['geometry_rejects'];row['fem_complete']=all(l['result'] is not None for l in row['levels'])
        row['observed_target_flags']=audit.flags(row);row['reported_targets_pass']=row['geometry_legal'] and row['aesthetic_pass'] and all(row['observed_target_flags'].values())
    initial=audit.read(OUT/'initial-state.json');parents={n:audit.read(ROOT/audit.master.receipt_rel(n)) for n in range(9,14)}
    olddir=OUT/'parent_receipts_before';olddir.mkdir(exist_ok=True);history={}
    for n in parents:
        p=ROOT/audit.master.receipt_rel(n);assert audit.digest(p)==initial['receipts'][str(n)];shutil.copy2(p,olddir/f'phase{n}_receipt.json')
    for n,fields in audit.METADATA_FIELDS.items():
        history[str(n)]=[]
        for keys in fields:
            before=audit.master.get_field(parents[n],keys);after=audit.metadata_after(ROOT,keys,parents)
            audit.master.set_field(parents[n],keys,after);history[str(n)].append(dict(keys=keys,before=before,after=after))
        (ROOT/audit.master.receipt_rel(n)).write_text(json.dumps(parents[n],indent=2)+'\n')
    counts=audit.counts(rows)
    r=dict(task='G16-06-RSV2-08-p14',state='complete',errors=[],cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2,solver='faer',max_concurrent_evaluations=2,
        expected_cases=16,completed_cases=16,mesh_observations=48,grid=dict(dn37_mm=audit.DNS,ea_deg=audit.ANGLES,dn36_factor=.65,dn38_factor=.80,fixed_genes={str(k):v for k,v in audit.FIXED.items()}),
        areas_coarse_to_fine_mm2=audit.AREAS,refinement_ratio_r=2.,gci_safety_factor=1.25,method_id='relative_gci.v2',metric_definitions=audit.METRICS,thresholds=audit.THRESHOLDS,observed_sequence_floor=1.25,
        seed=seed,results=rows,outcome_counts=counts,production_sources_changed=False,all_gate_qualification=False,
        unverified=['actual-export 3D Gate S','strict .0625mm2/certification .125mm2','A4 and remaining independent 3D qualification gates'],
        source_provenance=audit.read(OUT/'source_provenance.json'),parent_receipts_before_metadata_sha256=initial['receipts'],parent_metadata_updates=history,
        parent_reports_sha256={audit.STUDY+f'/PHASE{n}.md':audit.digest(HERE/f'PHASE{n}.md') for n in range(9,14)},sha256={})
    lines=['# G16-06-RSV2-08 Phase 14','',
        f'Complete:16/16 assigned candidates and48/48 native mesh observations. Geometry passes={counts["geometry_legal"]}/16; aesthetic passes={counts["aesthetic_pass"]}/16; both={counts["geometry_and_aesthetic_pass"]}/16. **The requested confirmation of16/16 passing both gates is contradicted by native measurement.** Every candidate fails the unchanged holder-envelope gate (measured oriented boxes exceed200x100mm); none is qualified. All native rejection strings are retained below.','',
        'Candidates are generated mechanically from native `Genome::c4_seed(6.0)` (Ti-6Al-4V). Fixed genes: EDGE_TH=.192mm,TAPER_X0=148mm,T_TIP=4.4mm,RING_WALL=6.20mm,P1_ON=1,POCKET_D=.25mm. The native P1 canonicalization additionally sets the seed FLUTE gene78 from.6 to0; no other unassigned seed gene changes. Raw native C4 seed, before-clamp genes, canonical P1 seed, material, full128-gene offspring CSVs and native control points are recorded. Offspring change only CP36=.65*dn37,CP37=dn37,CP38=.80*dn37 and EA_IN=EA_SP. Positive CP genes are native outward-normal distances, not Cartesian coordinates. dn37=.30/.40/.50/.60mm crossed with EA=18.0/18.2/18.4/18.6deg. Every offspring must already equal native clamp; no repair or design alteration.','',
        'Mesh and observable definitions were unspecified in the assignment. A clarification was submitted and the stated native defaults were used: coarse/medium/fine maximum triangle areas4/1/.25mm², h=2/1/.5mm, uniform refinement r=2. Each uses unbiased production `fem::report_fields` and production `export::thickness_at`; geometry, loads, supports, exclusions, material and gates are unchanged. These levels support the native GCI method and do not supply the prior .125mm² certification or .0625mm² strict observations.','',
        '`F_lat`=native Fem.lat_rob (joint-adjusted lateral tip-load capacity in N). `F_ax`=native Fem.hm_tip_inplane_n, the conservative envelope of chord-axial and transverse in-plane tip-load capacities in N; pure axial-only capacity is not separately exposed by this native API. `SF_fat`=native Fem.fat_sf, Goodman fatigue under180N cyclic lateral load. Tip GCI applies to F_lat, a tip-loaded whole-plate capacity outside the native4mm exclusion, not a local tip-zone-only or solid3D observable. Torsional GCI applies to Fem.twist_joint_nmm, native global tip-loaded chord torsion with native section correction. The separate native annular tipzone SF remains recorded in the receipt.','',
        'Both estimates call unmodified native `gci::compute_gci`, relative_gci.v2, Fs=1.25, fine-to-coarse values and distinct exact mesh/thickness SHA256 identities. All native fit flags, rejection reasons, order p, extrapolated value and relative fine GCI are retained. Python reference arithmetic independently recomputes both estimates. `None` GCI means no admitted numerical estimate; asymptotic_validation=not_established for this three-level study. Admission is not qualification and no new GCI acceptance threshold is invented.','',
        'The assignment cites another lane\'s EA<=18.6deg/26deg solid Gate S result. This lane retains its native25deg law limit and.20mm apex limit unchanged: all assigned genes pass that law predicate. No exported solid is generated here; native law bounds alone cannot establish 3D S3 or full actual-export Gate S. All16 native geometry failures are binding. Existing engineering thresholds remain358N lateral, fatigue>=1, sequence1.20 certification/1.25 strict and native torsion/human-max floors. The receipt additionally reports each all-three-study-level comparison using the stricter1.25 sequence floor, explicitly as observations rather than certification.','',
        'Execution: `nice -n 10 taskset -c 12,13`, effective process nice='+str(r['nice'])+' (host inherited nice10), CARGO_BUILD_JOBS=2, two native workers, faer solver, CARGO_INCREMENTAL=0 and lane-local target. Native FEM/thickness/GCI sources unchanged. Full qualification=false. Actual-export3D Gate S, finer certification/strict levels, A4 and remaining independent3D gates remain unverified for these offspring. No staging, commit, push, PR or stacking.','',
        'Outcome counts: '+json.dumps(counts,sort_keys=True)+'.','',
        '| Candidate | Geometry pass | Aesthetic pass | Fine F_lat N | Fine F_ax envelope N | Fine SF_fat | Tip GCI fit | Tip relative GCI | Torsional GCI fit | Torsional relative GCI | Reported conjunction |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    lines += [audit.summary_line(v) for v in rows]
    lines += ['', '| Candidate | Level | Area mm² | F_lat N | F_ax envelope N | SF_fat | Torsional capacity Nmm | Sequence | Mesh/thickness SHA256 |','| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    lines += [l for v in rows for l in audit.mesh_lines(v)]
    lines += ['', '| Candidate | Native gates::check rejects | Native aesthetic::evaluate failures |','| --- | --- | --- |']
    lines += [audit.gate_line(v) for v in rows]
    lines += ['', 'Only dependent shared-test and receipt hash references in Phase9..13 were refreshed. The portable verifier restores recorded previous references to reconstruct every original parent receipt byte for byte. Prior measurements, verdicts, ranking, reports and historical checkers are unchanged. Raw target evidence is optional when absent; present evidence must match its hash and published measurement. The verifier needs no target, Git, Cargo, solver, affinity or worker environment and launches no processes.',
        '', 'Validation commands: `nice -n 10 taskset -c 12,13 python3 ops/ci/g16_06_rsv2_08/audit_phase14.py`; `nice -n 10 taskset -c 12,13 python3 -m unittest discover -s ops/ci/tests -p "test_portable_phase7.py"`; `git diff --check`. A relocated source/data fixture without target/.git is also checked. Floating-point tolerances apply only to cross-language GCI arithmetic and native CP distance identities; gate comparisons remain exact.','']
    report=HERE/'PHASE14.md';report.write_text('\n'.join(lines))
    refs=[report,Path(__file__).resolve(),HERE/'phase14.rs',HERE/'build_phase14.sh',HERE/'run_phase14.py',ROOT/'ops/ci/g16_06_rsv2_08/audit_phase14.py',ROOT/audit.TEST,
        OUT/'measure',OUT/'source_provenance.json',OUT/'build_provenance.json',OUT/'initial-state.json',OUT/'ASSIGNMENT.md',OUT/'observations.json']+list(seed_dir.glob('*'))
    for n in parents:refs += [ROOT/audit.master.receipt_rel(n),olddir/f'phase{n}_receipt.json']
    for p in refs:r['sha256'][str(p.relative_to(ROOT))]=audit.digest(p)
    (HERE/'phase14_receipt.json').write_text(json.dumps(r,indent=2)+'\n')
if __name__=='__main__':finalize()
