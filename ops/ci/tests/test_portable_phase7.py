"""Phase7/8 portability regressions plus Phase9 production-study integrity."""
import importlib.util
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT))
LEGACY=ROOT/'experiments/G16-06-RSV2-08'
sys.path.insert(0,str(LEGACY))
spec=importlib.util.spec_from_file_location('legacy_portable_phase7_tests',LEGACY/'test_portable_phase7.py')
legacy=importlib.util.module_from_spec(spec)
spec.loader.exec_module(legacy)
PortablePhase7Tests=legacy.PortablePhase7Tests
from ops.ci.g16_06_rsv2_08 import audit_phase9 as phase9
from ops.ci.g16_06_rsv2_08 import audit_phase10 as phase10
from ops.ci.g16_06_rsv2_08 import audit_phase11 as phase11
from ops.ci.g16_06_rsv2_08 import audit_phase12 as phase12
from ops.ci.g16_06_rsv2_08 import audit_phase13 as phase13

REAL_STAT=Path.stat
def absent_target(path,*args,**kwargs):
    if path.is_relative_to(ROOT/'target'):raise FileNotFoundError(str(path))
    return REAL_STAT(path,*args,**kwargs)

class PortablePhase9Tests(unittest.TestCase):
    def document(self):return json.loads((ROOT/phase9.STUDY/'phase9_receipt.json').read_text())
    def report(self):return (ROOT/phase9.STUDY/'PHASE9.md').read_text()
    def check(self,r):phase9.audit_document(ROOT,r,self.report())

    def test_default_audit(self):
        r=phase9.audit()
        self.assertEqual((r['status'],r['cases'],r['mesh_observations'],r['control_mesh_observations']),('pass',36,108,6))

    def test_without_target_and_worker_environment(self):
        with patch.object(Path,'stat',new=absent_target),patch('os.sched_getaffinity',return_value={0},create=True),patch.dict('os.environ',{},clear=True):
            r=phase9.audit()
        self.assertEqual(r['status'],'pass')
        self.assertGreater(r['optional_raw_unavailable'],0)
        self.assertEqual(r['native_replay'],'not_run')

    def test_missing_tracked_candidate_is_fatal(self):
        genome=ROOT/self.document()['results'][0]['genome']
        def missing(p,*args,**kwargs):
            if p==genome:raise FileNotFoundError(str(p))
            return REAL_STAT(p,*args,**kwargs)
        with patch.object(Path,'stat',new=missing),self.assertRaisesRegex(ValueError,'Missing tracked candidate'):
            phase9.audit()

    def test_present_raw_corruption_is_fatal(self):
        r=self.document();raw=ROOT/r['results'][0]['raw'];real_digest=phase9.digest
        def corrupt(p):return '0'*64 if p==raw else real_digest(p)
        def present(p):return True if p==raw else Path.is_file(p)
        with patch.object(phase9,'exists',side_effect=present),patch.object(phase9,'digest',side_effect=corrupt),self.assertRaisesRegex(ValueError,'Artifact SHA-256 mismatch'):
            phase9.audit()

    def test_threshold_change_rejected(self):
        r=self.document();r['thresholds']['lateral_n']=180.
        with self.assertRaisesRegex(ValueError,'thresholds changed'):self.check(r)

    def test_grid_change_rejected(self):
        r=self.document();r['results'][0]['genes'][47]=135.
        with self.assertRaisesRegex(ValueError,'Cartesian grid mismatch'):self.check(r)

    def test_missing_mesh_rejected(self):
        r=self.document();r['results'][0]['levels'].pop()
        with self.assertRaisesRegex(ValueError,'FEM refinement grid changed'):self.check(r)

    def test_point_margin_change_rejected(self):
        r=self.document();r['results'][0]['exact_law_point_samples'][0]['margin_mm']=1.
        with self.assertRaisesRegex(ValueError,'Direct point margin mismatch'):self.check(r)

    def test_section_modulus_change_rejected(self):
        r=self.document();r['results'][0]['sections'][0]['estimates'][0]['Z_out_of_plane_mm3']*=2.
        with self.assertRaisesRegex(ValueError,'Section modulus identity mismatch'):self.check(r)

    def test_fem_denominator_change_rejected(self):
        r=self.document();r['results'][0]['levels'][0]['result']['metrics']['lateral_sf_358']*=2.
        with self.assertRaisesRegex(ValueError,'denominator changed'):self.check(r)

    def test_false_gate_pass_rejected(self):
        r=self.document();r['results'][0]['gate_s_pass']=not r['results'][0]['gate_s_pass']
        with self.assertRaisesRegex(ValueError,'Gate S aggregate mismatch'):self.check(r)

    def test_false_lateral_fatigue_pass_rejected(self):
        r=self.document();r['results'][0]['lateral_fatigue_screen']=not r['results'][0]['lateral_fatigue_screen']
        with self.assertRaisesRegex(ValueError,'screen flag mismatch'):self.check(r)

    def test_report_change_rejected(self):
        r=self.document();name=r['results'][0]['name'];text=self.report()
        text=text.replace('| '+name+' | 4.0 |','| CHANGED | 4.0 |',1)
        with self.assertRaisesRegex(ValueError,'Report section table mismatch'):phase9.audit_document(ROOT,r,text)

    def test_report_fem_change_rejected(self):
        r=self.document();name=r['results'][0]['name'];text=self.report()
        changed=text.replace('| '+name+' | 0.250000000 |','| CHANGED | 0.250000000 |',1)
        self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report FEM table mismatch'):phase9.audit_document(ROOT,r,changed)

    def test_control_comparison_change_rejected(self):
        r=self.document();name=r['results'][0]['name']
        r['matched_control_ratios'][name]['area_mm2'][0]=0.
        with self.assertRaisesRegex(ValueError,'control comparison mismatch'):self.check(r)

