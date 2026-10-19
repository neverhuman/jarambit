#!/usr/bin/env python3
"""Render all 16 gate plates and three studio views of one assessed genome.

python3 ops/render/blender/rerender.py GENOME_ID
Default admission requires a qualified champion and every input committed on
origin/main. --preview explicitly permits failed/runtime candidates outside docs/.
No export, evaluator, search, record, or certification is run or changed.
"""
import argparse
import ast
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT/'ops/wave_figures'))
import figlib


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare(genome_id, leaderboard, source_root, preview=False):
    from ops.ci.evaluate_cad_candidate import validate_solid_artifacts
    source_root = source_root.resolve()
    if not preview and source_root != ROOT:
        raise ValueError('Final renders must use the committed inputs in this worktree')
    board = json.loads(leaderboard.read_text())
    rows = [r for r in board['designs'] if r['label'] == genome_id]
    if len(rows) != 1:raise ValueError('Genome ID must identify exactly one assessed row')
    row = rows[0]
    if board['schema'] != 'jarambit.auto_evolve.leaderboard.v2' or board['assessed_count'] != len(board['designs']):
        raise ValueError('Invalid automatic-wave leaderboard')
    relative_genome = Path(row['genome_csv'])
    # Captures may retain the original out/WAVE/... spelling while committing
    # the complete by-content tree beside the captured leaderboard.
    if 'by-content' in relative_genome.parts:
        tail = relative_genome.parts[relative_genome.parts.index('by-content'):]
        beside_board = leaderboard.parent.joinpath(*tail)
    else:beside_board = source_root/relative_genome
    genome = (beside_board if beside_board.is_file() else source_root/relative_genome).resolve()
    genome.relative_to(source_root)
    folder = genome.parent
    suffixes = dict(genome='_genome.csv', stl='.stl', step='.step', d1='_D1.json',
        scorecard='_master_scorecard.json', rs='_rs.json', solid_tip='_solid_tip.json',
        mesh='_solid_mesh.npz', skfem_raw='_skfem_raw.json', dolfinx_raw='_dolfinx_raw.json',
        dolfinx_sidecar='_dolfinx_sidecar.json', runtime='_runtime.json', surface='_surface.json',
        step_check='_step_check.json', plate_gci='_plate_gci.json')
    assets = {k:folder/(genome_id+suffix) for k, suffix in suffixes.items()}
    assets['leaderboard'] = leaderboard.resolve()
    if assets['genome'] != genome:raise ValueError('Genome label/path mismatch')
    for path in assets.values():
        path.relative_to(source_root)
        if not path.is_file():raise ValueError('Missing render/certificate input: '+str(path))
    docs = {k:json.loads(assets[k].read_text()) for k in ('scorecard', 'rs', 'solid_tip', 'd1')}
    identity = row['certificate_identity']
    if any(d.get('certificate_identity') != identity for d in docs.values()):
        raise ValueError('Assessment identity mismatch')
    solid = docs['solid_tip'];score = docs['scorecard'];rs = docs['rs']
    for key, name in [('genome_sha256','genome'), ('d1_sha256','d1'), ('source_stl_sha256','stl'),
                       ('source_step_sha256','step'), ('source_surface_sha256','surface')]:
        if solid.get(key) != sha(assets[name]):raise ValueError('Certificate input hash mismatch: '+name)
    if score['genome_sha256'] != sha(genome) or not validate_solid_artifacts(solid, folder, genome_id):
        raise ValueError('Raw independent observations do not validate this assessment')
    if solid.get('copied_from_another_genome') is not False or solid.get('genome_data_row_matches_csv') is not True:
        raise ValueError('Copied or unbound genome observation')
    sidecar = json.loads(assets['dolfinx_sidecar'].read_text())
    if (set(sidecar['inputs'].values()) != {sha(assets['mesh'])} or
        sidecar['legacy_stdout'] != json.loads(assets['dolfinx_raw'].read_text()) or
        sidecar['solver']['ksp_reason'] <= 0 or sidecar['solver']['ksp_its'] <= 0):
        raise ValueError('Independent solver sidecar is not bound to this mesh/observation')
    gates = score['weighted_cad_score']['gates']
    if len(gates) != 16 or any(type(v) is not bool for v in gates.values()):
        raise ValueError('Expected 16 boolean engineering verdicts')
    if (row['all_16_gates'] != all(gates.values()) or
        row['all_16_gates'] != score['weighted_cad_score']['all_engineering_gates_pass'] or
        row['rs_v2_all_hard'] != rs['all_hard']):
        raise ValueError('Leaderboard and certificate verdicts disagree')
    if board['unverified'] or rs['unverified']:
        raise ValueError('Unverified assessments are not admitted')
    qualified = row['all_16_gates'] and row['rs_v2_all_hard']
    if row['qualified'] is not qualified:raise ValueError('Qualification flags disagree')
    if not preview:
        if not qualified or board['champion'] != row:
            raise ValueError('NOT-A-WINNER: final mode requires the qualified automated champion; use --preview')
        if board['qualified_count'] != sum(d['qualified'] for d in board['designs']):
            raise ValueError('Qualified count mismatch')
        for path in assets.values():
            rel = path.relative_to(source_root).as_posix()
            committed = subprocess.run(['git', 'show', 'origin/main:'+rel], cwd=source_root, capture_output=True)
            if committed.returncode or committed.stdout != path.read_bytes():
                raise ValueError('Final render input not committed unchanged on origin/main: '+rel)
        subprocess.run(['git','merge-base','--is-ancestor',score['source_commit'],'origin/main'],cwd=source_root,check=True)
    status = 'NOT-A-WINNER · LOCAL PREVIEW · NOT FOR PAPER' if preview else 'CERTIFIED AUTOMATED CHAMPION · COMMITTED ON MAIN'
    return dict(schema=1, label=genome_id, certificate_identity=identity, preview=preview,
        status=status, rs_v2_all_hard=rs['all_hard'], all_16_gates=row['all_16_gates'],
        source_commit=score['source_commit'], gates=gates,
        artifacts={k:str(p) for k,p in assets.items()}, input_sha256={str(p):sha(p) for p in assets.values()})


