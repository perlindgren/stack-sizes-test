use std::fs;

fn main() {
    let file_path = "ex1";

    println!("In file {file_path}");

    let contents = fs::read(file_path).expect("Should have been able to read the file");

    let r = stack_sizes::analyze_executable(&contents);
    println!("r {:?}", r);
}
