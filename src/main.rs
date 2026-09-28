fn create_process() -> Vec<i32> {
    let process_ids = vec![101, 202, 303];
    process_ids
}
fn add_process(process_ids: &mut Vec<i32>, process_id: i32) {
    process_ids.push(process_id);
}
fn remove_process(process_ids: &mut Vec<i32>) -> Option<i32> {
    return process_ids.pop();
}
fn main() {
    let mut process_ids = create_process();
    println!("Process IDs: {:?}", process_ids);
    let process_id: i32 = 404;
    add_process(&mut process_ids, process_id);
    println!("After Adding: {:?}", process_ids);
    let removed_process: Option<i32> = remove_process(&mut process_ids);
    println!(
        "Removed process: {:?}\nFinal processes: {:?}\nProcesses Length: {}",
        removed_process,
        process_ids,
        process_ids.len()
    );
}
