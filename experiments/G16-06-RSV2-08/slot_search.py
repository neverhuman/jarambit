#!/usr/bin/env python3
from run import *
jobs=[]
for w0 in (7.,8.5,10.,12.):
 for w1 in (3.5,5.,6.):
  for x1 in (58.,70.):
   changes={44:18.,55:4.,51:x1,52:w0,53:w1,**{k:3. for k in range(14,19)},**{k:3. for k in range(23,27)}}
   jobs.append((f'slot_w{w0}_{w1}_end{x1}',changes,{'STUDY_NECK_FULL':'1'}))
with ThreadPoolExecutor(max_workers=2) as pool:
 results=list(pool.map(lambda j:run(*j),jobs))
(OUT/'slot_summary.json').write_text(json.dumps(results,indent=2)+'\n')
