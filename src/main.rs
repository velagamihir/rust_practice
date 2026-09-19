fn main() {
    let ganeshos_cores = [0, 1, 2, 3, 4, 5];
    let group2_cores: &[i32] = &ganeshos_cores[2..5];
    //Variable to store the sum of all the elements in group2_cores slice
    let sum: i32 = sum_cores(group2_cores);
    println!("Sum of Core IDs: {}", sum);
}
// Function to calculate the sum of all the elements in an array slice
fn sum_cores(cores: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for num in cores {
        sum += num;
    }
    sum
}
