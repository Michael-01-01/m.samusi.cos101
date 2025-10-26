// Rust program to calculate the roots of a quadratic equation
use std::io;

fn main() {
    println!("what is your age\n");
    let mut age = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("You entered an invalid age");
    let age:f32 = age.trim().parse().expect("You entered an invalid age");
      println!("How experienced are you in a scale of 1.0 - 10.0 \n");
    let mut exp = String::new();              //boolean 
    io::stdin()
    .read_line(&mut exp)
    .expect("You entered an invalid input");
    let exp:f32 = exp.trim().parse().expect("You entered an invalid experience range");
   
    if age >= 40.0 && exp > 0.0 && exp <=10.0  {
        println!("Congratulations! your monthly incentive is $1_560_000");
        let inc_1 = 12 * 1_560_000;
        println!("Congratulations! your Annual incentive is {}",inc_1);
    }

    
     else if age >= 30.0 &&  age < 40.0 && exp > 0.0 && exp <=10.0  {
        println!("Congratulations! your monthly incentive is $1_480_000");
        let inc_2 = 12 * 1_480_000;
         println!("Congratulations! your Annual incentive is {}",inc_2);
    }

     else if age < 28.0 && exp > 0.0 && exp <=10.0  {
        println!("Congratulations! your monthly incentive is $1_300_000");
        let inc_3 = 12 * 1_300_000;
         println!("Congratulations! your Annual incentive is {}",inc_3);
    }
     else if exp == 0.0 {
        println!("Congratulations! your monthly incentive is $100_000");
        let inc_4 = 12 * 100_000;
         println!("Congratulations! your Annual incentive is {}",inc_4);
    }
    else  if  exp < 0.0  {
        println!("Out of experience range");
    }
     else  if  exp > 10.0  {
        println!("Out of experience range");}
}