def publication(args, parser):
    """Isolated, lossless bpy rendering; leave the archived atlas untouched."""
    if args.genome_id is not None or args.preview or args.leaderboard or args.output_dir:
        parser.error('publication profile uses only the frozen assessed-example inputs')
    if args.scene not in (None,'hero_oblique'):
        parser.error('publication profile currently admits only hero_oblique')
    if not args.output_root:
        parser.error('publication rendering requires --output-root')
    out=Path(args.output_root).resolve()
    allowed=[(ROOT/'docs/paper/figures/v2').resolve(),(ROOT/'target/paper/blender-v2').resolve()]
    if not any(out==base or out.is_relative_to(base) for base in allowed):
        parser.error('publication output must be inside docs/paper/figures/v2 or target/paper/blender-v2')
    if not (128<=args.width<=4800 and 128<=args.height<=4800 and 1<=args.samples<=1024):
        parser.error('publication dimensions/samples exceed the bounded render profile')
    spec_path=ROOT/'docs/paper/figure-inputs.v1.json';spec=json.loads(spec_path.read_text())
    for path,digest in spec['inputs'].items():
        if hashlib.sha256((ROOT/path).read_bytes()).hexdigest()!=digest:
            raise ValueError('publication input changed: '+path)
    proof=json.loads((ROOT/'evidence/evolve_certifiable/manifest.json').read_text())['automatic']
    board=json.loads((ROOT/proof['artifacts']['leaderboard']).read_text())
    rs=json.loads((ROOT/proof['artifacts']['rs']).read_text())
    if proof['label']!=spec['subject'] or proof['certificate_identity']!=spec['certificate_identity']:
        raise ValueError('publication subject identity differs')
    if board['champion'] is not None or board['qualified_count']!=0 or rs['all_hard']:
        raise ValueError('publication assessed-example status requires editorial readmission')
    out.mkdir(parents=True,exist_ok=True)
    name='hero_'+args.recipe;figure=out/(name+'.png')
    script=ROOT/'ops/render/blender/candidate_scenes.py'
    bound=[spec_path,script,ROOT/'ops/render/blender/paper_scenes.py',Path(__file__).resolve()]+[ROOT/p for p in spec['inputs']]
    input_hashes={str(p.relative_to(ROOT)):sha(p) for p in bound}
    blender=os.environ.get('BLENDER','blender')
    version=subprocess.check_output([blender,'--version'],text=True).splitlines()[0]
    cmd=[blender,'-b','-t','4','--python-exit-code','1','-P',str(script),'--',
         '--scene','hero_oblique','--profile','publication','--recipe',args.recipe,
         '--output',str(figure),'--samples',str(args.samples),'--width',str(args.width),'--height',str(args.height)]
    logdir=ROOT/'target/paper/blender-v2';logdir.mkdir(parents=True,exist_ok=True)
    log_path=logdir/f'{name}-{args.width}x{args.height}-{args.samples}.log'
    with log_path.open('w') as log:
        subprocess.run(cmd,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=1800)
    scene_path=figure.with_suffix('.scene.json');scene=json.loads(scene_path.read_text())
    if scene['geometry_changed'] or scene['stl_sha256']!=spec['geometry']['sha256']:
        raise ValueError('rendered geometry identity differs')
    for path,digest in input_hashes.items():
        if sha(ROOT/path)!=digest:raise ValueError('Input changed during publication render: '+path)
    receipt={'schema':'jarambit.publication_render.v1','subject':spec['subject'],
             'figure':str(figure.relative_to(ROOT)),'figure_sha256':sha(figure),
             'script':str(script.relative_to(ROOT)),
             'certificate_identity':spec['certificate_identity'],'status':spec['status'],
             'command':cmd,'blender':version,'scene':scene_path.name,
             'inputs':input_hashes,
             'outputs':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [figure,scene_path]},
             'scope':'Presentation only; exact assessed geometry; no score or field generated'}
    figure.with_suffix('.manifest.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(figure,figure.stat().st_size,flush=True)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('genome_id',nargs='?');p.add_argument('--leaderboard',type=Path)
    p.add_argument('--source-root',type=Path,default=ROOT)
    p.add_argument('--preview',action='store_true');p.add_argument('--output-dir',type=Path)
    p.add_argument('--scene');p.add_argument('--samples',type=int,default=48)
    p.add_argument('--plan-only',action='store_true')
    p.add_argument('--profile',choices=['legacy','publication'],default='legacy')
    p.add_argument('--output-root');p.add_argument('--recipe',choices=['neutral','dark','editorial'],default='neutral')
    p.add_argument('--width',type=int,default=1600);p.add_argument('--height',type=int,default=1000)
    args = p.parse_args()
    if args.profile=='publication':publication(args,p);return
    if not args.genome_id:p.error('genome_id is required for the legacy assessment renderer')
    if args.samples < 1:p.error('samples must be positive')
    board = args.leaderboard
    if board is None:
        names = subprocess.check_output(['git','ls-tree','-r','--name-only','origin/main'],cwd=args.source_root,text=True).splitlines()
        matches = [args.source_root/n for n in names if n.endswith('leaderboard.json') and
            any(r['label']==args.genome_id for r in json.loads((args.source_root/n).read_text())['designs'])]
        if len(matches) != 1:p.error('Specify --leaderboard to disambiguate the assessed genome')
        board = matches[0]
    elif not board.is_absolute():board = args.source_root/board
    try:context = prepare(args.genome_id, board, args.source_root, args.preview)
    except (ValueError, KeyError) as error:p.exit(1,str(error)+'\n')
    out = (args.output_dir or (ROOT/('target/paper/rerender-preview' if args.preview else 'docs/figures/whitepaper/winner')/args.genome_id)).resolve()
    if args.preview and any(out.is_relative_to(p/'docs') for p in (ROOT, args.source_root.resolve())):
        p.error('Preview output must stay outside the paper')
    script = ROOT/'ops/render/blender/candidate_scenes.py'
    library = ROOT/'ops/render/blender/paper_scenes.py'
    tree = ast.parse(library.read_text())
    scenes = next(ast.literal_eval(n.value) for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='SCENES' for t in n.targets))
    names = [args.scene] if args.scene else list(scenes)
    if any(n not in scenes for n in names):p.error('Unknown scene')
    if args.plan_only:
        print(json.dumps(dict(context, output_dir=str(out), scenes=names, samples=args.samples),indent=2));return
    out.mkdir(parents=True,exist_ok=True)
    config = out/'render_context.json';config.write_text(json.dumps(context,indent=1,sort_keys=True)+'\n')
    logdir = out/'raw';logdir.mkdir(exist_ok=True)
    # The distribution's 4.0.2 build lacks OpenImageDenoiser. Reuse the official
    # build already installed for the retained atlas; BLENDER can select another.
    installed = Path.home()/'opt/blender-4.5.3-linux-x64/blender'
    blender = os.environ.get('BLENDER',str(installed) if installed.is_file() else 'blender')
    version = subprocess.check_output([blender,'--version'],text=True).splitlines()[0]
    env = dict(os.environ,JARAMBIT_RENDER_CONTEXT=str(config))
    font = Path('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf')
    inputs = [Path(s) for s in context['input_sha256']] + [script, library, Path(__file__).resolve(), config]
    for name in names:
        raw = logdir/(name+'.png');figure = out/(name+'.webp')
        cmd = ['nice','-n','10',str(Path.home()/'bin/memcap'),'32G','--',blender,'-b','-t','4','-P',str(script),'--',
               '--scene',name,'--output',str(raw),'--samples',str(args.samples)]
        with (logdir/(name+'.log')).open('w') as log:
            subprocess.run(cmd,cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
        # Refuse publication if a live runtime artifact changed during rendering.
        for source, digest in context['input_sha256'].items():
            if sha(Path(source)) != digest:raise ValueError('Input changed during render: '+source)
        captions = json.loads(raw.with_suffix('.labels.json').read_text())
        with Image.open(raw) as source:
            im=source.convert('RGB');draw=ImageDraw.Draw(im);scale=im.width/.285
            for caption in captions:
                face=ImageFont.truetype(str(font),round(caption['size']*scale*1.25))
                draw.text((im.width/2+caption['x']*scale,im.height/2-caption['y']*scale),
                    caption['text'],font=face,fill=tuple(caption['rgb']),anchor='ls')
            im.save(figure,format='WEBP',quality=92,method=6)
        figlib.write_manifest(str(figure), inputs, str(script), extra=dict(
            subject=context['label'],certificate_identity=context['certificate_identity'],
            preview=args.preview,status=context['status'],scene=name,gate_verdict=context['gates'].get(name),
            rs_v2_all_hard=context['rs_v2_all_hard'],all_16_gates=context['all_16_gates'],
            source_commit=context['source_commit'],blender=version,engine='Cycles CPU',threads=4,
            samples=args.samples,denoise=True,seed=42,captions=captions,font_sha256=sha(font),
            geometry_frame_sha256=sha(raw),
            field='Retained D1 multi-load plate envelope; no new solve or load-specific solid stress field' if scenes[name][2] in ('lateral','pull','sequence') else 'Illustrative finish, fixtures and loads; no stress field',
            command=' '.join([sys.executable,*sys.argv]),
            geometry='Exact assessed STL; no remodeled geometry; section view clips at z=0'))
        print(name, context['status'], figure.stat().st_size, flush=True)


if __name__ == '__main__':main()
