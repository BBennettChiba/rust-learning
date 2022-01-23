use std::fs::{File, self};
use std::io::{Read, ErrorKind, self};
fn main() {
    fn a() {
        b();
    }
    fn b() {
        c(4);
    }
    fn c(arg: i32) {
        if arg == 3 {
            panic!("no!!!!!");
        }
    }
    a();

    // let text = fs::read_to_string("hello.txt");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(file) => file,
                Err(e) => panic!("error creating file: {:?}", e)
            },
            other_error => panic!("other problem opening file, {:?}", other_error)
        } 
    };

    // let f2 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("failed to open");
    println!("f2: {:?}", f2);

    let username = read_username_from_file();
    println!("username: {:?}", username.unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    //let mut f = File::open("hello.txt")?; //? here does the same thing as the commented out block below
    // File::open("hello.txt")?.read_to_string(& mut s)?; // simplified version of line above and line 51
    // let mut f = match f {
    //     Ok (file) => file,
    //     Err (e) => return Err(e)
    // };

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e)=> Err(e)
    // }

    // f.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("hello.txt")
}