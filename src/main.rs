fn get_number() -> Option<i32> {
    Some(10)
}
fn main() {
    let result = get_number();
    match result {
        Some(x) => println!("Result is: {}", x),
        None => println!("None"),
    }
}
