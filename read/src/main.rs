use std::io::{BufReader,BufRead};
use std::fs::File;
 
fn main() {
    let file = File::open("src/main.rs").expect("Can't read file");
    for line in BufReader::new(file).lines() {
        println!("{}", line.unwrap());
    }
}
