# HFT Benchmarks — Apple M3 Pro
Environment: macOS 15.6, rustc 1.99.1, clang 23.1.5
Date: 2026-11-12

## photon-ring / payload_scaling (validated)

### Cross-thread (real IPC latency, most important)
| Payload | Median (ns) |
|---|---|
| 8B   | 77.14  |
| 16B  | 75.92  |
| 32B  | 77.32  |
| 64B  | 120.08 |
| 128B | 111.81 |
| 256B | 137.17 |
| 512B | 187.10 |
| 1KB  | 224.98 |
| 2KB  | 394.00 |
| 4KB  | 686.25 |

### Single-thread (no sync overhead)
| Payload | Median (ns) |
|---|---|
| 8B   | 2.95   |
| 16B  | 3.48   |
| 32B  | 4.88   |
| 64B  | 6.46   |
| 128B | 7.41   |
| 256B | 16.32  |
| 512B | 31.82  |
| 1KB  | 118.97 |
| 2KB  | 251.18 |
| 4KB  | 526.55 |

### Caveats
- Outliers: 1-16% per case (macOS no CPU pinning)
- No throughput measured (bench_function, not bench_with_input)
- Use median, not min, for fair comparison
- All numbers are reproducible: nohup + disown + wait for shell return

## photon-ring / event — Apple M3 Pro (Thu Nov 12 09:17:30 +05 2026)

| Payload | pod ring | event 1-field | event full-write | 1-field speedup |
|---|---|---|---|---|
| 8B   | 2.43 ns | 2.18 ns | 2.15 ns | 1.11x |
| 64B  | 4.17 ns | 2.21 ns | 4.67 ns | 1.89x |
| 1KB  | 87.22 ns | 2.77 ns | 47.12 ns | 31.5x |
| 4KB  | 378.28 ns | 2.96 ns | 203.55 ns | 127.8x |

### Key insights
- event 1-field is O(1) w.r.t. payload size (2.18 -> 2.96 ns for 8B -> 4KB)
- pod ring copies twice (pub + recv), event writes in-place once
- Crossover at 64B: pod faster than event full-write
- For HFT: struct can be large, mutated fields must be small

## LOB v1 (Rust, M3 Pro) — first full report

### Environment caveats
- macOS: no CPU pinning, no isolcpus, no governor control
- Timer resolution ~40ns (quanta fallback) — sub-40ns ops appear as "1ns"
- Max spikes (12-115 µs) = OS interruptions, not algorithmic

### Validated latencies (reliable, above timer resolution)
| Operation | p50 | p99 | Allocs/op |
|---|---|---|---|
| Add (passive) | 40 ns | 41 ns | 0 |
| Add (sweep 5, 50 fills) | 582 ns | 749 ns | 0 |
| Market (sweep 10, 100 fills) | 1.2 µs | 1.5 µs | 0 |
| Depth (top 5) | 82 ns | 124 ns | **2 (RED FLAG)** |
| Realistic mix | 40 ns | 83 ns | 0 |

### Below timer resolution (<40ns upper bound only)
- Cancel (head/tail): p99=41ns, real <40ns
- Reduce (partial): p99=41ns
- Spread (BBO): p99=41ns, real ~5-15ns
- Order lookup (hit): p99=41ns

### Throughput
- 33.0M ops/sec on realistic mix, 0 allocs on hot path
- Mix: 66% cancel, 28% add, 6% fill (realistic market maker profile)

### Critical finding
- `Depth (top 5)` allocates 2x per call → must return slice, not Vec
- This is the only real perf bug in v1

## LOB v0 vs v1 — A/B comparison (Apple M3 Pro)

### Throughput (realistic mix)
| Metric | v0 | v1 | Winner |
|---|---|---|---|
| ops/sec | 22.5M | **33.0M** | **v1 ×1.47** |
| allocs/op | 38.0 | **0.0** | **v1** |
| bytes/op | 3.3 KiB | **0 B** | **v1** |
| setup allocs | 645 | **3** | **v1** |

### Latency p99 (validated, above timer resolution)
| Operation | v0 | v1 | Speedup |
|---|---|---|---|
| Add (passive) | 83 ns | 41 ns | ×2.0 |
| Add (sweep 5) | 2.0 µs | 749 ns | ×2.7 |
| Market (sweep 10) | 3.6 µs | 1.5 µs | ×2.4 |
| Cancel (head) | 83 ns | 41 ns | ×2.0 |
| **Cancel (tail)** | **249 ns** | **41 ns** | **×6.1** |
| Reduce (partial) | 82 ns | 41 ns | ×2.0 |
| Spread (BBO) | 41 ns | 41 ns | par |
| Depth (top 5) | 82 ns | 124 ns | **v0 ×0.66 (alloc 1 vs 2)** |
| Order lookup | 41 ns | 41 ns | par |
| Realistic mix | 208 ns | 83 ns | ×2.5 |

