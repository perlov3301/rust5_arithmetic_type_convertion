fn main() {
    println!("rust5: arithmetic with type conversion");
    // by default integer is i32
    let x:u8 = 9;
    let y:u8 = 10;
    println!("result is {}", (x+y));
    let x:f64 = 255.; // 0 .. 255
    let y:f64 = 10.; // -128 .. 127
    let mut z = x/y;
    println!("result is : {}", z);
    z = x*y;
    println!("result is : {}", z);
    z = x % y; // that gives us remander
    println!("result is : {}", z);
}
