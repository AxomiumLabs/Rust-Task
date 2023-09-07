//Test are uses to check the codes are functioning in expected manner
/**3 actions performed by test are
 *       =>setup the datas
 *       =>run the code 
 *       =>assert the result to what you expect
 * 
 * 
 **/
  //lib crate provide this default test function 
//we can  test this using cargo test command
pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test] //thiis attribute suggest that this is a test function 
    pub fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);//this test pass only if the two parameters inside asserteq are equal
    }

 
    //when running and test this function compiler shows the name and its status
    //eg : it_works..OK     test result :OK      1 pass 0 fail
    //eg : it_works..Failed  test result :Failed     0 pass 1 fail

    //if we mark a test as ignored ,that function  doesn't run in the particular instance

    //we can filter out tests 
    //Eg: cargo test it_works    this will only run the test matches with the parameter name with command 

    #[test] 
    pub fn nomore() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    } 
// cargo test no => this will run the nomore test


//we can see Doc-tests  in output window this indicates that the documentation tests
    //we dont have any documentation test => o testing 0 failed 0 passed

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 9,
            height: 6,
        };
        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(larger.can_hold(&smaller));//can hold method return true here so test pass
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); //we can pass 5 here but test failes 5+2 != 4
        //we can use assert_ne! =>The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal

    }
     
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
        // instead of checking for exact equality to the value returned from the greeting function
        //  we will just assert that the output contains the text of the input parameter.
    }

    #[test]
    #[should_panic]//The test passes if the code inside the function panics
    //the test fails if the code inside the function doesn’t panic.
    fn greeting_contains_name_not() { //THIS TEST WILL FAIL
        let result = greeting("john");
        assert!(
            result.contains("priya"),
            "Greeting did not contain name, value was `{}`",// custom error message 
            result
        );
    }


    #[test]   
    fn adding() -> Result<(), String> {
        if 2 + 9 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))// return an Err instead of panicking
        //  In the body of the function, rather than calling the assert_eq! macro, we return Ok
        //  when the test passes and an Err with a String inside when the test fails.
        }
    }
}

//use super::* =>We use a glob here so anything we define in the outer module is available to this tests module.
//assert_eq! and assert_ne!  =>These macros compare two arguments for equality or inequality, respectively.
//They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed
//
