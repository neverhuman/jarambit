"""Neutral document controls: packaging provenance, no engineering assessment."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from ops.render.paper import build_pdf

ROOT = Path(__file__).resolve().parents[3]
NAME = 'ENGINEERING_GATES_AND_EVOLUTION'


class PdfProvenance(unittest.TestCase):
    def setUp(self):
        folder = ROOT/'target/paper/test-pdf-provenance'
        folder.mkdir(parents=True, exist_ok=True)
        self.tmp = tempfile.TemporaryDirectory(dir=folder)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.head = 'a'*40
        docs = self.root/'docs';docs.mkdir()
        self.source = docs/(NAME+'.md')
        self.source.write_text('# Neutral report\n\nA software packaging control.\n')
        (docs/'VERSION').write_text('1.0.0\n')
        self.here = self.root/'ops/render/paper';self.here.mkdir(parents=True)
        for name in ('build_pdf.py', 'publication.lua', 'header.tex'):
            (self.here/name).write_text('neutral software fixture\n')

    def run_build(self, mutation=None, staged_mutation=None):
        def capture(command, **kwargs):
            if command[:3] == ['git', 'rev-parse', 'HEAD']:return self.head+'\n'
            if command[:2] == ['git', 'show']:return '1792195200\n'
            if command[:2] == ['pandoc', '--version']:return build_pdf.PINNED_PANDOC_VERSION+'\n'
            if command[:2] == ['lualatex', '--version']:return build_pdf.PINNED_LUALATEX_VERSION+'\n'
            return 'neutral tool version\n'

        def render(command, **kwargs):
            # Deliberately no LaTeX/Blender/solver execution or physical data.
            if staged_mutation:staged_mutation(command)
            Path(command[command.index('-o')+1]).write_bytes(b'neutral PDF packaging control')
            if mutation:mutation()
            return subprocess.CompletedProcess(command, 0)

        with patch.object(build_pdf, 'ROOT', self.root), \
             patch.object(build_pdf, 'HERE', self.here), \
             patch.object(build_pdf, '__file__', str(self.here/'build_pdf.py')), \
             patch.dict('sys.modules', {'PIL':None, 'matplotlib':None}), \
             patch.object(build_pdf.subprocess, 'check_output', side_effect=capture), \
             patch.object(build_pdf.subprocess, 'run', side_effect=render):
            build_pdf.build(NAME)

    def test_unchanged_document_manifest_binds_consumed_bytes(self):
        image = self.root/'docs/neutral.png';image.write_bytes(b'neutral image software control')
        with self.source.open('a') as f:
            f.write('\n![Neutral placeholder](neutral.png)\n\n![Repeated placeholder](neutral.png)\n')
        original = self.source.read_bytes()
        self.run_build()
        manifest = json.loads((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).read_text())
        expected = hashlib.sha256(original).hexdigest()
        self.assertEqual(manifest['source_sha256'], expected)
        self.assertEqual(manifest['inputs']['docs/'+NAME+'.md'], expected)
        self.assertEqual(manifest['inputs']['docs/neutral.png'],
                         hashlib.sha256(image.read_bytes()).hexdigest())
        snapshots = list((self.root/'target/paper/images').glob('neutral-*.png'))
        self.assertEqual(len(snapshots), 1)
        self.assertEqual(snapshots[0].read_bytes(), image.read_bytes())
        command = manifest['command']
        actual = [Path(command[1]), snapshots[0],
                  Path(command[command.index('--lua-filter')+1])]
        actual += [Path(command[i+1]) for i, arg in enumerate(command) if arg=='--include-in-header']
        self.assertEqual(set(manifest['renderer_inputs']),
                         {str(path.relative_to(self.root)) for path in actual})
        for relative, digest in manifest['renderer_inputs'].items():
            self.assertEqual(hashlib.sha256((self.root/relative).read_bytes()).hexdigest(), digest)

    def test_source_substitution_during_render_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'changed'):
            self.run_build(lambda:self.source.write_text('# Different neutral report\n'))
        self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_render_support_substitution_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'changed'):
            self.run_build(lambda:(self.here/'header.tex').write_text('changed neutral support\n'))
        self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_image_substitution_is_rejected(self):
        image = self.root/'docs/neutral.png';image.write_bytes(b'neutral image software control')
        with self.source.open('a') as f:f.write('\n![Neutral placeholder](neutral.png)\n')
        with self.assertRaisesRegex(ValueError, 'changed'):
            self.run_build(lambda:image.write_bytes(b'changed neutral image'))
        self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_version_substitution_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'changed'):
            self.run_build(lambda:(self.root/'docs/VERSION').write_text('2.0.0\n'))
        self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_checkout_change_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'changed'):
            self.run_build(lambda:setattr(self, 'head', 'b'*40))
        self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_image_change_between_repeated_references_is_rejected(self):
        image = self.root/'docs/neutral.png';image.write_bytes(b'neutral image software control')
        with self.source.open('a') as f:
            f.write('\n![First placeholder](neutral.png)\n\n![Repeated placeholder](neutral.png)\n')
        capture = build_pdf.capture
        reads = 0

        def capture_then_change(path, inputs):
            nonlocal reads
            raw = capture(path, inputs)
            if path == image:
                reads += 1
                if reads == 1:image.write_bytes(b'changed neutral image')
            return raw

        with patch.object(build_pdf, 'capture', side_effect=capture_then_change):
            with self.assertRaisesRegex(ValueError, 'changed'):
                self.run_build()
        self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_all_actual_renderer_input_substitutions_are_rejected(self):
        image = self.root/'docs/neutral.png';image.write_bytes(b'neutral image software control')
        with self.source.open('a') as f:f.write('\n![Neutral placeholder](neutral.png)\n')
        for mode in ('markdown', 'image', 'header', 'version', 'lua'):
            with self.subTest(mode=mode):
                def substitute(command):
                    markdown = Path(command[command.index('pandoc')+1])
                    images = [Path(p) for p in re.findall(r'!\[[^\]]*\]\(([^)]+)\)', markdown.read_text())]
                    headers = [Path(command[i+1]) for i, arg in enumerate(command) if arg=='--include-in-header']
                    target = {'markdown':markdown, 'image':images[0], 'header':headers[1],
                              'version':headers[0], 'lua':Path(command[command.index('--lua-filter')+1])}[mode]
                    target.write_bytes(b'substituted neutral renderer input')
                with self.assertRaisesRegex(ValueError, 'Renderer input changed'):
                    self.run_build(staged_mutation=substitute)
                self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_unpinned_toolchain_is_rejected(self):
        for bad_tool, bad_ver in (('pandoc', 'pandoc 3.1.2\n'), ('lualatex', 'This is LuaHBTeX, Version 1.16.0\n')):
            with self.subTest(bad_tool=bad_tool):
                def capture_bad(command, **kwargs):
                    if command[:3] == ['git', 'rev-parse', 'HEAD']:return self.head+'\n'
                    if command[:2] == ['git', 'show']:return '1792195200\n'
                    if command[:2] == [bad_tool, '--version']:return bad_ver
                    if command[:2] == ['pandoc', '--version']:return build_pdf.PINNED_PANDOC_VERSION+'\n'
                    if command[:2] == ['lualatex', '--version']:return build_pdf.PINNED_LUALATEX_VERSION+'\n'
                    return 'neutral tool version\n'
                with patch.object(build_pdf, 'ROOT', self.root), \
                     patch.object(build_pdf, 'HERE', self.here), \
                     patch.object(build_pdf, '__file__', str(self.here/'build_pdf.py')), \
                     patch.object(build_pdf.subprocess, 'check_output', side_effect=capture_bad):
                    with self.assertRaisesRegex(RuntimeError, 'Unpinned'):
                        build_pdf.build(NAME)
                self.assertFalse((self.root/'target/paper'/(NAME+'.pdf.manifest.json')).exists())

    def test_two_builds_across_distinct_roots_hash_identically(self):
        from PIL import Image
        folder = ROOT/'target/paper/test-pdf-provenance'
        orig_check_output = subprocess.check_output
        pdfs = []
        manifests = []
        for label in ('root-a', 'root-b'):
            with tempfile.TemporaryDirectory(dir=folder, prefix=label+'-') as tmp:
                root = Path(tmp)
                docs = root/'docs';docs.mkdir()
                (docs/'VERSION').write_text('1.0.0\n')
                im = Image.new('RGB', (16, 16), (36, 54, 65))
                im.save(docs/'neutral.png', format='PNG', compress_level=9)
                im.save(docs/'neutral.webp', format='WEBP', lossless=True)
                (docs/(NAME+'.md')).write_text(
                    '# Neutral report\n**Jepson Taylor**\n\n'
                    'A deterministic packaging control with inline code `ops/render/paper/build_pdf.py` '
                    'and relative link [VERSION](VERSION).\n\n'
                    '| Control | Status |\n|---|---|\n| Packaging | Deterministic |\n\n'
                    '![Neutral PNG](neutral.png)\n\n![Neutral WebP](neutral.webp)\n'
                )
                here = root/'ops/render/paper';here.mkdir(parents=True)
                for support_name in ('build_pdf.py', 'publication.lua', 'header.tex'):
                    (here/support_name).write_bytes((ROOT/'ops/render/paper'/support_name).read_bytes())
                def capture_git(command, **kwargs):
                    if command[:3] == ['git', 'rev-parse', 'HEAD']:return self.head+'\n'
                    if command[:2] == ['git', 'show']:return '1792195200\n'
                    return orig_check_output(command, **kwargs)
                with patch.object(build_pdf, 'ROOT', root), \
                     patch.object(build_pdf, 'HERE', here), \
                     patch.object(build_pdf, '__file__', str(here/'build_pdf.py')), \
                     patch.object(build_pdf.subprocess, 'check_output', side_effect=capture_git):
                    build_pdf.build(NAME)
                pdf_bytes = (root/'target/paper'/(NAME+'.pdf')).read_bytes()
                manifest = json.loads((root/'target/paper'/(NAME+'.pdf.manifest.json')).read_text())
                log_text = (root/'target/paper'/(NAME+'.build.log')).read_text()
                self.assertEqual(log_text, '')
                self.assertEqual(manifest['pandoc'], build_pdf.PINNED_PANDOC_VERSION)
                self.assertEqual(manifest['latex'], build_pdf.PINNED_LUALATEX_VERSION)
                self.assertEqual(hashlib.sha256(pdf_bytes).hexdigest(), manifest['output_sha256'])
                self.assertEqual(manifest['output_sha256'], '9fd1af093b50947d05880607b60ac87139c0110d70cd722d9163be8cb3b87b06')
                pdfs.append(pdf_bytes)
                manifests.append(manifest)
        self.assertEqual(hashlib.sha256(pdfs[0]).hexdigest(), hashlib.sha256(pdfs[1]).hexdigest())
        self.assertEqual(pdfs[0], pdfs[1])
        self.assertEqual(manifests[0]['output_sha256'], manifests[1]['output_sha256'])
        self.assertEqual(manifests[0]['inputs'], manifests[1]['inputs'])

    def test_verify_reproducible_writes_sha256sums_and_rejects_nondeterminism(self):
        def capture(command, **kwargs):
            if command[:3] == ['git', 'rev-parse', 'HEAD']:return self.head+'\n'
            if command[:2] == ['git', 'show']:return '1792195200\n'
            if command[:2] == ['pandoc', '--version']:return build_pdf.PINNED_PANDOC_VERSION+'\n'
            if command[:2] == ['lualatex', '--version']:return build_pdf.PINNED_LUALATEX_VERSION+'\n'
            return 'neutral tool version\n'

        def render_ok(command, **kwargs):
            Path(command[command.index('-o')+1]).write_bytes(b'neutral PDF packaging control')
            return subprocess.CompletedProcess(command, 0)

        with patch.object(build_pdf, 'ROOT', self.root), \
             patch.object(build_pdf, 'HERE', self.here), \
             patch.object(build_pdf, '__file__', str(self.here/'build_pdf.py')), \
             patch.object(build_pdf.subprocess, 'check_output', side_effect=capture), \
             patch.object(build_pdf.subprocess, 'run', side_effect=render_ok):
            sums_path = build_pdf.verify_reproducible((NAME,))
        lines = sums_path.read_text().strip().splitlines()
        self.assertEqual(len(lines), 2)
        for line in lines:
            digest, filename = line.split('  ', 1)
            self.assertEqual(digest, hashlib.sha256((self.root/'target/paper'/filename).read_bytes()).hexdigest())

        calls = 0
        def render_nondet(command, **kwargs):
            nonlocal calls
            calls += 1
            Path(command[command.index('-o')+1]).write_bytes(f'neutral pass {calls}'.encode())
            return subprocess.CompletedProcess(command, 0)

        with patch.object(build_pdf, 'ROOT', self.root), \
             patch.object(build_pdf, 'HERE', self.here), \
             patch.object(build_pdf, '__file__', str(self.here/'build_pdf.py')), \
             patch.object(build_pdf.subprocess, 'check_output', side_effect=capture), \
             patch.object(build_pdf.subprocess, 'run', side_effect=render_nondet):
            with self.assertRaisesRegex(RuntimeError, 'Non-reproducible PDF'):
                build_pdf.verify_reproducible((NAME,))


if __name__ == '__main__':unittest.main()

