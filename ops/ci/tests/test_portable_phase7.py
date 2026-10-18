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

if __name__=='__main__':unittest.main(verbosity=2)
