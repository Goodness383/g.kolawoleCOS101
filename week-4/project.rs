use std::io;

fn main(){
	let mut input1 = String::new();
	println!("Enter your value for a:  ");
	io::stdin(&mut input1).read_line().expect("Value not valid");
	let a:i32=input1().trim().parse().expect("Not a valid Number");

	let mut input2 = String::new();
	println!("Enter your value for b: ");
	io::stdin(&mut input2).read_line().expect("Value not valid");
	let b:i32=input2().trim().parse().expect("Not a valid Number");

	let mut input3 =String::new();
	println!("Enter your value for b: ");
	io::stdin(&mut input3).read_line().expect("Value not valid");
	let c:i32=input3().trim().parse().expect("Not a valid Number");
   
   let root_one = (-b+(((b).powi(2))-(4*a*c)).powi(1/2))/2*a;
   let root_two = (-b-(((b).powi(2))-(4*a*c)).powi(1/2))/2*a;

   println!("Your first value is {} and Your second value is {}", root_one, root_two);





}

