use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input and Output\n";
    let dept = "Department of computer science";

    let mut file = std::fs::File::create("data.txt").expect("Create Failed");
    file.write_all("Welcome to Rust programming\n"
        .as_bytes()).expect("Write failed");
    file.write_all(announce.as_bytes()).expect("Write failed");
    println!("\nData written to file.");
}
