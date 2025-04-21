use std::fs;

struct SumError{
    message: String,
    cause: i32
}

fn main(){
    let file_content = fs::read_to_string("C:/Users/A/Desktop/Rust/error-handling/src/example.txt");

    // Without error handling
        // println!("File content: {}", file_content.unwrap());
    

    // With error handling

    // if let Result::Ok(str) =  file_content{
    //     println!("File content: {}", str);
    // }
    // else if let Result::Err(err) = file_content{
    //     println!("Error while reading file: {}", err.to_string());
    // }

    match file_content {
        Ok(str)=> println!("File content: {}", str),
        Err(err)=> println!("Error while reading file: {}", err.to_string())
    }

    let mut num1 = 2;
    let num2 = 3;
    let sum1 = add1(num1, num2);

    match sum1 {
        Ok(sum) => println!("Sum is {}", sum),
        Err(err)=>println!("Error is {}", err)
    }
    num1 = 1;
    let sum2 = add2(num1, num2);

    match sum2 {
        Ok(sum) => println!("Sum is {}", sum),
        Err(err)=>println!("Error is {} and cause is {}", err.message, err.cause)
    }
}

// Custom error throwing functions
fn add1(num1: i32, num2:i32) -> Result<i32, String>{
    if num1 == 1 {
        return Result::Err("Num1 cannot be 1".to_string());
    }
    else {return Result::Ok(num1+num2)};
}
fn add2(num1: i32, num2:i32) -> Result<i32, SumError>{
    if num1 == 1 {
        return Result::Err(SumError{message:"Num1 cannot be 1".to_string(), cause: 1});
    }
    else {return Result::Ok(num1+num2)};
}