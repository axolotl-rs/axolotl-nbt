use std::env::current_dir;
use std::fs::{File, read};
use axolotl_nbt::{NBTDataType};
use axolotl_nbt::value::Value;

pub fn test_file(file_name: &str) {
    let working_directory = current_dir().expect("a current directory").join("tests");
    let read_file = working_directory.join(format!("{}.nbt", file_name));
    if !read_file.exists() {
        panic!("File does not exist: {}", read_file.display());
    }
    let mut file = File::open(&read_file).expect("a file");
    let value = Value::read(&mut file).expect("a value");
    drop(file);

    let write_test = working_directory.join(format!("{}.write.nbt", file_name));
    if write_test.exists() {
        std::fs::remove_file(&write_test).expect("a file");
    }
    let mut file = File::create(&write_test).expect("a file");
    value.clone().write_alone(&mut file).expect("a write");
    drop(file);
    let mut file = File::open(&write_test).expect("a file");
    let value2 = Value::read(&mut file);
    match value2 {
        Ok(ok) => {
            assert_eq!(value, ok);
        }
        Err(err) => {
            drop(file);
            println!("Error: {}", err);
            // This will fail but it allows for a nice diff
            assert_eq!(read(&read_file).expect("a file"), read(&write_test).expect("a file"));
        }
    }
}

#[test]
pub fn test_one() {
    test_file("test_one");
}

#[test]
pub fn test_two() {
    test_file("test_two");
}