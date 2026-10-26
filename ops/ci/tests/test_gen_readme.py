import json, os, re, subprocess, sys, unittest
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
if ROOT not in sys.path:
    sys.path.insert(0, ROOT)


def readme():
    return open(os.path.join(ROOT, 'README.md'), encoding='utf-8').read()


def release():
    v = open(os.path.join(ROOT, 'docs/VERSION')).read().strip()
    return v, json.load(open(os.path.join(ROOT, f'evidence/g3/v{v}/release.json')))


class GenReadme(unittest.TestCase):
    def test_check_passes(self):
        r = subprocess.run([sys.executable, os.path.join(ROOT, 'ops/ci/gen_readme.py'), '--check'], capture_output=True, text=True)
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_release_link_matches_docs_version(self):
        v, rec = release()
        t = readme()
        self.assertEqual(rec['version'], v)
        self.assertIn(f'releases/tag/v{v}', t)
        self.assertNotIn('jepsontaylor/jarambit/releases', t)

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
        for p in paths:
            self.assertTrue(os.path.isfile(os.path.join(ROOT, p)), p)
            self.assertIn(p, t)

    def test_no_older_data(self):
        # Owner, 2026-10-26: the README describes only the release's best individual. Older designs and runs stay in
        # docs/RELEASES.md and docs/WHITEPAPER.md.
        t = readme()
        for old in ('w282', 'w283', 'ec3i042', 'Individual 2000', 'hz4i012_g00085_4cdfea55', 'hz2i012_g00609_d55af923',
                    'hz4i005_g00310_966138c2', 'hz4i007_g00855_4797529d',
                    'cohort-d', 'edge-pilot', '41.77', 'G16-', 'Wave 27'):
            self.assertNotIn(old, t, old)
        self.assertIsNone(re.search(r'\bPASS\b', t))


if __name__ == '__main__':
    unittest.main()
