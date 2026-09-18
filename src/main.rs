fn main() {
    let os_name: String = String::from("GaneshOS");
    let reference1: &String = &os_name;
    let reference2: &String = &os_name;
    let reference3: &String = &os_name;
    inspect(reference1, reference2, reference3);
}
fn inspect(reference1: &String, reference2: &String, reference3: &String) {
    println!("Inspector 1: {}", reference1);
    println!("Inspector 2: {}", reference2);
    println!("Inspector 3: {}", reference3);
}
