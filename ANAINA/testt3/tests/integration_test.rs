use testt3;
mod common;
#[test]
fn it_adds_two() {
    assert_eq!(4, testt3::add_two(2));
    common::callme();
}
