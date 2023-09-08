use adder;

 #[test]
fn another_Integration() {
    assert_eq!(4, adder::add_two(2));
}

// another integration test



//If our project is a binary crate that doesn’t have a lib.rs file,
//  we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.
//  Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.