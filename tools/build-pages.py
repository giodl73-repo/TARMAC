"""Verify frozen BTS custody and build independent source/graph browser tasks."""
import json
import shutil
import subprocess
from pathlib import Path
from bts_source import verify

ROOT=Path(__file__).resolve().parents[1]

def output(*args):
    return subprocess.check_output(args,cwd=ROOT,text=True,encoding='utf-8')

def main():
    verify()
    cache=ROOT/'.cache';cache.mkdir(exist_ok=True)
    refs=[]
    for mode in ['reset','outage','changed']:
        path=f'fixtures/browser-{mode}.json'
        refs.append({'input':(ROOT/path).read_text(),'canonical':output('cargo','run','--locked','--quiet','-p','tarmac-web','--bin','tarmac-browser','--',path).strip()})
    (cache/'native-results.json').write_text(json.dumps(refs),encoding='utf-8')
    source_refs=[]
    for mode in ['recent','pandemic','reverse','same']:
        path=f'fixtures/bts-query-{mode}.json'
        source_refs.append({'input':(ROOT/path).read_text(),'canonical':output('cargo','run','--locked','--quiet','-p','tarmac-web','--bin','tarmac-browser','--',path).strip()})
    (cache/'bts-native-results.json').write_text(json.dumps(source_refs),encoding='utf-8')
    subprocess.run(['cargo','build','--locked','--release','--target','wasm32-unknown-unknown','-p','tarmac-web','--lib'],cwd=ROOT,check=True)
    metadata=json.loads(output('cargo','metadata','--no-deps','--format-version','1'))
    dest=ROOT/'dist/TARMAC';(dest/'pkg').mkdir(parents=True,exist_ok=True)
    shutil.copy2(Path(metadata['target_directory'])/'wasm32-unknown-unknown/release/tarmac_web.wasm',dest/'pkg/tarmac_web.wasm')
    for file in (ROOT/'web').iterdir():
        if file.is_file():shutil.copy2(file,dest/file.name)
    for mode,target in [('reset','example.json'),('outage','outage.json')]:shutil.copy2(ROOT/f'fixtures/browser-{mode}.json',dest/target)
    for name in ['LICENSE','THIRD-PARTY-LICENSES.txt']:shutil.copy2(ROOT/name,dest/name)
    (dest/'sources').mkdir(exist_ok=True)
    shutil.copy2(ROOT/'data/bts/transtats-arrivals.html',dest/'sources/transtats-arrivals.html')
    shutil.copy2(ROOT/'data/bts/custody.json',dest/'sources/custody.json')
    shutil.copy2(ROOT/'fixtures/bts/summary.json',dest/'bts-summary.json')
    shutil.copy2(ROOT/'data/sources.md',dest/'SOURCE-NOTICE.txt')
    provenance={'commit':output('git','rev-parse','HEAD').strip(),'dirty':bool(output('git','status','--porcelain').strip()),'rust':output('rustc','--version').strip(),'source':'https://github.com/giodl73-repo/TARMAC','synthetic':True,'scope':'local fictional Cedar graph ratings'}
    provenance.pop('synthetic');provenance.pop('scope')
    provenance['tasks']=['frozen BTS January-August reporting-carrier arrival comparison','independent fictional Cedar regional graph']
    provenance['source_capture']=json.loads((ROOT/'data/bts/custody.json').read_text())['captured_at']
    (dest/'provenance.json').write_text(json.dumps(provenance,indent=2),encoding='utf-8');(dest/'.nojekyll').touch()
    size=sum(path.stat().st_size for path in dest.rglob('*') if path.is_file())
    if size>=5_000_000:raise ValueError('Artifact exceeds 5 MB')
    print(f'TARMAC Pages: {size:,} bytes')

if __name__=='__main__':main()
