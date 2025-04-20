enum Direction {
    North,
    South,
    East,
    West
}

enum Shape{
    Circle(f32),
    Rectangle(i32, i32)
}

fn main(){
    let mut my_direction = Direction::East;
    // match my_direction{
    //     Direction::North => print!("Direction is north"),
    //     Direction::East => print!("Direction is East"),
    //     Direction::South => print!("Direction is South"),
    //     Direction::West => print!("Direction is West")
    // }
    if let Direction::North = my_direction{
        println!("Direction is North");
    } else if let Direction::East = my_direction{
        println!("Direction is East");
    } else if let Direction::South = my_direction{
        println!("Direction is South");
    } else if let Direction::West = my_direction{
        println!("Direction is West");
    }
    my_direction = Direction::South;
    match my_direction{
        Direction::North => println!("Direction is North"),
        Direction::East => println!("Direction is East"),
        Direction::South => println!("Direction is South"),
        Direction::West => println!("Direction is West")
    }
    my_direction = Direction::North;
    println!("{}", determine_direction(my_direction));
    my_direction = Direction::West;
    println!("{}", determine_direction(my_direction));

    let circle = Shape::Circle(4.0);
    println!("Area of circle is {}",calculate_area(circle));
    let rectangle = Shape::Rectangle(4, 2);
    println!("Area of Rectangle is {}",calculate_area(rectangle));
}

fn determine_direction (dir: Direction) -> String{
    match dir {
        Direction::North => String::from("Direction is North"),
        Direction::South => String::from("Direction is South"),
        Direction::East => String::from("Direction is East"),
        Direction::West => String::from("Direction is West"),
    }
}

fn calculate_area(shape: Shape) -> f32{
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(length, breadth) => (length*breadth) as f32
    }
}



