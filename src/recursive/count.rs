fn count(arr: &mut Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    arr.pop();

    1 + count(arr)
}

#[cfg(test)]
mod test_count {
    use crate::recursive::count::count;

    #[test]
    fn simple_count() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let result = count(&mut arr);
        assert_eq!(result, 5);
    }
}