fn main() {
    let ganeshos_cores = [0, 1, 2, 3, 4, 5];
    let group2_cores: &[i32] = &ganeshos_cores[2..5];
    let no_of_cores: usize = inspect_cores(group2_cores);
    println!("Number of cores: {}", no_of_cores);
}
fn inspect_cores(group2_cores: &[i32]) -> usize {
    // Returning the number of cores in the group
    group2_cores.len()
}
