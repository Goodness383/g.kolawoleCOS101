use std::io;

fn main() {
    println!("Menu");
    println!("Booktitle:Rust for Beginners Code:R Price: 15000");
    println!("Booktitle:AI Basics Code:A Price: 12500");
    println!("Booktitle:Data Structures in Rust Code:D Price: 20000");
    println!("Booktitle:Networking Essentials Code:N Price: 18000");

    println!("Enter book code: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid Input");
   

    println!("Enter your quantity: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input ");
    let quantity:i32 = input2.trim().parse().expect("Invalid input");


   
   
     
    if input1 == "r"   &&  quantity >0{
    let amount  = quantity *15000;
    println!("amount = {}",amount ); }

    else if input1 == "a"&&  quantity >0{
    let amount  = quantity *12500;
    println!("amount = {}",amount ); }
    
    else if input1 == "d" && quantity > 0{
    let amount  = quantity *20000 ;
    println!("amount = {}",amount );}

    else if input1 == "n" && quantity >0{
    let amount  = quantity *18000;
    println!("amount = {}",amount ); }

 

}
