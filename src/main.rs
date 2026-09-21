fn count_active_cores(cores: &[i32]) -> usize {
    let mut active_cores = 0;
    for core in cores {
        if *core == 1 {
            active_cores += 1;
        }
    }
    active_cores
}

fn active_inactive_cores(cores: &mut [i32]) -> usize {
    let mut activated_cores: usize = 0;
    for core in cores {
        if *core == 0 {
            *core = 1;
            activated_cores += 1;
        }
    }
    activated_cores
}

fn total_cores(cores: &[i32]) -> usize {
    cores.len()
}

fn main() {
    let mut cpu_cores = [1, 0, 1, 0, 0, 1];
    let group1_cores: &mut [i32] = &mut cpu_cores[..3];
    let group2_cores: &mut [i32] = &mut cpu_cores[3..];
    let total_cores_group1: usize = total_cores(group1_cores);
    let active_cores_group1: usize = count_active_cores(group1_cores);
    println!(
        "Total Cores: {}\nActive Cores: {}",
        total_cores_group1, active_cores_group1
    );
    let activated_cores_group1 = active_inactive_cores(group1_cores);
    println!("Actived count: {}", activated_cores_group1);
    // Activating group 2
    let activated_cores_group2: usize = active_inactive_cores(group2_cores);
    println!("Activated cores group2: {}", activated_cores_group2);
    println!("Updated Cores: {:?}", cpu_cores);
}
