// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// 考察 ref
// ref 和 & 的区别: 
// - & 属于 pattern 的一部分，因此匹配的时候也会匹配一个引用
// - ref 则表示对当前值的引用，Foo(ref foo) 与 Foo(foo) 都会对同一个对象进行匹配，只是前者不会移动所有权

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
