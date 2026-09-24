fn get_score() -> Result<i32, String> {
    Err(String::from("Score unavailable"))
}
fn main() {
    let result = get_score();
    println!(
        "{:?}",
        result.map_err(|x| format!("Failed to get Score: {}", x))
    );
}
