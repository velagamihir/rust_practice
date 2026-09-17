fn main() {
    let os_name = String::from("GaneshOS");
    let kernel_os_name: String = os_name;
    println!("{}", kernel_os_name);
    let kernel_name = String::from("GaneshOS Kernel");
    let kernel_name_clone = kernel_name.clone();
    println!("{}\n{}", kernel_name, kernel_name_clone);
    let sample_number: i8 = 16;
    let sample_number_copy: i8 = sample_number;
    println!("{}\n{}", sample_number, sample_number_copy);
}
