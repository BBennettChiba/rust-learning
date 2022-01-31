#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }
}

pub fn double(x: i32) -> i32 {
    x * 2
}

fn greet(name: &str) -> String {
    format!("Hello {}!", name)
    // format!("Hello!")
}   

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value < 0 || value > 100 {
            panic!("Value must be between 0 and 100");
        } else {
            Guess { value }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn fail(){
        // panic!("fail");
    }
    #[test]
    fn rect_test(){
        let bigger = Rectangle { width: 70, height: 80 };
        let smaller = Rectangle { width: 50, height: 70 };
        assert!(bigger.can_hold(&smaller));
        assert!(!smaller.can_hold(&bigger));
    }
    #[test]
    fn double_test(){
        assert_eq!(double(5), 10);
        assert_ne!(double(4), 10);
    }
    #[test]
    fn test_greet(){
        let name = "bryson";
        let greeting = greet(name);
        assert!(greeting.contains(name), "Greeting did not contain name: {}, value was {}", name, greeting);
        assert_eq!(greeting,"Hello bryson!");
    }

    #[test]
    #[should_panic(expected = "Value must be between 0 and 100")]
    fn test_guess_new(){
        Guess::new(200);
    }
}
