fn scan_processes(processes: &Vec<i32>) {
    for process in processes {
        println!("Process: {}", process);
    }
}
fn main() {
    let processes = vec![101, 202, 303, 404];
    println!("Scanning started");
    scan_processes(&processes);
    println!("All processes scanned\nProcesses: {:?}", processes);
}
