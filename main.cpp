// HFT matching engine (reference, macOS-compatible).
#include <cstdio>
#include <cstdint>
#include <vector>
#include <algorithm>
#include <chrono>
#include <random>

struct Order { uint64_t id; int64_t price; uint64_t qty; bool is_bid; };

struct Matcher {
    std::vector<Order> bids, asks;
    uint64_t trades = 0, volume = 0;

    void add(const Order& o) {
        if (o.is_bid) bids.push_back(o);
        else          asks.push_back(o);
    }

    void sweep() {
        std::sort(bids.begin(), bids.end(),
                  [](const Order& a, const Order& b) { return a.price > b.price; });
        std::sort(asks.begin(), asks.end(),
                  [](const Order& a, const Order& b) { return a.price < b.price; });
        size_t i = 0, j = 0;
        while (i < bids.size() && j < asks.size() && bids[i].price >= asks[j].price) {
            uint64_t fill = std::min(bids[i].qty, asks[j].qty);
            trades++;
            volume += fill;
            bids[i].qty -= fill;
            asks[j].qty -= fill;
            if (bids[i].qty == 0) i++;
            if (asks[j].qty == 0) j++;
        }
    }
};

static void run_burst(int n, uint64_t seed) {
    std::mt19937_64 rng(seed);
    Matcher m;
    m.bids.reserve(n);
    m.asks.reserve(n);
    for (int k = 0; k < n; ++k) {
        Order o;
        o.id    = static_cast<uint64_t>(k);
        o.price = 10000 + static_cast<int64_t>(rng() % 100);
        o.qty   = 1 + (rng() % 1000);
        o.is_bid = (rng() & 1) == 0;
        m.add(o);
    }
    auto t0 = std::chrono::high_resolution_clock::now();
    m.sweep();
    auto t1 = std::chrono::high_resolution_clock::now();
    double ms = std::chrono::duration<double, std::milli>(t1 - t0).count();
    double ops = n / (ms / 1000.0);
    std::printf("n=%6d  %7.3f ms  %10.0f ops/sec  trades=%llu\n",
                n, ms, ops,
                static_cast<unsigned long long>(m.trades));
}

int main() {
    run_burst(10'000,  42);
    run_burst(50'000,  43);
    run_burst(100'000, 44);
    run_burst(200'000, 45);
    return 0;
}
