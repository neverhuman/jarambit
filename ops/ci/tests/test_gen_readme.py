import hashlib, json, os, re, subprocess, sys, unittest, zipfile
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
if ROOT not in sys.path:
    sys.path.insert(0, ROOT)


def readme():
    return open(os.path.join(ROOT, 'README.md'), encoding='utf-8').read()


def cad_readme():
    return open(os.path.join(ROOT, 'cad/README.md'), encoding='utf-8').read()


def is_g5():
    v = open(os.path.join(ROOT, 'docs/VERSION')).read().strip()
    return int(v.split('.')[0]) >= 5 and os.path.exists(os.path.join(ROOT, 'evidence/g5/RECORD'))


def release():
    """(release version, receipt of the cohort of record named by evidence/g5/RECORD or evidence/g3/RECORD)"""
    v = open(os.path.join(ROOT, 'docs/VERSION')).read().strip()
    if is_g5():
        rv = open(os.path.join(ROOT, 'evidence/g5/RECORD')).read().strip()
        return v, json.load(open(os.path.join(ROOT, f'evidence/g5/v{rv}/release.json')))
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
        pdf = f'docs/g5/paper/g5-v{v}.pdf' if is_g5() else f'docs/g3/paper/g3-v{v}.pdf'
        self.assertTrue(os.path.isfile(os.path.join(ROOT, pdf)), pdf)
        self.assertIn(f']({pdf})', readme())

    def test_headline_numbers_come_from_the_release_receipt(self):
        _, rec = release()
        t = readme()
        self.assertIn(rec['design_id'], t)
        self.assertIn(f"{rec['score']['score_100']:.2f}", t)
        if is_g5():
            self.assertIn(rec['provenance']['decoder_commit_label'], t)
            self.assertIn(f"{rec['mass']['of_record_g']:.1f} g", t)
            for edge in ('edge_inner', 'edge_spine'):
                self.assertGreaterEqual(rec['blade'][edge]['teeth'], 3)
        else:
            self.assertIn(rec['provenance']['git_commit'][:12], t)
            self.assertIn(f"{rec['mass']['mesh_g']:.1f} g", t)
            for edge in ('edge_inner', 'edge_spine'):
                self.assertGreaterEqual(rec['blade'][edge]['teeth'], 3)  # both edges serrated (SERR-1..4)

    def test_every_image_and_cad_file_is_committed(self):
        _, rec = release()
        if is_g5():
            return self.check_g5_files(rec)
        lineage_imgs = [c['image'] for c in rec['evolution']['lineage']]
        evo_figs = [rec['evolution']['figure']] if is_g5() else list(rec['evolution']['figures'].values())
        paths = list(rec['images'].values()) + evo_figs + lineage_imgs + rec['cad_in_git']
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
            self.assertIn('Ti-6Al-4V', model_head)
        dxf_txt = open(os.path.join(ROOT, f'cad/{wid}_drawing_mm.dxf'), encoding='utf-8').read()
        self.assertIn('UNITS: MILLIMETERS, 1 DRAWING UNIT = 1 MM', dxf_txt)
        self.assertIn('0\nCIRCLE\n8\nRING_BORE\n', dxf_txt)
        self.assertIn('MIDPLANE_Z0_OUTLINE', dxf_txt)
        self.assertIn('Z_EXTENT_ENVELOPE', dxf_txt)
        self.assertTrue(dxf_txt.endswith('0\nEOF\n'))

    def check_g5_files(self, rec):
        """G5: images and lineage from the render manifest, CAD from docs/g5/v<RECORD>/cad/; every file committed, named in
        the README, within the 50 MiB repository limit; the cad/ kit holds the same bytes as the archive."""
        rv, wid, t, ct = rec['version'], rec['design_id'], readme(), cad_readme()
        man = json.load(open(os.path.join(ROOT, f'docs/g5/v{rv}/img/manifest.json')))
        for p in list(man['images'].values()) + [c['image'] for c in man['lineage']] + [rec['evolution']['figure']]:
            self.assertTrue(os.path.isfile(os.path.join(ROOT, p)), p)
            self.assertIn(p, t)
        arch = os.path.join(ROOT, f'docs/g5/v{rv}/cad')
        for n in os.listdir(arch):
            top = os.path.join(ROOT, 'cad', n)
            if os.path.isfile(top) and not n.endswith('.json'):
                h = lambda f: hashlib.sha256(open(f, 'rb').read()).hexdigest()
                self.assertEqual(h(os.path.join(arch, n)), h(top), f'cad/{n} differs from its archive copy')
        for ext in ('AMbuild_mm.stl', 'AMbuild_mm.3mf', 'envelope_mm.stl', 'drawing_mm.dxf', 'spec_paper.pdf', 'xray_cutaway.jpg'):
            rel = f'cad/{wid}_{ext}'
            self.assertTrue(os.path.isfile(os.path.join(ROOT, rel)), rel)
            self.assertLessEqual(os.path.getsize(os.path.join(ROOT, rel)), 50 * 1024 * 1024, f'{rel} exceeds 50 MiB')
            self.assertIn(rel, t)
            self.assertIn(f'{wid}_{ext}', ct)
        with zipfile.ZipFile(os.path.join(ROOT, f'cad/{wid}_AMbuild_mm.3mf')) as zf:
            self.assertIn('unit="millimeter"', zf.open('3D/3dmodel.model').read(1024).decode('utf-8'))
        for a in rec['release_assets']:
            self.assertIn(a['file'], t)

    def test_cad_readme_discloses_status_and_below_target_safety_factors(self):
        _, rec = release()
        ct = cad_readme()
        self.assertIn(rec['status'], ct)
        self.assertIn('not qualified for use as a knife', ct)
        m = re.search(r'```text\n(.*?)\n```', ct, re.S)
        self.assertIsNotNone(m, 'missing copy-paste order notes block in cad/README.md')
        notes = m.group(1)
        self.assertIn(rec['status'], notes)
        self.assertIn('not qualified for use as a knife', notes)
        if is_g5():
            # the lowest computed margins are disclosed next to the status
            low = sorted((v for v in rec.get('margins_min_over_plate_meshes', rec['margins']).values() if isinstance(v, float)))[:3]
            for v in low:
                self.assertIn(f'{v:.2f}', ct)
        else:
            i = rec['inputs']
            fe = rec['fe_certification']
            for sf_str in (f'{i["solid_tip_sf"]:.2f}', f'{i["grip_sf_min"]:.2f}', f'{fe["grip_sf"]["spine"]:.2f}',
                           f'{i["fat_sf"]:.2f}', f'{i["seq_ratio"]:.2f}'):
                self.assertIn(sf_str, ct)
                self.assertIn(sf_str, notes)

    def test_no_older_data(self):
        # Owner, 2026-10-26: the README describes only the release's best individual. Older designs and runs stay in
        # docs/RELEASES.md and docs/WHITEPAPER.md.
        old_ids = ['w282', 'w283', 'ec3i042', 'Individual 2000', 'hz4i012_g00085_4cdfea55', 'hz2i012_g00609_d55af923',
                   'hz4i005_g00310_966138c2', 'hz4i007_g00855_4797529d',
                   'cohort-d', 'edge-pilot', '41.77', 'G16-', 'Wave 27']
        if is_g5():
            # the G3 winner of record and its headline numbers belong to docs/g3/, not to the G5 README
            old_ids.extend(['hz4i013_g00995_d206e3c9', 'hz4i008_g00849_597619df', 'hz4i', 'W_ERGOLIGHT_v1'])
            _, rec = release()
            if 'lineage' in rec and rec['design_id'] != 'kv4i012_g00009_dd6ffea5':
                old_ids.append('kv4i012_g00009_dd6ffea5')  # the v5.0.0 winner: its comparison is in docs/RELEASES.md and the paper
        for doc in (readme(), cad_readme()):
            for old in old_ids:
                self.assertNotIn(old, doc, old)
            self.assertIsNone(re.search(r'\bPASS\b', doc))


