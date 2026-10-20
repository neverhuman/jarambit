"""Publication admission and provenance controls; no new physical observations."""
import copy
import hashlib
import json
from pathlib import Path
import statistics
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from ops.render.blender.rerender import prepare
from ops.render.paper import wave_history
from ops.render.paper.wave_history import build_insertion, collect, generate, read_board

ROOT = Path(__file__).resolve().parents[3]
BOARD = ROOT/'evidence/evolve_certifiable/automatic/leaderboard.json'
LABEL = 'cert_pipeline_r01_p14_c02'
HISTORY = ROOT/'evidence/evolution_plateau/2026-10-17/xbabe2/out/w298/leaderboard.json'


def at(document, pointer):
    for part in pointer.split('/')[1:]:
        document = document[int(part)] if isinstance(document, list) else document[part]
    return document


class WavePublicationPrep(unittest.TestCase):
    def setUp(self):
        folder = ROOT/'target/paper/test-wave-publication';folder.mkdir(parents=True,exist_ok=True)
        self.tmp = tempfile.TemporaryDirectory(dir=folder)
        self.addCleanup(self.tmp.cleanup)
        self.folder = Path(self.tmp.name)

    def committed_board_fixture(self, document=None, relative='out/control/leaderboard.json'):
        # A tiny Git fixture, not a copy of the project. It retains the real
        # committed observations and needs no cached/fetched remote ref in CI.
        path = self.folder/relative
        path.parent.mkdir(parents=True)
        path.write_bytes(HISTORY.read_bytes() if document is None else
                         (json.dumps(document)+'\n').encode())
        for args in [('init', '-q'), ('add', relative),
                     ('-c','user.name=Publication control','-c','user.email=control@example.invalid',
                      '-c','core.hooksPath=/dev/null','commit','-q','-m','Committed observation fixture'),
                     ('update-ref','refs/remotes/origin/main','HEAD')]:
            subprocess.run(['git',*args],cwd=self.folder,check=True,capture_output=True)
        return path

    def neutral_metadata(self):
        # Synthetic software control only; never publication/physical evidence.
        row = dict(label='neutral-record', mass_g=1.0, w_cad=0.0, g1=0.0,
                   all_16_gates=False, rs_v2_all_hard=False, qualified=False)
        return dict(schema='jarambit.auto_evolve.leaderboard.v2', designs=[row],
                    assessed_count=1, qualified_count=0, champion=None, unverified=[])

    def test_history_rejects_non_boolean_verdicts_and_non_integer_counts(self):
        data = self.neutral_metadata()
        path = self.committed_board_fixture(data)
        self.assertEqual(read_board(path, self.folder)[0], data)
        positive = copy.deepcopy(data)
        positive['designs'][0].update(all_16_gates=True, rs_v2_all_hard=True, qualified=True)
        positive.update(qualified_count=1, champion=positive['designs'][0])
        path.write_text(json.dumps(positive))
        self.assertEqual(read_board(path, self.folder, preview=True)[0], positive)
        for flag in ('all_16_gates', 'rs_v2_all_hard', 'qualified'):
            for value in ('false', 'true', 0, 1, None, [], {}):
                with self.subTest(flag=flag, value=value):
                    invalid = copy.deepcopy(data)
                    invalid['designs'][0].update(all_16_gates=True,
                        rs_v2_all_hard=True, qualified=True)
                    invalid['designs'][0][flag] = value
                    invalid['qualified_count'] = 1
                    invalid['champion'] = invalid['designs'][0]
                    path.write_text(json.dumps(invalid))
                    with self.assertRaisesRegex(ValueError, 'boolean'):
                        read_board(path, self.folder, preview=True)
        for count in ('assessed_count', 'qualified_count'):
            for value in (True, False, 0.0, 1.0, '0', '1', None, -1):
                with self.subTest(count=count, value=value):
                    invalid = copy.deepcopy(data);invalid[count] = value
                    path.write_text(json.dumps(invalid))
                    with self.assertRaisesRegex(ValueError, 'integer'):
                        read_board(path, self.folder, preview=True)

    def test_history_rejects_source_substitution_after_admission(self):
        original = self.neutral_metadata()
        path = self.committed_board_fixture(original)
        changed = copy.deepcopy(original);changed['designs'][0]['mass_g'] = 2.0
        read = wave_history.read_board

        def read_then_change(*args, **kwargs):
            result = read(*args, **kwargs)
            path.write_text(json.dumps(changed)+'\n')
            return result

        output = self.folder/'figures'
        with patch.object(wave_history, 'ROOT', self.folder), \
             patch.dict('sys.modules', {'matplotlib':None, 'cycler':None}), \
             patch.object(wave_history, 'read_board', side_effect=read_then_change):
            with self.assertRaisesRegex(ValueError, 'Source changed'):
                generate([path], self.folder, output)
        self.assertFalse(output.exists())
        path.write_text(json.dumps(original)+'\n')
        with patch.object(wave_history, 'ROOT', self.folder):
            data = collect([path], self.folder)
        self.assertEqual(data['source_commit'], subprocess.check_output(
            ['git', 'rev-parse', 'origin/main'], cwd=self.folder, text=True).strip())
        # Pure publication boundary: no plotting dependency or physical fixture.
        output.mkdir()
        figure = output/'neutral.png';figure.write_bytes(b'neutral software control')
        ledger = output/'points.json';ledger.write_text(json.dumps(data))
        snippet = output/'insertion.md';snippet.write_text(build_insertion(data['waves']))
        manifest = wave_history.write_history_manifest(
            figure, ledger, snippet, data, self.folder, [path])
        self.assertEqual(manifest['inputs'][wave_history.figlib.rel(path)],
                         data['waves'][0]['sha256'])
        late = output/'changed.png';late.write_bytes(figure.read_bytes())
        path.write_text(json.dumps(changed)+'\n')
        with self.assertRaisesRegex(ValueError, 'Source changed'):
            wave_history.write_history_manifest(late, ledger, snippet, data, self.folder, [path])
        self.assertFalse(Path(str(late)+'.manifest.json').exists())

    def test_main_root_wave_paths_require_exact_main_bytes(self):
        # Neutral publication metadata only, not physical/evolution evidence.
        original = self.neutral_metadata()
        relative = 'evidence/evolution_waves/2026-10-17/control/leaderboard.json'
        path = self.committed_board_fixture(original, relative)
        base = subprocess.check_output(['git', 'rev-parse', 'origin/main'],
                                       cwd=self.folder, text=True).strip()
        with patch.object(wave_history, 'ROOT', self.folder):
            self.assertEqual(collect([path], self.folder)['source_commit'], base)
            changed = copy.deepcopy(original)
            changed['designs'][0]['mass_g'] = 2.0
            path.write_text(json.dumps(changed)+'\n')
            with self.assertRaisesRegex(ValueError, 'origin/main'):
                read_board(path, self.folder)
            with self.assertRaisesRegex(ValueError, 'origin/main'):
                collect([path], self.folder)
            self.assertEqual(read_board(path, self.folder, preview=True)[0], changed)

            untracked = path.parent.parent/'untracked/leaderboard.json'
            untracked.parent.mkdir()
            untracked.write_text(json.dumps(original)+'\n')
            with self.assertRaisesRegex(ValueError, 'origin/main'):
                read_board(untracked, self.folder)
            with self.assertRaisesRegex(ValueError, 'origin/main'):
                collect([untracked], self.folder)
            self.assertEqual(read_board(untracked, self.folder, preview=True)[0], original)

            # A local-only commit still cannot impersonate pinned origin/main.
            subprocess.run(['git', 'add', relative], cwd=self.folder, check=True,
                           capture_output=True)
            subprocess.run(['git', '-c', 'user.name=Publication control', '-c',
                'user.email=control@example.invalid', '-c', 'core.hooksPath=/dev/null',
                'commit', '-q', '-m', 'Local-only neutral change'],
                cwd=self.folder, check=True, capture_output=True)
            with self.assertRaisesRegex(ValueError, 'origin/main'):
                read_board(path, self.folder)
            with self.assertRaisesRegex(ValueError, 'origin/main'):
                collect([path], self.folder)
        self.assertEqual(subprocess.check_output(['git', 'rev-parse', 'origin/main'],
            cwd=self.folder, text=True).strip(), base)

    def test_mixed_committed_wave_inputs_keep_a_truthful_snapshot(self):
        # The older dated-folder commit does not contain the later input.
        first = self.committed_board_fixture(self.neutral_metadata(),
            'evidence/evolution_waves/2026-10-17/control-old/leaderboard.json')
        second_rel = 'evidence/evolution_waves/2026-10-18/control-new/leaderboard.json'
        second = self.folder/second_rel
        second.parent.mkdir(parents=True)
        later = self.neutral_metadata()
        later['designs'][0]['label'] = 'neutral-later'
        later['designs'][0]['mass_g'] = 2.0
        second.write_text(json.dumps(later)+'\n')
        for args in [('add', second_rel),
                     ('-c', 'user.name=Publication control', '-c',
                      'user.email=control@example.invalid', '-c',
                      'core.hooksPath=/dev/null', 'commit', '-q', '-m',
                      'Later committed neutral input'),
                     ('update-ref', 'refs/remotes/origin/main', 'HEAD')]:
            subprocess.run(['git', *args], cwd=self.folder, check=True,
                           capture_output=True)
        base = subprocess.check_output(['git', 'rev-parse', 'origin/main'],
                                       cwd=self.folder, text=True).strip()
        with patch.object(wave_history, 'ROOT', self.folder):
            ledger = collect([first, second], self.folder)
        self.assertEqual(ledger['source_commit'], base)
        for row in ledger['waves']:
            committed = subprocess.check_output(
                ['git', 'show', ledger['source_commit']+':'+row['source']],
                cwd=self.folder)
            self.assertEqual(hashlib.sha256(committed).hexdigest(), row['sha256'])

    def test_failed_assessment_cannot_enter_final_render_mode(self):
        with self.assertRaisesRegex(ValueError, 'NOT-A-WINNER'):
            prepare(LABEL, BOARD, ROOT)
        context = prepare(LABEL, BOARD, ROOT, preview=True)
        self.assertIn('NOT-A-WINNER', context['status'])
        self.assertFalse(context['all_16_gates'])
        self.assertFalse(context['rs_v2_all_hard'])
        for source, sha in context['input_sha256'].items():
            self.assertEqual(hashlib.sha256(Path(source).read_bytes()).hexdigest(), sha)

    def test_uncommitted_history_requires_preview_and_contradictions_fail(self):
        path = self.committed_board_fixture()
        data = json.loads(path.read_text())
        self.assertEqual(read_board(path, self.folder)[0], data)
        # The changed bytes are only on the local branch; origin/main retains
        # the original observation. Publication must not trust HEAD instead.
        path.write_text(json.dumps(data)+'\n')
        subprocess.run(['git','add','.'],cwd=self.folder,check=True)
        subprocess.run(['git','-c','user.name=Publication control','-c','user.email=control@example.invalid',
            '-c','core.hooksPath=/dev/null','commit','-q','-m','Local formatting change'],cwd=self.folder,check=True)
        with self.assertRaisesRegex(ValueError, 'origin/main'):
            read_board(path, self.folder)
        self.assertEqual(read_board(path, self.folder, preview=True)[0], data)
        contradictory = copy.deepcopy(data);contradictory['designs'][0]['qualified'] = True
        path.write_text(json.dumps(contradictory))
        with self.assertRaisesRegex(ValueError, 'qualification flags'):
            read_board(path, self.folder, preview=True)
        unverified = copy.deepcopy(data);unverified['unverified'] = ['control: missing independent solve']
        path.write_text(json.dumps(unverified))
        with self.assertRaisesRegex(ValueError, 'unverified'):
            read_board(path, self.folder, preview=True)

    def test_every_summary_matches_original_board_pointers_without_plotting_packages(self):
        path = self.committed_board_fixture()
        with patch.object(wave_history, 'ROOT', self.folder), patch.dict('sys.modules', {'matplotlib':None, 'cycler':None}):
            ledger = collect([path], self.folder, preview=False)
            self.assertFalse(ledger['preview'])
            for point in ledger['points']:
                original = json.loads((self.folder/point['source']).read_text())
                if 'pointers' in point:
                    values = [at(original, pointer) for pointer in point['pointers']]
                    expected = {'min':min, 'median':statistics.median, 'max':max}[point['reduction']](values)
                else:expected = at(original, point['pointer'])
                self.assertEqual(point['value'], expected)
            snippet = build_insertion(ledger['waves'])
            self.assertIn('champion=null', snippet)
            self.assertIn('not full-population statistics', snippet)
            with self.assertRaisesRegex(ValueError, 'outside the paper'):
                generate([path], self.folder, self.folder/'docs/preview-not-allowed', preview=True)

    def test_render_rejects_a_relabelled_certificate_identity(self):
        path = self.folder/'leaderboard.json'
        data = json.loads(BOARD.read_text())
        data['designs'][0]['certificate_identity'] = '0'*64
        path.write_text(json.dumps(data))
        with self.assertRaisesRegex(ValueError, 'identity mismatch'):
            prepare(LABEL, path, ROOT, preview=True)


if __name__ == '__main__':unittest.main()
