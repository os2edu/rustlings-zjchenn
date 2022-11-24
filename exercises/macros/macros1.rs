// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

// 调用宏时需要加上 !

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
