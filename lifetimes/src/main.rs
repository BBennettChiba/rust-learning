// fn main() {
//     let x;
//     {
//         let r = 1;
//         x = &r;
//     }
//     println!("{}", x);
// }
// fn main(){
//     let x = "hello";
//     let y = "there";
//     let l = longest(x, y);
//     println!("{}", l);
// }
// fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main(){
    let novel = "There was a hole in the ground. Something something hobbits";
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);
    let r = i.announce_and_return("hello world");
    println!("r: {}",r);
}

