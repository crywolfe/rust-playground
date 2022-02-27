fn info<T: AsRef<str>>(a: &T) {
    println!("{}", a.as_ref())
}

fn main() {
    let a = "?";
    let b = "Hello, linkedin!".to_string();
    info(&a);
    info(&b);


}

#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}