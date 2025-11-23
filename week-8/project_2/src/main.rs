use std::io;
fn main() {
    let yrs_exp = vec![0,1,2,3,4,5,6,7];

    println!("Welcome to the EY Nigeria interview,please can you tell us your years of experience:");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
    let mut index:usize = input.trim().parse().expect("Error! please input an integer");

    checker(&mut index);   
}

fn checker (index: &mut usize) {
    if *index == 0 {
        println!("Oops!,sorry but you don't have enough level of experience,\nyou can try again in years to come.");
    }

     else if *index == 1 {
        println!("Oops!,sorry but you don't have enough level of experience,\nyou can try again in years to come.");
    }
     else if *index == 2 {
        println!("Oops!,sorry but you don't have enough level of experience,\nyou can try again in years to come.");
    }
     else if *index == 3 {
        println!("Oops!,sorry but you don't have enough level of experience,\nyou can try again in years to come.");
    }
     else if *index == 4 {
        println!("Oops!,sorry but you don't have enough level of experience,\nyou can try again in years to come.");
    }

    else if *index == 5 {
        println!("Oops!,sorry but you don't have enough level of experience,\nyou can try again in years to come.");
    }
    else if *index == 6 {
        println!("Oops!,sorry but you don't have enough level of experience,\nyou can try again in years to come.");
    }

    else if *index == 7 {
        println!("Congratulations! you are now qualified to work with EY Nigeria.");
    }
    else if *index > 7 {
        println!("Wow! your level of experience surpassed our expectation you are now qualified.");
    }
}