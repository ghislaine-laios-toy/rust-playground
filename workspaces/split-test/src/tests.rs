use super::*;

#[test]
pub(super) fn it_works() {
    assert_eq!(4, 2 + 2);
}

#[test]
fn internal() {
    assert_eq!(4, internal_adder(2, 2));
}