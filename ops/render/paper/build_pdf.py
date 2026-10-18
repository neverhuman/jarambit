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
import shutil
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import quote

ROOT=Path(__file__).resolve().parents[3]
HERE=Path(__file__).resolve().parent
PINNED_PANDOC_VERSION='pandoc 3.1.3'
PINNED_LUALATEX_VERSION='This is LuaHBTeX, Version 1.17.0 (TeX Live 2023/Debian)'


def check_toolchain():
    try:
        pandoc_ver=subprocess.check_output(['pandoc','--version'],text=True).splitlines()[0].strip()
        latex_ver=subprocess.check_output(['lualatex','--version'],text=True).splitlines()[0].strip()
    except (FileNotFoundError,subprocess.CalledProcessError,IndexError) as exc:
        raise RuntimeError(f'Required PDF toolchain missing or failed ({PINNED_PANDOC_VERSION}; {PINNED_LUALATEX_VERSION})') from exc
    if pandoc_ver!=PINNED_PANDOC_VERSION:
        raise RuntimeError(f'Unpinned pandoc version: got {pandoc_ver!r}, expected {PINNED_PANDOC_VERSION!r}')
    if latex_ver!=PINNED_LUALATEX_VERSION:
        raise RuntimeError(f'Unpinned lualatex version: got {latex_ver!r}, expected {PINNED_LUALATEX_VERSION!r}')
    return pandoc_ver,latex_ver


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def capture(path, inputs):
    raw=path.read_bytes()
    relative=str(path.relative_to(ROOT));digest=hashlib.sha256(raw).hexdigest()
    if relative in inputs and inputs[relative]!=digest:
        raise ValueError('Input changed between PDF references: '+relative)
    inputs[relative]=digest
    return raw


def validate_output_dir(out):
    out=Path(out)
    if out.is_symlink() or ROOT.is_symlink():
        raise ValueError(f'Symlinked output directory rejected: {out}')
    try:
        rel=out.relative_to(ROOT)
    except ValueError as exc:
        raise ValueError(f'Output directory must be inside ROOT ({ROOT}): {out}') from exc
    if len(rel.parts)<2 or rel.parts[0]!='target' or rel.parts[-1]!='paper':
        raise ValueError(f'Output directory must be a target/.../paper path under ROOT: {out}')
    cur=ROOT
    for part in rel.parts:
        cur=cur/part
        if cur.is_symlink():
            raise ValueError(f'Symlinked output path component rejected: {cur}')
    if out.resolve()!=(ROOT.resolve()/rel):
        raise ValueError(f'Redirected output directory rejected: {out}')
    if out.exists() and not out.is_dir():
        raise ValueError(f'Output path is not a directory: {out}')
    return out


def stage(path, raw, renderer_inputs):
    if path.is_symlink() or path.parent.is_symlink():
        raise ValueError(f'Symlinked staged path rejected: {path}')
    renderer_inputs[str(path.relative_to(ROOT))]=hashlib.sha256(raw).hexdigest()
    path.write_bytes(raw)


def build(name, out=None):
    pandoc_ver,latex_ver=check_toolchain()
    out=validate_output_dir(ROOT/'target/paper' if out is None else out)
    out.mkdir(parents=True,exist_ok=True)
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
        if staged_image.parent.is_symlink():
            raise ValueError(f'Symlinked images directory rejected: {staged_image.parent}')
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
    log_file=out/f'{name}.build.log'
    manifest_file=out/f'{name}.pdf.manifest.json'
    for out_file in (pdf,log_file,manifest_file):
        if out_file.is_symlink():
            raise ValueError(f'Symlinked output file rejected: {out_file}')
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
    with log_file.open('w') as log:
        run=subprocess.run(['nice','-n','10']+cmd,cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT)
    if run.returncode:raise RuntimeError(f'PDF build failed; see {log_file}')
    pandoc_after,latex_after=check_toolchain()
    record={'schema':1,'source_commit':head,'source':str(source.relative_to(ROOT)),
            'source_sha256':inputs[str(source.relative_to(ROOT))],
            'inputs':inputs,'renderer_inputs':renderer_inputs,
            'output':str(pdf.relative_to(ROOT)),'output_sha256':sha(pdf),
            'pandoc':pandoc_after,
            'latex':latex_after,
            'command':cmd,'scope':'Publication render only; no new engineering solves or certification'}
    if subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()!=head:
        raise ValueError('Checkout changed during PDF build')
    for relative,expected in inputs.items():
        if sha(ROOT/relative)!=expected:
            raise ValueError('Input changed during PDF build: '+relative)
    for relative,expected in renderer_inputs.items():
        if sha(ROOT/relative)!=expected:
            raise ValueError('Renderer input changed during PDF build: '+relative)
    manifest_file.write_text(json.dumps(record,indent=1,sort_keys=True)+'\n')
    print(pdf.relative_to(ROOT),pdf.stat().st_size,flush=True)


def owned_artifact_paths(out, names=('WHITEPAPER','ENGINEERING_GATES_AND_EVOLUTION')):
    out=validate_output_dir(out)
    if not out.exists():return []
    exact={'version.tex','images','SHA256SUMS','texput.log'}
    for name in names:
        for suffix in ('.md','.pdf','.pdf.manifest.json','.build.log','.aux','.log','.out'):
            exact.add(f'{name}{suffix}')
    found=[]
    for child in sorted(out.iterdir(),key=lambda p:p.name):
        if child.name in exact or re.fullmatch(r'(publication-[0-9a-f]{12}\.lua|header-[0-9a-f]{12}\.tex)',child.name):
            if child.is_symlink():
                raise ValueError(f'Symlinked artifact path rejected: {child}')
            if child.is_dir():
                for sub in child.rglob('*'):
                    if sub.is_symlink():
                        raise ValueError(f'Symlinked artifact path rejected: {sub}')
            found.append(child)
    return found


