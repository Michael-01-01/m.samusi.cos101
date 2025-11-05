fn main(){
let name = "Aisha Lawal";
let uni:&str = "Pan-Atlantic university";
let addr:&str = "Km 52 lekki-epe expressway,ibeju-lekki, Lagos";
println!("Name:{}",name );
println!("university: {}, \nAddress: {}",uni,addr );



let department:&'static str = "Computer Science";
let school:&'static str = "School of science and technology";
println!("Department: {}, \nSchool: {}",department,school );


}