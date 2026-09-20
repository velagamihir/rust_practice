fn main() {
    let mut cpu_cores = [0, 0, 0, 0, 0, 0];
    let cpu_cores_slice: &mut [i32] = &mut cpu_cores[2..5];
    let activated_cores: usize = activate_cores(cpu_cores_slice);
    println!("Activated cores: {}", activated_cores);
    println!("Updated Cores: {:?}", cpu_cores);
}
fn activate_cores(cpu_cores_slice: &mut [i32]) -> usize {
    let mut cores_activated = 0;
    for core in cpu_cores_slice {
        *core = 1;
        cores_activated += 1;
    }
    cores_activated
}