class PortablePhase10Tests(unittest.TestCase):
    def document(self):return json.loads((ROOT/phase10.STUDY/'phase10_receipt.json').read_text())
    def report(self):return (ROOT/phase10.STUDY/'PHASE10.md').read_text()
    def check(self,r):phase10.audit_document(ROOT,r,self.report())

    def test_default_audit(self):
        r=phase10.audit()
        self.assertEqual((r['status'],r['cases'],r['mesh_observations']),('pass',36,108))

    def test_without_target_or_worker_environment(self):
        with patch.object(Path,'stat',new=absent_target),patch('os.sched_getaffinity',return_value={0},create=True),patch.dict('os.environ',{},clear=True):
            r=phase10.audit()
        self.assertEqual(r['status'],'pass');self.assertGreater(r['optional_raw_unavailable'],0)
        self.assertEqual(r['native_replay'],'not_run')

    def test_missing_observation_rejected(self):
        r=self.document();r['results'][0]['plate_gci']['levels_fine_to_coarse'].pop()
        with self.assertRaisesRegex(ValueError,'mesh level order changed'):self.check(r)

    def test_reversed_levels_rejected(self):
        r=self.document();r['results'][0]['plate_gci']['levels_fine_to_coarse'].reverse()
        with self.assertRaisesRegex(ValueError,'mesh level order changed'):self.check(r)

    def test_sequence_valid_change_rejected(self):
        r=self.document();e=r['results'][0]['plate_gci']['seq_ratio_gci'];e['sequence_valid']=not e['sequence_valid']
        with self.assertRaisesRegex(ValueError,'GCI sequence_valid mismatch'):self.check(r)

    def test_observable_valid_change_rejected(self):
        r=self.document();e=r['results'][0]['plate_gci']['seq_ratio_gci'];e['observable_valid']=not e['observable_valid']
        with self.assertRaisesRegex(ValueError,'GCI observable_valid mismatch'):self.check(r)

    def test_false_admission_rejected(self):
        r=self.document();e=r['results'][0]['plate_gci']['seq_ratio_gci'];e['admitted']=not e['admitted']
        with self.assertRaisesRegex(ValueError,'GCI admitted mismatch'):self.check(r)

    def test_observable_binding_change_rejected(self):
        r=self.document();r['results'][0]['plate_gci']['seq_ratio_gci']['levels'][0]['observable']['region']='other_region'
        with self.assertRaisesRegex(ValueError,'mesh/observable binding mismatch'):self.check(r)

    def test_gci_arithmetic_change_rejected(self):
        r=self.document();r['results'][0]['plate_gci']['seq_ratio_gci']['p']=123.
        with self.assertRaisesRegex(ValueError,'GCI p arithmetic mismatch'):self.check(r)

    def test_safety_factor_change_rejected(self):
        r=self.document();r['gci_safety_factor']=1.20
        with self.assertRaisesRegex(ValueError,'method/thresholds changed'):self.check(r)

    def test_phase9_binding_change_rejected(self):
        r=self.document();r['results'][0]['phase9_fine_binding']['sequence_relative_error']=1.
        with self.assertRaisesRegex(ValueError,'Phase9 replay binding mismatch'):self.check(r)

    def test_report_gci_change_rejected(self):
        r=self.document();row=r['results'][0];e=row['plate_gci']['seq_ratio_gci'];text=self.report()
        old=phase10.line([row['name']]+[e[k] for k in ['sequence_valid','observable_valid','admitted','fit_status','reason','p','gci_fine']]+[row['phase9_fine_binding']['within_owner_bar']])
        changed=text.replace(old,old.replace(row['name'],'CHANGED'),1);self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report GCI table mismatch'):phase10.audit_document(ROOT,r,changed)

    def test_report_mesh_change_rejected(self):
        r=self.document();name=r['results'][0]['name'];text=self.report()
        changed=text.replace('| '+name+' | 0.25 |','| CHANGED | 0.25 |',1);self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report mesh table mismatch'):phase10.audit_document(ROOT,r,changed)

    def test_present_raw_corruption_rejected(self):
        raw=ROOT/self.document()['results'][0]['raw'];real_digest=phase10.digest
        def corrupt(p):return '0'*64 if p==raw else real_digest(p)
        def present(p):return True if p==raw else Path.is_file(p)
        with patch.object(phase10,'exists',side_effect=present),patch.object(phase10,'digest',side_effect=corrupt),self.assertRaisesRegex(ValueError,'Artifact SHA-256 mismatch'):
            phase10.audit()

    def test_valid_metadata_does_not_admit_oscillation(self):
        g=self.document()['results'][0]['plate_gci'];levels=[]
        for l in g['seq_ratio_gci']['levels']:
            levels.append(phase10.Level(phase10.MeshRef(**l['mesh']),l['h'],phase10.Observable(**l['observable']),l['refinement']))
        e=phase10.compute_gci(1.20,1.18,1.21,2.,1.25,levels=levels)
        self.assertTrue(e.sequence_valid and e.observable_valid)
        self.assertFalse(e.admitted);self.assertEqual(e.fit_status.value,'oscillatory')

