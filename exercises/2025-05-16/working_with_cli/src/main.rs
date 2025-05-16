use std::{
    env,
    fs,
    io::{self /*imports itself */, Read},
    path::{Path, PathBuf}, //an owned, mutable file‐system path (from the standard library).
};
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> io::Result<()> {
    /*
    Reading the command‐line arguments Binding into local variables
    - pattern is a String holding whatever you passed as the first argument.
    - path is also a String, holding your second argument.
     */
    let pattern = env::args().nth(1).expect("no pattern given");
    let path = env::args().nth(2).expect("no path given");
    // Constructing the Cli instance
    let args = Cli {
        pattern,
        path: PathBuf::from(path), // convert String → PathBuf
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    // 2. the recursive walk in the path
    search_dir(&args.path, &args.pattern)?;
    Ok(())
}

fn search_dir(dir: &Path, pattern: &str) -> io::Result<()> {
    // Try to read the directory entries
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into subdirectory
            search_dir(&path, pattern)?;
        } else if path.is_file() {
            // Try to open & read the file
            match fs::File::open(&path) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    // If it's valid UTF-8 text and Check for a substring match
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
