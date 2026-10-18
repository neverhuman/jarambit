#!/usr/bin/env python3
from run import *
jobs=[]
for y in (1.,2.,3.,4.):
 for bow in (0.,2.):
  changes={44:8.,55:y,54:bow,52:7.,53:6.,42:6.4,**{k:3. for k in range(14,19)},**{k:3. for k in range(23,28)}}
  jobs.append((f'fine_neck_r6.4_y{y}_bow{bow}',changes,{'STUDY_NECK_FULL':'1','STUDY_AREAS':'.25'},'fem'))
with ThreadPoolExecutor(max_workers=2) as pool:results=list(pool.map(lambda j:run(*j),jobs))
(OUT/'fine_neck_summary.json').write_text(json.dumps(results,indent=2)+'\n')
