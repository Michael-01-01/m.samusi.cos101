use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {

    let message = "Welcome to file creation\n";
     let mut file = File::create("Welcome_message.txt").expect("Create Failed");
    file.write_all("Diving deeper into Rust programming\n"
        .as_bytes()).expect("Write failed");
    file.write_all(message.as_bytes()).expect("Write failed");
    println!("\nData written to file.");

    file = File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
