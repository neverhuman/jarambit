#!/usr/bin/env python3
from run import *
import math
rows=list(csv.reader((HERE/'short_slot.csv').open())); seed=[float(x) for x in rows[1][:-1]]
jobs=[]
base={'STUDY_NECK_FULL':'1'}
for y in (.5,2.,4.):
 for fillet in (8.,13.,18.):
  for width in (0.,1.5,3.):
   changes={55:y,44:fillet,**{k:min(3.,seed[k]+width) for k in range(14,19)},**{k:min(3.,seed[k]+width) for k in range(23,27)}}
   jobs.append((f'y{y}_f{fillet}_w{width}',changes,base))
with ThreadPoolExecutor(max_workers=2) as pool:
 results=list(pool.map(lambda j:run(*j),jobs))
(OUT/'coordinate_summary.json').write_text(json.dumps(results,indent=2)+'\n')
