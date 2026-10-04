fn update_processes(processes: &mut Vec<i32>) {
    for process in processes {
        *process += 1000;
    }
}
fn main() {
    let mut processes = vec![101, 202, 303, 404];
    println!("Before: {:?}", processes);
    update_processes(&mut processes);
    println!("After: {:?}", processes);
}
