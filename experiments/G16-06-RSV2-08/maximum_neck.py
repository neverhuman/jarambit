#!/usr/bin/env python3
from run import *
rows=list(csv.reader((HERE/'short_slot.csv').open())); seed=[float(x) for x in rows[1][:-1]]
jobs=[]
for y in (.5,2.,4.):
 for fillet in (8.,13.,18.):
  for top in (1.5,3.):
   changes={55:y,44:fillet,**{k:3. for k in range(14,19)},**{k:top for k in range(23,27)}}
   jobs.append((f'max_y{y}_f{fillet}_top{top}',changes,{'STUDY_NECK_FULL':'1'}))
with ThreadPoolExecutor(max_workers=2) as pool:
 results=list(pool.map(lambda j:run(*j),jobs))
(OUT/'maximum_neck_summary.json').write_text(json.dumps(results,indent=2)+'\n')
