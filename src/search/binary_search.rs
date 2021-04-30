
fn binary_search(list: Vec<i32>, item: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = list[mid];
        if guess == item {
            return Some(mid);
        }

        if guess > item {
            if mid == 0 {
                return None;
            }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod test_binary_search {
    use crate::common::range;
    use test::Bencher;
    use crate::search::binary_search::binary_search;

    #[test]
    fn found_binary_search() {
        let nums = range(1, 5);
        let result = binary_search(nums, 1);
        assert_eq!(result, Some(2));
    }

    #[bench]
    fn bench_found_binary_search(b: &mut Bencher) {
        let nums = range(0, 10_000_000);
        b.iter(|| binary_search(nums.clone(), 10_000_000));
    }

    #[test]
    fn not_found_binary_search() {
        let nums = range(0, 10_000);
        let result = binary_search(nums, -1);
        assert_eq!(result, None);
    }
}