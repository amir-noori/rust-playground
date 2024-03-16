use std::convert::From;
use std::convert::TryFrom;

#[derive(Debug)]
struct MyNumber {
    value: i32,
    extra: String
}

#[derive(Debug)]
struct EvenNumber {
    value: i32
}

impl From<i32> for MyNumber {
    fn from(data: i32) -> Self {
        MyNumber {
            value: data,
            extra: String::from("")
        }
    }
}

impl TryFrom<i32> for EvenNumber {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber{value: value})
        } else {
            Err(String::from("Not Even!"))
        }
    }
}

pub fn test() {
    let num = MyNumber::from(200);
    println!("my number is: {:?}", num);

    let even = EvenNumber::try_from(101);
    match even {
        Err(error) => println!("error converting: {}", error),
        Ok(_) => println!("convertion is ok.")
    }
}

