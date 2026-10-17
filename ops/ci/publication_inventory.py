#!/usr/bin/env python3
"""Inventory inline Markdown images and their provenance at one Git commit.

This checks document packaging, not scientific claims or image quality. It never
opens the working-tree copies of the documents, images, or manifest inputs.
Missing manifest sidecars are listed; existing sidecars must validate.
"""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import posixpath
import re
import subprocess
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[2]
IMAGE = re.compile(r'!\[[^\]]*\]\((<[^>]+>|[^\s)]+)(?:\s+"[^"]*")?\)')


class NotCommitted(ValueError):
    pass


class Snapshot:
    def __init__(self, root, revision):
        self.root = root
        self.commit = subprocess.check_output(
            ['git', 'rev-parse', '--verify', '--end-of-options', revision+'^{commit}'],
            cwd=root, text=True).strip()
        self.blobs = {}

    def read(self, path):
        if path not in self.blobs:
            result = subprocess.run(['git', 'cat-file', 'blob', self.commit+':'+path],
                                    cwd=self.root, capture_output=True)
            if result.returncode:
                raise NotCommitted(f'Not committed at {self.commit}: {path}')
            self.blobs[path] = result.stdout
        return self.blobs[path]


def sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def repository_path(path, parent=''):
    if not isinstance(path, str) or not path or path.startswith('/') or '\\' in path:
        raise ValueError(f'Expected repository-relative path: {path}')
    normalized = posixpath.normpath(posixpath.join(parent, path))
    if normalized == '..' or normalized.startswith('../') or ':' in normalized:
        raise ValueError(f'Path escapes repository or names a revision: {path}')
    return normalized


def prose(markdown):
    """Omit fenced code examples; inventory only inline image syntax."""
    fence = None
    for line in markdown.splitlines():
        marker = re.match(r'^ {0,3}(`{3,}|~{3,})(.*)$', line)
        if marker:
            run, rest = marker.groups()
            if fence is None:
                fence = run
            elif run[0] == fence[0] and len(run) >= len(fence) and not rest.strip():
                fence = None
            continue
        if fence is None:
            yield line


def inventory(root, revision, documents):
    snapshot = Snapshot(root, revision)
    report = {'schema': 1, 'commit': snapshot.commit,
              'scope': 'Inline Markdown image packaging; no scientific or visual-quality assessment',
              'documents': {}}
    for document in documents:
        document = repository_path(document)
        raw = snapshot.read(document)
        text = '\n'.join(prose(raw.decode('utf-8')))
        headings = Counter(len(m.group(1)) for m in re.finditer(
            r'^ {0,3}(#{1,6})\s+\S', text, re.MULTILINE))
        figures, remote, references = {}, 0, 0
        for match in IMAGE.finditer(text):
            target = match.group(1).removeprefix('<').removesuffix('>')
            link = urlsplit(target)
            if link.scheme or link.netloc:
                remote += 1
                continue
            image = repository_path(unquote(link.path), posixpath.dirname(document))
            references += 1
            if image in figures:
                continue
            image_sha = sha256(snapshot.read(image))
            manifest_path = image+'.manifest.json'
            try:
                manifest_raw = snapshot.read(manifest_path)
            except NotCommitted:
                figures[image] = {'sha256': image_sha, 'manifest': None}
                continue
            manifest = json.loads(manifest_raw)
            if not isinstance(manifest, dict):
                raise ValueError(f'Expected manifest object: {manifest_path}')
            if manifest.get('figure') != image or manifest.get('figure_sha256') != image_sha:
                raise ValueError(f'Figure identity/hash mismatch: {manifest_path}')
            inputs = manifest.get('inputs')
            if not isinstance(inputs, dict) or not inputs:
                raise ValueError(f'Missing input ledger: {manifest_path}')
            for source, digest in inputs.items():
                source = repository_path(source)
                if sha256(snapshot.read(source)) != digest:
                    raise ValueError(f'Input hash mismatch: {manifest_path}: {source}')
            script = repository_path(manifest['script'])
            figures[image] = {'sha256': image_sha, 'manifest': manifest_path,
                              'manifest_sha256': sha256(manifest_raw),
                              'script': script, 'script_sha256': sha256(snapshot.read(script)),
                              'input_count': len(inputs)}
        report['documents'][document] = {
            'sha256': sha256(raw),
            'heading_counts': {str(level): headings[level] for level in range(1, 7)},
            'local_image_references': references,
            'unique_local_images': len(figures),
            'images_without_manifest': sorted(path for path, record in figures.items()
                                              if record['manifest'] is None),
            'remote_image_references_unchecked': remote,
            'figures': figures}
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--ref', default='HEAD', help='Commit or ref, resolved once')
    parser.add_argument('documents', nargs='+', help='Repository-relative Markdown files')
    args = parser.parse_args()
    try:
        report = inventory(ROOT, args.ref, args.documents)
    except (ValueError, KeyError, TypeError, subprocess.CalledProcessError) as error:
        parser.exit(1, f'publication_inventory: {error}\n')
    print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == '__main__':
    main()