class PublicWording(unittest.TestCase):
    # v3.0.3 errata (research-integrity and G4 audits): overclaims that must not return to the public G3/G5 text. A
    # simulation is not a certification, the tip mesh series is not monotone, and the ergonomic multiplier allows trades.
    BANNED = ('as printed', 'until it converged', 'no person designed', 'cannot trade comfort', '3d-printable',
              'printable titanium', 'certified', 'certification', 'cvt-map-elites', '1.82 n·m pry', '1.82~n$\\cdot$m pry',
              '1.82 n$\\cdot$m pry')

    def test_public_g3_text_has_no_overclaims(self):
        paper_g3 = os.path.join(ROOT, 'docs/g3/paper')
        files = ['README.md', 'cad/README.md', 'docs/g3/paper/main.tex'] + sorted(
            f'docs/g3/paper/sections/{f}' for f in os.listdir(os.path.join(paper_g3, 'sections')) if f.endswith('.tex'))
        paper_g5 = os.path.join(ROOT, 'docs/g5/paper')
        if os.path.isdir(paper_g5):
            files += ['docs/g5/paper/main.tex'] + sorted(
                f'docs/g5/paper/sections/{f}' for f in os.listdir(os.path.join(paper_g5, 'sections')) if f.endswith('.tex'))
        for rel in files:
            text = open(os.path.join(ROOT, rel), encoding='utf-8').read().lower()
            for phrase in self.BANNED:
                self.assertNotIn(phrase, text, f'{rel}: "{phrase}"')

    def test_diagnostics_are_listed_and_hashed(self):
        _, rec = release()
        if is_g5():
            d = os.path.join(ROOT, rec['provenance']['vv'])  # v6 keeps the v5.0.0 studies of the unchanged decoder
            sums = dict(reversed(l.split()) for l in open(os.path.join(d, 'SHA256SUMS')) if l.strip())
            self.assertGreaterEqual(len(sums), 3)
            for fname, expected_sha in sums.items():
                body = open(os.path.join(d, fname), 'rb').read()
                self.assertEqual(expected_sha, hashlib.sha256(body).hexdigest(), fname)
        else:
            d = os.path.join(ROOT, f'evidence/g3/v{rec["version"]}/diagnostics')
            sums = dict(reversed(l.split()) for l in open(os.path.join(d, 'SHA256SUMS')) if l.strip())
            for name in ('seq_per_lc', 'tip_raster', 'zone_mass', 'plate_meshconv'):
                body = open(os.path.join(d, f'{name}.json'), 'rb').read()
                self.assertEqual(sums[f'{name}.json'], hashlib.sha256(body).hexdigest(), name)
                self.assertEqual(json.loads(body)['design_id'], rec['design_id'], name)


