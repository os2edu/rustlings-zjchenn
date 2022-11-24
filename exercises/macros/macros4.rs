// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// 宏对不同参数进行匹配时，需要使用`;`对其进行分隔

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
