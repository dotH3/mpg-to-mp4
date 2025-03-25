
fn main() {
    let path = "./input";
    let z = std::fs::read_dir(path).unwrap();

    let mut file_counter_success = 0;
    
    for n in z {
        let n = n.unwrap();
        let file_name = n.file_name(); 
        let is_file = n.file_type().unwrap().is_file();
        let is_mpg = file_name.to_str().unwrap().ends_with(".mpg");
        if is_file == false || is_mpg == false {
            continue;
        };

        // convertir a mp4
        let buffer = std::fs::read(n.path()).unwrap();
        let buffer = buffer.len() as f64 / 1024.0 / 1024.0;
        println!("buffer size: {:.2}MB",buffer);
        

        println!("{:?} {:?}", file_name, n.path());
        file_counter_success += 1 ;
    }

    println!("Total de archivos mpg: {}", file_counter_success);
}
