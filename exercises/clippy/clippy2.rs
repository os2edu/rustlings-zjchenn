// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

// 使用 if let 来解构一个 Some 对象

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
