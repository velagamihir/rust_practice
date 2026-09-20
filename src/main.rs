fn main() {
    let cpu_cores: [i32; 6] = [1, 0, 1, 0, 1, 1];
    let cpu_cores_slice = &cpu_cores[1..5];
    let active_cores: usize = inspect_cpu_status(cpu_cores_slice);
    println!("Number of active cores: {}", active_cores);
}
fn inspect_cpu_status(cpu_cores_slice: &[i32]) -> usize {
    let mut active_cores = 0;
    for core in cpu_cores_slice {
        if *core == 1 {
            active_cores += 1;
        }
    }
    active_cores
}
