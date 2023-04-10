use std::{env, fs::{self, ReadDir}};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Data {
    bin: Vec<Bin>
}

#[derive(Deserialize, Debug)]
struct Bin {
    name: String,
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 3 {
        println!("Usage: copy-wasms <YOUR_RUST_REPO> <TARGET_PATH>");
        return
    }

    let dir = &argv[1];
    let target_path = format!("{}/{}", dir, &argv[2]);
    
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
        match copy(&bin.name, dir, &target_path){
            Ok(()) => {},
            Err(err) => {
                println!("Failed to copy files: {:?}", err.message);
                return
            }
        };
    });
    
}

#[derive(Debug)]
struct Error {
    message: String
}

fn get_extension(paths: ReadDir, binary: &String) -> Result<String, Error> {
    for path in paths {
        match path {
            Ok(path) => {
                let fileentry = match path.file_name().to_str() {
                    Some(s) => s.to_owned(),
                    None => return Err(Error{message: format!("Failed to get file name")})
                };

                match fileentry {
                    entry if entry == format!("{}.wasm", binary) => return Ok(String::from(".wasm")),
                    entry if &entry == binary => return Ok(String::from("")),
                    _ => {}
                }
            },
            Err(err) => return Err(Error{message: format!("Failed to read directory entry: {:?}", err)})
        }
    }

    Err(Error{message:format!("Not found")})
}

fn copy(binary: &String, dir: &String, target_path: &String) -> Result<(), Error> {
    let paths = match fs::read_dir(target_path){
        Ok(res) => res,
        Err(err) => return Err(Error{message: format!("Failed to read target directory: {}", err.to_string())})
    };

    let extension = match get_extension(paths, binary) {
        Ok(ext) => ext,
        Err(err) => return Err(Error{message: format!("Failed to find file extension: {:?}", err)})
    };

    let source = format!("{}/{}{}", target_path, binary, extension);
    let destination = format!("{}/bin/{}/app{}", dir, binary, extension);
    let dest_directory = format!("{}/bin/{}", dir, binary);

    match fs::create_dir_all(dest_directory.to_owned()) {
        Ok(_) => println!("Created dir {}", dest_directory),
        Err(err) => return Err(Error{message:format!("Failed to create file directory {:?}: {:?}", dest_directory, err)})
    };

    match fs::copy(source.to_owned(), destination.to_owned()) {
        Ok(_) => println!("Copied {} to {}", source, destination),
        Err(err) => return Err(Error{message:format!("Failed to copy file {:?} to {:?}: {:?}", source, destination, err)})
    };

    Ok(())
}