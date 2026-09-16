fn main() {
    let ram = 16;
    let cpu_cores = 8;
    let cpu_threads = 16;
    let kernel_running = false;
    println!("=========");
    println!("MihirOS Check!");
    println!("=========");
    println!(
        "RAM: {} GB\nCPU Cores: {}\nCPU Threads: {}",
        ram, cpu_cores, cpu_threads
    );
    let ram_pass = ram >= 8;
    let cpu_cores_pass = (cpu_cores >= 4) && (cpu_threads >= 8);
    if kernel_running {
        println!("RUNNING...");
    } else {
        println!("Starting Kernel");
    }
    let mut system_req = "FAIL";
    if ram_pass && cpu_cores_pass {
        system_req = "PASS";
    }
    println!("System Requirements: {}", system_req);
}
