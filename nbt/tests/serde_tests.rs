use axolotl_nbt::binary::Binary;
use axolotl_nbt::serde_impl;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimplePlayer {
    level: i32,
    name: String,
    experience: f32,
}

impl Default for SimplePlayer {
    fn default() -> Self {
        Self {
            level: 5,
            name: "Player".to_string(),
            experience: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListTests {
    list: Vec<i32>,
    list2: Vec<i64>,
    list3: Vec<i8>,
    list4: Vec<i16>,
    list_of_compounds: Vec<SimplePlayer>,
}

fn test_output() -> PathBuf {
    let buf = current_dir()
        .expect("a current directory")
        .join("tests")
        .join("output");
    buf
}

#[test]
pub fn generic_compound() {
    let player = SimplePlayer {
        level: 1,
        name: "KingTux".to_string(),
        experience: 0.0,
    };
    let path = test_output().join("generic_compound.nbt");
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    let mut file = File::create(&path).expect("a file");
    serde_impl::to_writer(&mut file, &player).unwrap();
    drop(file);
    let player: SimplePlayer =
        serde_impl::from_buf_reader::<'_, Binary, BufReader<File>, SimplePlayer>(BufReader::new(
            File::open(path).unwrap(),
        ))
            .unwrap();
    println!("{:?}", player);
}

#[test]
pub fn test_lists() {
    let tests = ListTests {
        list: vec![1, 2, 3, 4, 5],
        list2: vec![1, 2, 3, 4, 5],
        list3: vec![1, 2, 3, 4, 5],
        list4: vec![1, 2, 3, 4, 5],
        list_of_compounds: vec![SimplePlayer::default(), SimplePlayer::default()],
    };
    let path = test_output().join("list_tests.nbt");
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    let mut file = File::create(&path).expect("a file");
    serde_impl::to_writer(&mut file, &tests).unwrap();

    drop(file);
    let data: ListTests = serde_impl::from_buf_reader::<'_, Binary, BufReader<File>, ListTests>(
        BufReader::new(File::open(path).unwrap()),
    )
        .unwrap();
    println!("{:?}", data);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComplexList {
    one: Vec<Vec<SimplePlayer>>,
    two: Vec<Vec<i8>>,

    three: Vec<Vec<i32>>,
    four: Vec<Vec<i64>>,
    five: Vec<Vec<i16>>,
}

#[test]
pub fn complex_list() {
    let tests = ComplexList {
        one: vec![
            vec![SimplePlayer::default(), SimplePlayer::default()],
            vec![SimplePlayer::default(), SimplePlayer::default()],
        ],
        two: vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]],
        three: vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]],
        four: vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]],

        five: vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]],
    };
    let path = test_output().join("complex_list.nbt");
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    let mut file = File::create(&path).expect("a file");
    serde_impl::to_writer(&mut file, &tests).unwrap();

    drop(file);
    let data: ComplexList =
        serde_impl::from_reader::<'_, Binary, File, ComplexList>(File::open(path).unwrap())
            .unwrap();
    println!("{:?}", data);
}
