fn main() {
    let total_ram: u32 = 16;
    let used_ram: u32 = 6;

    let free_ram = total_ram - used_ram;

    println!("Total RAM: {} GB", total_ram);
    println!("Used RAM: {} GB", used_ram);
    println!("Free RAM: {} GB", free_ram);

    let cpu_cores: u8 = 8;
    let threads_per_core: u8 = 2;
    let total_threads = cpu_cores * threads_per_core;

    //Memory blocks
    let total_memory = 16;
    let each_block_size = 2;
    let number_of_blocks = total_memory / each_block_size;

    //Printing values
    println!("Total number of threads: {}", total_threads);
    println!("Number of blocks: {}", number_of_blocks);
}
