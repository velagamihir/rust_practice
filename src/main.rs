fn get_score() -> Result<i32, String> {
    Err(String::from("Score Unavailable"))
}
fn main() {
    let result = get_score().unwrap_or_else(|error| {
        println!("Error: {}", error);
        0
    });
    println!("Result: {}", result);
}
