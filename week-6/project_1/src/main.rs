use std::io;

fn main() {
    println!("Welcome to Goodness' Kitchen");
    println!("Menu                            Price");
    println!("P= Poundo Yam/ Edinkaiako Soup  3,200");
    println!("F= Fried Rice & Chicken         3000");
    println!("A= Amala & Ewedu Soup           2500");
    println!("E= Eba & Egusi Soup             2000");
    println!("W= White Rice & Stew            2500");
    

    let mut input1=String::new();
    println!("Input the first letter of your order in lowercase");
    io::stdin().read_line(&mut input1).expect("Wrong input");
    let choice:String = input1.to_lowercase().trim().parse().expect("Wrong Input");

    let mut input2=String::new();
    println!("Input Quantity");
    io::stdin().read_line(&mut input2).expect("Wrong input");
    let quantity: f32 = input2.trim().parse().expect("This value is invalid");

    let a = 3200.0;
    let b = 3000.0;
    let c = 2500.0;
    let e = 2000.0;
    let w = 2500.0;

    let discount_price:f32=amount - (0.05*amount);

    if choice=="p" && quantity>0.0{
        amount= a*quantity;
        println!("Your amount is {}",amount );
        
    }
    else if choice=="f" && quantity>0.0{
        amount = b*quantity;
        println!("Your amount is {}",amount );
    }
    else if choice=="a" && quantity>0.0{
        amount = c* quantity;
        println!("Your amount is {}", amount);
    
    }
    else if choice=="e" && quantity>0.0{
        amount = e*quantity;
        println!("Your amount is {}",amount );
    }
    else if choice=="w" && quantity>0.0{
        amount = w*quantity;
        println!("Your amount is {}",amount );
    }
    else{
        println!("Invalid Inputs");

    }

    if amount>10000.0{
       let discount_price:f32=amount - (0.05*amount);
        println!("Your discount is {}",discount_price );
    }
    
    
    
}
   