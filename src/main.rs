fn get_name() -> Option<i32> {
    Some(50)
}
fn main() {
    let result: Option<i32> = get_name();
    if result.is_some() {
        println!("Value Exists\nValue: {}", result.unwrap());
    }
}
