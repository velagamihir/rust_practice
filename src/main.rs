fn get_number() -> Option<i32> {
    Some(10)
}
fn main() {
    let result: Option<i32> = get_number().map(|x| x * 5);
    println!("Result: {:?}", result);
}
