#[test]
fn empty_integers() {
    assert_eq!(super::add_binary_integers(&[], &[]).unwrap(), [false]);
}

#[test]
fn zero_sums() {
    assert_eq!(
        super::add_binary_integers(&[false], &[false]).unwrap(),
        [false, false]
    );
}

#[test]
fn mismatched_lengths() {
    assert_eq!(super::add_binary_integers(&[], &[false]).is_err(), true);
    assert_eq!(
        super::add_binary_integers(&[false, false], &[false]).is_err(),
        true
    );
}
#[test]
fn ones() {
    assert_eq!(
        super::add_binary_integers(&[true], &[true]).unwrap(),
        [true, false]
    );
}
