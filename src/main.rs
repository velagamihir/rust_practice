fn main() {
    let mut processes = create_processes();
    println!("Initial processes: {:?}", processes);
    let pid = 404;
    add_process(&mut processes, pid);
    println!(
        "After adding: {:?}\nFound Processes: {:?}",
        processes,
        find_process(&processes, 1),
    );
    println!("After Removal: {:?}", remove_process(&mut processes));
    println!(
        "Final Processes: {:?}\nProcesses Count: {}",
        processes,
        processes.len()
    );
}
fn create_processes() -> Vec<i32> {
    vec![101, 202, 303]
}
fn add_process(processes: &mut Vec<i32>, pid: i32) {
    processes.push(pid);
}
fn find_process(processes: &Vec<i32>, index: usize) -> Option<&i32> {
    processes.get(index)
}
fn update_processes(processes: &mut Vec<i32>) {
    for process in processes {
        *process += 1000;
    }
}
fn remove_process(processes: &mut Vec<i32>) -> Option<i32> {
    processes.pop()
}
