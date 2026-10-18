"""Plot actual exported point intersections and face-angle observations."""
from pathlib import Path
import json
import os

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/rsv2-07/v3'
os.sched_setaffinity(0, {14, 15})
os.environ['MPLCONFIGDIR'] = str(OUT.parent / 'mplconfig')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

final = OUT / 'critical_accepted_summary.json'
accepted = json.loads(final.read_text()) if final.exists() else []
spine_export = accepted[0]['selected_from'] if accepted else 'follow_export_1'
fig, axes = plt.subplots(1, 2, figsize=(11, 4), layout='constrained')
for name, label in [('export_4', 'Original compound point'),
                    (spine_export, 'Revised point + verified spine design')]:
    doc = json.loads((OUT / name / 'result.json').read_text())
    samples = doc['measurement']['point']['samples']
    axes[0].plot(*zip(*samples), label=label, linewidth=2)
axes[0].axhline(5.7, color='gray', linestyle='--', label='95% of 6 mm core')
axes[0].scatter([.25, 1, 2], [.30, .60, 1], marker='v', color='black', label='Point thickness ceilings')
axes[0].axvline(5, color='gray', linestyle=':', linewidth=1)
axes[0].set(xlabel='Distance along measured bisector (mm)', ylabel='Exported thickness (mm)',
            title='Actual triangle-intersection point profiles', xlim=(0, 10), ylim=(0, 6.1))
axes[0].legend(fontsize=8, loc='lower right')
names = ['export_0', 'export_2', 'export_3', 'follow_export_4', spine_export]
measurements = [json.loads((OUT / n / 'result.json').read_text())['measurement']['sharpness'] for n in names]
angles = [m['max_half_angle_deg'] for m in measurements]
axes[1].bar(range(len(names)), angles, color=['#286f50' if m['pass'] else '#ba493b' for m in measurements])
axes[1].axhline(25, color='black', linestyle='--', linewidth=1, label='Unchanged 25° ceiling')
axes[1].set(xticks=range(len(names)), xticklabels=['Early .42', 'Early .45', 'Ref. .50', 'Follow .45', 'Final spine'],
            ylabel='Maximum measured half-angle (degrees)', title='Exported faces decide the angle gate', ylim=(0, 95))
for i, angle in enumerate(angles):
    axes[1].text(i, angle+1, f'{angle:.2f}°', ha='center', fontsize=8)
axes[1].legend(fontsize=8)
dest = Path(__file__).resolve().parent / 'results'
dest.mkdir(exist_ok=True)
fig.savefig(dest / 'export_measurements.png', dpi=160)
