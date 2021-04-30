fn binary_search(arr: Vec<i32>, item: i32, low: usize, high: usize) -> Option<usize> {
    if low <= high {
        let mid = (low + high) / 2;
        let guess = arr[mid];

        if guess == item {
            return Some(mid);
        }

        if guess > item {
            if mid == 0 {
                return None;
            }
            return binary_search(arr, item, low, mid - 1);
        }

        return binary_search(arr, item, mid + 1, high);
    }

    None
}

#[cfg(test)]
mod test_binary_search {
    use crate::recursive::binary_search::binary_search;
    use crate::common::range;

    #[test]
    fn test_binary_search() {
        let arr = range(0, 10_000);
        let len = arr.len() - 1;
        let result = binary_search(arr, 5_000, 0, len);
        assert_eq!(result, Some(5000));
    }
}