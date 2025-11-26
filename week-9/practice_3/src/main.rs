use std::fs::File;
use std::io::Write;

fn main() {
         let message = "Welcome to file deletion\n";
     let mut file = File::create("remove_file.txt").expect("Create Failed");
    file.write_all("Diving more into Rust programming\n"
        .as_bytes()).expect("Write failed");
    file.write_all(message.as_bytes()).expect("Write failed");
    println!("\nData written to file.");
    std::fs::remove_file("remove_file.txt").expect("Could not remove file");
    println!("File is removed");
}
