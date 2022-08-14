use std::fs::File;
use std::io::{Read, Write};
use std::path;
use std::path::PathBuf;

use ron::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Reads file in `/resources` folder and deserializes it using [serde].
pub fn read_deserializable<T: DeserializeOwned>(file_name: PathBuf) -> T {
    let path = path::PathBuf::from("./resources/").join(file_name);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
    let data: T = match from_str(s.as_str()) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    data
}

/// Serializes object using [serde] and writes to file in `/resources` folder.
pub fn write_serializable(data: &impl Serialize, file_name: PathBuf) {
    let serialization_config = PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let s = to_string_pretty(&data, serialization_config).expect("Serialization failed");
    let path = path::PathBuf::from("./resources/").join(file_name);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(s.as_bytes()) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }
}
