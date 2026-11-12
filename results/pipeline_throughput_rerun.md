
# Matching pipeline

| Property | Value |
|----------|-------|
| Timestamp | 2026-11-12T06:49:05Z |
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

## Pipeline

| Property | Value |
|----------|-------|
| sample | LOBSTER_SampleFiles/GOOG_2012-06-21_34200000_57600000_message_1.csv |
| queue_slots | 4096 |
| producer_pin_core | Could not pin core 2 |
| consumer_pin_core | Could not pin core 3 |

### Throughput

| Scenario | ops/sec | allocs/op | deallocs/op | bytes/op | setup allocs | setup bytes |
|----------|---------|-----------|-------------|----------|--------------|-------------|
| Pipeline (Lobster data) | 17.1M | 0.2 | 0.0 | 0B | 9 | 21.8MiB |

| Scenario | Accepted | Rejected | Fill | Filled | Cancelled |
|----------|----------|----------|------|--------|-----------|
| Pipeline (Lobster data) | 961.6k | 224.8k | 620.1k | 633.5k | 325.5k |

