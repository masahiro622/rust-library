fn to_base(mut n: usize, base: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut s = String::new();
    while n > 0 {
        s.push(std::char::from_digit((n % base) as usize, 10).unwrap());
        n /= base;
    }
    s.chars().rev().collect()
}

fn from_str_radix_custom(s: &str, base: usize) -> usize {
    usize::from_str_radix(s, base).unwrap()
}
