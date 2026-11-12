cmake_minimum_required(VERSION 3.20)
project(hft-bench-macos CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# macOS-compatible build (excludes Linux-only kernel_bypass_udp.cpp)
add_executable(hft_matcher main.cpp)

# Rust benchmark (built separately via cargo)
# See examples/rust_depth_bench/ for Depth() fix benchmark
