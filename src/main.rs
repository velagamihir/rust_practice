// Booting function
fn boot() {
    println!("GaneshOS Booting...");
}
// Function to check the hardware requirements
fn check_hardware() {
    println!("Checking hardware...");
    println!("Hardware check complete!");
}
//Function to calculate the free ram
fn calculate_free_ram(total_ram: i8, used_ram: i8) -> i8 {
    let free_ram: i8 = total_ram - used_ram;
    free_ram
}
fn start_kernel() {
    println!("Starting GaneshOS kernel...");
    println!("kernel started!");
}
fn main() {
    boot();
    check_hardware();
    let total_ram = 16;
    let used_ram = 10;
    let free_ram: i8 = calculate_free_ram(total_ram, used_ram);
    println!("Free RAM: {}", free_ram);
    start_kernel();
}
