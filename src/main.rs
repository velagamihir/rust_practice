fn check_number(number: i32) -> Result<i32, String> {
    if number >= 0 {
        return Ok(number);
    }
    return Err(String::from("Number is negative"));
}
fn main() {
    let result: Result<i32, String> = check_number(10);
    if result.is_ok() {
        println!("Number is positive: {}", result.ok().unwrap());
    } else {
        println!("Number is not positive");
    }
}
