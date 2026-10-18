from run import OUT
import json,math
load=lambda p:json.loads(p.read_text())
refs={'neck_control':load(OUT/'inputs/neck/target/G16-06-RSV2-08/candidate_validation.json'),'hotspot_control':load(OUT/'inputs/hotspot/target/rsv2-07/v3/hotspot_strict_3/result.json')}
result={}
for name,ref in refs.items():
 local=load(OUT/(name+'_fem.json'));errors=[]
 assert local['input_was_canonical'] and local['within_genome_domain'] and local['rs_v2'] and local['aes_pass'] and not local['rejects']
 for l in local['levels']:
  r=next(x for x in ref['levels'] if x['area']==l['area'])
  for k,v in r['metrics'].items():
   if isinstance(v,(float,int)):
    actual=l['metrics'][k];errors.append(abs(actual-v)/max(abs(v),1e-30))
 assert max(errors)<1e-10,(name,max(errors))
 result[name]={'all_reported_scalar_metrics_match':True,'max_relative_error':max(errors),'areas':[l['area'] for l in local['levels']]}
(OUT/'control_verification.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
