Rust has two main string types:

1. String: A growable, heap-allocated, owned string type.
2. &str: A string slice, which is a reference to a sequence of UTF-8 bytes.

Normally, when people say “String” in Rust, they mean the owned String type.

<b>Creating a String</b>
```rust
let mut s = String::new(); // Creates an empty String

let s2 = String::from("hello"); // Creates a String from a string literal

let s3 = "world".to_string(); // Another way to create a String
```
<b>Adding to a String</b>
```rust
let mut s = String::from("Hello");
s.push(' ');            // Add a single character
s.push_str("World!");   // Add a string slice
println!("{}", s);      // Prints "Hello World!"
```
<b>Concatenating with +</b>
```rust
let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = s1 + " " + &s2; // s1 is moved, s2 is borrowed
println!("{}", s3);     // Prints "Hello World!"
```
<b>Concatenating with format!</b>
```rust
let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = format!("{} {}", s1, s2); // s1 and s2 are moved
println!("{}", s3);     // Prints "Hello World!"
```
<b>Indexing into a String</b>
```rust
let s = String::from("hello");
let first = &s[0..1];
println!("{}", first); // Prints "h"
```
<b>Iterating over a String</b>
```rust
let s = String::from("hello");
for c in s.chars() {
    println!("{}", c);
}
```
<b>Splitting a String</b>
```rust
let s =     String::from("hello");
let parts: Vec<&str> = s.split_whitespace().collect();
println!("{:?}", parts); // Prints ["hello"]

let s = String::from("hello rust world");
let parts: Vec<&str> = s.split_whitespace().collect();
println!("{:?}", parts); // Prints ["hello", "rust", "world"]
```
