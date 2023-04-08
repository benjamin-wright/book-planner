use std::{env, fs};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Data {
    bin: Vec<Bin>
}

#[derive(Deserialize, Debug)]
struct Bin {
    name: String,
    path: String,
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 3 {
        println!("Usage: copy-wasms <YOUR_RUST_REPO> <TARGET_PATH>");
        return
    }

    let dir = &argv[1];
    let target_path = &argv[2];
    
    let cargofile = format!("{}/Cargo.toml", dir);

    let contents = match fs::read_to_string(cargofile) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to open cargo file: {:?}", err);
            return
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(err) => {
            println!("Failed to parse cargo file: {:?}", err);
            return
        }
    };

    data.bin.iter().for_each(|bin| {
        match copy(&bin.name, &target_path){
            Ok(()) => {},
            Err(err) => {
                println!("Failed to copy files: {:?}", err);
                return
            }
        };
    });
    
}

#[derive(Debug)]
struct Error {
    message: String
}

fn copy(binary: &String, target_path: &String) -> Result<(), Error> {
    let paths = match fs::read_dir(target_path){
        Ok(res) => res,
        Err(err) => {
            return Err(Error{message: format!("Failed to read target directory: {}", err.to_string())});
        }
    };

    let mut target = None;

    for path in paths {
        match path {
            Ok(path) => {
                let fileentry = path.file_name().to_str().unwrap().to_owned();

                match fileentry {
                    entry if &entry == binary => {
                        target = Some(entry.to_string());
                    },
                    entry if entry == format!("{}.wasm", binary) => {
                        target = Some(entry.to_string());
                    },
                    _ => {}
                }
            },
            Err(err) => {
                return Err(Error{message: format!("Failed to read directory entry: {:?}", err)});
            }
        }
    }

    let source = format!("{}/{}", target_path, target.unwrap());
    let destination = format!("bin/{}/app.wasm", binary);

    println!("Copying {} to {}", source, destination);

    Ok(())
}