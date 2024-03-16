#[path = "./data_types.rs"]
mod data_types;

use self::data_types::User;

fn check_user(user: &User) {
    println!(
        "user info: first name: {}, last name: {}, age: {}",
        user.first_name, user.last_name, user.age
    )
}

fn update_user(mut user: User) -> User {
    let name = String::from("Jim");
    user.first_name = name;
    // name is no longer available here cause it is already borrowed by user
    user
}

fn take_ownership(name: String) {
    println!("value is {name}");
}

fn use_reference(name: &String) {
    println!("value is {name}");
}

fn takes_and_gives_back(name: String) -> String {
    println!("value is {name}");
    name
}

pub fn test() {
    let mut user = User {
        first_name: String::from("Joe"),
        last_name: String::from("Brown"),
        age: 30,
    };

    check_user(&user);
    user = update_user(user);
    check_user(&user);

    // let name = String::from("Joe");
    // take_ownership(name);
    // // name is no longer available here

    // let mut name2 = String::from("Jim");
    // use_reference(&name2);
    // println!("after calling use_reference {name2}");

    // name2 = takes_and_gives_back(name2);
    // println!("after calling takes_and_gives_back {name2}");
}
