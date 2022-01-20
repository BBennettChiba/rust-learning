struct User {
    username: String,
    email: String,
    signin_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self)-> u32  {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user1 = User {
        username: "Hurin".to_string(),
        email: "bryson@gmail.com".to_string(),
        signin_count: 1,
        active: true,
    };
    let name = user1.username;
    println!("{}", name);
    user1.username = "Hurinfan".to_string();
    println!("{}", name);
    let user2 = build_user("Jim".to_string(), "jim@gmail.com".to_string());
    println!("{}", user2.username);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rect1);
    println!("{:?}", rect1.area());
    println!("{:?}", Rectangle::square(10));
}

fn build_user(username: String, email: String)-> User {
    User {
        username,
        email,
        signin_count: 1,
        active: true,
    }
}