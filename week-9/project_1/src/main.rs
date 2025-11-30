use std::fs::File;
use std::io::Write;

fn main() {
    let announce = "Welcome to Nigeria Breweries plc";
    let drink_1 = "33 Export      Legend         Maltina\n";
    let drink_2 = "Desperados     Turbo King     Amstel Malta\n";
    let drink_3 = "Goldberg       Williams       Malta Gold\n";
    let drink_4 = "Gulder                        Fayrouz\n";
    let drink_5 = "Heineken\n";
    let drink_6 = "Star\n";


    let mut file = File::create("Drinks store.txt").expect("Failed to create file");
    file.write_all(announce.as_bytes()).expect("Write failed");
    file.write_all("\nLAGER          STOUT          NON-ALCOHOLIC\n".as_bytes()).expect("Write failed");
    file.write_all(drink_1.as_bytes()).expect("Write failed");
    file.write_all(drink_2.as_bytes()).expect("Write failed");
    file.write_all(drink_3.as_bytes()).expect("Write failed");
    file.write_all(drink_4.as_bytes()).expect("Write failed");
    file.write_all(drink_5.as_bytes()).expect("Write failed");
    file.write_all(drink_6.as_bytes()).expect("Write failed");


    println!("Data written to file");
}