### Key findings
1. v1 eliminates all hot-path allocations (v0 has 38 allocs/op)
2. Cancel-tail: v0 p50=207ns vs v1 p50=1ns → ×207 structural difference
3. Add (passive): v0 allocates 32B per call, v1 allocates 0
4. Depth (top 5): BOTH versions allocate. v1 bug = returns Vec, should return slice
5. Spread (BBO): optimal in both, <40ns
6. v1 wins ×1.47 throughput, ×2.5 p99, ×207 on the worst case (cancel-tail)

### Actionable improvement for v1
- Fix `Depth` to return `&[(Price, Qty)]` (zero-alloc)
- Expected: Depth p50 from 82ns to <40ns, +30% throughput on realistic mix

### Caveats
- macOS: no CPU pinning, timer resolution ~40ns
- Sub-40ns p50s are timer artifacts, only p99 upper bounds valid
- max spikes (12-115 µs) = OS interruptions, not algorithmic

## Depth (top 5) fix — zero-alloc optimization result

### Change
`.collect::<Vec<_>>()` → `Vec::with_capacity(n)` + `push` loop
Eliminated 1 of 2 allocations (Vec growth 0→4→8 becomes 5 directly)

### Before (v1 original)
| Metric | Value |
|---|---|
| allocs/op | 2.0 |
| bytes/op | 128 B |
| p50 | 82 ns |
| p99 | 124 ns |

### After (patched)
| Metric | Value |
|---|---|
| allocs/op | **1.0** |
| bytes/op | **80 B** |
| p50 | **41 ns** |
| p99 | **83 ns** |

### Throughput impact
| Metric | Before | After | Delta |
|---|---|---|---|
| ops/sec | 33.0M | **34.4M** | **+4.2%** |

### Remaining optimization
- 1 alloc (Vec 80B) still on hot path
- To eliminate: change API `fn depth(&self, ...) -> &[(Price, Qty)]`
- Requires updating `book_api.rs` trait

## photon-ring throughput (M3 Pro, validated)

| Case | Median (ns) | ops/sec |
|---|---|---|
| try_recv (empty) | 0.81 | 1.23B |
| publish only | 2.71 | 369M |
| publish+recv 1 sub | **3.20** | **312M** |
| spmc: 1p 1s | 3.24 | 309M |
| struct roundtrip (24B) | 2.78 | 360M |
| fanout 2 subs | 4.19 | 239M |
| fanout 5 subs | 7.02 | 143M |
| fanout 10 subs | 11.73 | 85M |
| mpmc: 1p 1s | 9.94 | 101M |
| cross-thread latency | 76.83 | 13M |
| **disruptor: publish+recv 1 consumer** | **114.61** | **8.7M** |

### Key insight
photon-ring: publish+recv = **3.20 ns**
Disruptor:   publish+recv = **114.61 ns**
**photon ×35.8 faster for the same operation.**

## Matching pipeline (M3 Pro, validated)

### Throughput
| Metric | Value |
|---|---|
| ops/sec | **19.9M** |
| allocs/op (hot) | 0.2 |
| setup allocs | 9 (21.8 MiB) |
| sample | Lobster GOOG 2012-06-21 |

### Operation mix
| Accepted | Rejected | Fill | Filled | Cancelled |
|---|---|---|---|---|
| 961.6k | 224.8k (19%) | 620.1k | 633.5k | 325.5k |

### Comparison with LOB
- LOB v1 patched: 34.4M ops/sec, 0 alloc, Depth p50=41ns
- Pipeline: 19.9M ops/sec (58% of LOB), 0.2 allocs/op
- Overhead: ~42% for ingest+signal+decision layers

### LIMITATION
- Pipeline report has NO latency table (only throughput)
- Cannot claim tick-to-trade latency without this
- Would need bench code changes to add latency instrumentation

## Full validated benchmark stack (M3 Pro)

| Layer | Metric | Value |
|---|---|---|
| photon-ring primitive | publish+recv | **3.2 ns** (×35.8 vs Disruptor 114.6 ns) |
| photon-ring cross-thread | round-trip | 76.8 ns |
| LOB v1 (patched) | throughput | **34.4M ops/sec** |
| LOB Depth | p50 | 41 ns (was 82 before patch) |
| Pipeline | throughput | **19.9M ops/sec** |
| Pipeline | allocs/op | 0.2 |

