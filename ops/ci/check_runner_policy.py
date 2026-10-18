#!/usr/bin/env python3
"""Reject workflow jobs that can leave the local xbabe general runner pool."""
import argparse
from pathlib import Path
import re
import sys

import yaml


REQUIRED_LABELS = frozenset({'self-hosted', 'xbabe', 'general'})


class WorkflowLoader(yaml.BaseLoader):
    """Read YAML scalars literally and reject duplicate job/routing keys."""

    def construct_mapping(self, node, deep=False):
        result = {}
        for key_node, value_node in node.value:
            key = self.construct_object(key_node, deep=deep)
            if not isinstance(key, str) or key in result:
                raise ValueError(f'duplicate or invalid YAML key: {key!r}')
            result[key] = self.construct_object(value_node, deep=deep)
        return result


def check_workflow(document, root):
    errors = []
    jobs = document.get('jobs') if isinstance(document, dict) else None
    if not isinstance(jobs, dict) or not jobs:
        return ['workflow must define a nonempty jobs mapping']
    for name, job in jobs.items():
        if not isinstance(job, dict):
            errors.append(f'{name}: job must be a mapping')
            continue
        if 'uses' in job:
            target = job['uses']
            if (not isinstance(target, str)
                    or not re.fullmatch(r'\./\.github/workflows/[A-Za-z0-9_.-]+\.ya?ml', target)
                    or not (root / target).is_file()):
                errors.append(f'{name}: reusable workflows must exist locally so their runners can be audited')
            if 'runs-on' in job:
                errors.append(f'{name}: reusable workflow calls cannot define runs-on')
            continue
        labels = job.get('runs-on')
        if (not isinstance(labels, list) or not labels
                or any(not isinstance(label, str) or not label.strip() for label in labels)):
            errors.append(f'{name}: runs-on must be an explicit label list containing self-hosted, xbabe, general')
            continue
        missing = REQUIRED_LABELS - {label.casefold() for label in labels}
        if missing:
            errors.append(f'{name}: missing local runner labels: {", ".join(sorted(missing))}')
    return errors


def check_repository(root):
    directory = root / '.github/workflows'
    workflows = sorted([*directory.glob('*.yml'), *directory.glob('*.yaml')])
    if not workflows:
        return ['no GitHub Actions workflows found']
    errors = []
    for path in workflows:
        try:
            document = yaml.load(path.read_text(), Loader=WorkflowLoader)
            findings = check_workflow(document, root)
        except (OSError, ValueError, yaml.YAMLError) as error:
            findings = [str(error)]
        errors.extend(f'{path.relative_to(root)}: {finding}' for finding in findings)
    return errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    errors = check_repository(args.root)
    if errors:
        for error in errors:
            print(f'::error::{error}', file=sys.stderr)
        return 1
    print('Runner policy passed: every workflow job requires self-hosted + xbabe + general.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
