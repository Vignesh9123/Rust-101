use std::collections::HashMap;

fn main() {
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert("Vignesh".to_string(), 1);
    users.insert("Someone".to_string(), 3);

    for i in &users{
        println!("{:?}->{}", i.0, i.1);
    }

    // for (name, mapped_val) in &users{ // Same as above
    //     println!("{:?}->{}", name, mapped_val);
    // }

    let mut rooms: HashMap<i32, i32> = HashMap::new();
    rooms.insert(1, 2);
    rooms.insert(2, 10);
    println!("Mapped value for \"Vignesh\" is {}", users.get("Vignesh").unwrap());
    println!("Mapped value for \"Someone\" is {}", users.get(&String::from("Someone")).unwrap());

    println!("{} present in room 1",rooms.get(&1).unwrap() );
    println!("{} present in room 2",rooms.get(&2).unwrap() );

    users.remove("Vignesh");

    println!("{:?}", users); // {"Someone": 3}    

}
