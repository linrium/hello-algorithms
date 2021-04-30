
fn stupid_search(list: Vec<i32>, item: i32) -> Option<usize> {
    for (index, v) in list.into_iter().enumerate() {
        if v == item {
            return Some(index)
        }
    }

    None
}

#[cfg(test)]
mod test_binary_search {
    use crate::common::range;
    use test::Bencher;
    use crate::search::stupid_search::stupid_search;

    #[test]
    fn found_simple_search() {
        let nums = range(0, 10_000);
        let result = stupid_search(nums, 2);
        assert_eq!(result, Some(2));
    }

    #[bench]
    fn bench_found_simple_search(b: &mut Bencher) {
        let nums = range(0, 10_000_000);
        b.iter(|| stupid_search(nums.clone(), 10_000_000));
    }

    #[test]
    fn not_found_simple_search() {
        let nums = range(0, 10_000);
        let result = stupid_search(nums, -1);
        assert_eq!(result, None);
    }
}