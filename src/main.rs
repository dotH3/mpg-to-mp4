
fn main() {
    // Comentario en Rust
    let mut x: &str = "Hola Mundo!";
    println!("{}",x);
    x = "Hello World!";
    println!("{}",x);

    let path = "./input";
    let z = std::fs::read_dir(path).unwrap();
    
    for n in z {
        let n = n.unwrap();
        let file_name = n.file_name(); 
        let file_type = n.file_type().unwrap().is_dir();
        println!("{:?} {:?}", file_name, file_type);
    }
}