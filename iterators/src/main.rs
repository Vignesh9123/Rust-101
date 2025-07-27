// Three main types of iterators
// 1. iter -> Provides an immutable reference to the values in the collection
// 2. iter_mut -> Provides a mutable reference to the values in the collection
// 3. into_iter -> Moves the (ownership of) collection into the iterator

fn main(){
   iterators();
   adaptors();
}


fn iterators(){
    let mut vec1 = vec!(1,2,3);
    
    let normal_iter = vec1.iter();

    for i in normal_iter{
        println!("{}", i);
    }

    println!("{:?}", vec1);

    
    let mut_iter = vec1.iter_mut();
    
    for i in mut_iter{
        *i = *i + 1;
        println!("{}", i);
    }
    
    println!("{:?}", vec1);
    
    let into_iterator = vec1.into_iter();

    for mut i in into_iterator{
        i = i+1;
        println!("{}", i);
    }

    // println!("{:?}", vec1); // Cannot do this as vec1 is moved in this type of iterator

    // iteration using the .next() (For loop does this under the hood)
    println!("Using next");
    let vec2 = vec!(1,2,3);
    let mut vec2iter = vec2.iter();
    while let Some(val) = vec2iter.next(){
        println!("{}", val);
    }
}

fn adaptors(){
    let vec1 = vec![1,2,4];
    let vec1_iter= vec1.iter();
    let sum: i32 = vec1_iter.sum(); // This is a consumable adaptor because it consumes the iterator
    println!("The sum is {}", sum);

    // for val in vec1_iter{} // Cannot be used as the vec1_iter is consumed or moved
    
    let vec1_iter2 = vec1.iter();

    let another_iter = vec1_iter2.map(|x| x+1); // This is an iterator adaptor, this also owns the iterator but produce another iterator by changing something

    
    
    // for val in vec1_iter2{} // Cannot be used as the vec1_iter2 is moved
    let vec2: Vec<i32> = another_iter.collect(); // This is also a consumable adaptor that converts the iterator into explicitly typed collection
    println!("Newly created Vec 2");
    for val in vec2{
        println!("{}", val);
    } 
}
