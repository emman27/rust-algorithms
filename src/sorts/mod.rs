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
            let tmp = collection[i].clone();
            collection[i] = smallest.clone();
            collection[smallest_index] = tmp;
        }
    }
    collection
}

#[cfg(test)]
mod tests;
