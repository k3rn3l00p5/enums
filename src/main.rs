// enums allow you to define a type by enumerating its possible variants
// enumerators are a set of name values called e
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // enumerator definition with multiple variants
    enum IpAddr {
        V4(u8, u8, u8, u8), // variants of the enumerator
        V6(String),         // associated String values
    } // new custom data type with variants
      // variants of enum are namespaced under its identifier
      // you can put any kind of data inside enums variants: strings, numeric types, or even structs

    // each variant can have different types and amount of associated data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    // data is attached to each variant so there is no need for a struct
    // both are new types called IpAddr with a string attached to the variant

    // enumerator with 4 variants
    enum Message {
        Quit,                       // no data associated with it
        Move { x: i32, y: i32 },    // includes an anonymous struct inside
        Write(String),              // includes a single String
        ChangeColor(i32, i32, i32), // includes 3 i32 values
    }

    // enumerator methods
    impl Message {
        fn call(&self) {
            // method body
        }
    }

    let m = Message::Write(String::from("hello"));
    // m is passed as self in Message method
    m.call();

    // Option enum is a standard library enumerator that allows you to have a value that could be something or nothing
    // Rust doesn't have the null feature
    // This is because if you try to use a null value as a not-null value you'll get an error
    // This property is pervasive and extremely easy to make that mistake
    // A null value is a value that is currently invalid or absent
    // Rust instead has a Option<T> enum that allows a value to be present or absent
    // <T> syntax is a generic type parameter
    // it means the variant of the Option enum can hold one piece of data of any type
    let some_number = Some(5);
    let some_string = Some("a string"); // a value which is something
    let absent_number: Option<i32> = None; // a value which is nothing
                                           // compiler won't let us use some and none values as valid values because they are different types
                                           // if we use Some we need a type annotation for <T> because the compiler can't infer the type

    #[derive(Debug)]
    // the match control flow operator is extremely useful for comparing a value against a series of patterns
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Penny);

    // good usage of some type in Option enum
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // Matches Are Exhaustive so if we forgot match case none it would give us an error
            // this means we must exhaust every last possibility in order for the code to be valid
            // _ however is used to match any possibility left
            // None => None,
            Some(i) => Some(i + 1),
            _ => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest
    let some_u8_value = Some(0u8);
    // replace with better code
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // if let statement (more concise)
    if let Some(3) = some_u8_value {
        // expression is given to the match and the pattern is its first arm
        println!("three");
    } else if let Some(4) = some_u8_value {
    } else {
    } // can have else statements as well as else if let statements
}
