use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {
    let names = ["Oluchi Mordi","Adams Aliyu"," Shania Bolade","Adekunle Gold","Blanca Edemoh"]; 

    let matric_no = ["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"]; 

    let department = ["Accounting","Economics","Computer","Electrical","Mechanical"]; 

    let level = [300,100,200,200,100];

    println!("{:?},{:?},{:?},{:?}",names,matric_no,department,level);

    let mut file = File::create("Student Details.csv").expect("Unable to create file");
    writeln!(file,"Name,matric_no,department,level").expect("write failed");

    for i in 0..names.len() {
        writeln!(file,"{},{},{},{}",names[i],matric_no[i],department[i],level[i]).expect("Failed to write");
    }

   
}
