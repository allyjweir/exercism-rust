pub fn twofer(name: &str) -> String {
    if name.is_empty() {
        String::from("One for you, one for me.")
    } else {
        format!("One for {}, one for me.", name)
    }
}
