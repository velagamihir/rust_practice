fn get_name() -> Option<String> {
    Some(String::from("Mihir"))
}
fn main() {
    let result = get_name();
    if let Some(x) = result {
        println!("Name: {}", x);
    }
}
