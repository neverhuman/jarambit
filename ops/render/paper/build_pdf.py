#!/usr/bin/env python3
"""Build the two existing papers with Pandoc/LuaLaTeX into lane-local target/paper.

Requires pandoc, texlive-luatex/latex-extra, fonts-texgyre, Pillow.
No PDF binary is committed. Every build writes source/output hashes and tool versions.
"""
import argparse
import hashlib
import io
import json
import os
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import quote

ROOT=Path(__file__).resolve().parents[3]
HERE=Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def capture(path, inputs):
    raw=path.read_bytes()
    relative=str(path.relative_to(ROOT));digest=hashlib.sha256(raw).hexdigest()
    if relative in inputs and inputs[relative]!=digest:
        raise ValueError('Input changed between PDF references: '+relative)
    inputs[relative]=digest
    return raw


def stage(path, raw, renderer_inputs):
    renderer_inputs[str(path.relative_to(ROOT))]=hashlib.sha256(raw).hexdigest()
    path.write_bytes(raw)


def build(name):
    out=ROOT/'target/paper';out.mkdir(parents=True,exist_ok=True)
    head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
    inputs={};renderer_inputs={}
    source=ROOT/'docs'/f'{name}.md';text=capture(source,inputs).decode('utf-8')
    source_rel=str(source.relative_to(ROOT))
    commit_epoch=int(subprocess.check_output(['git','show','-s','--format=%ct',head],cwd=ROOT,text=True))
    epoch=int(os.environ.get('SOURCE_DATE_EPOCH',commit_epoch))
    version_path=ROOT/'docs/VERSION';version=capture(version_path,inputs).decode('utf-8').strip()
    if not re.fullmatch(r'[0-9A-Za-z.-]+',version):raise ValueError('invalid version')
    trailer_id=hashlib.sha256(f'{head}:{name}:{version}:{epoch}:{inputs[source_rel]}'.encode('utf-8')).hexdigest()[:32].upper()
    version_header=out/'version.tex'
    version_tex=f'\\newcommand{{\\PaperVersion}}{{{version}}}\n\\newcommand{{\\PaperTrailerID}}{{{trailer_id}}}\n'
    stage(version_header,version_tex.encode('utf-8'),renderer_inputs)
    title=text.splitlines()[0].removeprefix('# ')
    text=text.split('\n',1)[1].replace('**Jepson Taylor**\n','',1)
    if name=='WHITEPAPER':
        text=text.split('## Appendix A — prior draft')[0]
        text+='\n## Historical source archive\n\nThe prior draft remains in `docs/WHITEPAPER.md`, Appendix A, for traceability. It is excluded from this publication rendering because its earlier model and requirements are historical.\n'
    def image(match):
        path=(source.parent/match.group(2)).resolve()
        if not path.is_file():raise FileNotFoundError(path)
        raw=capture(path,inputs);digest=inputs[str(path.relative_to(ROOT))]
        manifest=Path(str(path)+'.manifest.json')
        if manifest.is_file():capture(manifest,inputs)
        staged_image=out/'images'/(path.stem+'-'+digest[:12]+('.png' if path.suffix=='.webp' else path.suffix))
        staged_image.parent.mkdir(exist_ok=True)
        if path.suffix=='.webp':
            from PIL import Image
            encoded=io.BytesIO()
            with Image.open(io.BytesIO(raw)) as im:im.convert('RGB').save(encoded,format='PNG',compress_level=9)
            raw=encoded.getvalue()
        stage(staged_image,raw,renderer_inputs)
        return f'![{match.group(1)}]({staged_image})'
    text=re.sub(r'!\[([^\]]*)\]\(([^)]+)\)',image,text)
    def link(match):
        target=match.group(2)
        if '://' in target or target.startswith('#'):return match.group(0)
        path,sep,fragment=target.partition('#');local=(source.parent/path).resolve()
        relative=local.relative_to(ROOT)
        kind='tree' if local.is_dir() else 'blob'
        url=f'https://github.com/neverhuman/jarambit/{kind}/{head}/{quote(str(relative),safe="/")}'
        return f'[{match.group(1)}]({url}{sep}{fragment})'
    text=re.sub(r'(?<!!)\[([^\]]+)\]\(([^)]+)\)',link,text)
    staged=out/f'{name}.md';stage(staged,text.encode('utf-8'),renderer_inputs)
    pdf=out/f'{name}.pdf'
    capture(Path(__file__).resolve(),inputs)
    support={}
    for path in (HERE/'publication.lua',HERE/'header.tex'):
        raw=capture(path,inputs)
        snapshot=out/(path.stem+'-'+hashlib.sha256(raw).hexdigest()[:12]+path.suffix)
        stage(snapshot,raw,renderer_inputs);support[path.name]=snapshot
    # The source already supplies captions; inline placement keeps them with their image.
    cmd=['pandoc',str(staged),'--from=markdown+tex_math_dollars-implicit_figures','--pdf-engine=lualatex',
         '--resource-path',str(ROOT/'docs'),'-V','documentclass=article','-V','fontsize=10pt',
         '-V','geometry:margin=22mm','-V','mainfont=TeX Gyre Pagella',
         '-V','sansfont=TeX Gyre Heros','-V','monofont=DejaVu Sans Mono',
         '-V','colorlinks=true','--metadata',f'title={title}','--metadata','author=Jepson Taylor',
         '--metadata','date='+datetime.fromtimestamp(epoch,timezone.utc).strftime('%d %B %Y')+f' · Version {version} (corrected certification)',
         '--standalone','--lua-filter',str(support['publication.lua']),
         '--include-in-header',str(version_header),'--include-in-header',str(support['header.tex']),'-o',str(pdf)]
    env=dict(os.environ,TEXMFOUTPUT=str(out),SOURCE_DATE_EPOCH=str(epoch),FORCE_SOURCE_DATE='1',TZ='UTC',LC_ALL='C.UTF-8')
    with (out/f'{name}.build.log').open('w') as log:
        run=subprocess.run(['nice','-n','10']+cmd,cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT)
    if run.returncode:raise RuntimeError(f'PDF build failed; see {out/name}.build.log')
    record={'schema':1,'source_commit':head,'source':str(source.relative_to(ROOT)),
            'source_sha256':inputs[str(source.relative_to(ROOT))],
            'inputs':inputs,'renderer_inputs':renderer_inputs,
            'output':str(pdf.relative_to(ROOT)),'output_sha256':sha(pdf),
            'pandoc':subprocess.check_output(['pandoc','--version'],text=True).splitlines()[0],
            'latex':subprocess.check_output(['lualatex','--version'],text=True).splitlines()[0],
            'command':cmd,'scope':'Publication render only; no new engineering solves or certification'}
    if subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()!=head:
        raise ValueError('Checkout changed during PDF build')
    for relative,expected in inputs.items():
        if sha(ROOT/relative)!=expected:
            raise ValueError('Input changed during PDF build: '+relative)
    for relative,expected in renderer_inputs.items():
        if sha(ROOT/relative)!=expected:
            raise ValueError('Renderer input changed during PDF build: '+relative)
    (out/f'{name}.pdf.manifest.json').write_text(json.dumps(record,indent=1,sort_keys=True)+'\n')
    print(pdf.relative_to(ROOT),pdf.stat().st_size,flush=True)


def main():
    p=argparse.ArgumentParser();p.add_argument('--document',choices=['WHITEPAPER','ENGINEERING_GATES_AND_EVOLUTION']);args=p.parse_args()
    for name in [args.document] if args.document else ['WHITEPAPER','ENGINEERING_GATES_AND_EVOLUTION']:build(name)


if __name__=='__main__':main()
