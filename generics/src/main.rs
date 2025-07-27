
struct User{
    name:String
}
fn main() {
    println!("{}", general_compare(1,2));
    println!("{}", general_compare("Vignesh","Harkirat"));
    println!("{}", general_compare(true,false));
    let user1 = User{
        name:"Vignesh".to_string()
    };
    let user2 = User{
        name:"Someone".to_string()
    };
    // println!("{:?}", general_compare(user1,user2).name); // This fails because the struct User doesn't implement PartialOrd

}

fn general_compare<T:std::cmp::PartialOrd>(a:T, b:T)->T{ // Here T is a generic and :std::cmp::PartialOrd says that T should be something which can be compared eg number, strings etc. It cannot be a struct or something which cannot be compared
    if a > b{
        return a;
    }
    else{
        return b;
    }
}
