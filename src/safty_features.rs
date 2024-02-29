#[path = "./data_types.rs"]
mod data_types;

fn test_references(immutable_ref: &data_types::User, mutable_ref: &mut data_types::User) {
    println!("immutable user first name: {}", immutable_ref.first_name);

    println!("[before] mutable user first name: {}", mutable_ref.first_name);   
    mutable_ref.first_name = String::from("Joe");
    println!("[after] mutable user first name: {}", mutable_ref.first_name);
}


/**
 * the annotation 'a will specifiy that the borrowed value is still available after return
 */
fn test_lifetime_annotation<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test() {
    let user1 = data_types::User {
        first_name: String::from("Amir"),
        last_name: String::from("Noori"),
        age: 37,
    };

    let mut user2 = data_types::User {
        first_name: String::from("Jack"),
        last_name: String::from("Brown"),
        age: 40,
    };

    test_references(&user1, &mut user2);

    test_lifetime_annotation("abcd", "xyz");
}
