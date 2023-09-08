// integration test
//these are entirely external to your library
 use adder;

 #[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// output include the unit tests, the integration test, and the doc tests

//we can  run a particular integration test function by specifying the test function’s name as an argument to cargo test
//eg :cargo test --test integration_test

