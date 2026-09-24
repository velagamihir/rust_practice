fn get_number() -> Result<i32, String> {
    Ok(20)
}
fn add_ten(value: i32) -> Result<i32, String> {
    Ok(value + 10)
}
fn divide(value: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(value / divisor)
}
fn calculate() -> Result<i32, String> {
    let result = get_number()?;
    let result = add_ten(result)?;
    let result = divide(result, 2)?;
    Ok(result)
}
fn main() {
    let result = calculate();
    println!("Result: {:?}", result);
}
