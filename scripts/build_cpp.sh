#!/usr/bin/env bash
# Build and run C++ matcher (macOS-compatible).
set -euo pipefail
cmake -B build -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=clang++
cmake --build build -j"$(sysctl -n hw.ncpu)"
./build/hft_matcher
