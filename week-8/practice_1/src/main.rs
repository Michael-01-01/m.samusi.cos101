fn main() {

    //Using vec::new()
    let vector : Vec<i64> = Vec::new();

    //Printing the size of vector
    println!("\nThe length of Vec::new is {}",vector.len() );

    //Using macro
    let vector_macro = vec!["Michael","Effiong","Obinna","Kareem"];
    //Printing the size of vector
    println!("\nThe length of vec macro is:{}",vector_macro.len());
}
