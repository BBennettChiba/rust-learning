use trait_objects::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self){
        println!("draw for select box!");
    }
}

impl std::fmt::Display for SelectBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SelectBox (width: {}, height: {}, options: {})", self.width, self.height, self.options.len())
    }
}
fn main() {
    let screen = Screen {components: vec![
        Box::new(SelectBox {width:100, height:100, options:vec!["Yes".to_string(), "No".to_string(), "maybe".to_string()]}),
        Box::new(Button {width:100, height:100, label:"Cool".to_string()}),
    ]};
    screen.run();
}
