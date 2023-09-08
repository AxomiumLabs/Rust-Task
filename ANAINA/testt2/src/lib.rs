fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

// Test Attributes:

// `#[test]`        - Indicates a function is a test to be run. This function
//                    takes no arguments.
// `#[bench]`       - Indicates a function is a benchmark to be run. This
//                    function takes one argument (test::Bencher).
// `#[should_panic]` - This function (also labeled with `#[test]`) will only pass if
//                     the code causes a panic (an assertion failure or panic!)
//                     A message may be provided, which the failure string must
//                     contain: #[should_panic(expected = "foo")].
// `#[ignore]`       - When applied to a function which is already attributed as a
//                     test, then the test runner will ignore these tests during
//                     normal test runs. Running with --ignored or --include-ignored will run
//                     these tests.

// cargo run compiles your code and then runs the resulting binary
// cargo test compiles your code in test mode and runs the resulting test binary.

//  cargo test --help displays the options you can use with cargo test
// cargo test -- --test-threads=1-->run the tests one at a time.

// To show the output of successful tests with --show-output

// If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used,
//  you can send the --test-threads flag and the number of threads you want to use to the test binary.
//if you want to test only one part then -->cargo test <test function name>

// We can pass the name of any test function to cargo test to run only that test:
// #[ignore] line to the test we want to exclude.
// . If we want to run only the ignored tests, we can use cargo test -- --ignored:
// If you want to run all tests whether they’re ignored or not, you can run cargo test -- --include-ignored.
