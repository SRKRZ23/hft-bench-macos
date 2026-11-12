
# Limit Order Book (v0)

| Property | Value |
|----------|-------|
| Timestamp | 2026-11-12T04:28:51Z |
| CPU | Apple M3 Pro |
| Cores | 11 |
| Memory | 18.0 GB |
| OS | Darwin 15.6 (aarch64) |
| Host | macbook |
| Rust | rustc 1.99.1 (58cf8f9c3 2026-10-13) (Homebrew) |
| Clock | OS clock (platform fallback via quanta) |
| ASLR | sysctl failed (status exit status: 1): sysctl: unknown oid 'kern.randomize_va_space' |
| CPU governor | not exposed via sysfs (macOS; see `pmset -g` / Energy settings) |
| IRQ affinity (sample) | not applicable (macOS) |
| Isolated CPUs | not applicable (macOS; no isolcpus sysfs — use thread affinity / QoS) |
| Swap | total = 2048.00M  used = 743.12M  free = 1304.88M  (encrypted) |
| Turbo / boost | not exposed via sysfs (macOS) |

## Latency

| Property | Value |
|----------|-------|
| book_levels | 100 |
| orders_per_level | 10 |
| BENCH_ITERS | 100000 |
| WARMUP_ITERS | 10000 |
| Default pinned core | Could not pin core 2 |

### Latency

| Operation | min | p50 | p90 | p99 | p99.9 | max | mean | stdev | allocs/op | deallocs/op | bytes/op |
|-----------|-----|-----|-----|-----|-------|-----|------|-------|-----------|-------------|----------|
| Add (passive) | 1ns | 41ns | 82ns | 83ns | 124ns | 15.4μs | 47ns | 52ns | 1.0 | 0.0 | 32B |
| Add (sweep 5 levels, 50 fills) | 1.1μs | 1.4μs | 1.5μs | 2.0μs | 7.1μs | 52.6μs | 1.4μs | 495ns | 0.0 | 6.0 | 0B |
| Market (sweep 10 levels, 100 fills) | 2.4μs | 2.8μs | 2.9μs | 3.6μs | 8.2μs | 66.8μs | 2.8μs | 567ns | 0.0 | 14.0 | 0B |
| Cancel (head of queue) | 1ns | 41ns | 41ns | 83ns | 249ns | 26.0μs | 39ns | 108ns | 0.0 | 0.0 | 0B |
| Cancel (tail of queue) | 141ns | 207ns | 208ns | 249ns | 291ns | 16.6μs | 190ns | 108ns | 0.0 | 0.0 | 0B |
| Reduce (partial) | 1ns | 41ns | 41ns | 82ns | 416ns | 36.4μs | 34ns | 200ns | 0.0 | 0.0 | 0B |
| Spread (BBO query) | 1ns | 1ns | 1ns | 41ns | 83ns | 21.9μs | 4ns | 77ns | 0.0 | 0.0 | 0B |
| Depth (top 5) | 1ns | 41ns | 41ns | 82ns | 249ns | 18.0μs | 33ns | 73ns | 1.0 | 1.0 | 80B |
| Order lookup (hit) | 1ns | 1ns | 40ns | 41ns | 249ns | 9.6μs | 7ns | 59ns | 0.0 | 0.0 | 0B |
| Realistic mix (per-op) | 1ns | 41ns | 83ns | 208ns | 790ns | 15.4μs | 58ns | 121ns | 0.4 | 0.0 | 13B |

## Throughput (realistic mix)

| Property | Value |
|----------|-------|
| book_levels | 100 |
| orders_per_level | 10 |
| Default pinned core | Could not pin core 2 |

### Throughput

| Scenario | ops/sec | allocs/op | deallocs/op | bytes/op | setup allocs | setup bytes |
|----------|---------|-----------|-------------|----------|--------------|-------------|
| Throughput (realistic mix) | 22.5M | 38.0 | 35.0 | 3.3KiB | 645 | 499.5KiB |

| Scenario | Accepted | Rejected | Fill | Filled | Cancelled |
|----------|----------|----------|------|--------|-----------|
| Throughput (realistic mix) | 116.0M | 0 | 32.0M | 40.0M | 76.0M |

