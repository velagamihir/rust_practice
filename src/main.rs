fn main() {
    struct CpuCore {
        id: i32,
        state: i32,
        frequency: i32,
    }
    let mut cpu_core: CpuCore = CpuCore {
        id: 0,
        state: 0,
        frequency: 3200,
    };
    println!(
        "Cpu Core Id: {}\nCPU Core State: {}\nCPU Core frequency: {}",
        cpu_core.id, cpu_core.state, cpu_core.frequency
    );
    cpu_core.state = 1;
    println!("Updated State: {}", cpu_core.state);
}
