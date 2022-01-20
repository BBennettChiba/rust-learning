fn main() {
    let mut counter = 0;
    let sum = loop {
        if counter == 100 {
            break counter * 2;
        }
        counter += 1;
    };
    println!("The sum is {}", sum);
}
