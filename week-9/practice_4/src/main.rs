use std::fs::OpenOptions;
use std::io::Write;

fn main() {
     let mut file = std::fs::File::create("data.txt").expect("Create Failed");
    file.write_all("Welcome to Rust programming\n"
        .as_bytes()).expect("Write failed");
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Cannot open file");
    file.write_all("\nHello class".as_bytes()).expect("Write failed");
     file.write_all("This is the appendage to the document.It wasn't quite easy".as_bytes()).expect("Write failed");
     println!("file append success");
}
