use std::{env, path::PathBuf};

fn main() {
    let mut args = env::args();
    println!("Hello, world!");
    let options = Options::build(args);
    println!("{:#?}", options);
}

#[derive(Debug)]
struct Options {
    a: bool,
    l: bool,
    h: bool,
    path: PathBuf,
}

impl Options {
    fn build(mut args: impl Iterator<Item = String>) -> Self {
        let mut a = false;
        let mut l = false;
        let mut h = false;
        let mut path = PathBuf::new();
        
        args.next();
        while let Some(arg) = args.next() {
            if arg.starts_with("-") {
                for ch in arg.chars() {
                    match ch {
                        '-' => continue,
                        'a' => a = true,
                        'l' => l = true,
                        'h' => h = true,
                        _ => todo!()
                    };
                }
            } else {
                path.push(arg);
            }
        }
        Self { a, l, h, path }
    }
}