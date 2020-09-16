#[test]
fn empty() {
    assert_eq!(super::linear(vec![], 1), None)
}

#[test]
fn not_found() {
    assert_eq!(super::linear(vec![1, 2, 3], 4), None)
}

#[test]
fn start() {
    assert_eq!(super::linear(vec![1, 2, 3], 1), Some(0))
}

#[test]
fn end() {
    assert_eq!(super::linear(vec![1, 2, 3], 3), Some(2))
}
