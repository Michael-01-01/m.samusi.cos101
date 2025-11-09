use std::io;

fn main() {
    println!("MENU                               PRICE");
    println!("P = Poundo yam/Edinkaiko soup      -3200");
    println!("F = Fried rice & Chicken           -3000");
    println!("A = Amala and Ewedu soup           -2500");
    println!("E = Eba and egusi soup             -2000");
    println!("W = White rice & stew              -2500");

    println!("Choose your choice from the list above:");
    let mut food = String::new();
    io::stdin()
    .read_line(&mut food)
    .expect("Failed to read input");
    let food = food.trim().to_uppercase();

     println!("Input Quantity:");
    let mut qty = String::new();
    io::stdin()
    .read_line(&mut qty)
    .expect("Failed to read input");
    let qty:f32 = qty.trim().parse().expect("Input is not an integer");

    match food.as_str() {
        "P" =>{let _price = 3200.0;
            let total = qty * _price;
            println!("Total is {}",total );
            if total > 10_000.0 {let g_total = total * 0.95;
         println!("Total price after 5% discount is {}",g_total );}   
    },
        "F" => {let _price = 3000.0;
             let total = qty * _price;
            println!("Total is {}",total );
            if total > 10_000.0 {let g_total = total * 0.95;
         println!("Total price after 5% discount is {}",g_total );}   
    },
        "A" => {let _price = 2500.0;
            let total = qty * _price;
            println!("Total is {}",total );
            if total > 10_000.0 {let g_total = total * 0.95;
         println!("Total price after 5% discount is {}",g_total );}   
    },
        "E" =>{let _price = 2000.0;
             let total = qty * _price;
            println!("Total is {}",total );
           if total > 10_000.0  {let g_total = total * 0.95;
         println!("Total price after 5% discount is {}",g_total );}   
    },
        "W" => {let _price = 2500.0;
             let total = qty * _price;
            println!("Total is {}",total );
            if total > 10_000.0 {let g_total = total * 0.95;
         println!("Total price after 5% discount is {}",g_total );}   
    },

        _=> println!("You have entered an invalid choice"),

    }
}
