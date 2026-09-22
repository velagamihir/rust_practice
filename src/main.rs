fn main() {
    let result: Option<i32> = get_score();
    let result_final = result.unwrap_or(0);
    println!("Score: {}", result_final);
}
fn get_score() -> Option<i32> {
    None
}
