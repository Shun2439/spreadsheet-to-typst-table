use std::{fs::File, io::{self, Read}, path::Path};

fn getline() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.to_string()
}

fn main() {
    // let path = Path::new("demo.txt");
    // let display = path.display();
    // println!("Path: {}", display);

    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => println!("{} contains:\n{}", display, s),
    // }

    print!("Input amout of rows: ");
    let n = getline().trim().parse::<i32>().unwrap();
    println!("Input string: ");

    let mut s = String::new();

    for _ in 0..n {
        let l = getline();
        s.push_str(&l.as_str());
    }

    print!("[");
    for i in s.chars() {
        if i == ' ' || i == '　' || i == '\t' {
            print!("], [");
            continue;
        }
        else if i == '\n' {
           print!("],\n[");
           continue;
        }
        print!("{}", i);
    }
}
