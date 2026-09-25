fn get_score() -> Result<i32, String> {
    Ok(201)
}
fn add_bonus(score: i32) -> Result<i32, String> {
    Ok(score + 10)
}
fn validate_score(score: i32) -> Result<i32, String> {
    if score > 100 {
        return Err(String::from("Score greater than 100")).map_err(|x| format!("Error: {}", x));
    }
    Ok(score)
}
fn calculate() -> Result<i32, String> {
    let score = get_score()?;
    let score = add_bonus(score)?;
    let score = validate_score(score);
    score
}
fn main() {
    let score = calculate();
    println!("Result: {:?}", score);
}
