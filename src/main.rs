fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Not able to divide by zero"));
    }
    Ok(a / b)
}
fn main() {
    let b = 2;
    let result: Result<i32, String> = Ok(20).and_then(|x| divide(x, b));
    println!("Result: {:?}", result);
}
