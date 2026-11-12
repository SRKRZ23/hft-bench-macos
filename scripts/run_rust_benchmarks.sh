#!/usr/bin/env bash
# Reproduce Rust benchmarks. Requires upstream rust-low-latency-hft cloned.
set -euo pipefail
REPO="${1:-$HOME/rust-low-latency-hft}"
OUT="${2:-$HOME/hft-bench-results}"
mkdir -p "$OUT"
cd "$REPO"

echo "=== photon-ring payload_scaling ==="
cargo bench --bench payload_scaling -- --noplot > "$OUT/payload.log" 2>&1 || true

echo "=== LOB v1 patched ==="
cargo bench -p limit-order-book --bench lob -- \
  --save-md "$OUT/lob_v1_patched.md" \
  > "$OUT/lob_run.log" 2>&1 || true

echo "Done. Results in $OUT"
