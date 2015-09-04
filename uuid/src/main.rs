extern crate uuid;

use uuid::Uuid;

fn main() {
    println!("Testing the UUID module!");

    // UUID4 (random)
    let uuid4 = Uuid::new_v4();

    println!("{}", uuid4.to_string());
    println!("{}", uuid4.to_hyphenated_string());
}
