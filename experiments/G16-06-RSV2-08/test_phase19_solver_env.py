"""Portable audit of recorded environments; never query installed packages."""
import copy
import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import tomllib
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
STUDY = Path('experiments/G16-06-RSV2-08')
AUDIT = STUDY / 'phase19_solver_environment_audit.json'
AUDIT_SHA256 = '73d85168599a1f41ddc9c9a533184d231ae293daa4a70eaffc4a12fba926716d'
PACKAGES = {'scikit-fem': '12.0.2', 'scipy': '1.18.1', 'numpy': '2.5.4', 'meshio': None}
SOLID_SETTINGS = {'R_CUT': 30., 'NL': 4, 'NSUB': 1, 'PATCH_mm': 2., 'T_Nmm': 1820.}


def receipt_path(phase):
    directory = Path('experiments') / ('g16-06-rsv2-08' if 9 <= phase <= 14 else 'G16-06-RSV2-08')
    names = {15: 'phase15_gate_s_compliant_joint_audit.json',
             16: 'phase16_cross_phase_provenance_index.json',
             17: 'phase17_legacy_path_inventory.json', 18: 'phase18_complete_study_manifest.json'}
    return directory / names.get(phase, f'phase{phase}_receipt.json')


def load(path):
    return json.loads(path.read_bytes())


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observations(value, key, path=()):
    if isinstance(value, dict):
        for name, child in value.items():
            if name == key:
                yield dict(keypath=list(path + (name,)), value=child)
            yield from observations(child, key, path + (name,))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            yield from observations(child, key, path + (index,))


def solid_settings(document):
    actual = {key: document['solid'][key] for key in SOLID_SETTINGS if key != 'T_Nmm'}
    actual['T_Nmm'] = document['T_Nmm']
    if actual != SOLID_SETTINGS:
        raise ValueError('Solid geometry/load settings changed')
    solver = document['solver']
    if solver['algorithm'] != 'skfem_p2_amg_cg' or solver['converged'] is not True:
        raise ValueError('Primary solver/convergence changed')
    residual = solver['relative_residual']
    if isinstance(residual, bool) or not 0 <= residual <= 1e-10:
        raise ValueError('Primary residual threshold violated')
    return actual


def generate(root=ROOT):
    root = Path(root)
    inputs = {}
    def read(relative):
        inputs[relative.as_posix()] = digest(root / relative)
        return load(root / relative)
    lock = Path('Cargo.lock'); lock_hash = digest(root / lock); inputs[lock.as_posix()] = lock_hash
    packages = tomllib.loads((root / lock).read_text())['package']
    faer = [p['version'] for p in packages if p['name'] == 'faer']
    if faer != ['0.24.4']:
        raise ValueError('Recorded faer lock version changed')
    phases = []
    for phase in range(7, 19):
        relative = receipt_path(phase); document = read(relative)
        flag = document['all_gate_qualification'] if phase <= 15 else document['full_qualification']
        if flag is not False:
            raise ValueError('Historical qualification changed')
        solver_rows = list(observations(document, 'solver')) if phase <= 15 else []
        for observed in solver_rows:
            side_a4 = observed['keypath'][-2:] == ['a4', 'solver']
            observed['role'] = 'native side-A4' if side_a4 else 'native plate'
            expected = 'native side_a4 faer sparse Cholesky / K-orthogonal Lanczos' if side_a4 else 'faer'
            if observed['value'] != expected:
                raise ValueError('Native plate solver inconsistency or side-A4 drift')
        refs = {**document.get('sha256', {}), **document.get('source_provenance', {})}
        lock_binding = refs.get('Cargo.lock')
        if lock_binding is not None and lock_binding != lock_hash:
            raise ValueError('Historical Cargo.lock binding differs')
        versions = list(observations(document, 'versions')) if phase <= 15 else []
        recorded = document.get('runtime', {}).get('versions', {})
        mapped = {name: recorded.get('skfem' if name == 'scikit-fem' else name) for name in PACKAGES}
        if phase == 15 and mapped != PACKAGES:
            raise ValueError('Recorded Python package versions differ')
        if phase != 15 and versions:
            raise ValueError('Unexpected version evidence requires review')
        row = dict(phase=phase, receipt=relative.as_posix(),
                   execution_scope='saved-data summary; no new numerical solve' if phase in (12,16,17,18) else 'measurement receipt',
                   native_solver_observations=solver_rows,
                   native_solver_status='recorded faer' if solver_rows else 'not recorded here',
                   faer_version=faer[0] if lock_binding else None,
                   faer_version_basis='receipt-bound Cargo.lock' if lock_binding else 'no lockfile binding in this receipt',
                   package_versions=mapped, package_version_observations=versions,
                   primary_solids=[], qualified_count=0, full_qualification=False)
        if phase == 15:
            for candidate in document['results']:
                plate_path = Path(candidate['plate']); solid_path = Path(candidate['solid'])
                plate = read(plate_path); solid = read(solid_path)
                for p in [plate_path, solid_path]:
                    if inputs[p.as_posix()] != document['sha256'][p.as_posix()]:
                        raise ValueError('Tracked measurement binding differs')
                if plate['solver'] != 'faer':
                    raise ValueError('Native plate solver inconsistency')
                row.setdefault('tracked_plate_observations', []).append(dict(path=plate_path.as_posix(), solver=plate['solver']))
                row['primary_solids'].append(dict(path=solid_path.as_posix(), settings=solid_settings(solid),
                    solver=solid['solver'], package_versions_basis='same-phase runtime versions; not independently embedded in each solid JSON'))
        phases.append(row)
    return dict(task='G16-06-RSV2-08-p20', study_phase=19, qualified_count=0, full_qualification=False,
                recorded_values_consistent=True, all_phase_versions_fully_pinned=False,
                missing_evidence=['meshio version not recorded', 'Python versions absent before Phase15',
                                 'Phase7/8 receipts name faer without Cargo.lock version binding',
                                 'Phase12/16/17/18 summaries establish no new runtime environment'],
                faer_lock_version=faer[0], expected_primary_solid_settings=SOLID_SETTINGS,
                input_sha256=inputs, phases=phases)


