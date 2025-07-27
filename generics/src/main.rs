fn main() {
    println!("{}", general_compare(1,2));
    println!("{}", general_compare("Vignesh","Harkirat"));
    println!("{}", general_compare(true,false));
}

fn general_compare<T:std::cmp::PartialOrd>(a:T, b:T)->T{ // Here T is a generic and :std::cmp::PartialOrd says that T should be something which can be compared eg number, strings etc. It cannot be a struct or something which cannot be compared
    if a > b{
        return a;
    }
    else{
        return b;
    }
}
