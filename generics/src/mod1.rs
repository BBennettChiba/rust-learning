#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix<V, W>(self, other: Point<V, W>) -> Point<U, W> {
        Point {
            x: self.y,
            y: other.y,
        }
    }
}

pub fn run() {
    let point1 = Point { x: 1, y: 1.1 };
    let point2 = Point { x:"hey", y: 'h' };
    let point3 = point1.mix(point2);
    // println!("{:?}", point1);
    // println!("{:?}", point2);
    println!("{:?}", point3);
}
