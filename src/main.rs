use std::env;
use std::fs;

fn get_dir_contents(path: &String) -> Vec<String> {
    let mut contents = Vec::new();
    for file_result in fs::read_dir(path).unwrap() {
        let file = file_result.unwrap();

        // Recursively get dir contents
        if fs::metadata(file.path()).unwrap().is_dir() {
            contents.extend(get_dir_contents(&file.path().to_string_lossy().to_string()));
        } else {
            contents.push(file.path().to_str().unwrap().to_string());
        }
    }
    contents
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) => match arg.as_str() {
            // Serve file to another client 
            "serve" => {
                println!("Serving...");

                // Get dir / file paths
                let path = args.split_at(2).1;
                
                for p in path {
                    if !fs::metadata(&p).is_ok() { continue };
                    if fs::metadata(&p).unwrap().is_dir() {
                        let dir_contents = get_dir_contents(p);
                        println!("{}", dir_contents.join("\n"));
                    } else if fs::metadata(&p).unwrap().is_file() {

                    }
                }

                //fs::read()
            }
            // Listen for incoming packets
            "listen" => {
                println!("Listening...");
            }
            _ => {

            }
        }
        _ => {}
    }
}