def next_attempt_dir(out):
    out=validate_output_dir(out)
    base=out/'repro-attempts'
    if base.is_symlink():
        raise ValueError(f'Symlinked repro-attempts directory rejected: {base}')
    base.mkdir(parents=True,exist_ok=True)
    idx=1
    while True:
        candidate=base/f'attempt-{idx:04d}'
        if candidate.is_symlink():
            raise ValueError(f'Symlinked attempt directory rejected: {candidate}')
        if not candidate.exists():
            candidate.mkdir()
            return candidate
        idx+=1


def clean_paper_outputs(out, names=('WHITEPAPER','ENGINEERING_GATES_AND_EVOLUTION'), dest_dir=None):
    out=validate_output_dir(out)
    paths=owned_artifact_paths(out,names)
    if not paths:return []
    if dest_dir is None:
        dest_dir=next_attempt_dir(out)/'preserved'
    else:
        dest_dir=Path(dest_dir)
        if dest_dir.is_symlink():
            raise ValueError(f'Symlinked preservation directory rejected: {dest_dir}')
    dest_dir.mkdir(parents=True,exist_ok=False)
    moved=[]
    for path in paths:
        target=dest_dir/path.name
        path.rename(target)
        moved.append(target)
    return moved


def write_sha256sums(out, names):
    out=validate_output_dir(out)
    lines=[]
    for name in names:
        for suffix in ('.pdf','.pdf.manifest.json'):
            filename=f'{name}{suffix}'
            target=out/filename
            if target.is_symlink():
                raise ValueError(f'Symlinked checksum target rejected: {target}')
            lines.append(f'{sha(target)}  {filename}')
    sums_path=out/'SHA256SUMS'
    if sums_path.is_symlink():
        raise ValueError(f'Symlinked SHA256SUMS path rejected: {sums_path}')
    sums_path.write_text('\n'.join(lines)+'\n',encoding='utf-8')
    return sums_path


def verify_reproducible(names=('WHITEPAPER','ENGINEERING_GATES_AND_EVOLUTION'), out=None):
    check_toolchain()
    out=validate_output_dir(ROOT/'target/paper' if out is None else out)
    out.mkdir(parents=True,exist_ok=True)
    attempt_dir=next_attempt_dir(out)
    clean_paper_outputs(out,names,dest_dir=attempt_dir/'pre-existing')
    passes=[]
    for pass_idx in (1,2):
        if pass_idx==2:
            clean_paper_outputs(out,names,dest_dir=attempt_dir/'pass-1')
        snap={}
        for name in names:
            build(name,out=out)
            log_path=out/f'{name}.build.log'
            log_text=log_path.read_text(encoding='utf-8') if log_path.is_file() else ''
            if log_text:
                raise RuntimeError(f'Non-empty PDF build log on pass {pass_idx} for {name}: {log_text}')
            pdf_bytes=(out/f'{name}.pdf').read_bytes()
            manifest_bytes=(out/f'{name}.pdf.manifest.json').read_bytes()
            manifest=json.loads(manifest_bytes.decode('utf-8'))
            pdf_sha=hashlib.sha256(pdf_bytes).hexdigest()
            if manifest['output_sha256']!=pdf_sha:
                raise RuntimeError(f'Manifest output_sha256 mismatch for {name}: {manifest["output_sha256"]} != {pdf_sha}')
            snap[name]={
                'pdf_bytes':pdf_bytes,
                'pdf_sha256':pdf_sha,
                'manifest_bytes':manifest_bytes,
                'manifest_sha256':hashlib.sha256(manifest_bytes).hexdigest(),
            }
        passes.append(snap)
    for name in names:
        if passes[0][name]['pdf_sha256']!=passes[1][name]['pdf_sha256'] or passes[0][name]['pdf_bytes']!=passes[1][name]['pdf_bytes']:
            raise RuntimeError(f'Non-reproducible PDF across two clean builds for {name}: {passes[0][name]["pdf_sha256"]} != {passes[1][name]["pdf_sha256"]}')
        if passes[0][name]['manifest_sha256']!=passes[1][name]['manifest_sha256'] or passes[0][name]['manifest_bytes']!=passes[1][name]['manifest_bytes']:
            raise RuntimeError(f'Non-reproducible manifest across two clean builds for {name}: {passes[0][name]["manifest_sha256"]} != {passes[1][name]["manifest_sha256"]}')
    sums_path=write_sha256sums(out,names)
    print(sums_path.read_text(encoding='utf-8'),end='',flush=True)
    return sums_path


def main():
    p=argparse.ArgumentParser()
    p.add_argument('--document',choices=['WHITEPAPER','ENGINEERING_GATES_AND_EVOLUTION'])
    p.add_argument('--check-toolchain',action='store_true')
    p.add_argument('--verify-reproducible',action='store_true')
    args=p.parse_args()
    if args.check_toolchain:
        pandoc_ver,latex_ver=check_toolchain()
        print(f'PDF toolchain OK: {pandoc_ver} | {latex_ver}',flush=True)
        return
    names=[args.document] if args.document else ['WHITEPAPER','ENGINEERING_GATES_AND_EVOLUTION']
    if args.verify_reproducible:
        verify_reproducible(tuple(names))
        return
    for name in names:build(name)
    write_sha256sums(ROOT/'target/paper',names)


if __name__=='__main__':main()
