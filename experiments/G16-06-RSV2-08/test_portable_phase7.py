"""Portability and integrity regressions without deleting or renaming saved evidence."""
import io
from contextlib import redirect_stdout
import json
from pathlib import Path
import runpy
import sys
import unittest
from unittest.mock import patch
import audit_phase7 as phase7
import portable_study_audit as portable

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
REAL_STAT=Path.stat

def absent_target(path,*args,**kwargs):
    if path.is_relative_to(ROOT/'target'):raise FileNotFoundError(str(path))
    return REAL_STAT(path,*args,**kwargs)

class PortablePhase7Tests(unittest.TestCase):
    def test_default_command(self):
        output=io.StringIO()
        with redirect_stdout(output):runpy.run_path(str(HERE/'audit_phase7.py'),run_name='__main__')
        result=json.loads(output.getvalue())
        self.assertEqual((result['cases'],result['mesh_observations']),(144,720))
        self.assertEqual(result['status'],'pass')

    def test_default_command_without_any_target(self):
        output=io.StringIO()
        with patch.object(Path,'stat',new=absent_target),redirect_stdout(output):
            runpy.run_path(str(HERE/'audit_phase7.py'),run_name='__main__')
        result=json.loads(output.getvalue())
        self.assertEqual((result['cases'],result['mesh_observations']),(144,720))
        self.assertEqual(result['raw_evidence']['checked'],0)
        self.assertEqual(result['raw_evidence']['native_replay'],'not_run')
        self.assertEqual(result['raw_evidence']['split_hash_groups'],0)
        self.assertGreater(result['raw_evidence']['unavailable'],0)

    def test_audit_does_not_require_worker_cpu_mask(self):
        with patch.object(Path,'stat',new=absent_target),patch('os.sched_getaffinity',return_value={0},create=True):
            self.assertEqual(phase7.audit()['status'],'pass')

    def test_phase8_command_without_target(self):
        if not (HERE/'phase8_receipt.json').is_file():self.skipTest('Phase8 receipt is not present')
        import audit_phase8
        with patch.object(Path,'stat',new=absent_target):result=audit_phase8.audit()
        self.assertEqual((result['cases'],result['mesh_observations']),(12,60))
        self.assertEqual(result['raw_evidence']['checked'],0)

    def document(self):return json.loads((HERE/'phase7_receipt.json').read_text())

    def test_missing_observation_rejected(self):
        r=self.document();r['results'][0]['levels'].pop()
        with self.assertRaisesRegex(ValueError,'mesh observation'):
            phase7.audit_document(ROOT,r,(HERE/'PHASE7.md').read_text())

    def test_threshold_change_rejected(self):
        r=self.document();r['strict_sequence_floor']=1.20
        with self.assertRaisesRegex(ValueError,'thresholds changed'):
            phase7.audit_document(ROOT,r,(HERE/'PHASE7.md').read_text())

    def test_false_pass_rejected(self):
        r=self.document();r['results'][0]['fine_targets_pass']=True
        with self.assertRaisesRegex(ValueError,'pass flag mismatch'):
            phase7.audit_document(ROOT,r,(HERE/'PHASE7.md').read_text())

    def test_report_observation_change_rejected(self):
        r=self.document();text=(HERE/'PHASE7.md').read_text()
        row=r['results'][0];area=row['levels'][0]['area']
        text=text.replace(f'| {row["name"]} | {area} |',f'| CHANGED | {area} |',1)
        with self.assertRaisesRegex(ValueError,'Report mesh table mismatch'):
            phase7.audit_document(ROOT,r,text)

    def test_present_raw_corruption_is_not_treated_as_missing(self):
        r=self.document();rel,want=next((p,h) for p,h in r['sha256'].items() if p.startswith('target/'))
        with patch.object(portable,'raw_path',return_value=ROOT/rel),patch.object(portable,'digest',return_value='0'*64):
            with self.assertRaisesRegex(ValueError,'SHA-256 mismatch'):portable.check_raw(ROOT,r)

    def test_missing_tracked_genome_never_optional(self):
        genome=ROOT/self.document()['results'][0]['genome']
        def missing(path,*args,**kwargs):
            if path==genome:raise FileNotFoundError(str(path))
            return REAL_STAT(path,*args,**kwargs)
        with patch.object(Path,'stat',new=missing),self.assertRaisesRegex(ValueError,'Missing tracked genome'):
            phase7.audit()

if __name__=='__main__':unittest.main(verbosity=2)
