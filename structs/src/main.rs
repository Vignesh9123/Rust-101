struct User{
    name: String,
    age: u16,
    active: bool,
    initial: char,
    has_a_gf: Option<bool>
}



fn main(){
    let user = User {
        name: String::from("Vignesh"),
        age: 21,
        active: true,
        initial: 'D',
        has_a_gf: Some(false)
    };
    // The object above is immutable i,e
    // user.has_a_gf = Some(true); // This is not valid, hence Vignesh cannot have a gf ever 🥲
    let name=  "Someone".to_string();

    let mut user2 = User {
        name,
        age:20,
        active: false,
        initial: 'D',
        has_a_gf:Some(false)
    };
    user2.has_a_gf = Some(true); // This is valid as user2 is mutable

    print!("Name:{:?}\nInitial:{:?}\nAge:{:?}\nActive:{:?}\nHas a Gf:{}\nIsAdult:{}", user.name,user.initial, user.age, user.active, user.has_a_gf.unwrap_or(false), user.is_adult() ); // :? adds a double quote around if it's a string or single quote if its a char that is being printed, remains same for other datatypes
}

impl User { // Implementation of Structs
    fn is_adult(&self) -> bool{
        if self.age < 18 {return false;}
        else {return true;}
    }
}