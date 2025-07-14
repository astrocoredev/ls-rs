use std::{process, fs, env, path::PathBuf};
use std::os::unix::fs::PermissionsExt;
fn main() {
    let args = env::args();
    let options = Options::build(args);
    run(options);
}

#[derive(Debug)]
struct Options {
    a: bool,
    l: bool,
    path: PathBuf,
}

fn run(options: Options) {
    let mut dirs = fs::read_dir(&options.path).unwrap_or_else(|err| {
        eprintln!("Failed to read directory: {err}");
        process::exit(0);
    }).map(|entry| entry.unwrap().path())
    .collect::<Vec<_>>();

    if !options.a {
        dirs = dirs
            .into_iter()
            .filter(|entry| {
                !entry
                    .file_name()
                    .unwrap()
                    .to_owned()
                    .into_string()
                    .unwrap()
                    .starts_with(".")
            })
            .collect::<Vec<_>>();
    }


    if options.l {
        let permissions = dirs
            .clone()
            .into_iter()
            .map(|entry| {
                let mode = fs::metadata(entry)
                    .unwrap()
                    .permissions()
                    .mode();
                let owner = (mode >> 6) & 0b111;
                let group = (mode >> 3) & 0b111;
                let others = mode & 0b111;
                let owner = format!(
                    "{}{}{}",
                    if owner & 0b100 != 0 { "r" } else { "-" },
                    if owner & 0b010 != 0 { "w" } else { "-" },
                    if owner & 0b001 != 0 { "x" } else { "-" },
                );
                let group = format!(
                    "{}{}{}",
                    if group & 0b100 != 0 { "r" } else { "-" },
                    if group & 0b010 != 0 { "w" } else { "-" },
                    if group & 0b001 != 0 { "x" } else { "-" },
                );
                let others = format!(
                    "{}{}{}",
                    if others & 0b100 != 0 { "r" } else { "-" },
                    if others & 0b010 != 0 { "w" } else { "-" },
                    if others & 0b001 != 0 { "x" } else { "-" },
                );
                format!("{}{}{}", owner, group, others)
            })
            .collect::<Vec<_>>();
        for (idx, item) in dirs.iter().enumerate() {
            println!("{} {}", permissions[idx], item.file_name().unwrap().to_owned().into_string().unwrap());
        }
    } else {
        for dir in dirs {
            println!("{}", dir.file_name().unwrap().to_owned().into_string().unwrap())
        }
    }
}

impl Options {
    fn build(mut args: impl Iterator<Item = String>) -> Self {
        let mut a = false;
        let mut l = false;
        let mut path = String::new();
        
        args.next();
        while let Some(arg) = args.next() {
            if arg.starts_with("-") {
                for ch in arg.chars() {
                    match ch {
                        '-' => continue,
                        'a' => a = true,
                        'l' => l = true,
                        _ => todo!()
                    };
                }
            } else {
                path.push_str(&arg);
            }
        }
        if path.is_empty() {
            path = String::from(".");
        }
        Self { a, l, path: PathBuf::from(path) }
    }
}