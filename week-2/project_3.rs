fn main() {
	let p: f64 = 510000.00;
	let r: f64 = 5.00;
	let t: u32 = 3;

	// Depreciation 
	let amount  = p*(1.00-(r/100.00)).powi(t as i32);
	let  depreciation= amount - p ;
	println!("The depreciation is {} ", depreciation);


}





