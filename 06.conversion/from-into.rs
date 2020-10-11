use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {

    let my_str    : &str    = "hello";               // stack allocated
    let my_string : String = String::from(my_str);   // heap allocated
    
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let new_num : Number = int.into();
    println!("My number is {:?}", new_num);
}
