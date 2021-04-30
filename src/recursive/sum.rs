fn sum(arr: &mut Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    arr.remove(0) + sum(arr)
}

#[cfg(test)]
mod test_sum {
    use crate::recursive::sum::sum;

    #[test]
    fn simple_sum() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let result = sum(&mut arr);
        assert_eq!(result, 15);
    }
}