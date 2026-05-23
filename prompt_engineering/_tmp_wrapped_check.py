from pathlib import Path
import re, json, subprocess, tempfile
from collections import Counter

root=Path('/app/results/ab_test/v3/alp14')
preamble_path=Path('/app/training-dataset/specs/alp14/preamble.rs')
verus='/opt/verus/source/target-verus/release/verus'

sig_re=re.compile(r'pub\s+open\s+spec\s+fn\s+(\w+)\s*(\((?:[^(){};]|\([^)]*\))*\))\s*->\s*bool\s*\{', re.S)

def extract_fn_block(src:str):
    m=sig_re.search(src)
    if not m:
        return None, None, None
    fn,params=m.group(1),m.group(2)
    i=m.end()-1
    depth=0
    while i<len(src):
        ch=src[i]
        if ch=='{':
            depth+=1
        elif ch=='}':
            depth-=1
            if depth==0:
                return fn,params,src[m.start():i+1]
        i+=1
    return fn,params,src[m.start():]

def split_params(ps):
    inner=ps.strip()[1:-1].strip()
    if not inner:
        return []
    out=[]
    cur=''
    d=0
    for ch in inner:
        if ch in '(<':
            d+=1
        elif ch in ')>':
            d-=1
        if ch==',' and d==0:
            out.append(cur.strip())
            cur=''
        else:
            cur+=ch
    if cur.strip():
        out.append(cur.strip())
    pairs=[]
    for p in out:
        i=p.find(':')
        if i>0:
            pairs.append((p[:i].strip(),p[i+1:].strip()))
    return pairs

pre=preamble_path.read_text(encoding='utf-8',errors='ignore').rstrip()
pre=re.sub(r'(?m)^struct\b','pub struct',pre)

results=[]
for d in sorted([x for x in root.iterdir() if x.is_dir()]):
    cmd=d.name
    # g=(d/'generated.formatted.rs').read_text(encoding='utf-8',errors='ignore')
    o=(d/'oracle.formatted.rs').read_text(encoding='utf-8',errors='ignore')
    g = o

    fn,ps,block=extract_fn_block(g)
    mode='as_is'
    if not block:
        g2='\n'.join(line for line in g.splitlines() if not line.strip().startswith('```')).strip()
        ofn,ops,_=extract_fn_block(o)
        if ofn and ops and g2:
            block=f'pub open spec fn {ofn}{ops} -> bool {{\n    {g2}\n}}'
            fn,ps=ofn,ops
            mode='wrapped_from_oracle_sig'
        else:
            results.append({'cmd':cmd,'ok':False,'reason':'cannot_build_fn','mode':mode})
            continue

    params=split_params(ps)
    args=', '.join(n for n,_ in params)
    probe=(f'proof fn check_compile_{fn}{ps}\n'
           f'    requires {fn}({args})\n'
           f'    ensures true\n'
           f'{{}}')

    test_src=pre+'\n\n'+block+'\n\n'+probe+'\n\n} // verus!\n'
    with tempfile.NamedTemporaryFile('w',suffix=f'_{cmd}.rs',delete=False) as tf:
        tf.write(test_src)
        tmp=tf.name

    cp=subprocess.run([verus,'--crate-type','lib',tmp],capture_output=True,text=True)
    out=(cp.stdout or '')+'\n'+(cp.stderr or '')
    low=out.lower()
    if cp.returncode==0:
        reason='ok'
    elif 'cannot find' in low or 'unresolved' in low:
        reason='missing_symbol'
    elif 'expected item' in low or 'parse error' in low or 'unknown start of token' in low or 'unclosed delimiter' in low:
        reason='parse_error'
    else:
        reason='verus_error'

    results.append({'cmd':cmd,'ok':cp.returncode==0,'reason':reason,'mode':mode,'rc':cp.returncode,'head':'\n'.join(out.strip().splitlines()[:6])})

ok=sum(1 for r in results if r['ok'])
fail=[r for r in results if not r['ok']]
print('TOTAL',len(results))
print('PASS',ok)
print('FAIL',len(fail))
print('PASS_RATE',f'{(ok/len(results)*100 if results else 0):.2f}%')
print('MODE_COUNTS',dict(Counter(r['mode'] for r in results)))
print('REASON_COUNTS',dict(Counter(r['reason'] for r in fail)))

outp=Path('/app/results/ab_test/v3/alp14_verus_check_wrapped_signature.json')
outp.write_text(json.dumps({'total':len(results),'pass':ok,'fail':len(fail),'results':results},indent=2),encoding='utf-8')
print('JSON',outp)
print('FIRST_20_FAILS')
for r in fail[:20]:
    print('-',r['cmd'],r['mode'],r['reason'])
