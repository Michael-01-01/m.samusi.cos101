use std::io;
fn main() {
    let vec1 = vec!["Intern","Paralegal","Placement"];
    let vec2 = vec!["Administrator","Research assistant","Junior Associate","Classroom Teacher"];
    let vec3 = vec!["Senior Administrator","phD Candidate","Associate","Snr teacher"];
    let vec4 = vec!["Office manager","Post-Doc Researcher","Senior Associate 1-2","Leading Teacher"];
    let vec5 = vec!["Director","Senior lecturer","Senior Associate 3-4","Deputy principal"];
    let vec6 = vec!["CEO","Dean","Patner","Principal"];
    println!("Please input public service:");
    let mut input1 = String::new();
    io::stdin()
    .read_line(&mut input1)
    .expect("Failed to read input");
    let mut input:&str =input1.trim();

    match input {
        "Intern" => println!("Your Aps level is 1-2"),
         "Paralegal" => println!("Your Aps level is 1-2"),
         "Placement" => println!("Your Aps level is 1-2"),
         "Administrator" => println!("Your Aps level is 3-5"),
         "Research assistant" => println!("Your Aps level is 3-5"),
         "Junior Associate" => println!("Your Aps level is 3-5"),
         "Classroom Teacher" => println!("Your Aps level is 3-5"),
         "Senior Administrator" => println!("Your Aps level is 5-8"),
         "phD Candidate" => println!("Your Aps level is 5-8"),
         "Associate" => println!("Your Aps level is 5-8"),
         "Snr teacher" => println!("Your Aps level is 5-8"),
         "Office manager" => println!("Your Aps level is EL1 8-10"),
         "Post-Doc Researcher" => println!("Your Aps level is EL1 8-10"),
         "Senior Associate 1-2" => println!("Your Aps level is EL1 8-10"),
         "Leading Teacher" => println!("Your Aps level is EL1 8-10"),
         "Director" => println!("Your Aps level is EL2 10-13"),
         "Senior lecturer" => println!("Your Aps level is EL2 10-13"),
         "Senior Associate 3-4" => println!("Your Aps level is EL2 10-13"),
         "Dean" => println!("Your Aps level is EL2 10-13"),
         "Patner" => println!("Your Aps level is SES"),
         "Deputy principal" => println!("Your Aps level is SES"),
         "CEO" => println!("Your Aps level is SES"),
         "Principal" => println!("Your Aps level is SES"),
         _ =>println!("Wrong input"),
    }

}