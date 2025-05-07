#### Unit Testing
```
fn square(x: i32) -> i32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(4), 16);
    }
}
```
The above is an example of a function and its testcase.

#### How to interpret it?
- `#[cfg(test)]`
Tells Compiler to complile this code only if `cargo test` is run. Prevents this code from running when `cargo build` or `cargo run` is run.
- `mod tests`
In Rust tests are grouped under `mod tests` inside each file which separates them from production code.
- `use super::*;`
Within the test namespace, we are in a new namespace. `super::*` imports everything from parent module thus allowing tests to call functions written in the same file.
- `#[test]`
This is an attribute or a metadata that tells the compiler how to process this piece of code that follows.

