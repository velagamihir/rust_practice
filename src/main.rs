fn main() {
    let full_name = String::from("GaneshOS Kernel");
    let os_name: &str = &full_name[0..8];
    let kernel_name: &str = &full_name[9..15];
    println!(
        "OS: {}\nComponent: {}\nFull Name: {}",
        os_name, kernel_name, full_name
    );
}
