//! Minimal reproduction: Depth() zero-alloc for LOB.
//! Demonstrates why `Vec::with_capacity` beats `.collect()` for small, bounded N.

#[derive(Clone, Copy, Default)]
pub struct PriceLevel {
    pub price: u64,
    pub total_qty: u64,
}

pub struct PriceLevels {
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
}

impl PriceLevels {
    pub fn new(n: usize) -> Self {
        Self {
            bids: (0..n).map(|i| PriceLevel { price: 100 - i as u64, total_qty: 100 }).collect(),
            asks: (0..n).map(|i| PriceLevel { price: 101 + i as u64, total_qty: 100 }).collect(),
        }
    }

    // BEFORE: .collect() → 2 allocations (Vec grows 0→4→8, capacity 8 for 5 items)
    pub fn depth_before(&self, levels: usize) -> Vec<(u64, u64)> {
        self.bids
            .iter()
            .take(levels)
            .map(|pl| (pl.price, pl.total_qty))
            .collect()
    }

    // AFTER: with_capacity → 1 allocation (exact size)
    pub fn depth_after(&self, levels: usize) -> Vec<(u64, u64)> {
        let mut out = Vec::with_capacity(levels);
        for pl in self.bids.iter().take(levels) {
            out.push((pl.price, pl.total_qty));
        }
        out
    }
}
