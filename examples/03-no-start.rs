use d_macro::*;

// If the `d!()` macro gets used without calling `d_start()`, `d!()` will call
// `d_start()` implicitly.
fn main() {
    std::thread::sleep(std::time::Duration::from_millis(10));
    d!();

    d_end();
}
