use std::io;

fn main() {
    let mut input1= String::new();
    println!("Input your Temperature Scale in Celsius");
    io::stdin().read_line(&mut input1).expect("Wrong value inputed");
    let celsius:f32 =input1.trim().parse().expect("Wrong value input");

    if celsius < 0.0 {
        println!("The Temperature is at freezing point ");
    }
    else if celsius >=0.0 && celsius<=30.0{
        println!("The Temperature is at Normal range");
    }
    else if celsius>30.0{
        println!("The Temperature is hot");
    }
    else if celsius < -273.0{
        println!("invalid input");
    }
    else{
        println!("invalid input");
    }
     println!("The Temperature in Celsius is {}",celsius );
    
    

    let fahrenheit= (9.0/5.0)*(celsius + 32.0);
      println!("The Temperature in Fahrenheit is {}",fahrenheit );
    let kelvin=celsius + 273.15;
      println!("The Temperature in Kelvin is {}",kelvin );
    
}
