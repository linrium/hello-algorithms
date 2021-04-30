fn find_max(arr: &mut Vec<i32>, max: i32) -> i32 {
    if arr.is_empty() {
        return max;
    }

    let first = arr.remove(0);
    if max < first {
        return find_max(arr, first);
    }

    find_max(arr, max)
}

#[cfg(test)]
mod test_find_max {
    use crate::recursive::find_max::find_max;

    #[test]
    fn simple_find_max() {
        let mut arr = vec![1, 2, 7, 3, 4, 5];
        let max = arr[0];
        let result = find_max(&mut arr, max);
        assert_eq!(result, 7);
    }
}