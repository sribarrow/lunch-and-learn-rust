fn main() {
    let mut v: Vec<i32> = Vec::new(); // Create an empty vector of i32
    v.push(1); // Add elements
    v.push(2);
    v.push(3);

    let v2 = vec![10, 20, 30]; // Create a vector with initial values

    println!("{:?}, {:?}", v, v2);

    let v = vec![1, 2, 3, 4, 5];
    println!("{}", v[0]); // Prints 1 (indexing starts at 0)

    // Safe access using get (returns Option<&T>)
    match v.get(2) {
        Some(value) => println!("Third element is {}", value),
        None => println!("No third element"),
    }

    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    v.pop(); // Removes the last element
    v.remove(2); // Removes the element at index 2
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    if v.is_empty() {
        println!("Vector is empty");
    } else {
        println!("Vector is not empty");
    }

    let v = vec![1, 2, 3];
    println!("Length of vector: {}", v.len());

    let v = vec![1, 2, 3];
    if v.contains(&2) {
        println!("Vector contains 2");
    } else {
        println!("Vector does not contain 2");
    }

    let v1 = vec![1, 2, 3];
    let v2 = v1; // v is moved to v2

    println!("{:?}", v1);

    let v = vec![1, 2, 3];
    let v2 = &v; // v is borrowed
    println!("{}", v2[0]); // Safe access

    let mut v = vec![1, 2, 3];
    let v2 = &mut v; // v is mutably borrowed
    v2.push(4); // Safe modification
}
