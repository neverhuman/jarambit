#!/usr/bin/env python3
"""Native-only bounded ring legality preflight; preserve every failed probe."""
import csv
import json
import os
import subprocess
from prepare_phase5 import HERE, ROOT, digest
from run_phase5 import environment
OUT=ROOT/'target/G16-06-RSV2-08/phase7'

def main():
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    base=list(csv.reader((HERE/'phase5_sources/q7.csv').open()))
    directory=OUT/'geometry';directory.mkdir();records=[];selected=[]
    def probe(ring,dx,drop):
        name=f'r{ring:.2f}_n{len(records):03d}';path=directory/f'{name}.csv';out=directory/f'{name}.json'
        data=[list(v) for v in base]
        for i,v in [(40,drop),(41,dx),(42,ring)]:
            if v!=float(data[1][i]):data[1][i]=str(v)
        with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
        with out.with_suffix('.log').open('w') as log:
            subprocess.run([str(OUT/'native_adapter'),'geometry',str(out),str(path)],env=environment(),stdout=log,stderr=log,check=True)
        d=json.loads(out.read_text())
        row=dict(name=name,ring_wall=ring,grip_dx=dx,drop=drop,cp13=float(base[1][13]),
            legal=bool(d['input_was_canonical'] and d['rs_v2'] and d['aes_pass'] and not d['rejects']),
            grip_len_mm=d['grip_len_mm'],holder_fit=d['holder_fit'],rejects=d['rejects'],aes_fails=d['aes_fails'],
            genome=str(path.relative_to(ROOT)),genome_sha256=digest(path),native_output=str(out.relative_to(ROOT)),native_output_sha256=digest(out))
        records.append(row)
        (OUT/'geometry_controls.json').write_text(json.dumps(dict(state='running',records=records,selected=selected,bounds=d['bounds']),indent=2)+'\n')
        print(row,flush=True)
        return row,d
    for ring in [6.2,6.4,6.6,6.8]:
        first,doc=probe(ring,-.4,0.)
        winner=first if first['legal'] else None
        if winner is None:
            dx=max(-.4,-.4+60.01-first['grip_len_mm'])
            for drop in [0.,-.35,-.30,-.25,-.40,.25,.35,.4,.5]:
                row,doc=probe(ring,dx,drop)
                if row['legal']:winner=row;break
        assert winner is not None,ring
        selected.append(winner)
    (OUT/'geometry_controls.json').write_text(json.dumps(dict(state='complete',records=records,selected=selected,bounds=doc['bounds'],all_four_ring_levels_legal=True,all_CPs_byte_preserved=True),indent=2)+'\n')

if __name__=='__main__':main()
