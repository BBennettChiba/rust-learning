fn main() {
    let x = 1;
    let closure1 = || x + 2;
    println!("{}", closure1());
    println!("{}", x);

    let x = "hello world".to_string();
    let closure2 = || x + "!";
    // println!("{}", x);
    println!("{}", closure2());

    let mut x = 1;
    let mut closure3 = || {
        x += 2;
        x + 3
    };
    println!("{}", closure3());
    println!("{}", x);
}
