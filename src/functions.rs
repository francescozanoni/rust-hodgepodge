pub fn get_string_from_input() -> String {
    // https://www.tutorialspoint.com/rust/rust_input_output.htm
    let mut a_string = String::new();
    std::io::stdin().read_line(&mut a_string).unwrap();
    String::from(a_string.trim())
}

pub fn get_int_from_input() -> u32 {
    // https://www.programming-idioms.org/idiom/120/read-integer-from-stdin/1906/rust
    get_string_from_input().parse().unwrap()
}
