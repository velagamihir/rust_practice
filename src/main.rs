fn find_process(processes: &Vec<i32>, index: usize) -> Option<&i32> {
    processes.get(index)
}
fn main() {
    let processes = &vec![101, 202, 303, 404];
    let index = 1;
    let result: Option<&i32> = find_process(processes, index);
    match result {
        Some(&value) => println!("Process found: {}", value),
        _ => println!("Process not found"),
    }
}
