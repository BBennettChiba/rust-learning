use std::io;
enum Language {
    English,
    Spanish,
    Chinese,
    Japanese,
    Russian
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(line)=> {
            println!("{}", line);
        }
        Err(err)=> println!("Error reading line: {}", err)
    }
    // println!("language: {}", language);
    // let langage = match language {
    //     "English" => Language::English,
    //     "Spanish" => Language::Spanish,
    //     "Chinese" => Language::Chinese,
    //     "Japanese" => Language::Japanese,
    //     "Russian" => Language::Russian,
    //     _ => Language::English
    // };
    // match language {
    //     Language::English => println!("Hello!"),
    //     Language::Spanish => println!("Hola!"),
    //     Language::Chinese => println!("你好!"),
    //     Language::Japanese => println!("こんにちは!"),
    //     Language::Russian => println!("Привет!"),
    //     _ => println!("Hello!")
    // }
}