def verify(root=ROOT, document=None):
    root = Path(root)
    if document is None:
        document = load(root / AUDIT)
    if document != generate(root):
        raise ValueError('Environment audit mismatch')
    return dict(integrity='pass', inputs=len(document['input_sha256']), phases=12,
                recorded_values_consistent=True, all_phase_versions_fully_pinned=False,
                qualified_count=0, full_qualification=False)


class SolverEnvironmentTests(unittest.TestCase):
    def document(self):
        return load(ROOT / AUDIT)

    def test_exact_inputs_and_independent_digest_pin(self):
        self.assertEqual(digest(ROOT / AUDIT), AUDIT_SHA256)
        self.assertEqual(verify()['inputs'], 19)

    def test_all_phase_coverage_and_qualification(self):
        d = self.document()
        self.assertEqual([p['phase'] for p in d['phases']], list(range(7,19)))
        for row in [d, *d['phases']]:
            self.assertEqual(row['qualified_count'], 0); self.assertIs(row['full_qualification'], False)

    def test_missing_versions_never_inherit_current_environment(self):
        d = self.document(); self.assertFalse(d['all_phase_versions_fully_pinned'])
        self.assertEqual([p['phase'] for p in d['phases'] if p['faer_version']], [9,10,11,13,14,15])
        for p in d['phases']:
            self.assertIsNone(p['package_versions']['meshio'])
            if p['phase'] != 15:
                self.assertTrue(all(v is None for v in p['package_versions'].values()))
        self.assertEqual(d['phases'][8]['package_versions'], PACKAGES)

    def test_three_solids_use_exact_requested_policy(self):
        rows = self.document()['phases'][8]['primary_solids']; self.assertEqual(len(rows), 3)
        for row in rows:
            self.assertEqual(row['settings'], SOLID_SETTINGS)

    def test_settings_and_residual_relaxation_rejected(self):
        original = load(ROOT / self.document()['phases'][8]['primary_solids'][0]['path'])
        for key in SOLID_SETTINGS:
            d = copy.deepcopy(original)
            if key == 'T_Nmm': d[key] += 1
            else: d['solid'][key] += 1
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, 'geometry/load'):
                solid_settings(d)
        d = copy.deepcopy(original); d['solver']['relative_residual'] = 1e-9
        with self.assertRaisesRegex(ValueError, 'residual threshold'):
            solid_settings(d)

    def test_invented_pinning_or_qualification_rejected(self):
        for key, value in [('all_phase_versions_fully_pinned', True), ('qualified_count', 1), ('full_qualification', True)]:
            d = self.document(); d[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, 'audit mismatch'):
                verify(document=d)

    def test_solver_drift_rejected(self):
        original = load
        def changed(path):
            d = original(path)
            if path == ROOT / receipt_path(9): d['solver'] = 'skyline'
            return d
        with patch(__name__ + '.load', side_effect=changed), self.assertRaisesRegex(ValueError, 'plate solver'):
            generate()

    def test_new_files_have_no_literal_host_prefix(self):
        for name in ['PHASE19.md','phase19_solver_environment_audit.json','test_phase19_solver_env.py']:
            self.assertNotIn(('/'+'home/'+'ubuntu').encode(), (HERE / name).read_bytes())

    def test_relocated_read_only_without_runtime_packages(self):
        base = ROOT / 'target/G16-06-RSV2-08/phase20'; base.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=base) as temp:
            root = Path(temp)
            for relative in [*self.document()['input_sha256'], AUDIT.as_posix()]:
                target = root / relative; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, target)
            self.assertFalse((root / '.git').exists()); self.assertFalse((root / 'target').exists())
            original = Path.open
            def guarded(path, mode='r', *args, **kwargs):
                if not path.is_relative_to(root) or any(c in mode for c in 'wax+'):
                    raise AssertionError('Nonlocal read or write')
                return original(path, mode, *args, **kwargs)
            with patch.object(Path, 'open', new=guarded), patch.dict('os.environ', {}, clear=True), patch('subprocess.run', side_effect=AssertionError('Process launch')):
                self.assertEqual(verify(root)['integrity'], 'pass')


if __name__ == '__main__':
    unittest.main(verbosity=2)
