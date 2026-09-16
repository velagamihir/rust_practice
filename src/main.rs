fn main() {
    println!("===========");
    println!("GaneshOS Booting");
    println!("===========");
    for i in 1..=5 {
        println!("Boot Step {}", i);
    }
    println!();
    println!("Boot Sequence Complete!");
    println!("GaneshOS is ready!\n");

    println!("Shut down process initiated");
    let mut countdown = 5;
    while countdown > 0 {
        println!("Shutting down in {}", countdown);
        countdown -= 1;
    }
    println!("Shut down complete");
}
