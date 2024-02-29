

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
    let name = String::from("Joe");
    take_ownership(name);
    // name is no longer available here

    let mut name2 = String::from("Jim");
    use_reference(&name2);
    println!("after calling use_reference {name2}");

    name2 = takes_and_gives_back(name2);
    println!("after calling takes_and_gives_back {name2}");
}
