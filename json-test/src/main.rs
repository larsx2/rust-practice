extern crate rustc_serialize;

use rustc_serialize::json;
use rustc_serialize::json::Json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
enum Sex {
    Male,
    Female,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct User {
    name: String,
    age: u8,
    sex: Sex,
}

fn main() {
    let user = User {
        name: "Eduardo".to_string(),
        age: 28u8,
        sex: Sex::Male,
    };

    let encoded = json::encode(&user).unwrap();
    let decoded: User = json::decode(&encoded).unwrap();

    println!("Encoded: {}", encoded);
    println!("Decoded: {:?}", decoded);

    let from_str = "{ \"name\": \"Eddie\", \"age\": 28, \"sex\": \"male\" }";
    let encoded_from_str = json::Json::from_str(&from_str).unwrap();
    
    println!("Encoded from string: {}", encoded_from_str);

    let as_object = encoded_from_str.as_object().unwrap();
    let age = as_object.get("age").unwrap();

    println!("Age extracted from decoded object: {:?}", age);

    for (key, val) in as_object.iter() {
        println!("{} {}", key, match *val {
            Json::U64(v)    => format!("Found u64 -> {}", v),
            Json::String(ref v) => format!("Found String  -> {}", v),
            _               => format!("Other"),
        });
    }
}
