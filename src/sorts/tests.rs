#[test]
fn empty_collection() {
    let mut v: std::vec::Vec<i32> = vec![];
    super::insertion(&mut v);
    assert_eq!(v, vec![]);
}

#[test]
fn sorted() {
    let mut v: std::vec::Vec<i32> = vec![1, 2, 3];
    super::insertion(&mut v);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn unsorted() {
    let mut v: std::vec::Vec<i32> = vec![3, 2, 1];
    assert_eq!(*super::insertion(&mut v), vec![1, 2, 3]);
}

#[test]
fn book() {
    let mut v: std::vec::Vec<i32> = vec![5, 2, 4, 6, 1, 3];
    assert_eq!(*super::insertion(&mut v), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn string() {
    let mut v: std::vec::Vec<&str> = vec!["x", "a", "c"];
    assert_eq!(*super::insertion(&mut v), vec!["a", "c", "x"]);
}
