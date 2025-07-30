use std::path::Path;

fn main() {
    let path = Path::new("demo.txt");
    let display = path.display();
    println!("Path: {}", display);
}
