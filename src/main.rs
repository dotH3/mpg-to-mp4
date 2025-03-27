use std::{process::Command, time::Instant, fs};

fn main() {
    let start = Instant::now();
    let path = "./input";
    let path_output = "./output";

    // Crear directorios si no existen
    if !fs::metadata(path).is_ok() {
        fs::create_dir_all(path).unwrap();
    }

    if fs::metadata(path_output).is_ok() {
        fs::read_dir(path_output).unwrap().for_each(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                fs::remove_file(entry.path()).unwrap();
            }
        });
    } else {
        fs::create_dir_all(path_output).unwrap();
    }

    let z = fs::read_dir(path).unwrap();
    let mut file_counter_success = 0;
    
    for n in z {
        let n = n.unwrap();
        let file_name = n.file_name(); 
        let is_file = n.file_type().unwrap().is_file();
        let is_mpg = file_name.to_str().unwrap().ends_with(".MPG");
        if is_file == false || is_mpg == false {
            continue;
        };
        let file_stem = file_name.to_str().unwrap().rsplitn(2, '.').last().unwrap();

        // convertir a mp4
        let buffer = fs::read(n.path()).unwrap();
        let buffer = buffer.len() as f64 / 1024.0 / 1024.0;
        println!("[DOING] {} ({:.2}MB)",file_stem,buffer);

        convert(n.path().to_str().unwrap(), file_name.to_str().unwrap());

        file_counter_success += 1 ;
    }

    let elapsed = start.elapsed();
    println!("Converted {} files in {:.2?}", file_counter_success,elapsed);
}

fn convert(file_path: &str, file_name: &str) {
    let file_output = format!("./output/{}.mp4", file_name);

    let output = Command::new("ffmpeg")
        .args(&["-i", file_path, "-c:v", "libx264", "-preset", "fast", "-crf", "23", "-c:a", "aac", "-b:a", "128k", &file_output])
        .output();

    if let Ok(out) = output {
        if !out.status.success() {
            println!("[ERROR] {}", file_path);
            eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        }
    } else {
        println!("[ERROR] {}", file_path);
        eprintln!("No se pudo ejecutar ffmpeg.");
    }

    println!("[DONE] {}", file_output);
}
