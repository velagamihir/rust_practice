// Main function
fn main() {
    let full_os_name = String::from("GaneshOS Kernel");
    let mut os_component_name: &str = &full_os_name;
    check_component(os_component_name);
    os_component_name = &full_os_name[9..15];
    check_component(os_component_name);
}
// Function to check the os component
fn check_component(os_component_name: &str) {
    println!("Checking: {}...", os_component_name);
}
