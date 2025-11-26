use std::fs;

fn main() {
    fs::remove_file("goodness.txt").expect("could not remove file");
    println!("File is removed");
}
