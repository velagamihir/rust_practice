fn main() {
    let mut os_name: String = String::from("GaneshOS");
    let os: &String = &os_name;
    println!("Checking OS...{}", os);
    let os_name_reference: &mut String = &mut os_name;
    os_name_reference.push_str(" Kernel");
    println!("{}", os_name_reference);
}
