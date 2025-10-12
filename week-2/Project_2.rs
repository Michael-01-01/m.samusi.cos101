fn main(){
	let tosh:f64 = 450000.0;
	let mac:f64 = 1500000.0;
	let hp:f64 = 750000.0;
	let dell:f64 = 2850000.0;
	let acer:f64 = 250000.0;

	  // Unit price of laptops

	let a:f64 = 2.0 * tosh;
	let b:f64 = 1.0 * mac;
	let c:f64 = 3.0 * hp;
	let d:f64 = 3.0 * dell;
	let e:f64 = 1.0 * acer;

	    // Total amount sold per laptop

	let sum = a + b + c + d + e;
	println!("Total Sum of laptops {}", sum );

	let ave = sum / 5.0;
	println!("Average of sold laptops {}", ave );









}
