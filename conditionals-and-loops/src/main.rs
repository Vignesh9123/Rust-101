fn main() {
    
    // if

    let b1 = true;

    if b1 {
        println!("Hey it is true");
    }
    else{
        println!("Hey it is false");
    }

    let mut i1 = 2;
    if i1 == 1 {
        println!("It is 1");
    }
    else if i1 == 2 {
        println!("It is 2");
    }
    else {
        println!("It is neither 1 nor 2");
    }
    
    // for loop
    for i in 1..10{
        print!("{}", i); // prints 0 - 9
    }

    let string: String = String::from("Hello World");
    for c in string.chars(){
        print!("\n{}", c);
    }
    
    // while loop

    while i1 < 10 {
        i1 += 1;
        println!("{}", i1);
    }
}
