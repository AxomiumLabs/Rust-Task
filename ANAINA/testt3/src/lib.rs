pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// tests in terms of two main categories:
// unit tests
// integration tests.
// Unit tests -->are small and more focused, testing one module in isolation at a time, and can test private interfaces.

//  Integration tests--> are entirely external to your library and use your code in the same way any other external code would,
//   using only the public interface and potentially exercising multiple modules per test.

// #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build
// cfg stands for configuration --> tells Rust that the following item should only be included given a certain configuration option.
// integration tests go in a different directory, they don’t need the #[cfg(test)] annotation.
//  However, because unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.
// Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. To create integration tests, you first need a tests directory.
// we bring all of the test module’s parent’s items into scope with use super::*

//integration tests are entirely external to your library.
//  They use your library, which means they can only call functions that are part of your library’s public API
// --- purpose is to test whether many parts of your library work together correctly---

// To create integration tests, you first need a tests directory.
// create a tests directory at the top level of our project directory, next to src.
//  Cargo knows to look for integration test files in this directory.
// We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.
//
// adder
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     └── integration_test.rs

// three sections of output include the unit tests, the integration test, and the doc tests.
//if any test in a section fails, the following sections will not be run.
//integration test file has its own section, so if we add more files in the tests directory, there will be more integration test sections.
