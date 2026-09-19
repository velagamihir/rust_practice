fn main() {
    let mut sample_array = [0, 1, 2, 3];
    println!("Initial Array: {:?}", sample_array);
    sample_array[2] = 8;
    println!("Updated array: {:?}", sample_array);
    println!("Third Core: {}", sample_array[2]);
}
