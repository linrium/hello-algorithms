fn factorial(x: i32) -> i32 {
    if x == 1 {
        return 1;
    }

    x * factorial(x - 1)
}

#[cfg(test)]
mod test_factorial {
    use crate::recursive::factorial::factorial;

    #[test]
    fn simple_factorial() {
        let result = factorial(5);
        assert_eq!(result, 120);
    }
}