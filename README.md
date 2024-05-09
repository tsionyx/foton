# Lightspeed photo management tool

## Similar crates

- [clineup](https://crates.io/crates/clineup)
- [photo_sort](https://crates.io/crates/photo_sort)
- [media_organizer](https://crates.io/crates/media_organizer)

## Tag viewer

#### Show statistics of tags distribution

```shell
cargo run tags list | sort | python3 -c "import sys, itertools as it, collections as col
lines=filter(lambda x: not x.startswith('---') and x.strip(),sys.stdin)
for k,igrp in it.groupby(lines,lambda x: x.split(':')[0]):
  print(k); cnt=col.Counter(':'.join(e.split(':')[1:]).strip() for e in igrp)
  print(*['{:6} {}'.format(c, v) for v, c in cnt.most_common()], sep='\n'); print()" | less
```
