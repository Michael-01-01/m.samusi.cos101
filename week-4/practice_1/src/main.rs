//  Rust program to output name and age

use std::io;
fn main() {
    println!("\n Student information management system! ");

   //   Input name
   println!("\n Please enter your name ");
   let mut name = String::new();
     io::stdin()
     .read_line(&mut name)
    .expect("failed to read input");
     println!("Your name is {}",name );

     //   input age
     println!("\n Please enter your age");
     let mut age =  String::new();
      io::stdin()
     .read_line(&mut age)
    .expect("failed to read input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is {}",age);

}
