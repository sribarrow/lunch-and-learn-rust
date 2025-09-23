fn main() {
    let arr = [1, 2, 3, 4, 5];

    println!("{:?}", arr);
    let arr1 = [0; 5]; // Creates an array of size 5 with all elements initialized to 0

    println!("{:?}", arr1);

    let arr = [1, 2, 3, 4, 5];
    // println!("{}", arr[5]); // Prints 1 (indexing starts at 0)

    // Safe access using get (returns Option<&T>)
    match arr.get(5) {
        Some(value) => println!("sixty element is {}", value),
        None => println!("No sixth element"),
    }

    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("{}", i);
    }

    let mut arr = [1, 2, 3, 4, 5];
    arr[0] = 300;
    println!("{:?}", arr);

    let arr: [u32; 0] = [];
    if arr.is_empty() {
        println!("Array is empty");
    } else {
        println!("Array is not empty");
    }

    let arr = [1, 2, 3];
    println!("Length of array: {}", arr.len());
    let arr = [1, 2, 3];
    let p = 5;
    if arr.contains(&p) {
        println!("Array contains {}", p);
    } else {
        println!("Array does not contain {p}");
    }
    // ownership
    let arrx = [1, 2, 3];
    let arr2 = arrx; // arr is moved to arr2
    println!("{:?}", arr2);
    println!("{:?}", arrx); // Error: arr is no longer valid

    let arr = [1, 2, 3];
    let arr2 = &arr; // arr is borrowed
    println!("{:?}", arr);
    println!("{}", arr2[0]); // Safe access
}
