fn main() {
    let os_name: &str = "MihirOS";
    let os_version: &str = "0.0.1";
    let ram: u8 = 8;
    let cpu_cores: u8 = 8;
    let cpu_speed: &str = "16ghz";
    let kernel_running: bool = true;
    println!("==============");
    println!("MihirOS Hardware");
    println!("==============");
    println!(
        "OS: {}\nVersion: {}\nRAM: {}\nCPU Cores: {}\nCPU Speed: {}\nKernel Running: {}",
        os_name, os_version, ram, cpu_cores, cpu_speed, kernel_running
    );
}
