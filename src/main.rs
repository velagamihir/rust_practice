fn get_score() -> Result<i32, String> {
    Err(String::from("Score unavailable"))
}
fn main() {
    let score: i32 = get_score().unwrap_or_default();
    println!("Score: {}", score);
}
