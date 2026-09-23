fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    return Some(a / b);
}
fn main() {
    let b = 2;
    let option = Some(20);
    let result = option.and_then(|x| divide(x, b));
    println!("Result: {:?}", result);
}
