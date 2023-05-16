use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) => match arg.as_str() {
            // Serve file to another client 
            "serve" => {
                println!("Serving...");
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
