// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

// 嵌套 Option

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        while let Some(Some(integer)) = optional_integers.pop() {
            // vec.pop() 返回的本身就是对元素的 Option，因此这里得到的结果是 Option<Option<i8>>
            assert_eq!(integer, range);
            range -= 1;
        }
    }
}
