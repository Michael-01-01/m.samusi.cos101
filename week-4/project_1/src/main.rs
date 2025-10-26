// Rust program to calculate the roots of a quadratic equation
use std::io;

fn main() {
    println!("please input the value of a \n");
    let mut a = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("You entered an invalid input");
    let a:i32 = a.trim().parse().expect("Input not an integer");

     println!("please input the value of b \n");
    let mut b = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("You entered an invalid input");
    let b:i32 = b.trim().parse().expect("Input not an integer");

     println!("please input the value of c \n");
    let mut c = String::new();
    io::stdin()
    .read_line(&mut c)
    .expect("You entered an invalid input");
    let c:i32 = c.trim().parse().expect("Input not an integer");


    //Algorithm for the determinant

    let d = b * b -( 4 * a * c);
    let p = d ^ (1/2);
    let q = 2 * a;
    println!("The determinant is {}",d);
    println!("The p is {}",p);
    println!("The q is {}",q);
if d > 0 {
    
    let r1 = (-b + p) / q; 
     let r2 = (-b -  p) / q;
     println!("The roots of the quadratic equation are {},{}", r1,r2);
}
else if d == 0 {

      let r1 = (-b + p) / q; 
     let r2 =  (-b - p) / q; 
     println!("The root of the quadratic equation is {},{}", r1,r2);
}

else if d < 0 {


    println!("There are no real roots")
}


}
