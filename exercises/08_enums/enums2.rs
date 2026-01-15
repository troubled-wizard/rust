#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // A variant with named fields (struct-like)
    Resize { width: u64, height: u64 },

    // A variant holding a single type (the Point struct)
    Move(Point),

    // A variant holding a String
    Echo(String),

    // A variant holding three u8 values (tuple-like)
    ChangeColor(u8, u8, u8),

    // A variant with no data (unit-like)
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
