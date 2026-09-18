fn main(){
    let mut os=String::from("GaneshOS");
    add_version(&mut os);
}
fn add_version(os:&mut String){
    os.push_str(" v0.1");
    println!("{}",os);
}