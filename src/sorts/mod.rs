pub enum SortOrder {
    ASCENDING,
    DESCENDING,
}

pub fn insertion<T>(collection: &mut std::vec::Vec<T>, order: SortOrder) -> &std::vec::Vec<T>
where
    T: std::cmp::Ord,
    T: std::clone::Clone,
    T: std::fmt::Debug,
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

#[cfg(test)]
mod tests;
