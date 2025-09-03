pub fn info<T: AsRef<str> + std::fmt::Display>(text: &T) {
    println!("{}", text);
}
