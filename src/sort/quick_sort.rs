fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        return arr;
    }

    let pivot = arr[0];
    let less = arr[1..]
        .iter()
        .filter(|v| **v <= pivot)
        .cloned()
        .collect::<Vec<_>>();

    let greater = arr[1..]
        .iter()
        .filter(|v| **v > pivot)
        .cloned()
        .collect::<Vec<_>>();

    [quick_sort(less), vec![pivot], quick_sort(greater)].concat()
}

#[cfg(test)]
mod test_quick_sort {
    use crate::sort::quick_sort::quick_sort;

    #[test]
    fn simple_quick_sort() {
        let nums = vec![5, 3, 6, 2, 10];
        let result = quick_sort(nums);
        assert_eq!(result, [2, 3, 5, 6, 10]);
    }
}