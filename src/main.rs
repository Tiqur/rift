use std::env;
use std::fs;
use std::collections::HashMap;
use rand::Rng;

const CHUNK_SIZE: usize = 1024;

fn gen_id(length: u32) -> String {
    let mut rng = rand::thread_rng();
    let uid: String = (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    uid
}

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

struct Packet {
    path: String,
    index: u32,
    data: Vec<u8>
}

fn create_packets(path: &String) -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();
    let mut index = 0;
    let bytes = fs::read(path).unwrap();

    for chunk in bytes.chunks(CHUNK_SIZE) {
        packets.push(Packet {
            path: (path.to_owned()),
            index: (index),
            data: (chunk.to_vec())
        });

        index += 1;
    }
    packets
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut packet_map: HashMap<String, Packet> = HashMap::new();

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
                        //println!("{}", dir_contents.join("\n"));
                        for f in dir_contents {
                            for packet in create_packets(&f) {
                                packet_map.insert(gen_id(16), packet);
                            }
                        }
                    } else if fs::metadata(&p).unwrap().is_file() {
                        for packet in create_packets(p) {
                            packet_map.insert(gen_id(16), packet);
                        }
                    }
                }

                for packet in packet_map.iter() {
                    println!("{}", packet.0);
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
