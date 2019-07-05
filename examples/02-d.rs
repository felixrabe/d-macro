use d_macro::*;

// Examples for all ways to invoke `d!()`.
fn main() {
    // Explicitly calling `d_start()` at the beginning of `main` ensures that
    // all time measurements shown will be relative to the start of the
    // program.
    d_start();
    // =>  s ms  µs START

    // Macro variant `{}`:
    // Calling `d!()` without arguments works like calling `dbg!()` without
    // arguments.
    d!();
    // =>         9 [main]   examples/02-d.rs:14

    // Macro variant `{ $val:expr }`:
    // Calling `d!(value)` will show `value` using `Display`. This is useful
    // for displaying string labels.
    d!("this line has been executed");
    // =>        30 [main] this line has been executed  examples/02-d.rs:20

    // Macro variant `{ ? $val:expr }`:
    let value = vec!["one", "two"];
    d!(? &value);
    // =>        50 [main] &value = ["one", "two"]  examples/02-d.rs:25

    // Macro variant `{ #? $val:expr }`: (TODO: currently uglier than it has to be)
    d!(#? &value);
    // =>        72 [main] &value = [
    // ..     "one",
    // ..     "two",
    // .. ]  examples/02-d.rs:29

    // Explicitly calling `d_end()` at the end of `main` ensures that the last
    // time measurement shown will be when the program is about to finish.
    d_end();
    // =>        84 END
}
