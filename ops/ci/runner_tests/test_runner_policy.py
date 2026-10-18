"""Routing regressions: no hosted, dynamic or external-workflow escape."""
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from check_runner_policy import check_repository


class RunnerPolicy(unittest.TestCase):
    def check(self, content, extra=None):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory = root / '.github/workflows'
            directory.mkdir(parents=True)
            (directory / 'ci.yml').write_text(content)
            if extra:
                (directory / 'reusable.yaml').write_text(extra)
            return check_repository(root)

    def test_general_pool_with_capability_and_case_insensitive_labels(self):
        self.assertEqual([], self.check('jobs:\n  test:\n    runs-on: [self-hosted, XBabe, GENERAL, linux, jarambit-d1]\n'))

    def test_matrix_cannot_remove_required_labels(self):
        self.assertEqual([], self.check("jobs:\n  test:\n    runs-on: [self-hosted, xbabe, general, '${{ matrix.host }}']\n"))

    def test_github_hosted_is_rejected(self):
        self.assertTrue(self.check('jobs:\n  test:\n    runs-on: ubuntu-latest\n'))

    def test_self_hosted_outside_general_pool_is_rejected(self):
        for labels in ('self-hosted, xbabe', 'self-hosted, general', 'xbabe, general'):
            with self.subTest(labels=labels):
                self.assertTrue(self.check(f'jobs:\n  test:\n    runs-on: [{labels}]\n'))

    def test_dynamic_runs_on_and_runner_group_mapping_are_rejected(self):
        for value in ("'${{ matrix.runner }}'", '{group: general}'):
            with self.subTest(value=value):
                self.assertTrue(self.check(f'jobs:\n  test:\n    runs-on: {value}\n'))

    def test_remote_reusable_workflow_is_rejected(self):
        self.assertTrue(self.check('jobs:\n  test:\n    uses: other/repo/.github/workflows/ci.yml@main\n'))

    def test_local_reusable_workflow_is_also_audited(self):
        caller = 'jobs:\n  test:\n    uses: ./.github/workflows/reusable.yaml\n'
        self.assertEqual([], self.check(caller, 'jobs:\n  test:\n    runs-on: [self-hosted, xbabe, general]\n'))
        self.assertTrue(self.check(caller, 'jobs:\n  test:\n    runs-on: ubuntu-latest\n'))
        self.assertTrue(self.check(caller))

    def test_duplicate_routing_or_job_keys_are_rejected(self):
        self.assertTrue(self.check('jobs:\n  test:\n    runs-on: ubuntu-latest\n    runs-on: [self-hosted, xbabe, general]\n'))
        self.assertTrue(self.check('jobs:\n  test: {runs-on: ubuntu-latest}\n  test: {runs-on: [self-hosted, xbabe, general]}\n'))

    def test_empty_invalid_and_yaml_suffix_workflows_are_checked(self):
        for content in ('', 'jobs: {}', 'jobs: [test]', 'jobs: {test: []}', 'jobs: {'):
            with self.subTest(content=content):
                self.assertTrue(self.check(content))
        self.assertTrue(self.check('jobs:\n  test:\n    runs-on: [self-hosted, xbabe, general]\n',
                                   'jobs:\n  test:\n    runs-on: windows-latest\n'))


if __name__ == '__main__':
    unittest.main()
