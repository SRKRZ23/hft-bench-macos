# Patches applied to upstream

## 1. Rust: Depth() zero-allocation (limit-order-book/src/book_v1/book.rs)

Problem: depth() allocated 2x per call (128 B) due to .collect() with capacity 0.

Fix: replace .collect() with Vec::with_capacity(n) + push loop.

Result: allocs 2->1, bytes 128->80, p50 82->42 ns (5 runs).

File: limit-order-book/src/book_v1/book.rs, function depth()

## 2. C++: macOS build (exchange_core/CMakeLists.txt)

Problem: kernel_bypass_udp.cpp uses SO_BUSY_POLL (Linux-only).

Fix: exclude from macOS build via if(CMAKE_SYSTEM_NAME STREQUAL "Linux").

Result: builds and runs on macOS, 23.9M orders/sec.

File: hft-matching-engine/exchange_core/CMakeLists.txt
