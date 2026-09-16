fn main() {
    let os_name = "MihirOS";
    let mut os_version = "0.0.1";
    let developer = "Mihir Velaga";
    println!(
        "The OS Name is: {}\nThe Version is: {}\nThe Developer is: {}",
        os_name, os_version, developer
    );
    os_version = "0.0.2";
    println!("os_version: {}", os_version);
}
