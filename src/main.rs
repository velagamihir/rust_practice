fn main() {
    let mut cpu_cores: [i32; 6] = [1, 0, 0, 1, 0, 1];
    println!("Cores before updation: {:?}", cpu_cores);
    let cpu_cores_group: &mut [i32] = &mut cpu_cores[1..5];
    let activated_cores: i8 = activate_cores(cpu_cores_group);
    println!(
        "Inactive cores that were now activated: {}",
        activated_cores
    );
    println!("Updated cores: {:?}", cpu_cores);
}
// Function to activate only the inactive cores
fn activate_cores(cpu_cores_group: &mut [i32]) -> i8 {
    let mut activated_cores: i8 = 0;
    for core in cpu_cores_group {
        if *core == 0 {
            *core = 1;
            activated_cores += 1;
        }
    }
    activated_cores
}
