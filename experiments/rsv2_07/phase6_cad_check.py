"""Independent STEP reader and existing CAD checks for Phase-6 raw exports."""
from pathlib import Path
import json
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from ops.ci.evaluate_cad_candidate import verify_step_and_stl
from ops.ci.verify_genome_export import inspect_step, mesh_bore_profiles

out = Path(sys.argv[1])
doc = json.loads((out/'export.json').read_text())
cad = verify_step_and_stl(out/'candidate.step', out/'candidate.stl',
                          doc['export_stdout'], doc['mass_raster_g'])
step = inspect_step(out/'candidate.step', out/'surface.json')
profiles = mesh_bore_profiles(json.loads((out/'surface.json').read_text()))
# The meridional circle center is at each end of the straight axial bearing.
bearing = 2*min(abs(p['center_z_mm']) for p in profiles)
result = {'cad':cad, 'step_check':step, 'measured_bearing_width_mm':bearing,
          'minimum_mesh_roll_radius_mm':min(p['minor_radius_mm'] for p in profiles),
          'mesh_profile_count':len(profiles)}
(out/'cad_check.json').write_text(json.dumps(result, indent=2, allow_nan=False)+'\n')
print(json.dumps({k:v for k,v in result.items() if k != 'step_check'}))
