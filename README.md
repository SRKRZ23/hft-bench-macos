# HFT Benchmarks on Apple M3 Pro

Validated latency/throughput benchmarks of a Rust + C++ HFT stack on MacBook Pro M3 Pro.

**Reproducible** (every number has a log in results/). **Honest** (caveats documented).

## Environment

- CPU: Apple M3 Pro (11 cores: 6P + 6E)
- RAM: 18 GB unified
- OS: macOS 15.6 (Darwin 24.6.0)
- Rust: 1.99.1, Clang: 23.1.5
- CPU pinning: NOT available (macOS has no isolcpus)
- Timer resolution: ~40 ns (quanta)

## Headline results

### Primitive: photon-ring (Rust)
- 3.2 ns publish + recv (312M ops/sec)
- x35.8 faster than LMAX Disruptor (114.6 ns)
- cross-thread latency: 76.8 ns (reproduced in 2 benchmarks)

### Engine: Limit Order Book v1 (Rust, patched)
- 33M +/- 2M ops/sec on realistic mix (AC power, Chrome active)
- Depth (top 5) p50: 42 ns (was 82 before zero-alloc patch, reproduced 5 runs)
- Cancel (tail): <40 ns p99 (v0 was 207 ns)

### Pipeline: tick-to-trade (Rust, Lobster GOOG data)
- 17-20M ops/sec across 2 runs
- 0.2 allocs/op (setup only)
- Latency: NOT measured (bench tool outputs throughput only)

### C++ comparison: exchange_core
- 23.9M orders/sec on 100k burst, 49416 trades
- Built after excluding Linux-only kernel_bypass_udp.cpp

## The engineering result: Depth fix

Rust LimitOrderBook.depth() allocated 2x per call (128 B).
Fix: Vec::with_capacity(n) + push loop.
Result: p50 82 -> 42 ns, stable across 5 runs.
Details: see patches/README.md.

## Repository structure

  results/    All benchmark reports + raw logs (14 files)
  patches/    Documented code changes
  scripts/    Reproducibility scripts

## Honest claims

CAN cite:
- photon-ring publish+recv 3.2 ns, x35.8 vs Disruptor
- Depth p50 42 ns (was 82), 5-run reproducibility
- LOB v1 throughput 33M +/- 2M on AC power, macOS
- C++ exchange_core 23.9M orders/sec on macOS without pinning

CANNOT cite:
- 34.4M is the real max (best run, not median)
- Depth p99 = 84 ns (varies 84-125 ns due to Chrome/no pinning)
- Any sub-40 ns measurement as precise (timer limit)
- Pipeline tick-to-trade latency (not measured)
- Language X is faster (different workloads)

## Limitations

- No CPU pinning (macOS): P-E core migration adds 20-40 ns
- Timer ~40 ns: sub-40 ns p50 = artifact
- Background processes (Chrome, WindowServer): +/-5% noise
- Thermal throttling: sustained P-core load reduces frequency

For production HFT measurements: Linux + isolcpus + taskset + mlockall.

## License

Dual-licensed under MIT OR Apache-2.0. See LICENSE-MIT and LICENSE-APACHE.

## Credits

- bruno-medeiros/rust-low-latency-hft (photon-ring, LOB, pipeline)
- mtepenner/hft-matching-engine (C++ exchange_core)
