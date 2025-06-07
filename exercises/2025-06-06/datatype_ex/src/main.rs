/*
Function can take <T> declared a generic type parameter

fn — declares a function named print_type_of.

<T> — declares a generic type parameter named T.

(_: &T) — means the function takes a reference to a value of any type, but doesn't use the value itself (_ is the unused variable placeholder).
*/
// fn print_type_of<T>(_: &T) {
//     println!(
//         "{}",
//         std::any::type_name::/* here <T> is whatever was passed to the function */<T>()
//     );
// }

fn print_type_of<T: std::fmt::Debug>(val: &T) {
    println!("Value: {:?}, Type: {}", val, std::any::type_name::<T>());
}

fn main() {
    // magic with integers
    let a = 1; // no type annotation
    let b: u32 = 10_000; // anotated with separator
    print_type_of(&a);
    print_type_of(&b);

    // type inference
    let c: i32 = (a + /* just playing with comments */ b).try_into().unwrap(); // Only safe when you're absolutely sure
                                                                               // If you want to provide a default value when conversion fails:
    let d: u8 = (a + /* just playing with comments */ b)
        .try_into()
        .unwrap_or(0); //You have a reasonable default
    let x: i8 = (a + /* just playing with comments */ b)
        .try_into()
        .unwrap_or_else(|x| {
            //You want logic + default
            eprintln!("Conversion failed: {}", x);
            -1 // fallback value
        });

    println!("unwrapped {}, {}, {}", c, d, x)

    let mut my_name: String = "Sri".to_string();
    // let mut my_name = String::from("Sri"); alternative method
    // let mut my_name = "Sri".to_string(); // owned string. alternative method
    println!("{}", my_name);
    my_name.push_str(" Barrow");
    println!("{}", my_name);
    // num as char
    let my_number1 = 100;
    // Cast the number to an 8-bit unsigned integer.
    // Then it casts the u8 value into a Unicode character (char).
    println!("1. {}", my_number1 as u8 as char);

    // for i in 0u8..=255 {
    //     let c = i as char;
    //     println!("{:3} => {:?}", i, c);
    // }

    let my_number2 = 256;
    println!("2. {}", my_number2 as u8);
    // lossy casting
    let my_number3 = 600;
    println!("3. {}", my_number3 as u8);

    let my_number4: u8 = 100;
    println!("4. {}", my_number4 as char);

    // more on characters
    println!("1. Size of a char {}", std::mem::size_of::<char>());
    println!("2. Size of char a:  {}", "a".len());
    println!("3. {:?}", "a".as_bytes());
    println!("4. Size of German ß:  {}", "ß".len());
    println!("5. {:?}", "ß".as_bytes());
    println!("6. Size of French ç:  {}", "ç".len());
    println!("7. {:?}", "ç".as_bytes());
    println!("8. Size of Egyption 𓀇:  {}", "𓀇".len());
    println!("9. {:?}", "𓀇".as_bytes());
    println!("10. Size of Emoji 🤣:  {}", "🤣".len());
    println!("11. {:?}", "🤣".as_bytes());
    // other acceptable variable definitions
    let num1 = 10u8;
    let num2 = 10_u8;
    let num3 = 100_000____i32;
    let num4: u16 = 1000;
    println!(
        "num1: {}, num2: {}, num3: {}, num4: {}",
        num1, num2, num3, num4
    );
    let my_float = 5.;
    print_type_of(&my_float);
    let a_number = {
        let second_number = 11;
        second_number + 10
    };

    println!("a_number is : {:?}", a_number);
    // min and max
    println!("i8: Smallest: {}, Biggest {}", i8::MIN, i8::MAX);
    println!("u8: Smallest: {}, Biggest {}", u8::MIN, u8::MAX);
    println!("i16: Smallest: {}, Biggest {}", i16::MIN, i16::MAX);
    println!("u16: Smallest: {}, Biggest {}", u16::MIN, u16::MAX);
}
