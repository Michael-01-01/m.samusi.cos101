fn main() {
    let name1 = "Ayomide Adesokan";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Ayomide","Adebare");
    println!("You can also call me {}",name2);
    let facaulty = "Facaulty of science and technology";

    //find and replace
    let school = facaulty.replace("facaulty","School");
    println!("I am a student of the {}",school );
}
