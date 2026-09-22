enum Command {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Quit,
}
fn main() {
    let add = Command::Add(10, 5);
    let subtract = Command::Subtract(10, 5);
    let product = Command::Multiply(10, 5);
    let quit = Command::Quit;
    let add_result = execute_command(add);
    let sub_result = execute_command(subtract);
    let prod_result = execute_command(product);
    let quit_result = execute_command(quit);
    println!(
        "{}\n{}\n{}\n{}",
        add_result, sub_result, prod_result, quit_result
    );
}
fn execute_command(command: Command) -> i32 {
    let result = match command {
        Command::Add(x, y) => x + y,
        Command::Subtract(x, y) => x - y,
        Command::Multiply(x, y) => x * y,
        Command::Quit => 0,
    };
    result
}
