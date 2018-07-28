pub fn raindrops(n: usize) -> String {
    let mut  returnable = String::from("");

    if n%3 == 0 {
        returnable += "Pling";
    }
    if n%5 == 0 {
        returnable += "Plang";
    }
    if n%7 == 0 {
        returnable += "Plong";
    }
    if returnable.len() == 0 {
        return n.to_string();
    }
    return returnable;
}
