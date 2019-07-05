use d_macro::*;

// Displays how long it takes to get from `d_start()` to `d_end()`.
fn main() {
    d_start();
    std::thread::sleep(std::time::Duration::from_millis(10));
    d_end();
}