class PortablePhase11Tests(unittest.TestCase):
    def document(self):return json.loads((ROOT/phase11.STUDY/'phase11_receipt.json').read_text())
    def report(self):return (ROOT/phase11.STUDY/'PHASE11.md').read_text()
    def check(self,r):phase11.audit_document(ROOT,r,self.report())

    def test_default_audit(self):
        r=phase11.audit()
        self.assertEqual((r['status'],r['cases'],r['a4_observations']),('pass',36,36))

    def test_without_target_or_worker_environment(self):
        with patch.object(Path,'stat',new=absent_target),patch('os.sched_getaffinity',return_value={0},create=True),patch.dict('os.environ',{},clear=True):
            r=phase11.audit()
        self.assertEqual(r['status'],'pass');self.assertGreater(r['optional_raw_unavailable'],0)
        self.assertEqual((r['native_replay'],r['raw_meshes_checked']),('not_run',0))

    def test_present_raw_corruption_is_fatal(self):
        raw=ROOT/self.document()['results'][0]['raw'];real_digest=phase11.digest
        def corrupt(p):return '0'*64 if p==raw else real_digest(p)
        def present(p):return True if p==raw else Path.is_file(p)
        with patch.object(phase11,'exists',side_effect=present),patch.object(phase11,'digest',side_effect=corrupt),self.assertRaisesRegex(ValueError,'Artifact SHA-256 mismatch'):
            phase11.audit()

    def test_missing_tracked_genome_is_fatal(self):
        genome=ROOT/self.document()['results'][0]['genome']
        def missing(p,*args,**kwargs):
            if p==genome:raise FileNotFoundError(str(p))
            return REAL_STAT(p,*args,**kwargs)
        with patch.object(Path,'stat',new=missing),self.assertRaisesRegex(ValueError,'Missing tracked Phase11 genome'):
            self.check(self.document())

    def test_eigenvalue_threshold_exact(self):
        import math
        self.assertEqual(phase11.verdict(math.nextafter(1.,0.),2.,2.),(False,True))
        self.assertEqual(phase11.verdict(1.,2.,2.),(True,True))

    def test_support_sequence_threshold_exact(self):
        import math
        self.assertEqual(phase11.verdict(2.,math.nextafter(2.,0.),2.),(True,False))
        self.assertEqual(phase11.verdict(2.,2.,2.),(True,True))

    def test_positive_native_eigenvalue_does_not_pass_buckling(self):
        self.assertEqual(phase11.verdict(.5,3.,2.),(False,True))

    def test_false_buckling_pass_rejected(self):
        r=self.document();v=r['results'][0];v['buckling_pass']=not v['buckling_pass']
        with self.assertRaisesRegex(ValueError,'A4 gate verdict mismatch'):self.check(r)

    def test_false_support_sequence_pass_rejected(self):
        r=self.document();v=r['results'][0];v['support_sequence_pass']=not v['support_sequence_pass']
        with self.assertRaisesRegex(ValueError,'A4 gate verdict mismatch'):self.check(r)

    def test_certificate_minimum_change_rejected(self):
        r=self.document();r['results'][0]['seq_cert_input']+=.01
        with self.assertRaisesRegex(ValueError,'Certificate sequence minimum changed'):self.check(r)

    def test_support_constant_change_rejected(self):
        r=self.document();r['results'][0]['a4']['grip_foundation_stiffness']['k_foundation_n_per_mm3']=201.
        with self.assertRaisesRegex(ValueError,'Grip foundation model changed'):self.check(r)

    def test_mesh_binding_change_rejected(self):
        r=self.document();r['results'][0]['mesh_sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'Parent mesh/replay binding mismatch'):self.check(r)

    def test_missing_a4_observation_rejected(self):
        r=self.document();r['results'][0]['a4']=None
        with self.assertRaisesRegex(ValueError,'A4 completeness mismatch'):self.check(r)

    def test_changed_margin_rejected(self):
        r=self.document();r['results'][0]['margins']['sequence']=100.
        with self.assertRaisesRegex(ValueError,'A4 margin mismatch'):self.check(r)

    def test_prior_receipt_reconstruction_required(self):
        r=self.document();r['phase10_parent_receipt_sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'Phase10 changed beyond dependent hash metadata'):self.check(r)

    def test_report_a4_change_rejected(self):
        r=self.document();v=r['results'][0];a=v['a4'];text=self.report()
        old=phase11.line([v['name'],a['buckling_eigenvalue'],v['margins']['buckling'],a['supported_seq_ratio'],v['seq_cert_input'],v['margins']['sequence'],v['buckling_pass'],v['support_sequence_pass'],v['side_a4_pass'],a['status']])
        changed=text.replace(old,old.replace(v['name'],'CHANGED'),1);self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report A4 table mismatch'):phase11.audit_document(ROOT,r,changed)

    def test_report_mesh_change_rejected(self):
        r=self.document();v=r['results'][0];text=self.report()
        old='| '+v['name']+' | '+str(v['a4']['n_node'])+' |'
        changed=text.replace(old,'| CHANGED | '+str(v['a4']['n_node'])+' |',1);self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report A4 mesh table mismatch'):phase11.audit_document(ROOT,r,changed)

    def test_changed_gate_threshold_rejected(self):
        r=self.document();r['thresholds']['buckling_eigenvalue']=.5
        with self.assertRaisesRegex(ValueError,'A4 acceptance thresholds changed'):self.check(r)

class PortablePhase12Tests(unittest.TestCase):
    def document(self):return json.loads((ROOT/phase12.STUDY/'phase12_receipt.json').read_text())
    def report(self):return (ROOT/phase12.STUDY/'PHASE12.md').read_text()
    def check(self,r):phase12.audit_document(ROOT,r,self.report())
    def candidate(self,name,values):return dict(name=name,objectives=dict(zip(phase12.OBJECTIVES,values)))

    def test_default_master_audit(self):
        r=phase12.audit();self.assertEqual((r['status'],r['candidates']),('pass',36))
        self.assertEqual(set(r['phases']),{'7','8','9','10','11'});self.assertGreater(r['pareto_frontier_count'],0)

    def test_read_only_without_git_or_native_processes(self):
        original=Path.open
        def readonly(p,mode='r',*args,**kwargs):
            if any(c in mode for c in 'wax+'):raise AssertionError('Master audit attempted a write')
            return original(p,mode,*args,**kwargs)
        with patch.object(Path,'open',new=readonly),patch('subprocess.run',side_effect=AssertionError('Process launch')),patch('subprocess.Popen',side_effect=AssertionError('Process launch')):
            self.assertEqual(phase12.audit()['status'],'pass')

    def test_without_target_or_worker_environment(self):
        with patch.object(Path,'stat',new=absent_target),patch('os.sched_getaffinity',return_value={0},create=True),patch.dict('os.environ',{},clear=True):r=phase12.audit()
        self.assertEqual(r['status'],'pass');self.assertGreater(r['optional_raw_unavailable'],0)
        self.assertEqual(r['native_replay'],'not_run')

    def test_chain_failure_is_fatal(self):
        with patch.object(phase12.phase7,'audit',return_value={'status':'fail'}),self.assertRaisesRegex(ValueError,'Phase7 audit failed'):phase12.audit()

    def test_downstream_failure_is_fatal(self):
        with patch.object(phase12.phase11,'audit',side_effect=ValueError('A4 raw evidence corrupted')),self.assertRaisesRegex(ValueError,'A4 raw evidence corrupted'):phase12.audit()

    def test_pareto_tradeoffs_layers_and_equal_vectors(self):
        rows=[self.candidate('A',(3.,3.,3.)),self.candidate('B',(2.,2.,2.)),self.candidate('C',(4.,1.,3.)),self.candidate('D',(3.,3.,3.))]
        ranked=phase12.pareto_rank(rows)
        self.assertEqual([(v['name'],v['front']) for v in ranked],[('C',1),('A',1),('D',1),('B',2)])
        self.assertEqual(ranked[-1]['dominated_by'],['A','D'])

    def test_pareto_independent_of_input_order(self):
        rows=[self.candidate('Z',(2.,3.,4.)),self.candidate('A',(2.,3.,4.)),self.candidate('B',(1.,2.,3.))]
        self.assertEqual(phase12.pareto_rank(rows),phase12.pareto_rank(list(reversed(rows))))
        self.assertEqual([v['name'] for v in phase12.pareto_rank(rows)],['A','Z','B'])

    def test_pareto_has_no_epsilon(self):
        import math
        rows=[self.candidate('A',(math.nextafter(1.,2.),1.,1.)),self.candidate('B',(1.,1.,1.))]
        self.assertEqual([v['front'] for v in phase12.pareto_rank(rows)],[1,2])

    def test_nonfinite_objective_rejected(self):
        with self.assertRaisesRegex(ValueError,'Nonfinite Pareto objective'):phase12.pareto_rank([self.candidate('A',(float('nan'),1.,1.))])

    def test_cross_phase_certificate_change_rejected(self):
        r=self.document();r['results'][0]['side_a2']['certified_seq_ratio']+=.01
        with self.assertRaisesRegex(ValueError,'Cross-phase synthesis mismatch'):self.check(r)

    def test_cross_phase_support_margin_change_rejected(self):
        r=self.document();r['results'][0]['side_a4']['support_seq_margin']+=.01
        with self.assertRaisesRegex(ValueError,'Cross-phase synthesis mismatch'):self.check(r)

    def test_tip_metric_substitution_rejected(self):
        r=self.document();r['results'][0]['objectives']['tip_bending_safety_factor']=r['results'][0]['side_a2']['certified_native_hunter_sf']
        with self.assertRaisesRegex(ValueError,'Cross-phase synthesis mismatch'):self.check(r)

    def test_false_gci_admission_rejected(self):
        r=self.document();r['results'][0]['side_a3']['admitted']=True
        with self.assertRaisesRegex(ValueError,'Cross-phase synthesis mismatch'):self.check(r)

    def test_rank_change_rejected(self):
        r=self.document();r['pareto_ranking'][0]['rank']=100
        with self.assertRaisesRegex(ValueError,'Pareto ranking mismatch'):self.check(r)

    def test_frontier_change_rejected(self):
        r=self.document();r['frontier'].pop()
        with self.assertRaisesRegex(ValueError,'Pareto frontier mismatch'):self.check(r)

    def test_policy_change_rejected(self):
        r=self.document();r['pareto_policy']['directions'][0]='minimize'
        with self.assertRaisesRegex(ValueError,'Objective definition/Pareto policy changed'):self.check(r)

    def test_parent_receipt_reconstruction_required(self):
        r=self.document();r['parent_metadata_updates']['11'][0]['before']='0'*64
        with self.assertRaisesRegex(ValueError,'Phase11 scientific receipt changed'):self.check(r)

    def test_report_change_rejected(self):
        r=self.document();rank=r['pareto_ranking'][0];row=next(v for v in r['results'] if v['name']==rank['name']);text=self.report()
        old=phase12.summary_line(row,rank);changed=text.replace(old,old.replace(row['name'],'CHANGED'),1);self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report synthesis/Pareto table mismatch'):phase12.audit_document(ROOT,r,changed)

class PortablePhase13Tests(unittest.TestCase):
    def document(self):return json.loads((ROOT/phase13.STUDY/'phase13_receipt.json').read_text())
    def report(self):return (ROOT/phase13.STUDY/'PHASE13.md').read_text()
    def check(self,r):phase13.audit_document(ROOT,r,self.report())

    def test_default_audit_is_read_only(self):
        original=Path.open
        def readonly(p,mode='r',*args,**kwargs):
            if any(c in mode for c in 'wax+'):raise AssertionError('Audit attempted a write')
            return original(p,mode,*args,**kwargs)
        with patch.object(Path,'open',new=readonly),patch('subprocess.run',side_effect=AssertionError('Process launch')),patch('subprocess.Popen',side_effect=AssertionError('Process launch')):
            r=phase13.audit()
        self.assertEqual((r['status'],r['cases'],r['mesh_observations']),('pass',24,72))

    def test_without_target_or_worker_environment(self):
        with patch.object(Path,'stat',new=absent_target),patch('os.sched_getaffinity',return_value={0},create=True),patch.dict('os.environ',{},clear=True):r=phase13.audit()
        self.assertEqual(r['status'],'pass');self.assertGreater(r['optional_raw_unavailable'],0)
        self.assertEqual(r['native_replay'],'not_run')

    def test_present_raw_corruption_is_fatal(self):
        raw=ROOT/self.document()['results'][0]['raw'];real_digest=phase13.digest
        def corrupt(p):return '0'*64 if p==raw else real_digest(p)
        def present(p):return True if p==raw else Path.is_file(p)
        with patch.object(phase13,'exists',side_effect=present),patch.object(phase13,'digest',side_effect=corrupt),self.assertRaisesRegex(ValueError,'Artifact SHA-256 mismatch'):
            phase13.audit()

    def test_missing_tracked_candidate_is_fatal(self):
        genome=ROOT/self.document()['results'][0]['genome']
        def missing(p,*args,**kwargs):
            if p==genome:raise FileNotFoundError(str(p))
            return REAL_STAT(p,*args,**kwargs)
        with patch.object(Path,'stat',new=missing),self.assertRaisesRegex(ValueError,'Missing tracked Phase13 candidate'):self.check(self.document())

    def test_assignment_grid_change_rejected(self):
        r=self.document();r['grid']['cp_offsets_mm'][-1]=.8
        with self.assertRaisesRegex(ValueError,'Assignment grid changed'):self.check(r)

    def test_assigned_cp_change_rejected(self):
        r=self.document();r['results'][0]['genes'][36]+=.1
        with self.assertRaisesRegex(ValueError,'Assigned/unassigned gene mismatch'):self.check(r)

    def test_unassigned_parent_gene_change_rejected(self):
        r=self.document();r['results'][0]['genes'][40]+=.1
        with self.assertRaisesRegex(ValueError,'Assigned/unassigned gene mismatch'):self.check(r)

    def test_native_cp_displacement_change_rejected(self):
        r=self.document();r['results'][0]['control_points'][36][0]+=.2
        with self.assertRaisesRegex(ValueError,'Native outward CP displacement mismatch'):self.check(r)

    def test_missing_observation_rejected(self):
        r=self.document();r['results'][0]['levels'].pop()
        with self.assertRaisesRegex(ValueError,'Three-mesh grid changed'):self.check(r)

    def test_certification_minimum_change_rejected(self):
        r=self.document();r['results'][0]['certification']['lat_rob_n']+=1.
        with self.assertRaisesRegex(ValueError,'Certification/strict aggregation mismatch'):self.check(r)

    def test_tip_loaded_and_tipzone_metric_substitution_rejected(self):
        r=self.document();v=r['results'][0];old=v['summary']['twist_tip_sf'];v['summary']['twist_tip_sf']=v['strict']['twist_tipzone_sf'];self.assertNotEqual(old,v['summary']['twist_tip_sf'])
        with self.assertRaisesRegex(ValueError,'Requested metric mapping mismatch'):self.check(r)

    def test_human_max_identity_change_rejected(self):
        r=self.document();r['results'][0]['levels'][0]['result']['metrics']['hm_twist_sf']*=2.
        with self.assertRaisesRegex(ValueError,'torsion/human-max identity mismatch'):self.check(r)

    def test_false_target_flag_rejected(self):
        r=self.document();v=r['results'][0];v['target_flags']['sequence']=not v['target_flags']['sequence']
        with self.assertRaisesRegex(ValueError,'Unchanged target verdict mismatch'):self.check(r)

    def test_lateral_threshold_change_rejected(self):
        r=self.document();r['thresholds']['lateral_n']=260.
        with self.assertRaisesRegex(ValueError,'acceptance thresholds changed'):self.check(r)

    def test_prior_master_receipt_reconstruction_required(self):
        r=self.document();r['parent_metadata_updates']['12'][0]['before']='0'*64
        with self.assertRaisesRegex(ValueError,'Phase12 scientific receipt changed'):self.check(r)

    def test_duplicate_grid_case_rejected(self):
        r=self.document();r['results'][-1]=r['results'][0]
        with self.assertRaisesRegex(ValueError,'Cartesian grid coverage mismatch'):self.check(r)

    def test_report_certification_change_rejected(self):
        r=self.document();name=r['results'][0]['name'];text=self.report();changed=text.replace('| '+name+' |','| CHANGED |',1);self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report certification table mismatch'):phase13.audit_document(ROOT,r,changed)

    def test_report_mesh_change_rejected(self):
        r=self.document();name=r['results'][0]['name'];text=self.report();changed=text.replace('| '+name+' | 0.25 |','| CHANGED | 0.25 |',1);self.assertNotEqual(text,changed)
        with self.assertRaisesRegex(ValueError,'Report native mesh table mismatch'):phase13.audit_document(ROOT,r,changed)

if __name__=='__main__':unittest.main(verbosity=2)
