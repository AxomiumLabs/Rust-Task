use super::*;

#[test] //this attribute indicates this is a test function, so the test runner knows to treat this function as a test
pub fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
pub fn it_work2() {
    let result = add(2, 5);
    assert_eq!(result, 7);
}
