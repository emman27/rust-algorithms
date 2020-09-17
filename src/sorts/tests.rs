use super::SortOrder;

#[test]
fn empty_collection() {
    let mut v: std::vec::Vec<i32> = vec![];
    assert_eq!(
        *super::insertion(&mut v, SortOrder::ASCENDING),
        vec![] as std::vec::Vec<i32>
    );
    assert_eq!(
        *super::selection(&mut v, SortOrder::ASCENDING),
        vec![] as std::vec::Vec<i32>
    );
}

#[test]
fn sorted() {
    let mut v: std::vec::Vec<i32> = vec![1, 2, 3];
    assert_eq!(
        *super::insertion(&mut v, SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
    assert_eq!(
        *super::selection(&mut v, SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
}

#[test]
fn unsorted() {
    let mut v: std::vec::Vec<i32> = vec![3, 2, 1];
    assert_eq!(
        *super::insertion(&mut v, SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
    assert_eq!(
        *super::selection(&mut v, SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
}

#[test]
fn book() {
    let mut v: std::vec::Vec<i32> = vec![5, 2, 4, 6, 1, 3];
    assert_eq!(
        *super::insertion(&mut v, SortOrder::ASCENDING),
        vec![1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        *super::selection(&mut v, SortOrder::ASCENDING),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn string() {
    let mut v: std::vec::Vec<&str> = vec!["x", "a", "c"];
    assert_eq!(
        *super::insertion(&mut v, SortOrder::ASCENDING),
        vec!["a", "c", "x"]
    );
    assert_eq!(
        *super::selection(&mut v, SortOrder::ASCENDING),
        vec!["a", "c", "x"]
    );
}

#[test]
fn reversed() {
    let mut v: std::vec::Vec<i32> = vec![5, 2, 4, 6, 1, 3];
    assert_eq!(
        *super::insertion(&mut v, SortOrder::DESCENDING),
        vec![6, 5, 4, 3, 2, 1]
    );
    assert_eq!(
        *super::selection(&mut v, SortOrder::DESCENDING),
        vec![6, 5, 4, 3, 2, 1]
    );
}
