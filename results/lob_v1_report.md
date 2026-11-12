
# Limit Order Book (v1)

| Property | Value |
|----------|-------|
| Timestamp | 2026-11-12T04:25:56Z |
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
| Add (passive) | 1ns | 40ns | 41ns | 41ns | 82ns | 12.1μs | 23ns | 45ns | 0.0 | 0.0 | 0B |
| Add (sweep 5 levels, 50 fills) | 475ns | 582ns | 624ns | 749ns | 1.6μs | 34.9μs | 589ns | 175ns | 0.0 | 0.0 | 0B |
| Market (sweep 10 levels, 100 fills) | 975ns | 1.2μs | 1.2μs | 1.5μs | 4.1μs | 114.9μs | 1.2μs | 642ns | 0.0 | 0.0 | 0B |
| Cancel (head of queue) | 1ns | 1ns | 41ns | 41ns | 249ns | 7.9μs | 15ns | 43ns | 0.0 | 0.0 | 0B |
| Cancel (tail of queue) | 1ns | 1ns | 41ns | 41ns | 58ns | 13.3μs | 12ns | 46ns | 0.0 | 0.0 | 0B |
| Reduce (partial) | 1ns | 1ns | 40ns | 41ns | 41ns | 7.8μs | 5ns | 27ns | 0.0 | 0.0 | 0B |
| Spread (BBO query) | 1ns | 1ns | 1ns | 41ns | 41ns | 8.3μs | 2ns | 27ns | 0.0 | 0.0 | 0B |
| Depth (top 5) | 16ns | 82ns | 83ns | 124ns | 1.2μs | 18.8μs | 71ns | 147ns | 2.0 | 1.0 | 128B |
| Order lookup (hit) | 1ns | 1ns | 40ns | 41ns | 207ns | 12.4μs | 6ns | 57ns | 0.0 | 0.0 | 0B |
| Realistic mix (per-op) | 1ns | 40ns | 41ns | 83ns | 208ns | 5.4μs | 31ns | 38ns | 0.0 | 0.0 | 0B |

## Throughput (realistic mix)

| Property | Value |
|----------|-------|
| book_levels | 100 |
| orders_per_level | 10 |
| Default pinned core | Could not pin core 2 |

### Throughput

| Scenario | ops/sec | allocs/op | deallocs/op | bytes/op | setup allocs | setup bytes |
|----------|---------|-----------|-------------|----------|--------------|-------------|
| Throughput (realistic mix) | 33.0M | 0.0 | 0.0 | 0B | 3 | 1.9MiB |

| Scenario | Accepted | Rejected | Fill | Filled | Cancelled |
|----------|----------|----------|------|--------|-----------|
| Throughput (realistic mix) | 116.0M | 0 | 32.0M | 40.0M | 76.0M |

