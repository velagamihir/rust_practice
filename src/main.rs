fn get_score() -> Result<i32, String> {
    Err(String::from("Score unavailable"))
}
fn main() {
    let result: Result<i32, String> = get_score();
    println!("Result: {}", result.unwrap_or(0));
}
