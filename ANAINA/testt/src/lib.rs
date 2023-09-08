pub mod mod_check2;
pub mod mod_check3;
pub mod mod_check4;
pub mod mod_check5;

#[cfg(test)]
pub mod module_check;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//Rust includes support for writing automated software tests.
// Rust can’t check that this function will do precisely what we intend (logic) so we have to test
//a test in Rust is a function that’s annotated with the test attribute
//When you run your tests with the cargo test command,
// Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.
//new library project with Cargo, a test module with a test function in it is automatically generated for us

//test result: ok. means that all the tests passed
// Tests fail when something in the test function panics.

// feature helps keep your docs and your code in sync!

//Assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.
// If the value is true, nothing happens and the test passes. If the value is false, the assert! macro calls panic!

// asserteq  assert! macro and passing it an expression using the == operator

// assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal.

// should_panic to our test function. The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.
