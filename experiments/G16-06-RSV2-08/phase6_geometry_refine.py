#!/usr/bin/env python3
"""Refine legal ring controls using native DROP and the grip endpoint CP13."""
import csv
import json
import os
import subprocess
from phase6_geometry import HERE, ROOT, OUT, BASE, environment, verify_provenance, digest

if __name__=='__main__':
    assert os.sched_getaffinity(0)=={12,13} and os.getpriority(os.PRIO_PROCESS,0)>=10
    directory=OUT/'geometry_refinement';assert not directory.exists();directory.mkdir()
    base=list(csv.reader(BASE.open()));records=[];selected=[]
    def probe(ring,dx,drop,cp13):
        name=f'r{ring:.2f}_n{len(records):03d}'
        path=directory/f'{name}.csv';output=directory/f'{name}.json'
        data=[list(v) for v in base]
        for i,v in [(13,cp13),(40,drop),(41,dx),(42,ring)]:
            if v!=float(data[1][i]):data[1][i]=str(v)
        with path.open('w') as f:csv.writer(f,lineterminator='\n').writerows(data)
        with output.with_suffix('.log').open('w') as log:
            subprocess.run([str(OUT/'native_adapter'),'geometry',str(output),str(path)],cwd=ROOT,
                           env=environment(),stdout=log,stderr=log,check=True)
        doc=json.loads(output.read_text());verify_provenance(doc)
        assert doc['genes'][6:9]==[float(x) for x in base[1][6:9]]
        assert doc['genes'][19:22]==[0.,0.,0.]
        row=dict(name=name,ring_wall=ring,grip_dx=dx,drop=drop,cp13=cp13,
                 legal=bool(doc['input_was_canonical'] and doc['rs_v2'] and doc['aes_pass'] and not doc['rejects']),
                 grip_len_mm=doc['grip_len_mm'],holder_fit=doc['holder_fit'],
                 inside_concave=doc['inside_concave'],spine_convex=doc['spine_convex'],
                 rejects=doc['rejects'],aes_fails=doc['aes_fails'],genome=str(path.relative_to(ROOT)),genome_sha256=digest(path),
                 native_output=str(output.relative_to(ROOT)),native_output_sha256=digest(output))
        records.append(row)
        (OUT/'geometry_controls.json').write_text(json.dumps(dict(state='running',records=records,selected=selected,bounds=doc['bounds']),indent=2)+'\n')
        print(name,'dx/drop/cp13',dx,drop,cp13,'legal',row['legal'],'box',row['holder_fit'],'rejects',row['rejects'],row['aes_fails'],flush=True)
        return row,doc
    for ring in [6.4,6.65,6.85,7.]:
        cp13=float(base[1][13]);dx=max(-.4,-.4+(ring-6.4)-.16251305300084+.01)
        if ring==6.4:dx=-.4
        winner=None
        drops=[0.] if ring<=6.65 else [-.35,-.34,-.36,-.4,-.45,-.5,.4,.425,.45,.475,.5]
        if ring==7.:drops=[-.45,-.4,-.5,-.55,-.35,0.,.4,.45,.5]
        offsets=[0.] if ring<=6.65 else [0.,-.25,-.5,-.75,-1.,-1.1,-1.25,-1.5]
        for delta in offsets:
            c13=cp13+delta
            # CP13 is the grip endpoint used by native grip_len; its shift permits
            # GRIP_DX to stay at its native floor while preserving grip >=60.01.
            candidate,_=probe(ring,-.4,0.,c13)
            candidate_dx=max(-.4,-.4+60.01-candidate['grip_len_mm'])
            if candidate['legal']:winner=candidate;break
            for drop in drops:
                row,native=probe(ring,candidate_dx,drop,c13)
                if row['legal']:winner=row;break
            if winner:break
        if winner is None:raise RuntimeError(f'No native-legal refined ring control: {ring}')
        selected.append(winner)
    (OUT/'geometry_controls.json').write_text(json.dumps(dict(state='complete',records=records,selected=selected,bounds=native['bounds'],
         cp06_08_byte_preserved=True,pinned_cp19_21_preserved=True,all_four_ring_levels_legal=True),indent=2)+'\n')
    print('ALL FOUR RING LEVELS NATIVE-LEGAL',flush=True)
