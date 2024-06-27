# Lightspeed photo management tool

See the internal `cargo run -- --help`.

## Tag viewer

#### Show statistics of tags distribution

```shell
cargo run tags list | grep -v '^---' | sort | python3 -c "
import sys, itertools as it, collections as col

lines = filter(str.strip, sys.stdin)

for k, igrp in it.groupby(lines, lambda x: x.split(':')[0].strip()):
    cnt = col.Counter(e.split(':', maxsplit=1)[1].strip() for e in igrp)
    print(k, *('{:6} {}'.format(c, v) for v, c in cnt.most_common()), sep='\n')
print()
" | less
```

#### Print location-specific tags

```shell
cargo run tags list -p Alti -p Geo -p Lati -p North -p Longi -p East -p location
```

## Similar crates

- [clineup](https://crates.io/crates/clineup)
- [photo_sort](https://crates.io/crates/photo_sort)
- [media_organizer](https://crates.io/crates/media_organizer)
