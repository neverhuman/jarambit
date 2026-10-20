import hashlib, json, os, re, unittest
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
D = os.path.join(ROOT, 'designs/switchback/C9.3')


class SwitchbackC93(unittest.TestCase):
    def test_sha256sums(self):
        for line in open(os.path.join(D, 'SHA256SUMS')):
            h, rel = line.split(None, 1); rel = rel.strip()
            if rel.startswith('release-asset/'):
                continue
            with self.subTest(rel=rel):
                self.assertEqual(hashlib.sha256(open(os.path.join(D, rel), 'rb').read()).hexdigest(), h)

    def test_status_wording(self):
        t = open(os.path.join(D, 'README.md'), encoding='utf-8').read()
        self.assertIn('CURRENT RELEASE CANDIDATE', t); self.assertIn('NOT the design of record', t)
        self.assertIn('not** a jarambit RS-karambit-v2 verdict', t); self.assertIn('Publication pending', t)
        p = open(os.path.join(ROOT, 'designs/switchback/C9.2/README.md'), encoding='utf-8').read()
        self.assertIn('PREVIOUS RELEASE CANDIDATE', p)
        top = open(os.path.join(ROOT, 'README.md'), encoding='utf-8').read()
        # Keep the archived study status above; do not promote it in the root publication.
        self.assertNotRegex(top, r'SWITCHBACK Rev C\.9\.3.*(?:release candidate|current RC)')

    def test_board_and_meta(self):
        b = json.load(open(os.path.join(D, 'data', 'w77_results.json')))['board']
        self.assertEqual(b['governor'], 'V3'); self.assertAlmostEqual(b['governor_SF'], 1.597, places=3)
        self.assertTrue(all(v >= 5.0 for v in b['margins_pct'].values()))
        step = os.path.join(D, 'cad', 'switchback_revC9_3_ground.step')
        m = json.load(open(os.path.join(D, 'drawings', 'draw_meta.json')))
        self.assertEqual(m['sha256'], hashlib.sha256(open(step, 'rb').read()).hexdigest())
        self.assertTrue(m['sha256'].startswith('3a818183bcb5b9a9'))
        self.assertAlmostEqual(m['mass_g'], 66.773, delta=0.01)


if __name__ == '__main__':
    unittest.main()
