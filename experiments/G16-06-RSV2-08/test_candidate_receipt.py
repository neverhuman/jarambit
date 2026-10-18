"""Phase15 portable evidence and exact four-gate margin regressions."""
import copy,json,math,sys,unittest
from pathlib import Path
from unittest.mock import patch
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
sys.path.insert(0,str(HERE))
import audit_phase15 as phase15
class JointMarginTests(unittest.TestCase):
    def test_exact_boundary_passes(self):
        r=phase15.joint_margin(dict(phase15.FLOORS));self.assertTrue(r['passed']);self.assertEqual(r['joint_margin'],0.)
    def test_any_one_gate_deficit_fails(self):
        for key,floor in phase15.FLOORS.items():
            with self.subTest(key=key):
                values=dict(phase15.FLOORS);values[key]=math.nextafter(floor,-math.inf)
                self.assertFalse(phase15.joint_margin(values)['passed'])
    def test_normalized_bottleneck_not_average(self):
        values=dict(strict_lat=286.,twist_tip_sf=1.2,twist_tipzone_sf_strict=.8,solid_tip_sf_nodal_max=1.4)
        r=phase15.joint_margin(values);self.assertAlmostEqual(r['joint_margin'],-.2);self.assertFalse(r['passed'])
    def test_missing_solid_is_never_a_pass(self):
        values=dict(phase15.FLOORS);values['solid_tip_sf_nodal_max']=None;r=phase15.joint_margin(values)
        self.assertIsNone(r['passed']);self.assertIsNone(r['joint_margin']);self.assertEqual(r['missing_metrics'],['solid_tip_sf_nodal_max'])
    def test_known_failure_and_missing_still_fail_with_unknown_margin(self):
        values=dict(phase15.FLOORS);values['solid_tip_sf_nodal_max']=None;values['strict_lat']=100.
        r=phase15.joint_margin(values);self.assertFalse(r['passed']);self.assertIsNone(r['joint_margin'])
    def test_nonfinite_and_boolean_metrics_rejected(self):
        for v in [math.nan,math.inf,-math.inf,True,'1.0']:
            values=dict(phase15.FLOORS);values['solid_tip_sf_nodal_max']=v
            with self.subTest(v=v),self.assertRaisesRegex(ValueError,'Nonfinite/non-numeric'):phase15.joint_margin(values)
    def test_extra_or_missing_metric_rejected(self):
        values=dict(phase15.FLOORS);values['solid_tip_sf_nodal_avg']=values.pop('solid_tip_sf_nodal_max')
        with self.assertRaisesRegex(ValueError,'metric coverage'):phase15.joint_margin(values)
