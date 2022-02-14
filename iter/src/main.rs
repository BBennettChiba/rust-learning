#[derive(PartialEq, Debug)]
struct Shoe {
    size: u8,
    name: String,
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn filter_by_size(size: u8, shoes: Vec<Shoe>) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

fn main() {
    let vec = Vec::from([1, 2, 3, 4, 5, 6]);
    let vec_iter = vec.iter();
    for value in vec_iter {
        println!("{}", value);
    }
}

#[test]
fn interator_demo() {
    let vec = vec![1, 2, 3];
    let mut vec_iter = vec.iter();
    assert_eq!(vec_iter.next(), Some(&vec[0]));
    assert_eq!(vec_iter.next(), Some(&vec[1]));
    assert_eq!(vec_iter.next(), Some(&vec[2]));
    assert_eq!(vec_iter.next(), None);
}

#[test]
fn sum_test() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let vec_iter = vec.iter();
    let total: i32 = vec_iter.sum();
    assert_eq!(total, 21);
}

#[test]
fn map_test() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let map_vec: Vec<i32> = vec.iter().map(|x| x + 1).collect();
    assert_eq!(map_vec, vec![2, 3, 4, 5, 6, 7]);
}
#[test]
fn test_shoe_filter() {
    let shoes = vec![
        Shoe {
            size: 10,
            name: "bob".to_string(),
        },
        Shoe {
            size: 11,
            name: "bob2".to_string(),
        },
        Shoe {
            size: 10,
            name: "bobby".to_string(),
        },
    ];
    let filtered_shoes = filter_by_size(10, shoes);
    assert_eq!(
        filtered_shoes,
        vec![
            Shoe {
                size: 10,
                name: "bob".to_string()
            },
            Shoe {
                size: 10,
                name: "bobby".to_string()
            }
        ]
    );
}

#[test]
fn counter_test() {
    let mut counter = Counter { count: 0 };
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
#[test]
fn weird_test() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| {
            println!("a: {}, b: {}", a, b);
            a * b
        })
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}
