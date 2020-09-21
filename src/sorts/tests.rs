use super::SortOrder;

#[test]
fn empty_collection() {
    let v: std::vec::Vec<i32> = vec![];
    assert_eq!(
        *super::insertion(&mut v.clone(), SortOrder::ASCENDING),
        vec![] as std::vec::Vec<i32>
    );
    assert_eq!(
        *super::selection(&mut v.clone(), SortOrder::ASCENDING),
        vec![] as std::vec::Vec<i32>
    );
    assert_eq!(
        *super::merge(&mut v.clone(), SortOrder::ASCENDING),
        vec![] as std::vec::Vec<i32>
    );
}

#[test]
fn sorted() {
    let v: std::vec::Vec<i32> = vec![1, 2, 3];
    assert_eq!(
        *super::insertion(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
    assert_eq!(
        *super::selection(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
    assert_eq!(
        *super::merge(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
}

#[test]
fn unsorted() {
    let v: std::vec::Vec<i32> = vec![3, 2, 1];
    assert_eq!(
        *super::insertion(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
    assert_eq!(
        *super::selection(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
    assert_eq!(
        *super::merge(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3]
    );
}

#[test]
fn book() {
    let v: std::vec::Vec<i32> = vec![5, 2, 4, 6, 1, 3];
    assert_eq!(
        *super::insertion(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        *super::selection(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        *super::merge(&mut v.clone(), SortOrder::ASCENDING),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn string() {
    let v: std::vec::Vec<&str> = vec!["x", "a", "c"];
    assert_eq!(
        *super::insertion(&mut v.clone(), SortOrder::ASCENDING),
        vec!["a", "c", "x"]
    );
    assert_eq!(
        *super::selection(&mut v.clone(), SortOrder::ASCENDING),
        vec!["a", "c", "x"]
    );
    assert_eq!(
        *super::merge(&mut v.clone(), SortOrder::ASCENDING),
        vec!["a", "c", "x"]
    );
}

#[test]
fn reversed() {
    let v: std::vec::Vec<i32> = vec![5, 2, 4, 6, 1, 3];
    assert_eq!(
        *super::insertion(&mut v.clone(), SortOrder::DESCENDING),
        vec![6, 5, 4, 3, 2, 1]
    );
    assert_eq!(
        *super::selection(&mut v.clone(), SortOrder::DESCENDING),
        vec![6, 5, 4, 3, 2, 1]
    );
    assert_eq!(
        *super::merge(&mut v.clone(), SortOrder::DESCENDING),
        vec![6, 5, 4, 3, 2, 1]
    );
}
