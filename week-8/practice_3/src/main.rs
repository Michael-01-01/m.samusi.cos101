//Method to print the get value 
fn value(n:Option<&char>)
{
    println!("Element of vector {:?}",n );
}

fn main() {
    let v = vec!["R","V","S","T","A","C","I","A","N"];

    let mut input1 = String::new();
    println!("\nEnter an index value btw (0 - 8)");
    std::io::stdin().read_line(&mut input1).expect("Invalid input");

    //Getting value at given index value
    let ch: Option<&char> = v.get(3);
    value(ch);
}
