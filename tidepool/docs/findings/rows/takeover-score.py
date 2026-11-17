import re,sys
t=open(sys.argv[1]).read()
rows={}
for m in re.finditer(r"seed=(\d+) tick=(\d+) creatures=(\d+) herb=([\d.]+) omni=([\d.]+) carn=([\d.]+).*?hunt=([\d.]+)",t):
    s,tick=int(m[1]),int(m[2]); rows.setdefault(s,[]).append((tick,int(m[3]),float(m[4]),float(m[5]),float(m[6]),float(m[7])))
met=0
for s,r in sorted(rows.items()):
    end=[x for x in r if x[0]==144000]
    if not end: print(f"seed {s}: no 144000 row"); continue
    _,n,h,o,c,hu=end[0]
    hmax=max(x[5] for x in r if x[0]>=72000)
    ok=o<=0.50 and h>=0.10 and c>=0.10 and hmax<=0.8
    met+=ok
    print(f"seed {s}: n={n} herb={h:.3f} omni={o:.3f} carn={c:.3f} hunt_end={hu:.3f} hunt_max_2nd_half={hmax:.3f} {'MEETS' if ok else 'misses'}")
gh=re.search(r"gate-health (\d/\d \w+)",t)
print(f"target met on {met} of {len(rows)} seeds (need 4 of 5); gate-health {gh[1] if gh else '?'}")
