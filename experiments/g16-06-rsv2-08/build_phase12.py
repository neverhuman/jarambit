#!/usr/bin/env python3
"""Author a saved-result synthesis; the separate master verifier is read-only."""
import sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
sys.path.insert(0,str(ROOT))
import json,os,shutil
from ops.ci.g16_06_rsv2_08 import audit_phase12 as master
OUT=ROOT/'target/g16-06-rsv2-08/phase12'
def build():
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10 and os.environ['CARGO_BUILD_JOBS']=='2'
    report=HERE/'PHASE12.md';receipt=HERE/'phase12_receipt.json';assert not report.exists() and not receipt.exists()
    parents=master.load_receipts(ROOT);initial=json.loads((OUT/'initial-state.json').read_text());history={}
    olddir=OUT/'parent_receipts_before';olddir.mkdir()
    for n in [9,10,11]:
        rel=master.receipt_rel(n);assert master.digest(ROOT/rel)==initial['receipt_sha256'][str(n)]
        shutil.copy2(ROOT/rel,olddir/f'phase{n}_receipt.json')
    shared=master.digest(ROOT/master.TEST)
    for n,fields in master.METADATA_FIELDS.items():
        history[str(n)]=[]
        for keys in fields:
            before=master.get_field(parents[n],keys)
            after=shared if keys[-1] in [master.TEST,'current_sha256','shared_test_sha256_after'] else master.digest(ROOT/keys[-1])
            master.set_field(parents[n],keys,after);history[str(n)].append(dict(keys=keys,before=before,after=after))
        (ROOT/master.receipt_rel(n)).write_text(json.dumps(parents[n],indent=2)+'\n')
    phases=master.chain_audits(ROOT);parents=master.load_receipts(ROOT);rows=master.synthesize(parents);ranking=master.pareto_rank(rows)
    frontier=[v['name'] for v in ranking if v['front']==1];by_name={v['name']:v for v in rows}
    r=dict(task='G16-06-RSV2-08-p12',state='complete',errors=[],read_only_master=True,native_solves_run=0,production_sources_changed=False,
        execution=dict(cpus=[12,13],nice=os.getpriority(os.PRIO_PROCESS,0),cargo_build_jobs=2),candidate_count=36,
        tip_bending_definition=master.TIP_DEFINITION,pareto_policy=master.PARETO_POLICY,parent_metadata_updates=history,
        phase_receipts_before_metadata_sha256={str(n):initial['receipt_sha256'][str(n)] for n in [9,10,11]},
        phase_receipts_sha256={str(n):master.digest(ROOT/master.receipt_rel(n)) for n in range(7,12)},
        phase_reports_sha256={str(n):master.digest(ROOT/master.report_rel(n)) for n in range(7,12)},
        phase_audit_summaries={n:master.stable_audit_summary(v) for n,v in phases.items()},results=rows,pareto_ranking=ranking,frontier=frontier,
        qualified_frontier=[],outcome_counts=master.outcome_counts(rows),all_gate_qualification=False)
    lines=['# G16-06-RSV2-08 Phase 12','',
        f'Complete saved-evidence synthesis:36 candidates; {len(frontier)} members on Pareto front1; {max(v["front"] for v in ranking)} nondomination layers. Full qualification=false. No native solve, gate change, commit or push.','',
        'The read-only master audit chains all five saved Phase7..11 audits. Phase7 (144 cases/720 mesh observations) and Phase8 (12/60) are independently verified historical prototype studies. The same36 native delayed-taper profiles occur in Phase9 (108 fine observations plus two early controls), Phase10 (108 GCI observations) and Phase11 (36 A4 solves). Historical prototype and production constructions retain their own source/receipt identities.','',
        'The synthesis views are side_a2=Phase9 ring-clamp certificate, side_a3=Phase10 three-level plate GCI, side_a4=Phase11 elastic-foundation buckling/support consistency. All candidate names, families, genome bytes/genes, certificate minima and.25mm² mesh/thickness hashes are cross-checked. The certificate is min(seq_ratio at.25/.125); support_seq_margin=seq_a4-seq_cert. Native A4 does not replace the certificate.','',
        'Objective definition: tip_bending_safety_factor is Phase9 strict.native_hunter_sf at.0625mm², exactly strict.lat_rob_n /260N. This is the existing native tip-loaded lateral plate safety factor with its native stress region; no separately isolated4–15mm bending peak or independent3D safety factor is present in these receipts. Certification and all-three-mesh minimum hunter factors are retained alongside the strict objective. Torsional SF and fatigue SF retain their separate meanings.','',
        'Pareto policy: maximize (certified_seq_ratio, support_seq_margin, tip_bending_safety_factor). A dominates B iff every unrounded objective is>=B and at least one is>B. There is no epsilon, normalization, weighting or acceptance-gate substitution. Repeatedly remove the complete nondominated set to obtain fronts1,2,... . Within a front sort by descending objective tuple, then ascending candidate name. Equal vectors retain every name; input order does not affect rank. Tiny saved numerical differences remain visible in the ranking; no physical robustness or global optimum is inferred.','',
        'All36 profiles retain valid GCI sequence/observable metadata and pass both A4 predicates; all36 GCI fits remain oscillatory/unadmitted. All36 strict bending objectives are below1, Gate S passes0 and lateral/fatigue passes0. The Pareto frontier is descriptive and has no qualified candidate. Full qualification=false; every prior gate, threshold and unverified requirement remains in force.','',
        '## Complete cross-phase synthesis','',
        '| Rank | Front | Candidate | Certified sequence | Support sequence margin | Strict tip-loaded bending SF | GCI sequence valid | GCI observable valid | GCI admitted | A4 eigenvalue | A4 pass |',
        '| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |']
    lines.extend(master.summary_line(by_name[v['name']],v) for v in ranking)
    lines+=['','## Pareto frontier','',f'Front1 contains {len(frontier)} candidates in deterministic order:','']
    lines.extend(f'{i}. `{name}`' for i,name in enumerate(frontier,1))
    lines+=['','## Receipt lineage and verification','',
        'Shared-test additions require dependent hash refreshes in Phase9..11. Only the enumerated shared-test/current-test/parent-receipt references change. Restoring those references reconstructs each original receipt byte for byte; all prior report bytes, measurements, gate decisions and construction/source identities remain unchanged. Phase7/8 receipts remain byte-identical. The master requires each reconstruction and every current receipt/report hash.','',
        'The verifier reads files and performs deterministic arithmetic. It never writes metadata, launches a solver, calls Git, or requires Cargo/CPU affinity. Missing target files are optional in the chained audits; every present raw artifact remains subject to integrity checks. The saved report/receipt and candidate inputs are required. A relocated fixture without target/.git verifies the same synthesis.','',
        'Commands: `nice -n 10 taskset -c 12,13 python3 ops/ci/g16_06_rsv2_08/audit_phase12.py`; `nice -n 10 taskset -c 12,13 python3 -m unittest ops/ci/tests/test_portable_phase7.py -v`; `git diff --check`. Authoring uses Cargo jobs2 and CPU12,13/nice prefix; numerical solves run=0. The assignment forbids committing or pushing.','']
    report.write_text('\n'.join(lines))
    refs=[report,Path(__file__).resolve(),ROOT/'ops/ci/g16_06_rsv2_08/audit_phase12.py',ROOT/master.TEST,OUT/'ASSIGNMENT.md',OUT/'initial-state.json']
    for n in range(7,12):
        refs.extend([ROOT/master.receipt_rel(n),ROOT/master.report_rel(n),ROOT/(f'experiments/G16-06-RSV2-08/audit_phase{n}.py' if n<9 else f'ops/ci/g16_06_rsv2_08/audit_phase{n}.py')])
    refs.append(ROOT/'experiments/G16-06-RSV2-08/portable_study_audit.py')
    refs.extend(olddir/f'phase{n}_receipt.json' for n in [9,10,11])
    r['sha256']={str(p.relative_to(ROOT)):master.digest(p) for p in refs}
    receipt.write_text(json.dumps(r,indent=2)+'\n')
    print(json.dumps(dict(candidates=36,frontier_count=len(frontier),fronts=max(v['front'] for v in ranking),outcomes=r['outcome_counts'])))
if __name__=='__main__':build()
