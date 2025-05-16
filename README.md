# lunch-and-learn-rust

#### Next Planned Session 
May 9 2025

#### Why Rust
- [Popularity Chart] (https://pypl.github.io/PYPL.html)
- [Why is Rust the most admired language among developers] (https://github.blog/developer-skills/programming-languages-and-frameworks/why-rust-is-the-most-admired-language-among-developers/)
- [Is it time to consider learning Rust ] (https://medium.com/contino-engineering/is-it-time-to-consider-learning-rust-7849df850b67)
- [Rust Coding Security] (https://www.contino.io/insights/rust-coding-security)

#### Books and Useful Links
1. [Rust official documentation] (https://doc.rust-lang.org/stable/)
2. [The Rust Programming Language] (https://doc.rust-lang.org/stable/book/index.html)
3. [Rust Playground] (https://play.rust-lang.org/)
4. [Rustlings] (http://github.com/rust-lang/rustlings/?tab=readme-ov-file#installing-rustlings)
5. [Rust learn] (rust-lang.org/learn)

#### Task 1 -  (16 May 2025)
- Exercise: Use standard libraries to take path and pattern from user. Search the path and return files that contain pattern
- Concepts: Structs, file I/O, file Read
- Goal:
    1. Take 2 arguments pattern and path as input from CLI.
    2. Search the path for files with pattern
    3. Print path of files that contain pattern.

#### Task 2 -

- Exercise: Create a Contact struct with fields for name, email, and phone number.
- Concepts: Structs, file I/O, file R/W.
- Goal:
    1. Creates a list of contacts.
    2. Saves them to a file in a simple format (e.g., CSV).
    3. Reads the file back and rebuilds the list of contacts.

#### Task 3 - 

- Exercise: Build a Simple CLI To-Do App
- Concepts: Structs, enums, file I/O, CLI args
- Libraries: clap or structopt, serde, serde_json
- Goal: 
    Add, list, and mark tasks as done from the command line

#### Task 4 - 
- Exercise: JSON API Fetcher

Concepts: HTTP requests, async/await, error handling
Libraries: request, tokio, serde
Goal: 
Fetch and parse data from a public API (like JSONPlaceholder or a crypto price feed)

#### Task 5 - 

Exercise: CSV Processor with AWS Lambda

Concepts: Iterators, file I/O, error handling
Libraries: csv, serde
Goal: 
Read a CSV, transform it (e.g. filter rows), and write to a new file

#### Task 6 -

Exercise: Password Strength Checker with AWS Lambda and API Gateway

Concepts: Pattern matching, string manipulation
Goal: 
Input a password and score its strength based on length, characters, etc.