use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

struct Cli {
    pattern: String,
    path: PathBuf,
}
// for a simple cli app refer docs/simple_cli.txt
fn main() -> io::Result<()> {
    let pattern: String = std::env::args().nth(1).expect("no pattern provided");
    let path: String = std::env::args().nth(2).expect("no path provided");

    let args = Cli {
        pattern,
        path: PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    search_dir(&args.path, &args.pattern)?;
    Ok(())
}

fn search_dir(dir: &Path, pattern: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path: PathBuf = entry.path();

        if path.is_dir() {
            search_dir(&path, pattern)?;
        } else if path.is_file() {
            match fs::File::open(&path) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() && contents.contains(pattern) {
                        println!("pattern: {:?}, file: {:?}", pattern, path);
                    }
                }
                Err(err) => {
                    eprintln!("failed to open {:?}: {}", path, err);
                }
            }
        }
    }
    Ok(())
}
