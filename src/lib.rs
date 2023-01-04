use std::fs::File;
use std::io::Read;

pub mod commonlib;
pub mod smoke;

pub fn open(filename: &str) -> Result<String, std::io::Error> {
    let basepath = concat!(env!("CARGO_MANIFEST_DIR"), "/local");
    println!("BasePath: {basepath}");
    let filepath = format!("{basepath}/{filename}");
    println!("Opening file: {filepath}");
    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Ok(content)
}