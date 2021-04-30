fn find_smallest_index(items: &Vec<i32>) -> usize {
    let mut smallest_index = 0;
    let mut smallest = items[0];

    for (index, item) in items.into_iter().enumerate() {
        if *item < smallest {
            smallest = *item;
            smallest_index = index;
        }
    }

    smallest_index
}

fn selection_sort(mut items: Vec<i32>) -> Vec<i32> {
    let mut new_items = vec![];

    for _ in 0..items.len() {
        let smallest_index = find_smallest_index(&items);
        let smallest = items.remove(smallest_index);
        new_items.push(smallest);
    }

    new_items
}

#[cfg(test)]
mod test_selection_sort {
    use crate::sort::selection_sort::selection_sort;

    #[test]
    fn basic() {
        let nums = vec![5, 3, 6, 2, 10];
        let result = selection_sort(nums);
        assert_eq!(result, [2, 3, 5, 6, 10]);
    }
}