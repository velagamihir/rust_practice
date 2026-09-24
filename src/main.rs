fn check_age(age: i32) -> Result<i32, String> {
    if age >= 18 {
        return Ok(age);
    }
    Err(String::from("Age is below 18"))
}
fn main() {
    let result: Result<i32, String> = check_age(10);
    if let Ok(age) = result {
        println!("Age accepted: {}", age);
    } else {
        println!("Age rejected");
    }
}
