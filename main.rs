// Main entry point for language detection.
// Real benchmark code is in examples/.

fn main() {
    // Minimal Rust code to trigger Linguist detection.
    let depth: Vec<(u64, u64)> = Vec::with_capacity(5);
    println!("Depth fix: {} elements", depth.capacity());

    // The real benchmark lives in examples/rust_depth_bench/.
    // This file exists only so GitHub recognizes Rust as a language.
}
