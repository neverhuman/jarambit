"""Adverse software controls; synthetic records are never physical certification evidence."""
import copy
import math
import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch
from ops.ci.verify_genome_export import compare_bore
from ops.ci.evaluate_cad_candidate import run_independent_cross_check, independent_agreement, unmeasured_solid_record, solid_evidence_matches

class BoreComparison(unittest.TestCase):
    def setUp(self):
        self.profiles=[dict(theta=i*math.tau/8,side=s,major_radius_mm=14.,minor_radius_mm=1.6,center_z_mm=s*6.)
                       for s in [1,-1] for i in range(8)]
        self.faces=[dict(center_xy_mm=[0.,0.],axis_z=1.,angle_start=0.,angle_width=math.tau,
                         major_radius_mm=14.,minor_radius_mm=1.6,center_z_mm=s*6.) for s in [1,-1]]
    def test_matching_independent_geometry(self):
        self.assertTrue(compare_bore(self.faces,self.profiles)['bore_geometry_matches_mesh'])
    def test_major_radius_center_height_and_axis_misplacement(self):
        for key,value in [('major_radius_mm',15.),('center_z_mm',7.),('center_xy_mm',[.2,0.]),('axis_z',.9)]:
            faces=copy.deepcopy(self.faces);faces[0][key]=value
            self.assertFalse(compare_bore(faces,self.profiles)['bore_geometry_matches_mesh'],key)
    def test_one_face_missing_sector_and_local_small_roll_fail_closed(self):
        self.assertFalse(compare_bore(self.faces[:1],self.profiles)['bore_geometry_matches_mesh'])
        faces=copy.deepcopy(self.faces);faces[0]['angle_width']=math.tau-.01
        self.assertFalse(compare_bore(faces,self.profiles)['bore_geometry_matches_mesh'])
        profiles=copy.deepcopy(self.profiles);profiles[3]['minor_radius_mm']=1.3
        self.assertFalse(compare_bore(self.faces,profiles)['bore_geometry_matches_mesh'])

class IndependentFailures(unittest.TestCase):
    def test_docker_failure_is_unverified_and_inputs_are_read_only(self):
        runtime={'versions':{'site_packages':'/host/site'},'docker_image':'image@sha256:synthetic'}
        Path('target/pr84-docker-control').mkdir(parents=True,exist_ok=True)
        with patch('ops.ci.evaluate_cad_candidate.subprocess.check_output',side_effect=subprocess.CalledProcessError(125,['docker'])) as call:
            with self.assertRaisesRegex(RuntimeError,'independent Docker solve unverified'):
                run_independent_cross_check(Path('mesh.npz'),Path('target/pr84-docker-control'),'synthetic',runtime)
        args=call.call_args.args[0]
        self.assertIn('target/pr84-docker-control:/work:ro',args)
        self.assertNotIn('/ext_sp',str(args))
        self.assertIn('/host/site/scipy:/deps/scipy:ro',args)
    def test_disagreement_above_two_percent_is_a_failure(self):
        doc={'stored_solid':{'SF_skfem_nodal_max':1.2,'SF_skfem_nodal_avg':1.3},
             'dolfinx_cross_check':{'SF_nodal_max':1.,'SF_nodal_avg':1.}}
        self.assertFalse(independent_agreement(doc)['within_2pct'])
    def test_unmeasured_solid_tip_scores_zero_and_is_never_reusable_evidence(self):
        doc=unmeasured_solid_record('synthetic','a'*64,'primary solid solve unverified','b'*64,'c'*64)
        self.assertEqual(doc['stored_solid']['SF_nodal_max'],0.0)
        self.assertFalse(doc['dolfinx_cross_check']['within_2pct'])
        self.assertFalse(doc['entered_as_solver_run'])
        self.assertFalse(any(k in doc['stored_solid'] for k in ('n_tet','n_dof','ksp_its')))
        self.assertFalse(solid_evidence_matches(doc,'b'*64,'c'*64,'a'*64))