class PortablePhase15Tests(unittest.TestCase):
    def document(self):return json.loads((ROOT/phase15.RECEIPT).read_text())
    def report(self):return (HERE/'PHASE15.md').read_text()
    def check(self,r):phase15.audit_document(ROOT,r,self.report())
    def test_default_audit_read_only(self):
        original=Path.open
        def read_only(p,mode='r',*args,**kwargs):
            if any(c in mode for c in 'wax+'):raise AssertionError('write attempted')
            return original(p,mode,*args,**kwargs)
        with patch.object(Path,'open',new=read_only),patch('subprocess.run',side_effect=AssertionError('Process launch')),patch('subprocess.Popen',side_effect=AssertionError('Process launch')):
            r=phase15.audit()
        self.assertEqual((r['status'],r['cases'],r['observations']),('pass',3,12))
    def test_audit_without_target_or_environment(self):
        real=Path.is_file
        def absent(p):return False if p.is_relative_to(ROOT/'target') else real(p)
        with patch.object(Path,'is_file',new=absent),patch.dict('os.environ',{},clear=True),patch('os.sched_getaffinity',return_value={0}):r=phase15.audit()
        self.assertEqual(r['status'],'pass');self.assertGreater(r['optional_raw_unavailable'],0)
    def test_threshold_relaxation_rejected(self):
        r=self.document();r['floors']['strict_lat']=100.
        with self.assertRaisesRegex(ValueError,'policy/thresholds'):self.check(r)
    def test_q7_baseline_cannot_be_changed_to_hide_spine_shift(self):
        r=self.document();r['results'][0]['q7_baseline_cp'][37]+=.1
        with self.assertRaisesRegex(ValueError,'Q7 baseline CP binding'):self.check(r)
    def test_wrong_native_metric_rejected(self):
        r=self.document();r['results'][0]['metrics']['strict_lat']=300.
        with self.assertRaisesRegex(ValueError,'Native metric mapping'):self.check(r)
    def test_false_joint_pass_rejected(self):
        r=self.document();r['results'][0]['joint']['passed']=True
        with self.assertRaisesRegex(ValueError,'margin/verdict'):self.check(r)
    def test_phase14_lineage_cannot_be_invented(self):
        r=self.document();r['phase14_model_status']='Phase14 model verified'
        with self.assertRaisesRegex(ValueError,'Phase14 model lineage'):self.check(r)
    def test_slot_coverage_rejected(self):
        r=self.document();r['results'][-1]=copy.deepcopy(r['results'][0])
        with self.assertRaisesRegex(ValueError,'Slot coverage'):self.check(r)
    def test_unassigned_gene_change_rejected(self):
        r=self.document();r['results'][0]['genes'][42]+=.1
        with self.assertRaisesRegex(ValueError,'genes/material'):self.check(r)
    def test_false_export_gate_s_claim_rejected(self):
        r=self.document();r['results'][0]['exported_gate_s']='pass'
        with self.assertRaisesRegex(ValueError,'exported/independent'):self.check(r)
    def test_false_full_qualification_rejected(self):
        r=self.document();r['all_gate_qualification']=True
        with self.assertRaisesRegex(ValueError,'source/qualification'):self.check(r)
    def test_report_change_rejected(self):
        r=self.document();text=self.report().replace('| 8 |','| CHANGED |',1)
        with self.assertRaisesRegex(ValueError,'four-gate table'):phase15.audit_document(ROOT,r,text)
    def test_missing_tracked_evidence_fatal(self):
        raw=ROOT/self.document()['results'][0]['solid'];original=Path.read_text
        def missing(p,*args,**kwargs):
            if p==raw:raise FileNotFoundError(p)
            return original(p,*args,**kwargs)
        with patch.object(Path,'read_text',new=missing),self.assertRaises(FileNotFoundError):phase15.audit()
    def test_present_raw_corruption_fatal(self):
        raw=ROOT/'target/G16-06-RSV2-08/phase15/runtime.json';original=phase15.digest;real_is_file=Path.is_file
        with patch.object(Path,'is_file',new=lambda p:True if p==raw else real_is_file(p)),patch.object(phase15,'digest',side_effect=lambda p:'0'*64 if p==raw else original(p)),self.assertRaisesRegex(ValueError,'Artifact SHA-256'):phase15.audit()
    def test_solid_nodal_average_is_not_the_requested_observable(self):
        row=self.document()['results'][0];plate=phase15.load(ROOT/row['plate']);solid=phase15.load(ROOT/row['solid'])
        solid['solid']['SF_solid_nodal_avg']=10.*solid['solid']['SF_solid_nodal_max']
        _,metrics,_=phase15.derive(plate,solid)
        self.assertEqual(metrics['solid_tip_sf_nodal_max'],solid['solid']['SF_solid_nodal_max'])
    def test_unconverged_solid_never_supplies_a_gate_value(self):
        row=self.document()['results'][0];plate=phase15.load(ROOT/row['plate']);solid=phase15.load(ROOT/row['solid'])
        solid['solver']['converged']=False
        _,metrics,joint=phase15.derive(plate,solid)
        self.assertIsNone(metrics['solid_tip_sf_nodal_max']);self.assertIsNone(joint['joint_margin'])
    def test_changed_solid_load_rejected(self):
        row=self.document()['results'][0];plate=phase15.load(ROOT/row['plate']);solid=phase15.load(ROOT/row['solid']);solid['T_Nmm']=1000.
        with self.assertRaisesRegex(ValueError,'Solid load/refinement'):phase15.derive(plate,solid)
    def test_primary_residual_threshold_not_relaxed(self):
        row=self.document()['results'][0];plate=phase15.load(ROOT/row['plate']);solid=phase15.load(ROOT/row['solid']);solid['solver']['relative_residual']=1e-9
        with self.assertRaisesRegex(ValueError,'Invalid solid residual'):phase15.derive(plate,solid)
    def test_changed_strict_mesh_rejected(self):
        row=self.document()['results'][0];plate=phase15.load(ROOT/row['plate']);plate['levels'][2]['area_mm2']=.25
        with self.assertRaisesRegex(ValueError,'Certification/strict meshes'):phase15.derive(plate,None)
    def test_nonportable_paths_rejected(self):
        for p in ['/tmp/input.json','../input.json','target/../input.json','target\\input.json']:
            with self.subTest(p=p),self.assertRaisesRegex(ValueError,'Nonportable'):phase15.relative(ROOT,p)
if __name__=='__main__':unittest.main(verbosity=2)
