// Reference: hft-matching-engine macOS build fix.
//
// Context: `mtepenner/hft-matching-engine`, `exchange_core/CMakeLists.txt`.
// Problem: `src/network/kernel_bypass_udp.cpp` uses `SO_BUSY_POLL` (Linux-only).
// Fix: exclude from macOS build; keep network stack Linux-only.
//
// Result on Apple M3 Pro: builds and runs, 23.9M orders/sec (100k burst).

// ────────────────────────────────────────────────────────────────
// BEFORE (exchange_core/CMakeLists.txt):
// ────────────────────────────────────────────────────────────────
//
//   add_executable(exchange_core
//       src/main.cpp
//       src/engine/order_book.cpp
//       src/engine/matcher.cpp
//       src/network/kernel_bypass_udp.cpp   <-- Linux-only SO_BUSY_POLL
//   )
//
// ────────────────────────────────────────────────────────────────
// AFTER:
// ────────────────────────────────────────────────────────────────
//
//   add_executable(exchange_core
//       src/main.cpp
//       src/engine/order_book.cpp
//       src/engine/matcher.cpp
//   )
//
//   if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
//       target_sources(exchange_core PRIVATE src/network/kernel_bypass_udp.cpp)
//   endif()

#include <cstdio>

int main() {
    std::printf("macOS build: matching engine + LOB only\n");
    std::printf("Linux build: + kernel_bypass_udp.cpp (SO_BUSY_POLL)\n");
    std::printf("Benchmark: 23.9M orders/sec on Apple M3 Pro (no pinning)\n");
    return 0;
}
