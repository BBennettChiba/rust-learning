use rand;

pub fn add_one(x: i32) -> i32 {
    let mut num = rand::random::<u8>();
    let mut num2 = rand::random::<u8>();
    while num != num2 {
        println!("num:{} is not num2: {}", num, num2);
        num = rand::random::<u8>();
        num2 = rand::random::<u8>();
    }
    println!("num1: {} is num2: {}", num, num2);
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_one_test() {
        assert_eq!(add_one(5), 6);
    }
}
