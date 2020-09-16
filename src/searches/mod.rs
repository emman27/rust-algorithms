pub fn linear<T>(seq: std::vec::Vec<T>, key: T) -> Option<usize>
where
    T: std::cmp::Ord,
{
    for (idx, item) in seq.iter().enumerate() {
        match item.cmp(&key) {
            std::cmp::Ordering::Equal => return Some(idx),
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests;
