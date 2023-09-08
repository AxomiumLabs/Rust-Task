
// grep (globally search a regular expression and print)
//it searches a specified string or a file
//we can give filepath and string as arguments and grp go to the filepath ,re4ad the file and finds the string
use std::env;
use std::fs;

fn main(){
    let args_collection:Vec<String>=env::args().collect();
    println!("{:?}",args_collection);
    let query=&args_collection[1];
    let file_name=&args_collection[2];
    println!("{:?}",query);
    println!("{:?}",file_name);

    let contents=fs::read_to_string(file_name).expect("should be able to read file");
    println!("\n{contents}");

}

//cargo run --searchstring example-file.txt=> we can put this on terminal without an existing  file.txt , but this will give no output 
//example is the string and fil.txt is the filepath

//We can read the values in the commandline argument
//std::env::args =>this will help us 
//this returns an iterator over command line values
//we can use collect method to turn the iterator into a collection like vector
//we have specify the type of collection by type annotation
//std::env::args will panic when commandline containes invalid unicode
//we can use  std::env::args_os instead of this ,it will accept invalid unicode 
//cargo run searchstring example-file.txt =>the vec is  ["target\\debug\\minigrep.exe", "searchstring", "example-file.txt"]


//SECTION 2
//we can read a file using fs module




