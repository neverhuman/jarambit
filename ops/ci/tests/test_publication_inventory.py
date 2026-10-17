"""Neutral packaging fixtures; no physical models, solvers, or design data."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from ops.ci.publication_inventory import Snapshot, inventory


class PublicationInventory(unittest.TestCase):
    def setUp(self):
        folder = Path(__file__).resolve().parents[3]/'target/paper/test-publication-inventory'
        folder.mkdir(parents=True, exist_ok=True)
        temporary = tempfile.TemporaryDirectory(dir=folder)
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.git('init', '-q')
        self.git('config', 'user.email', 'neutral@example.invalid')
        self.git('config', 'user.name', 'Neutral Fixture')
        (self.root/'docs').mkdir()
        (self.root/'docs/report.md').write_text('# Report\n\n![Example](example.png)\n')
        (self.root/'docs/example.png').write_bytes(b'neutral image bytes')
        (self.root/'source.txt').write_bytes(b'neutral source bytes')
        (self.root/'renderer.py').write_text('# neutral fixture, never executed\n')
        self.manifest = {
            'figure': 'docs/example.png',
            'figure_sha256': self.digest(b'neutral image bytes'),
            'script': 'renderer.py',
            'inputs': {'source.txt': self.digest(b'neutral source bytes')}}
        self.write_manifest()
        self.commit()

    def git(self, *args):
        return subprocess.check_output(['git', *args], cwd=self.root, stderr=subprocess.PIPE).decode().strip()

    def commit(self):
        self.git('add', '.')
        self.git('commit', '-qm', 'Neutral packaging fixture')

    @staticmethod
    def digest(raw):
        return hashlib.sha256(raw).hexdigest()

    def write_manifest(self):
        (self.root/'docs/example.png.manifest.json').write_text(json.dumps(self.manifest))

    def audit(self, ref='HEAD'):
        return inventory(self.root, ref, ['docs/report.md'])

    def test_snapshot_is_unchanged_by_dirty_runtime_files(self):
        expected = self.audit()
        for path in ('docs/report.md', 'docs/example.png', 'source.txt', 'renderer.py',
                     'docs/example.png.manifest.json'):
            (self.root/path).write_text('uncommitted replacement')
        self.assertEqual(self.audit(), expected)
        record = expected['documents']['docs/report.md']
        self.assertEqual(record['unique_local_images'], 1)
        self.assertEqual(record['figures']['docs/example.png']['input_count'], 1)

    def test_image_existing_only_in_worktree_is_rejected(self):
        (self.root/'docs/report.md').write_text('![Runtime](runtime.png)\n')
        self.git('add', 'docs/report.md')
        self.git('commit', '-qm', 'Reference an absent asset')
        (self.root/'docs/runtime.png').write_bytes(b'uncommitted')
        with self.assertRaisesRegex(ValueError, 'Not committed.*runtime.png'):
            self.audit()

    def test_changed_committed_source_rejects_stale_manifest(self):
        (self.root/'source.txt').write_bytes(b'changed source')
        self.commit()
        with self.assertRaisesRegex(ValueError, 'Input hash mismatch'):
            self.audit()

    def test_changed_committed_image_rejects_stale_manifest(self):
        (self.root/'docs/example.png').write_bytes(b'changed image')
        self.commit()
        with self.assertRaisesRegex(ValueError, 'Figure identity/hash mismatch'):
            self.audit()

    def test_moving_ref_cannot_rebind_inputs_during_inventory(self):
        first = self.git('rev-parse', 'HEAD')
        expected = self.audit(first)
        (self.root/'source.txt').write_bytes(b'new committed source')
        self.manifest['inputs']['source.txt'] = self.digest(b'new committed source')
        self.write_manifest()
        self.commit()
        second = self.git('rev-parse', 'HEAD')
        self.git('update-ref', 'refs/heads/audit-input', first)
        read = Snapshot.read

        def move_ref_then_read(snapshot, path):
            self.git('update-ref', 'refs/heads/audit-input', second)
            return read(snapshot, path)

        with patch.object(Snapshot, 'read', new=move_ref_then_read):
            earlier = self.audit('refs/heads/audit-input')
        self.assertEqual(self.git('rev-parse', 'audit-input'), second)
        self.assertEqual(earlier, expected)
        self.assertNotEqual(earlier, self.audit())

    def test_code_examples_are_ignored_and_remote_images_are_not_fetched(self):
        with (self.root/'docs/report.md').open('a') as stream:
            stream.write('\n```markdown\n## Example only\n![Missing](missing.png)\n```\n'
                         '\n## Real heading\n![Remote](https://example.invalid/image.png)\n'
                         '![Repeated](example.png)\n')
        self.commit()
        record = self.audit()['documents']['docs/report.md']
        self.assertEqual(record['heading_counts']['2'], 1)
        self.assertEqual(record['remote_image_references_unchecked'], 1)
        self.assertEqual(record['local_image_references'], 2)
        self.assertEqual(record['unique_local_images'], 1)

    def test_manifest_input_cannot_escape_repository(self):
        self.manifest['inputs'] = {'../outside.txt': 'a'*64}
        self.write_manifest()
        self.commit()
        with self.assertRaisesRegex(ValueError, 'escapes repository'):
            self.audit()

    def test_missing_manifest_is_disclosed_in_coverage_report(self):
        (self.root/'docs/other.png').write_bytes(b'neutral unmanifested asset')
        with (self.root/'docs/report.md').open('a') as stream:
            stream.write('![Other](other.png)\n')
        self.commit()
        record = self.audit()['documents']['docs/report.md']
        self.assertEqual(record['unique_local_images'], 2)
        self.assertEqual(record['images_without_manifest'], ['docs/other.png'])
        self.assertIsNone(record['figures']['docs/other.png']['manifest'])


if __name__ == '__main__':
    unittest.main()
