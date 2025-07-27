// Traits are just like Abstract classes in Java or Interfaces in Typesccript

trait Human {
    fn is_adult(&self) -> bool;
    fn role(&self) -> String{
        return String::from("Role is User");
    } // Default function if not defined in the implementation
}
trait Human2 {
    fn print_name(&self);
}

struct User{
    name: String,
    age: i32,
    admin: bool
}

struct Owner{
    name:String
}


impl Human for User {
    fn is_adult(&self) -> bool {
        if self.age >= 18{
            true // same as return true;
        }
        else{
            false
        }
    }

    fn role(&self) -> String {
        if self.admin{
            return String::from("This is Admin");
        }
        return String::from("This is User");
    }
}

impl Human2 for User {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

impl Human for Owner {
    fn is_adult(&self) -> bool {
        true
    }
}



fn main(){

    let user1 = User{
        name:String::from("Vignesh"),
        age: 69,
        admin: true
    };
    let user2 = User{
        name:String::from("Someone"),
        age: 9,
        admin: false
    };

    let owner1 = Owner{
        name:"SomeOwner".to_string()
    };

    println!("User 1 is adult: {}", user1.is_adult());
    println!("User 2 is adult: {}", user2.is_adult());
    user1.print_name();
    println!("User 1 is admin: {}", user1.role());
    println!("User 2 is admin: {}", user2.role());

    user_name(&user2);

    general_name(&user1);
    
    println!("Owner 1 is admin: {}", owner1.is_adult());
    println!("Owner 1 is admin: {}", owner1.role());
    
    // human_name(&owner1); // Cannot be done as Owner struct does not implement Human2 trait
    // general_name(&owner1); // Fails as Owner does not implement Human2 trait
    
}

fn user_name(item: &impl Human2){
    item.print_name();
}

fn general_name<T:Human + Human2>(item: &T){
    item.print_name();
}


