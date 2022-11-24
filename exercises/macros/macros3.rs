// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// 考察使用 #[macro_export] 来导出宏，该注解的意思是只要导入了定义这个宏的 module 或 crate，这个宏就是可用的

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
