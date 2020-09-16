pub fn insertion<T: std::cmp::Ord>(collection: &mut std::vec::Vec<T>) -> &std::vec::Vec<T>
where
    T: std::cmp::Ord,
    T: std::clone::Clone,
    T: std::fmt::Debug,
{
    let len = collection.len();
    for j in 1..len {
        let key = collection[j].clone();
        let mut i = j as isize - 1;
        while i >= 0 && collection[i as usize] > key {
            collection[i as usize + 1] = collection[i as usize].clone();
            i -= 1;
        }
        collection[(i + 1) as usize] = key;
    }
    collection
}

#[cfg(test)]
mod tests;
