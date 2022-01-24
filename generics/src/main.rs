mod mod1;
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn y(&self) -> &f32 {
        &self.y
    }
}

fn main() {
    let list = vec![1, 46, 1, 67, 134, 623];
    let largest = get_largest(list);
    println!("The largest number is {}", largest);

    let list2 = vec![1, 23, 242, 2341, 54526, 234, 454];
    let largest2 = get_largest(list2);
    println!("The largest number is {}", largest2);

    let char_list = vec!['z', 'b', 'c', 'd', 'e', 'f'];
    let largest3 = get_largest(char_list);
    println!("The largest char is {}", largest3);

    let point1 = Point { x: 1, y: 1 };
    let point2 = Point { x: 2.2, y: 2.4 };
    point1.x();
    point2.y();
    mod1::run();
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
