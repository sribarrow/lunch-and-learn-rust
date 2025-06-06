fn print_type_of<T: std::fmt::Debug>(val: &T) {
    println!("Value: {:?}, Type: {}", val, std::any::type_name::<T>());
}

fn main() {
    let a = 1; // default is i32
    print_type_of(&a);
    let b: u32 = 10_000;
    print_type_of(&b);
    // let c = (a + /* just playing with comments */ b).try_into().unwrap();
    // let c = (a + /* just playing with comments */ b)
    //     .try_into()
    //     .unwrap_or(0);
    let c: i32 = (a + /* just playing with comments */ b)
        .try_into()
        .unwrap_or_else(|e| {
            //You want logic + default
            eprintln!("Conversion failed: {}", e);
            -1 // fallback value
        });
    println!("{}", c);
    print_type_of(&c);

    println!("Size of i32 is {} bytes", std::mem::size_of::<i32>());

    // strings
    let mut my_name = "Sri".to_string();
    my_name.push_str(" Barrow");
    println!("{}", my_name);
    print_type_of(&my_name);
    for i in 0u8..=255 {
        let c = i as char;
        println!("{:3} => {:?}", i, c);
    }

    let my_number1 = 100;
    println!("1. {}", my_number1 as u8 as char);
    let my_number2 = 10000;
    println!("2. {}", my_number2 as u8);
    let my_number3 = 600;
    println!("3. {}", my_number3 as u8);

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
