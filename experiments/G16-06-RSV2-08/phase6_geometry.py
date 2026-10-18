#!/usr/bin/env python3
"""Find native-legal ring controls without changing pinned/hand CPs or bounds."""
from pathlib import Path
import csv
import json
import os
import subprocess
from run_phase5 import environment, verify_provenance, milestone as old_milestone
from prepare_phase5 import HERE, ROOT, digest

OUT=ROOT/'target/G16-06-RSV2-08/phase6'
BASE=HERE/'phase5_sources/q7.csv'

def main():
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    assert os.environ['CARGO_BUILD_JOBS']=='2'
    directory=OUT/'geometry';assert not directory.exists();directory.mkdir()
    base=list(csv.reader(BASE.open()));records=[];selected=[]
    def probe(ring,dx,drop):
        name=f'r{ring:.2f}_dx{dx:.9f}_drop{drop:.3f}'
        csvpath=directory/f'{name}.csv';out=directory/f'{name}.json'
        data=[list(v) for v in base]
        for i,v in [(40,drop),(41,dx),(42,ring)]:
            if float(data[1][i])!=v:data[1][i]=str(v)
        with csvpath.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
        with out.with_suffix('.log').open('w') as log:
            subprocess.run([str(OUT/'native_adapter'),'geometry',str(out),str(csvpath)],
                           cwd=ROOT,env=environment(),stdout=log,stderr=log,check=True)
        d=json.loads(out.read_text());verify_provenance(d)
        assert d['genes'][6:9]==[float(x) for x in base[1][6:9]]
        assert d['genes'][19:22]==[0.,0.,0.]
        legal=d['input_was_canonical'] and d['rs_v2'] and d['aes_pass'] and not d['rejects']
        row=dict(name=name,ring_wall=ring,grip_dx=dx,drop=drop,legal=bool(legal),grip_len_mm=d['grip_len_mm'],
                 holder_fit=d['holder_fit'],inside_concave=d['inside_concave'],spine_convex=d['spine_convex'],
                 rejects=d['rejects'],aes_fails=d['aes_fails'],genome=str(csvpath.relative_to(ROOT)),
                 genome_sha256=digest(csvpath),native_output=str(out.relative_to(ROOT)),native_output_sha256=digest(out))
        records.append(row)
        (OUT/'geometry_receipt.json').write_text(json.dumps(dict(state='running',records=records,selected=selected,bounds=d['bounds']),indent=2)+'\n')
        print(name,'legal',legal,'grip',d['grip_len_mm'],'box',d['holder_fit'],'rejects',d['rejects'],d['aes_fails'],flush=True)
        return row,d
    for ring in [6.4,6.65,6.85,7.]:
        dx=-.4+1.1*(ring-6.4)
        first,native=probe(ring,dx,0.)
        winner=first if first['legal'] else None
        if winner is None:
            options=[max(-.4,dx+target-first['grip_len_mm']) for target in [60.01,60.10,60.25]]+[dx]
            seen={(round(dx,9),0.)}
            for candidate_dx in options:
                for drop in [0.,.25,.5,.75,1.,1.25,1.5,1.75,2.,2.5,3.,3.5,4.,-.25,-.5,-1.,-2.,-4.]:
                    key=(round(candidate_dx,9),drop)
                    if key in seen:continue
                    seen.add(key);row,_=probe(ring,candidate_dx,drop)
                    if row['legal']:winner=row;break
                if winner is not None:break
        if winner is None:
            (OUT/'geometry_receipt.json').write_text(json.dumps(dict(state='no_legal_control_in_bounded_search',records=records,selected=selected,bounds=native['bounds']),indent=2)+'\n')
            raise RuntimeError(f'No legal ring control in bounded native DROP/GRIP_DX search: {ring}')
        selected.append(winner)
    (OUT/'geometry_receipt.json').write_text(json.dumps(dict(state='complete',records=records,selected=selected,bounds=native['bounds'],cp06_08_preserved=True,pinned_cp19_21_preserved=True),indent=2)+'\n')
    print('ALL FOUR RING LEVELS NATIVE-LEGAL',flush=True)

if __name__=='__main__':main()
