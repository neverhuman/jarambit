import hashlib, json, os, re, subprocess, sys, unittest, zipfile
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
if ROOT not in sys.path:
    sys.path.insert(0, ROOT)


def readme():
    return open(os.path.join(ROOT, 'README.md'), encoding='utf-8').read()


def cad_readme():
    return open(os.path.join(ROOT, 'cad/README.md'), encoding='utf-8').read()


def release():
    """(release version, receipt of the G4 cohort of record named by evidence/g4/RECORD)"""
    v = open(os.path.join(ROOT, 'docs/VERSION')).read().strip()
    rv = open(os.path.join(ROOT, 'evidence/g4/RECORD')).read().strip()
    return v, json.load(open(os.path.join(ROOT, f'evidence/g4/v{rv}/release.json')))


class GenReadme(unittest.TestCase):
    def test_check_passes(self):
        r = subprocess.run([sys.executable, os.path.join(ROOT, 'ops/ci/gen_readme.py'), '--check'], capture_output=True, text=True)
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_release_link_matches_docs_version(self):
        v, rec = release()
        t = readme()
        self.assertEqual(rec['version'].split('.')[:2], v.split('.')[:2])
        self.assertIn(f'releases/tag/v{v}', t)
        self.assertNotIn('jepsontaylor/jarambit/releases', t)

    def test_readme_links_the_paper_of_this_release(self):
        v, _ = release()
        pdf = f'docs/g4/paper/g4-v{v}.pdf'
        self.assertTrue(os.path.isfile(os.path.join(ROOT, pdf)), pdf)
        self.assertIn(f']({pdf})', readme())
        g3_pdf = 'docs/g3/paper/g3-v3.0.2.pdf'
        self.assertTrue(os.path.isfile(os.path.join(ROOT, g3_pdf)), g3_pdf)
        self.assertIn(f']({g3_pdf})', readme())

    def test_headline_numbers_come_from_the_release_receipt(self):
        _, rec = release()
        t = readme()
        wr = rec['winner_record']
        obj = wr['objective']
        m = wr['mass']
        self.assertIn(rec['winner_id'], t)
        self.assertIn(f"{obj['w_ergolight_v2']:.2f}", t)
        self.assertIn(f"{m['as_finished_total_mass_g']:.2f} g", t)
        self.assertIn(f"{m['as_printed_total_mass_g']:.2f} g", t)
        self.assertTrue(obj['feasible'])
        self.assertGreaterEqual(wr['serration']['teeth_checked_in'], 3)
        self.assertGreaterEqual(wr['serration']['teeth_checked_sp'], 3)

    def test_every_image_and_cad_file_is_committed(self):
        _, rec = release()
        t = readme()
        ct = cad_readme()
        wid = rec['winner_id']
        for ext in ('AMbuild_mm.stl', 'AMbuild_mm.3mf', 'envelope_mm.stl', 'drawing_mm.dxf', 'spec_paper.pdf', 'xray_cutaway.jpg', 'print_spec.json'):
            rel = f'cad/{wid}_{ext}'
            fp = os.path.join(ROOT, rel)
            self.assertTrue(os.path.isfile(fp), rel)
            self.assertLess(os.path.getsize(fp), 100 * 1024 * 1024, f'{rel} exceeds 100 MiB quote form limit')
            self.assertIn(rel, t)
            self.assertIn(f'{wid}_{ext}', ct)
        with zipfile.ZipFile(os.path.join(ROOT, f'cad/{wid}_AMbuild_mm.3mf')) as zf:
            model_head = zf.open('3D/3dmodel.model').read(1024).decode('utf-8')
            self.assertIn('unit="millimeter"', model_head)
            self.assertIn('Ti-6Al-4V', model_head)
        dxf_txt = open(os.path.join(ROOT, f'cad/{wid}_drawing_mm.dxf'), encoding='utf-8').read()
        self.assertIn('UNITS: MILLIMETERS, 1 DRAWING UNIT = 1 MM', dxf_txt)
        self.assertIn('0\nCIRCLE\n8\nRING_BORE\n', dxf_txt)
        self.assertIn('MIDPLANE_Z0_OUTLINE', dxf_txt)
        self.assertIn('Z_EXTENT_ENVELOPE', dxf_txt)
        self.assertTrue(dxf_txt.endswith('0\nEOF\n'))

    def test_cad_readme_discloses_status_and_safety_factors(self):
        _, rec = release()
        ct = cad_readme()
        obj = rec['winner_record']['objective']
        m = rec['winner_record']['mass']
        self.assertIn('not qualified for use as a knife', ct)
        self.assertIn(f'{m["as_finished_total_mass_g"]:.2f} g', ct)
        self.assertIn(f'{obj["sf_solid_tip_torsion_0p80nm_lcb"]:.2f}', ct)
        self.assertIn(f'{obj["sf_goodman_fatigue_180n_lcb"]:.2f}', ct)
        self.assertIn(f'{obj["sf_per_case_sequence_min_lcb"]:.2f}', ct)
        m_notes = re.search(r'```text\n(.*?)\n```', ct, re.S)
        self.assertIsNotNone(m_notes, 'missing copy-paste order notes block in cad/README.md')
        notes = m_notes.group(1)
        self.assertIn('not qualified for use as a knife', notes)
        self.assertIn(f'{m["as_finished_total_mass_g"]:.2f} g', notes)

    def test_no_older_data(self):
        # Owner, 2026-10-26: the README describes only the release's best individual. Older designs and runs stay in
        # docs/RELEASES.md and docs/WHITEPAPER.md.
        for doc in (readme(), cad_readme()):
            for old in ('w282', 'w283', 'ec3i042', 'Individual 2000', 'hz4i012_g00085_4cdfea55', 'hz2i012_g00609_d55af923',
                        'hz4i005_g00310_966138c2', 'hz4i007_g00855_4797529d',
                        'cohort-d', 'edge-pilot', '41.77', 'G16-', 'Wave 27'):
                self.assertNotIn(old, doc, old)
            self.assertIsNone(re.search(r'\bPASS\b', doc))


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
