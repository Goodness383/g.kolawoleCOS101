fn main() {
	let p:f64 = 520000000.00;
	let r:f64 = 10.00;
	let t:f64 = 5.00;

	// compound interest 
	let a = p*(1.00 + (r/100.00))*t;
	println!(" The amount is equal to {}", a );
	let ci = a - p;
	println!("The Compound Interest is {} ", ci);
}



