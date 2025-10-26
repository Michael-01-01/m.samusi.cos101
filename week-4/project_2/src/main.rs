// Rust program to calculate the roots of a quadratic equation
use std::io;

fn main() {
    println!("please input your age \n");
    let mut age = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("You entered an invalid input");
    let age:i32 = age.trim().parse().expect("Input not an integer");

     println!("Rate your experience on a scale of 1.0-10.0 \n");
    let mut exp = String::new();
    io::stdin()
    .read_line(&mut exp)
    .expect("You entered an invalid input");
    let exp:i32 = exp.trim().parse().expect("Input not an integer");

    if age >=40  && exp >= 0 && exp <=10 {

        println!("congratulations! your monthly incentive is 1_560_000");
        let inc1 = 12 * 1_560_000;
         println!("congratulations! your annual incentive is {}",inc1);
    }
    else if age >=30 && age < 40 && exp >= 0 && exp <=10{

         println!("congratulations! your monthly incentive is 1_480_000");
         let inc2 = 12 * 1_480_000;
          println!("congratulations! your annual incentive is {}",inc2);
    }
    else if age < 28 && exp >= 0 && exp <=10{

        println!("congratulations! your monthly incentive is 1_300_000");
        let inc3 = 12 * 1_300_000;
        println!("congratulations! your annual incentive is {}",inc3);
    

    }

     else if exp == 0 {
     println!("congratulations! your monthly incentive is 100_000");
     let inc4 = 12 * 100_000;
     println!("congratulations! your annual incentive is {}",inc4);}

      else if exp < 0
    { println!("Invalid experience");}
       else if exp > 10
    { println!("Invalid experience");}
    
     

}
  