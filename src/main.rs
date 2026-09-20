fn main() {
    let cpu_cores = [0, 1, 2, 3, 4, 5, 6, 7];
    let cpu_cores_group1 = &cpu_cores[..2];
    let cpu_cores_group2 = &cpu_cores[3..6];
    let cpu_cores_group3 = &cpu_cores[6..];
    println!(
        "Group1: {}\nGroup2: {}\nGroup3: {}",
        inspect_group(cpu_cores_group1),
        inspect_group(cpu_cores_group2),
        inspect_group(cpu_cores_group3)
    )
}
fn inspect_group(cpu_cores_group: &[i32]) -> usize {
    cpu_cores_group.len()
}
