fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot be divided by 0"));
    }
    Ok(a / b)
}
fn main() {
    let a = 20;
    let b = 2;
    let result = divide(a, b);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(x) => println!("Error: {}", x),
    }
}
