




    use crate::add;

 
    
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










