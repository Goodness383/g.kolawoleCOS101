use std::io;

fn main() {
    println!("Enter your value for a:  ");
    let mut input1 =String::new();
    io::stdin().read_line(&mut input1).expect("Value not valid");
    let a:f64= input1.trim().parse().expect("Not a valid Number");

    println!("Enter your value for b: ");
    let mut input2=String::new();
    io::stdin().read_line(&mut input2).expect("Value not valid");
    let b:f64=input2.trim().parse().expect("Not a valid Number");

    
    println!("Enter your value for c: ");
    let mut input3=String::new();
    io::stdin().read_line(&mut input3).expect("Value not valid");
    let c:f64=input3.trim().parse().expect("Not a valid Number");

    let d = (b.powf(2.0)-(4.0*a*c)).sqrt();
    println!("{}",d );

    if d>0.0{
        let root_one=(-b+d)/(2.0*a);
        let root_two=(-b-d)/(2.0*a);
        println!("the roots of your equation are {} and {}",root_one, root_two );
        println!("They are real roots");
    }

    
   else if d == 0.0 {
    let root = -b / (2.0 * a);
    println!("The root of your equation is {}", root);
    println!("They are real and equal.");
} 
else {
    println!("There are no real roots.");
     }

   


}