/*
    data types:

    scalar:
        simple data types

        bool, char, i31, ...
        () : unit type

    compound:
        group of types

        String
        Array: [A, B, C], same types
        Tuple: (A, B , C), arbitary types, size cannot be changed
        &[u8]
        struct

*/

pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

pub enum LogLevel {
    Info,
    Debug,
    Error,
}

fn test_vector() {
    let mut id_list: Vec<i32> = Vec::new();
    id_list.push(100);
    id_list.push(200);
    id_list.push(300);

    for id in &id_list {
        println!("id: {id}")
    }
}

fn test_struct() {
    let user1 = User {
        first_name: String::from("Amir"),
        last_name: String::from("Noori"),
        age: 37,
    };
    print_user_info(&user1)
}

fn print_user_info(user: &User) {
    println!(
        "User {{ first name: {}, last name: {}, age: {} }}",
        &user.first_name, &user.last_name, &user.age
    )
}

fn test_enum() {
    let all_levels: [LogLevel; 3] = [LogLevel::Info, LogLevel::Debug, LogLevel::Error];
    for level in all_levels {
        match level {
            LogLevel::Info => println!("Level is Info"),
            LogLevel::Debug => println!("Level is Debug"),
            LogLevel::Error => println!("Level is Error"),
        }
    }
}

pub fn test() {
    test_vector();
    test_struct();
    test_enum();
}
