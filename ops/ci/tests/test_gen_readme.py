import hashlib, json, os, re, subprocess, sys, unittest, zipfile
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
if ROOT not in sys.path:
    sys.path.insert(0, ROOT)


def readme():
    return open(os.path.join(ROOT, 'README.md'), encoding='utf-8').read()


def cad_readme():
    return open(os.path.join(ROOT, 'cad/README.md'), encoding='utf-8').read()


def release():
    """(release version, receipt of the cohort of record named by evidence/g3/RECORD)"""
    v = open(os.path.join(ROOT, 'docs/VERSION')).read().strip()
    rv = open(os.path.join(ROOT, 'evidence/g3/RECORD')).read().strip()
    return v, json.load(open(os.path.join(ROOT, f'evidence/g3/v{rv}/release.json')))


class GenReadme(unittest.TestCase):
    def test_check_passes(self):
        r = subprocess.run([sys.executable, os.path.join(ROOT, 'ops/ci/gen_readme.py'), '--check'], capture_output=True, text=True)
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_release_link_matches_docs_version(self):
        v, rec = release()
        t = readme()
        self.assertEqual(rec['version'].split('.')[:2], v.split('.')[:2])  # a patch keeps its line's cohort of record
        self.assertIn(f'releases/tag/v{v}', t)
        self.assertNotIn('jepsontaylor/jarambit/releases', t)

    def test_readme_links_the_paper_of_this_release(self):
        v, _ = release()
        pdf = f'docs/g3/paper/g3-v{v}.pdf'
        self.assertTrue(os.path.isfile(os.path.join(ROOT, pdf)), pdf)
        self.assertIn(f']({pdf})', readme())

    def test_headline_numbers_come_from_the_release_receipt(self):
        _, rec = release()
        t = readme()
        self.assertIn(rec['design_id'], t)
        self.assertIn(f"| {rec['score']['score_100']:.2f} |", t)
        self.assertIn(f"{rec['mass']['mesh_g']:.1f} g", t)
        self.assertIn(rec['provenance']['git_commit'][:12], t)
        for edge in ('edge_inner', 'edge_spine'):
            self.assertGreaterEqual(rec['blade'][edge]['teeth'], 3)  # both edges serrated (SERR-1..4)

    def test_every_image_and_cad_file_is_committed(self):
        _, rec = release()
        paths = list(rec['images'].values()) + list(rec['evolution']['figures'].values()) + \
            [c['image'] for c in rec['evolution']['lineage']] + rec['cad_in_git']
        t = readme()
        ct = cad_readme()
        for p in paths:
            self.assertTrue(os.path.isfile(os.path.join(ROOT, p)), p)
            self.assertIn(p, t)
        for p in rec['cad_in_git']:
            top_p = f'cad/{os.path.basename(p)}'
            self.assertTrue(os.path.isfile(os.path.join(ROOT, top_p)), top_p)
            self.assertIn(top_p, t)
            self.assertIn(os.path.basename(p), ct)
            h1 = hashlib.sha256(open(os.path.join(ROOT, p), 'rb').read()).hexdigest()
            h2 = hashlib.sha256(open(os.path.join(ROOT, top_p), 'rb').read()).hexdigest()
            self.assertEqual(h1, h2, f'{top_p} differs from {p}')
        wid = rec['design_id']
        for ext in ('AMbuild_mm.stl', 'AMbuild_mm.3mf', 'envelope_mm.stl', 'drawing_mm.dxf', 'spec_paper.pdf', 'xray_cutaway.jpg'):
            rel = f'cad/{wid}_{ext}'
            fp = os.path.join(ROOT, rel)
            self.assertTrue(os.path.isfile(fp), rel)
            self.assertLess(os.path.getsize(fp), 100 * 1024 * 1024, f'{rel} exceeds 100 MiB quote form limit')
            self.assertIn(rel, t)
            self.assertIn(f'{wid}_{ext}', ct)
        with zipfile.ZipFile(os.path.join(ROOT, f'cad/{wid}_AMbuild_mm.3mf')) as zf:
            model_head = zf.open('3D/3dmodel.model').read(1024).decode('utf-8')
            self.assertIn('unit="millimeter"', model_head)
        dxf_txt = open(os.path.join(ROOT, f'cad/{wid}_drawing_mm.dxf'), encoding='utf-8').read()
        self.assertIn('$INSUNITS\n70\n4\n', dxf_txt)
        self.assertTrue(dxf_txt.endswith('0\nEOF\n'))

    def test_no_older_data(self):
        # Owner, 2026-10-26: the README describes only the release's best individual. Older designs and runs stay in
        # docs/RELEASES.md and docs/WHITEPAPER.md.
        for doc in (readme(), cad_readme()):
            for old in ('w282', 'w283', 'ec3i042', 'Individual 2000', 'hz4i012_g00085_4cdfea55', 'hz2i012_g00609_d55af923',
                        'hz4i005_g00310_966138c2', 'hz4i007_g00855_4797529d',
                        'cohort-d', 'edge-pilot', '41.77', 'G16-', 'Wave 27'):
                self.assertNotIn(old, doc, old)
            self.assertIsNone(re.search(r'\bPASS\b', doc))


if __name__ == '__main__':
    unittest.main()
