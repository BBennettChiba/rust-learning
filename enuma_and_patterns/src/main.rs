// enum IpAddrKind {
//     V4,
//     V6,
// }

// enum Something {
//     A,
//     B(String),
// }

// impl Something {
//     fn something_fun(){
//         println!("something");
//     }
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
//     Colorado,
//     Connecticut,
//     Delaware,
//     Florida,
//     Georgia,
//     Hawaii,
//     Idaho,
//     Illinois,
//     Indiana,
//     Iowa,
//     Kansas,
//     Kentucky,
//     Louisiana,
//     Maine,
//     Maryland,
//     Massachusetts,
//     Michigan,
//     Minnesota,
//     Mississippi,
//     Missouri,
//     Montana,
//     Nebraska,
//     Nevada
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let something = Something::A;
    // let something_b = Something::B(String::from("something"));
    // Something::something_fun();

    // let some_number = Some(5);
    // let some_string = Some("a string");

    // let absent_number: Option<i32> = None;

    // println!("{} + {} = {}", some_number.unwrap(), absent_number.unwrap_or(5), some_number.unwrap() + 5);

    // let coin = Coin::Quarter(UsState::Maine);
    // coin_to_cents(coin);
    let x = Some(5);
    let y = plus_one(x);
    println!("{:?}", y.unwrap());
}

// fn coin_to_cents(coin: Coin){
//     match coin {
//         Coin::Penny => println!("Penny"),
//         Coin::Nickel => println!("Nickel"),
//         Coin::Dime => println!("Dime"),
//         Coin::Quarter(state) => println!("Quarter from {:?}", state),
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}