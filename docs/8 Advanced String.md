<h3>Common Methods for String</h3>
<b>Length in bytes</b>

```rust
let s = String::from("hello");
println!("{}", s.len()); // Prints 5
```
<b>Length in characters</b>

```rust
let s = String::from("héllo😊");
println!("Bytes: {}", s.len());             // Number of bytes
println!("Characters: {}", s.chars().count()); // Number of Unicode characters
```
<b>Splitting by Whitespace or Custom Delimiter</b>

```rust
let text = "apple,banana,pear";
let fruits: Vec<&str> = text.split(',').collect();
println!("{:?}", fruits); // ["apple", "banana", "pear"]

for word in "hello world rust".split_whitespace() {
    println!("{}", word);
}
``` 
<b>Joining Strings</b>

```rust
let words = vec!["red", "green", "blue"];
let joined = words.join("-");
println!("{}", joined); // "red-green-blue"
```
<b>Replacing Substrings</b>

```rust
let text = "hello world";
let replaced = text.replace("world", "rust");
println!("{}", replaced); // "hello rust"
```
<b>Removing Substrings</b>

```rust
let text = "hello world";
let trimmed = text.trim_start_matches("hello");
println!("{}", trimmed); // " world"
```
<b>Checking for Substrings</b>

```rust
let text = "hello world";
let contains = text.contains("world");
println!("{}", contains); // true
```
<b>Case Conversion</b>

```rust
let text = "hello world";
let upper = text.to_uppercase();
let lower = text.to_lowercase();
println!("{}", upper); // "HELLO WORLD"
println!("{}", lower); // "hello world"
```
<b>Trimming Whitespace</b>

```rust
let text = " hello world ";
let trimmed = text.trim();
println!("{}", trimmed); // "hello world"
```
<b>Splitting into Lines</b>

```rust
let text = "hello\nworld";
let lines: Vec<&str> = text.lines().collect();
println!("{:?}", lines); // ["hello", "world"]
```
<b>Splitting into Words</b>

```rust
let text = "hello world";
let words: Vec<&str> = text.split_whitespace().collect();
println!("{:?}", words); // ["hello", "world"]
```
<b>Splitting by Custom Delimiter</b>

```rust
let text = "apple,banana,pear";
let fruits: Vec<&str> = text.split(',').collect();
println!("{:?}", fruits); // ["apple", "banana", "pear"]
```
<b>Splitting by Multiple Delimiters</b>

```rust
let text = "apple,banana;pear";
let fruits: Vec<&str> = text.split(|c: char| c == ',' || c == ';').collect();
println!("{:?}", fruits); // ["apple", "banana", "pear"]
```
<b>Splitting by Regex</b>

```rust
let text = "apple,banana;pear";
let fruits: Vec<&str> = text.split(|c: char| c == ',' || c == ';').collect();
println!("{:?}", fruits); // ["apple", "banana", "pear"]
```
<b>Splitting by Regex</b>

