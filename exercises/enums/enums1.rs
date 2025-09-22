// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move(String),
    ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("just")));
    println!("{:?}", Message::Move(String::from("try")));
    println!("{:?}", Message::ChangeColor(String::from("this")));
}
