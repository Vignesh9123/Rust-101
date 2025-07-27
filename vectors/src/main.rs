fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec); // [1, 2]
    
    
    for i in vec.iter().enumerate(){
        println!("{} Present at index {}", i.1,i.0);
        /*
        1 Present at index 0
        2 Present at index 1 
        */
    }
    
    // for i in vec{ // vec is moved at this stage
    //     print!("{} ", i); // 1 2 
    // }
    // println!("{:?}", vec); // This is invalid at this stage 
    
    
    /*
    THIS BLOCK IS VALID AS vec is BORROWED, not MOVED
    for i in &vec{
        print!("{} ", i); // 1 2 
    }
    println!("{:?}", vec); // [1, 2]
    */
    
    vec.remove(0);
    println!("{:?}", vec); // [2]
    
    let numbers = vec![1,2,3]; // using vec macro
    println!("{:?}", numbers); // [1, 2, 3]

}
