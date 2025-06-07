pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_and_double(a: i32, b: i32) -> i32 {
    (a + b) * 2
}

pub fn swap_tuple(input: (i32, bool)) -> (bool, i32) {
    let (x, y) = input; // tuple destructuring/unpacking
    (y, x)
}

pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_and_double() {
        assert_eq!(add_and_double(2, 3), 10);
    }

    #[test]
    fn test_swap_tuple() {
        assert_eq!(swap_tuple((42, true)), (true, 42));
    }

    #[test]
    fn test_sum_array() {
        assert_eq!(sum_array(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("ChatGPT"), "TPGtahC");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
        assert_eq!(reverse_string("😀🚀"), "🚀😀");
    }
}
