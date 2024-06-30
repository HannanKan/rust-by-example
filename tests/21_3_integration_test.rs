use rust_by_example::adder::add;


#[cfg(test)]
mod tests {
    #[test]
    fn test_add1() {
        assert_eq!(add(3, 2), 5);
    }


    use super::*;
    use pretty_assertions::assert_eq; // 仅用于测试, 不能在非测试代码中使用

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}