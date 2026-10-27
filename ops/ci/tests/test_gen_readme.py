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
        # a release keeps its line's cohort of record unless its RELEASES.md entry names another one
        from ops.ci.gen_readme import record_declared
        self.assertTrue(record_declared(v, rec['version']))
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
            self.assertIn('Ti-6Al-4V (design assumption: yield 828 MPa)', model_head)
        dxf_txt = open(os.path.join(ROOT, f'cad/{wid}_drawing_mm.dxf'), encoding='utf-8').read()
        self.assertIn('UNITS: MILLIMETERS, 1 DRAWING UNIT = 1 MM', dxf_txt)
        self.assertIn('0\nCIRCLE\n8\nRING_BORE\n10\n-5.9672\n20\n0.0000\n30\n0.0000\n40\n12.4500\n', dxf_txt)
        self.assertIn('MIDPLANE_Z0_OUTLINE', dxf_txt)
        self.assertIn('Z_EXTENT_ENVELOPE', dxf_txt)
        self.assertTrue(dxf_txt.endswith('0\nEOF\n'))

    def test_cad_readme_discloses_status_and_below_target_safety_factors(self):
        _, rec = release()
        ct = cad_readme()
        i = rec['inputs']
        fe = rec['fe_certification']
        self.assertIn(rec['status'], ct)
        self.assertIn('not qualified for use as a knife', ct)
        for sf_str in (f'{i["solid_tip_sf"]:.2f}', f'{i["grip_sf_min"]:.2f}', f'{fe["grip_sf"]["spine"]:.2f}',
                       f'{i["fat_sf"]:.2f}', f'{i["seq_ratio"]:.2f}'):
            self.assertIn(sf_str, ct)
        m = re.search(r'```text\n(.*?)\n```', ct, re.S)
        self.assertIsNotNone(m, 'missing copy-paste order notes block in cad/README.md')
        notes = m.group(1)
        self.assertIn(rec['status'], notes)
        self.assertIn('not qualified for use as a knife', notes)
        for sf_str in (f'{i["solid_tip_sf"]:.2f}', f'{i["grip_sf_min"]:.2f}', f'{fe["grip_sf"]["spine"]:.2f}',
                       f'{i["fat_sf"]:.2f}', f'{i["seq_ratio"]:.2f}'):
            self.assertIn(sf_str, notes)

    def test_no_older_data(self):
        # Owner, 2026-10-26: the README describes only the release's best individual. Older designs and runs stay in
        # docs/RELEASES.md and docs/WHITEPAPER.md.
        for doc in (readme(), cad_readme()):
            for old in ('w282', 'w283', 'ec3i042', 'Individual 2000', 'hz4i012_g00085_4cdfea55', 'hz2i012_g00609_d55af923',
                        'hz4i005_g00310_966138c2', 'hz4i007_g00855_4797529d',
                        'cohort-d', 'edge-pilot', '41.77', 'G16-', 'Wave 27'):
                self.assertNotIn(old, doc, old)
            self.assertIsNone(re.search(r'\bPASS\b', doc))


class PublicWording(unittest.TestCase):
    # v3.0.3 errata (research-integrity and G4 audits): overclaims that must not return to the public G3 text. A
    # simulation is not a certification, the tip mesh series is not monotone, and the ergonomic multiplier allows trades.
    BANNED = ('as printed', 'until it converged', 'no person designed', 'cannot trade comfort', '3d-printable',
              'printable titanium', 'certified', 'certification', 'cvt-map-elites', '1.82 n·m pry', '1.82~n$\\cdot$m pry',
              '1.82 n$\\cdot$m pry')

    def test_public_g3_text_has_no_overclaims(self):
        paper = os.path.join(ROOT, 'docs/g3/paper')
        files = ['README.md', 'cad/README.md', 'docs/g3/paper/main.tex'] + sorted(
            f'docs/g3/paper/sections/{f}' for f in os.listdir(os.path.join(paper, 'sections')) if f.endswith('.tex'))
        for rel in files:
            text = open(os.path.join(ROOT, rel), encoding='utf-8').read().lower()
            for phrase in self.BANNED:
                self.assertNotIn(phrase, text, f'{rel}: "{phrase}"')

    def test_diagnostics_are_listed_and_hashed(self):
        _, rec = release()
        d = os.path.join(ROOT, f'evidence/g3/v{rec["version"]}/diagnostics')
        sums = dict(reversed(l.split()) for l in open(os.path.join(d, 'SHA256SUMS')) if l.strip())
        for name in ('seq_per_lc', 'tip_raster', 'zone_mass', 'plate_meshconv'):
            body = open(os.path.join(d, f'{name}.json'), 'rb').read()
            self.assertEqual(sums[f'{name}.json'], hashlib.sha256(body).hexdigest(), name)
            self.assertEqual(json.loads(body)['design_id'], rec['design_id'], name)


class PythonInventory(unittest.TestCase):
    def test_python_exceptions_lists_exactly_the_tracked_python_files(self):
        # AGENTS.md: no new Python files; every existing one is listed with its disposition in PYTHON_EXCEPTIONS.md.
        tracked = subprocess.check_output(['git', 'ls-files', '*.py'], cwd=ROOT, text=True).split()
        text = open(os.path.join(ROOT, 'PYTHON_EXCEPTIONS.md'), encoding='utf-8').read()
        rows = re.findall(r'^\| `([^`]+\.py)` \| \d+ \| `(?:KEEP_WITH_REASON|PORT_TO_RUST)` \|', text, re.M)
        self.assertEqual(sorted(rows), sorted(tracked))
        m = re.search(r'Total Tracked `\.py` Files:\*\* `(\d+)`', text)
        self.assertEqual(int(m.group(1)), len(tracked))


if __name__ == '__main__':
    unittest.main()
