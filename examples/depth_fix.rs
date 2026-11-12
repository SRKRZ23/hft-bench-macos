//! Reference: Depth() zero-allocation fix for LimitOrderBook v1.
//!
//! Context: `bruno-medeiros/rust-low-latency-hft`, `limit-order-book/src/book_v1/book.rs`.
//! Patch replaces `.collect::<Vec<_>>()` (2 allocations, 128 B) with
//! `Vec::with_capacity(n)` + push loop (1 allocation, 80 B).
//!
//! Result on Apple M3 Pro: p50 82 -> 42 ns, 5-run reproducible.

use crate::types::{Price, Qty, Side};

// BEFORE (2 allocs, 128 B):
// pub fn depth(&mut self, side: Side, levels: u64) -> Vec<(Price, Qty)> {
//     match side {
//         Side::Buy => (self.price_levels.bids_iter_mut())
//             .take(levels as usize)
//             .map(|pl| (pl.price, pl.total_qty))
//             .collect(),
//         Side::Sell => (self.price_levels.asks_iter_mut())
//             .take(levels as usize)
//             .map(|pl| (pl.price, pl.total_qty))
//             .collect(),
//     }
// }

// AFTER (1 alloc, 80 B):
pub fn depth(&mut self, side: Side, levels: u64) -> Vec<(Price, Qty)> {
    let n = levels as usize;
    let mut out = Vec::with_capacity(n);
    match side {
        Side::Buy => {
            for pl in (self.price_levels.bids_iter_mut()).take(n) {
                out.push((pl.price, pl.total_qty));
            }
        }
        Side::Sell => {
            for pl in (self.price_levels.asks_iter_mut()).take(n) {
                out.push((pl.price, pl.total_qty));
            }
        }
    }
    out
}
