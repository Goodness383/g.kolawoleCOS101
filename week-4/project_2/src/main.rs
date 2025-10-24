use std::io;

fn main() {
    println!("On a scale of 1 to 10 how experienced are you?:  ");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).expect("your answer is not correct");
    let experience:u32=input1.trim().parse().expect("Not a valid number");

    println!("Whats your age?:  ");
    let mut input2=String::new();
    io::stdin().read_line(&mut input2).expect("your answer is not correct");
    let age:u32=input2.trim().parse().expect("Not a valid number");



    
    

if experience >= 6 && age >= 40 {
        println!("Your incentive is 1,560,000");
    } else if experience >= 6 && age >= 30 && age < 40 {
        println!("Your incentive is 1,480,000");
    } else if experience >= 6 && age < 28 {
        println!("Your incentive is 1,300,000");
    } else if experience < 6 {
        println!("You're not experienced, so your incentive is 100,000");
    } else {
        println!("error");
    }

}



