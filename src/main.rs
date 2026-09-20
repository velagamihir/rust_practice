fn main() {
    let cpu_group1 = [0, 1, 2, 3];
    let cpu_group2 = [4, 5, 6, 7, 8, 9];
    let cpu_group3 = [10, 11];
    println!("CPU Group1: {}", inspect_cpu_group(&cpu_group1));
    println!("CPU Group2: {}", inspect_cpu_group(&cpu_group2));
    println!("CPU Group3: {}", inspect_cpu_group(&cpu_group3));
}
fn inspect_cpu_group(cores_group: &[i32]) -> usize {
    cores_group.len()
}