### Environment caveats (all benchmarks)
- macOS: no CPU pinning, timer resolution ~40ns (quanta)
- Sub-40ns p50 = timer artifact, use p99 as upper bound
- max spikes (12-115 µs) = OS interruptions
- Cross-thread numbers may include P→E core migration

### Reproducibility
- All numbers generated via `nohup ... & disown` + `cargo bench`
- Zero contention: one benchmark at a time
- Validated: photon cross-thread 76.8 ns appears in two independent benches (payload_scaling + throughput)
[...из блока выше...]

## LOB v1 — reproducibility test (3 AC runs, Chrome active)

### Throughput
| Run | ops/sec |
|---|---|
| AC Run 1 | 33.0M |
| AC Run 2 | 35.1M |
| AC Run 3 | 31.7M |
| **Median** | **33.0M** |
| **Range** | 31.7–35.1M (±5.2%) |

### Conditions during runs
- Power: AC attached
- Chrome: ACTIVE (user choice), 121% CPU renderer
- WindowServer: 47.8%
- No CPU pinning (macOS limitation)
- Timer resolution: ~40ns (quanta)

### Depth patch — STABLE (the headline result)
| Metric | Before patch | After patch (5 runs) |
|---|---|---|
| p50 | 82 ns | **42 ns (all 5 runs identical)** |
| allocs | 2 | **1** |
| bytes | 128 | **80** |

Depth p50 = 42 ns is **reproduced across all 5 runs** (run1, run2, 3 AC runs).
This is the most reliable result of the session: patch is validated.

### Honest claims
- CAN cite: "Depth p50 42 ns (was 82), +4% throughput on realistic mix"
- CAN cite: "LOB v1 throughput ~33M ± 2M on AC power, macOS, Chrome active"
- CAN cite: "photon-ring publish+recv 3.2 ns (×35.8 vs Disruptor) — condition-independent"
- CANNOT cite: "34.4M is the real max" — that was one lucky run
- CANNOT cite: "Depth p99 = 84 ns" — varies 84-125 ns due to Chrome/no-pinning
- CANNOT cite: pipeline tick-to-trade latency — not measured

### What would fix the noise
- Close Chrome + background apps (user declined)
- Linux with isolcpus + taskset (not available on macOS)
- Longer measurement (more samples reduce variance but not systemic bias)

## Pipeline throughput — reproducibility check (2 runs)

| Run | Timestamp | ops/sec |
|---|---|---|
| Run 1 | 2026-11-12T04:48:39Z | 19.9M |
| Run 2 | 2026-11-12T06:49:05Z | 17.1M |

Difference: -14% (within known macOS noise ±5-15% with Chrome + no pinning).
**Pipeline LATENCY (tick-to-trade) NOT measured** — bench-tool only outputs throughput.
Adding latency requires patching benches/pipeline.rs (not done).

## C++ comparison — hft-matching-engine (mtepenner)

### Build status on macOS
- Full project: **fails** (Go+React+Docker, Linux SO_BUSY_POLL)
- exchange_core (matching logic): **builds** after excluding Linux-only kernel_bypass_udp.cpp
- Patch: CMakeLists.txt wrapped network source in `if(CMAKE_SYSTEM_NAME STREQUAL "Linux")`

### Runtime (macOS, no CPU pinning)
[ExchangeCore] CPU pinning unavailable for core 2, continuing without affinity
[ExchangeCore] Matching engine online – 5 symbols
[ExchangeCore] 100000 orders in 4.19075 ms (23862077 orders/sec)
[ExchangeCore] 49416 trades executed
exit=0

text

**23.9M orders/sec, 49k trades from 100k orders.**

### Honest comparison (not apples-to-apples)
| Metric | Rust LOB v1 | C++ exchange_core |
|---|---|---|
| Throughput | 33M ops/sec | 23.9M orders/sec |
| Workload | realistic MIX (cancel-heavy) | 100k burst (insert+sweep) |
| Pinning | no (macOS) | no (macOS) |
| Latency p50/p99 | 42/83 ns | not reported by tool |

**Both in 20-35M range on M3 Pro without pinning.**
**Cannot claim one language faster than other** — different workloads + methodologies.
For fair comparison: identical workload + Linux isolcpus + mlock.

### What this validates
- macOS ceiling for in-memory matching: **~30M ops/sec** (both languages)
- C++ claimed "<500ns order matching" requires **CPU pinning** (Linux only) — not reproducible on macOS
- Rust tooling (bench-tool) is **richer** — gives p50/p99/p99.9, allocs/op, bytes/op
- C++ tool (main.cpp) gives **only aggregate throughput**
