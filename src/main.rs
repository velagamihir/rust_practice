fn get_number() -> Result<i32, String> {
    Ok(20)
}
fn calculate() -> Result<i32, String> {
    let result = get_number()?;
    Ok(result + 10)
}
fn main() {
    let result = calculate();
    println!("Result: {:?}", result);
}
