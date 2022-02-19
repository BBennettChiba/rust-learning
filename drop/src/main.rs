
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let string1 = CustomSmartPointer{data: String::from("yo yo yo")};
    let string2 = CustomSmartPointer{data: String::from("yo yo yo 2")};
    println!("{}", string1.data);
    println!("{}", string2.data);
    drop(string1);
}
