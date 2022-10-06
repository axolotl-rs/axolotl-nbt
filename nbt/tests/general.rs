use axolotl_nbt::sync::{NBTReader, NBTWriter};
use std::env::current_dir;
use std::fs::read;
use std::path::Path;
macro_rules! read_then_repeat_test {
    ($file:ident) => {
        #[test]
        pub fn $file() {
            let path = current_dir().unwrap().join(Path::new("tests"));
            let read_from = path.join(concat!(stringify!($file),".nbt"));
            let mut file = std::fs::File::open(&read_from).unwrap();
            let mut reader = NBTReader::new(&mut file);
            let value = reader.read_value().unwrap();
            println!("{:?}", value);
            drop(reader);
            let path = path.join(concat!(stringify!($file),".write.nbt"));
            if path.exists() {
                std::fs::remove_file(&path).unwrap();
            }
            let mut file = std::fs::File::create(&path).unwrap();
            let mut writer = NBTWriter::new(&mut file);
            writer.write_value(value).unwrap();
            assert_eq!(read(read_from).unwrap(), read(path).unwrap());
        }
    };
}
read_then_repeat_test!(test_one);
read_then_repeat_test!(test_two);