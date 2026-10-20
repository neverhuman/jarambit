import hashlib, os, unittest
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
D = os.path.join(ROOT, 'designs/switchback/C9.2')


class SwitchbackC92(unittest.TestCase):
    def test_sha256sums(self):
        for line in open(os.path.join(D, 'SHA256SUMS')):
            h, rel = line.split(None, 1); rel = rel.strip()
            if rel.startswith('release-asset/'):
                continue  # published as a GitHub release asset, not in git
            with self.subTest(rel=rel):
                self.assertEqual(hashlib.sha256(open(os.path.join(D, rel), 'rb').read()).hexdigest(), h)

    def test_status_is_release_candidate(self):
        t = open(os.path.join(D, 'README.md'), encoding='utf-8').read()
        self.assertIn('RELEASE CANDIDATE', t); self.assertIn('NOT the design of record', t)
        self.assertIn('not** a jarambit RS-karambit-v2 verdict', t)
        top = open(os.path.join(ROOT, 'README.md'), encoding='utf-8').read()
        # Keep the archived study status above; do not promote it in the root publication.
        self.assertNotRegex(top, r'SWITCHBACK Rev C\.9\.2.*(?:release candidate|current RC)')


if __name__ == '__main__':
    unittest.main()


class TestC92ReviewFixes(unittest.TestCase):
    def test_governor_is_w75_1(self):
        import json
        w = json.load(open(os.path.join(D, 'data', 'w75_1_results.json')))
        self.assertAlmostEqual(w['a1']['SF_converged_design'], 1.529, places=3)
        self.assertIn('w75_1_results.json', open(os.path.join(D, 'drawings', 'draw_c92.py')).read())

    def test_draw_meta_matches_step(self):
        import json
        m = json.load(open(os.path.join(D, 'drawings', 'draw_meta.json')))
        self.assertEqual(m['sha256'], hashlib.sha256(open(os.path.join(D, 'cad', 'switchback_revC9_2_ground.step'), 'rb').read()).hexdigest())

    def test_readme_publication_pending(self):
        r = open(os.path.join(D, 'README.md')).read()
        self.assertIn('Publication pending', r)
        self.assertNotIn('is published as a release asset', r)
