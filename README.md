# infinite-uuid7

Print UUIDv7 (+ generator stats) infinitely

## Installation

```bash
cargo install --git https://github.com/LiosK/infinite-uuid7-rs.git
```

## Usage examples

Print UUIDv7 to stdout:

```text
% infinite-uuid7 2> /dev/null
0180ebdf-62a1-7635-8f9a-8bc97b34fb9d
0180ebdf-62a1-7635-8f9a-8bca4d20ba90
0180ebdf-62a1-7635-8f9a-8bcba33d7c19
0180ebdf-62a1-7635-8f9a-8bcceb072de3
0180ebdf-62a1-7635-8f9a-8bcdd403e976
0180ebdf-62a1-7635-8f9a-8bce72de4a62
0180ebdf-62a1-7635-8f9a-8bcfb8cd91e7
0180ebdf-62a1-7635-8f9a-8bd0d443f055
0180ebdf-62a1-7635-8f9a-8bd14c333b0c
0180ebdf-62a1-7635-8f9a-8bd2383c12d7
...
```

Print generator stats to stderr:

```text
infinite-uuid7 > /dev/null
Print stats every 10 seconds....
Total 121579551, ClockRollback 0
Total 121617861, ClockRollback 0
Total 121504514, ClockRollback 0
Total 121850439, ClockRollback 0
...
```

Measure throughput:

```text
% infinite-uuid7 2> /dev/null | pv -r > /dev/zero
[ 517MiB/s]
```

## License

Licensed under the Apache License, Version 2.0.
