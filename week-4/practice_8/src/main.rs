 fn main() {
    // while true 
    let mut x:i64 = 1;
    loop{
        x+=10;
        print!("x={}\n",x );

        if x>=100{
            break;
        }
    }
}
