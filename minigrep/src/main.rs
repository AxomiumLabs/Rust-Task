use crate::fs::File;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("In file {:?}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
//env-->Inspection and manipulation of the process’s environment.
//args()-->each argument on a separate line
//collect()--> used is to turn one collection into another.
