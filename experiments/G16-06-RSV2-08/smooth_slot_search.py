#!/usr/bin/env python3
from run import *
jobs=[]
for y in (1.,2.,3.):
 for w1 in (3.5,6.):
  changes={44:8.,55:y,54:0.,52:7.,53:w1,**{k:3. for k in range(14,19)},**{k:3. for k in range(23,27)}}
  jobs.append((f'smooth_y{y}_w1{w1}',changes,{'STUDY_NECK_FULL':'1','STUDY_SLOT_SMOOTH':'1'}))
with ThreadPoolExecutor(max_workers=2) as pool:results=list(pool.map(lambda j:run(*j),jobs))
(OUT/'smooth_slot_summary.json').write_text(json.dumps(results,indent=2)+'\n')