class RequirementSetV5(unittest.TestCase):
    # requirements/RS-karambit-v5.toml: every value names its sources, and PROVENANCE-v5.md resolves every key.
    def setUp(self):
        import tomllib
        self.text = open(os.path.join(ROOT, 'requirements/RS-karambit-v5.toml'), encoding='utf-8').read()
        self.rs = tomllib.loads(self.text)
        self.prov = open(os.path.join(ROOT, 'requirements/PROVENANCE-v5.md'), encoding='utf-8').read()

    def test_every_cited_source_resolves(self):
        cited = set()
        for line in self.text.splitlines():
            if '#' in line:
                for group in re.findall(r'\[([a-z0-9][a-z0-9-]*(?:, *[a-z0-9][a-z0-9-]*)*)\]', line.split('#', 1)[1]):
                    cited.update(k.strip() for k in group.split(','))
        rows = [l for l in self.prov.splitlines() if l.startswith('| `')]
        listed = set(k for l in rows for k in re.findall(r'`([a-z0-9][a-z0-9-]*)`', l.split('|')[1]))
        tables = set()
        def walk(d):
            for k, v in d.items():
                if isinstance(v, dict):
                    tables.add(k); walk(v)
        walk(self.rs)
        cited -= tables  # comments may refer to a section of this file, such as [margins]
        self.assertTrue(cited, 'no source keys found')
        self.assertEqual(sorted(cited - listed), [], 'source keys without a provenance row')

    def test_continuous_objective_and_owner_decisions(self):
        self.assertIs(self.rs['hard_gates'], False)  # owner: no hard gates
        for key in ('owner-ring', 'owner-reach', 'owner-balance', 'owner-objective', 'owner-point', 'owner-grip',
                    'owner-serration'):
            self.assertIn(f'| `{key}` |', self.prov, key)
        p, e = self.rs['point'], self.rs['edge']
        self.assertEqual((p['angle_max_at_1mm_deg'], p['angle_max_at_3mm_deg']), (75.0, 60.0))  # owner, 2026-10-27
        self.assertLessEqual(p['width_max_at_2mm'], 3.5)
        self.assertEqual((e['per_side_angle_min_deg'], e['per_side_angle_max_deg']), (20.0, 25.0))
        o = self.rs['objective']
        # 1 % of E must cost at least min_grams_per_percent_e of mass credit at a score near 80
        pts_per_g = 100.0 * o['mass_weight'] / (self.rs['mass']['ceiling_g'] - self.rs['mass']['floor_g'])
        self.assertGreaterEqual(o['e_exponent'] * 0.80 / pts_per_g, o['min_grams_per_percent_e'])
        self.assertAlmostEqual(o['mass_weight'] + sum(o['weights'].values()), 1.0, places=9)
        self.assertGreater(o['worst_margin_exponent'], 0.0)
        self.assertAlmostEqual(sum(o['betas'].values()), 1.0, places=9)
        self.assertEqual(self.rs['generation'], 'G5')


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
