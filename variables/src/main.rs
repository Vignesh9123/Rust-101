fn main() {
    numbers();
}

fn numbers(){
    let int32bit = 1000; // Default size of number without explicit specification is i32 -> 32 bits
    print!("32 bit int is: {}", int32bit);
    print!("\n");
    let int8bit: i8 = 127; // 8 bit memory
    println!("8 bit int is: {}", int8bit);
    // Similarly i16, i64, i128 also exists
    let unsignedint32bit: u32 = 1000;
    println!("32 bit unsigned int is: {}", unsignedint32bit);
    
    let unsignedint8bit: u8 = 255; 
    println!("8 bit unsigned int is: {}", unsignedint8bit);
    // Similarly u16, u64, u128 also exists
    
    let float64bit= 1000.001;// Default size of floating point number without explicit specification is f64 -> 64 bits
    println!("64 bit float is: {}", float64bit);
    
    // Note that f8 doesn't exist and f16 is unstable
}