#### Tuples
- A compound data type in Rust that group a collection of related items.
- Tuples can hold a mix of data types (unlike arrays)
- zero-based indexing
``` Usage
    let tup = (10, 'a', 10.5);
    let first_elem = tup.0;
```
#### Arrays
- A collection of elements of same data type.
- Fixed length
- zero-based indexing
```
    let arr = [10, 20, 30];
    let num1 = arr[0];
```
#### Functions
- Block of related sections of code organised together for reuse
- Declared using fn keyword
- main function as below serves as the entry point to begin program execution.
- all programming lines/statements end with ;
- when a line  does not end with ; the value evaluated is returned
```
    fn main() {

    }
```
#### Ownership
At a given time there can only be one owner. Once out of scope the memory is freed and the value dropped. This is how Rust manages memory without a garbage collector.

#### Stack
For all primitive datatypes, memory is allocated in stack. For variable to be stored in a stack, the compiler should know the size of the variable. Stack operates on Last In First Out (LIFO) and is very fast.

#### Heap
If the size of data cannot be known at compile time or the size changes dynamically, then the data is stored on the heap. Heap is slower than Stack Operation as there is the overhead for the memory allocator to find unused space and keep track of unused spaces.

#### Move
Assigning a variable of primitive data type copies the value. This is because they implement copy trait. Data types like vector, String etc are moved.
Below is an example of copy where a and b have value 1.
```
    let a = 1;
    let b = a;
```
Below is an example of Move where sa is freed after the operation.
```
    let sa = String::from("Learning Rust")
    let sb = sa;
```
#### Clone
By default, Rust does not copy heap data. To make a deep copy (i.e., to copy both stack and heap), we can use the clone. Below will have different memory address for sa and sb.
```
    let sa = String::from("Learning Rust")
    let sb = sa.clone();
```
#### Reference or Borrowing
Reference is the way of passing variable without taking ownership by passing the value by pointer reference to the address of the value. When a value stored on heap is passed as argument to a function, the original variable goes out of scope. If the original value should still be valid, we pass the &variable and this is called Borrowing.

