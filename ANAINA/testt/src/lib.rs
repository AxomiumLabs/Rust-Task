pub mod mod_check2;
pub mod mod_check3;
pub mod mod_check4;

#[cfg(test)]
pub mod module_check;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// #[test]
// #[should_panic]
// fn greater_than_100() {
//     Guess::new(200);
// }

// #[test]
// #[should_panic(expected = "less than or equal to 100")]
// fn greater_than_100() {
//     Guess::new(200);
// }

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

// cargo run compiles your code and then runs the resulting binary
// cargo test compiles your code in test mode and runs the resulting test binary.
//  cargo test --help displays the options you can use with cargo test

// When you run multiple tests, by default they run in parallel using threads, meaning they finish running faster and you get feedback quicker.
//  Because the tests are running at the same time, you must make sure your tests don’t depend on each other or on any shared state,
//   including a shared environment, such as the current working directory or environment variables.

// If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used,
//  you can send the --test-threads flag and the number of threads you want to use to the test binary.

// We can pass the name of any test function to cargo test to run only that test:
// #[ignore] line to the test we want to exclude.
// . If we want to run only the ignored tests, we can use cargo test -- --ignored:
// If you want to run all tests whether they’re ignored or not, you can run cargo test -- --include-ignored.

// tests in terms of two main categories: unit tests and integration tests.
// Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
//  Integration tests are entirely external to your library and use your code in the same way any other external code would,
//   using only the public interface and potentially exercising multiple modules per test.

// #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build
// cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
// integration tests go in a different directory, they don’t need the #[cfg(test)] annotation. However, because unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.
// Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. To create integration tests, you first need a tests directory.
