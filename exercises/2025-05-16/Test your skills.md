### Ex 1.1
Write a function add_and_double. It takes 2 integer arguments and returns an integer. The result should be double of the sum of the integers. It should pass the below testcase.
```
    #[test]
    fn test_add_and_double() {
        assert_eq!(add_and_double(2, 3), 10);
    }
```

### Ex 1.2
Write a function swap_tuple with parameter as tuple with integer and bool. The result is a tuple with bool and integer. Should pass the below testcase.
```
    #[test]
    fn test_swap_tuple() {
        assert_eq!(swap_tuple((42, true)), (true, 42));
}
```

### Ex 1.3
Write a function sum_array(arr: &[i32]) -> i32. Should pass the below testcase.
```
    #[test]
    fn test_sum_array() {
        assert_eq!(sum_array(&[1, 2, 3, 4]), 10);
    }
```

### Ex 1.4
Write a function reverse_string(s: &str) -> String that returns the reverse of the input string.
```
#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("ChatGPT"), "TPGtahC");
    assert_eq!(reverse_string(""), "");
    assert_eq!(reverse_string("a"), "a");
}

```
