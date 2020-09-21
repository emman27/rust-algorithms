pub enum SortOrder {
    ASCENDING,
    DESCENDING,
}

pub fn insertion<T>(collection: &mut std::vec::Vec<T>, order: SortOrder) -> &std::vec::Vec<T>
where
    T: std::cmp::Ord,
    T: std::clone::Clone,
{
    let cmp = match order {
        SortOrder::ASCENDING => std::cmp::Ordering::Greater,
        SortOrder::DESCENDING => std::cmp::Ordering::Less,
    };
    let len = collection.len();
    for j in 1..len {
        let key = collection[j].clone();
        let mut i = j as isize - 1;
        while i >= 0 && collection[i as usize].cmp(&key) == cmp {
            collection[i as usize + 1] = collection[i as usize].clone();
            i -= 1;
        }
        collection[(i + 1) as usize] = key;
    }
    collection
}

pub fn selection<T>(collection: &mut std::vec::Vec<T>, order: SortOrder) -> &std::vec::Vec<T>
where
    T: std::cmp::Ord,
    T: std::clone::Clone,
{
    let bottom = match order {
        SortOrder::ASCENDING => std::cmp::Ordering::Less,
        SortOrder::DESCENDING => std::cmp::Ordering::Greater,
    };
    for i in 0..collection.len() {
        let mut smallest_index = i;
        let mut smallest = collection[i].clone();
        for j in i + 1..collection.len() {
            let curr = &collection[j];
            if curr.cmp(&smallest) == bottom {
                smallest_index = j;
                smallest = curr.clone();
            }
        }
        let tmp = collection[i].clone();
        collection[i] = smallest.clone();
        collection[smallest_index] = tmp;
    }
    collection
}

pub fn merge<T>(collection: &mut std::vec::Vec<T>, order: SortOrder) -> &std::vec::Vec<T>
where
    T: std::cmp::Ord,
    T: std::clone::Clone,
{
    merge_sort_internal(collection, &order, 0, collection.len());
    collection
}

fn merge_sort_internal<T>(
    collection: &mut std::vec::Vec<T>,
    order: &SortOrder,
    left: usize,
    right: usize, // non-inclusive
) where
    T: std::cmp::Ord,
    T: std::clone::Clone,
{
    if left + 1 >= right {
        return;
    }
    let mid = (left + right) / 2;
    merge_sort_internal(collection, order, left, mid);
    merge_sort_internal(collection, order, mid, right);
    merge_subarrays(collection, order, left, mid, right);
}

fn merge_subarrays<T>(
    collection: &mut std::vec::Vec<T>,
    order: &SortOrder,
    left: usize,
    mid: usize,
    right: usize,
) where
    T: std::cmp::Ord,
    T: std::clone::Clone,
{
    let length_of_left_subarray = mid - left;
    let length_of_right_subarray = right - mid;
    let mut left_arr: std::vec::Vec<T> = std::vec::Vec::with_capacity(length_of_left_subarray);
    let mut right_arr: std::vec::Vec<T> =
        std::vec::Vec::with_capacity(length_of_right_subarray + 1);
    for i in 0..length_of_left_subarray {
        left_arr.push(collection[left + i].clone());
    }
    for i in 0..length_of_right_subarray {
        right_arr.push(collection[(mid + i) as usize].clone());
    }
    let mut i = 0;
    let mut j = 0;
    let bottom = match order {
        SortOrder::ASCENDING => std::cmp::Ordering::Less,
        SortOrder::DESCENDING => std::cmp::Ordering::Greater,
    };
    for k in left..right {
        if i == length_of_left_subarray {
            collection[k as usize] = right_arr[j].clone();
            j += 1;
        } else if j == length_of_right_subarray {
            collection[k as usize] = left_arr[i].clone();
            i += 1;
        } else {
            let cmp = left_arr[i].cmp(&right_arr[j]);
            if cmp == bottom || cmp == std::cmp::Ordering::Equal {
                collection[k as usize] = left_arr[i].clone();
                i += 1;
            } else {
                collection[k as usize] = right_arr[j].clone();
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests;
