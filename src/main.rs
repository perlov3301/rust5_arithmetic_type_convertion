fn main() {
    println!("rust5: arithmetic with type conversion");
    // by default integer is i32
    let x:u8 = 9; // 0 .. 255
    let y:u8 = 10; // -128 .. 127
    println!("result is {}", (x+y));
    // by default f64
    let x:f64 = 255.0; 
    let y:f64 = 10.0; 
    let  z = x/y;
    println!("result is : {}", z);
    let z = x*y;
    println!("result is : {}", z);
    let z = x % y; // that gives us remander
    println!("result is : {}", z);
    let x = 255.0f32;
    let y = 10.0f32;
    let z:f32 = x%y;
    println!("result is {}", z);
    let x = 255_u8;
    let y = 10_u8;
    let z = x%y;
    println!("the result is {}", z);
    let x = 127_000i32;
    let y = 10_i32;
    println!("result is {}", x/y);
    let x = 127_000 as i64;
    let y = 10i64;
    println!("result is {}", x/y);
    let x = 127000 as i64;
    let y =10_i32;
    let z = x/(y as i64);
    println!("result with casting is {} ", z);
    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;
    let z = x as i32 / y;
    println!("result after we gave overflow is {}", z);
}
