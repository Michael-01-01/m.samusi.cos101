struct laptops {
    name:String,
    price:u32,
    quantity:u32,
}

fn main() {
    let brand_1 = laptops{
        name: String::from("HP Laptop"),
        price:650_000,
        quantity:3,
    };

    let brand_2 = laptops{
        name :String::from("IBM Laptop"),
        price:755_000,
        quantity:3,
    };
    let brand_3 = laptops{
        name: String::from("Toshiba Laptop"),
        price:550_000,
        quantity:3,
    };
    let brand_4 = laptops{
        name:String::from("Dell Laptop"),
        price:850_000,
        quantity:3,
    } ;

    let purchase_1 = brand_1.price * brand_1.quantity;
    let purchase_2 = brand_2.price * brand_2.quantity;
    let purchase_3 = brand_3.price * brand_3.quantity;
    let purchase_4 = brand_4.price * brand_4.quantity;
    
    println!("The total purchase is {}",purchase_1 + purchase_2 + purchase_3 + purchase_4 );

}