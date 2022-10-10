use axolotl_nbt::binary::binary_uuid::BinaryUUID;
use axolotl_nbt::binary::Binary;
use axolotl_nbt::serde_impl;
use axolotl_nbt::value::{NameLessValue, Value};
use axolotl_nbt_macros::ListSerialize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use uuid::Uuid;


#[derive(Debug, ListSerialize)]
pub struct Armor {
    pub boots: f32,
    pub chestplate: f32,
    pub helmet: f32,
    pub leggings: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimplePlayer {
    level: i32,
    name: String,
    armor: Armor,
    experience: f32,
    uuid: BinaryUUID,
}

impl Default for SimplePlayer {
    fn default() -> Self {
        Self {
            level: 5,
            name: "Player".to_string(),
            armor: Armor {
                boots: 0.0,
                chestplate: 0.0,
                helmet: 0.0,
                leggings: 0.0,
            },
            experience: 0.0,
            uuid: BinaryUUID::from(Uuid::new_v4()),
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
        armor: Armor {
            boots: 0.0,
            chestplate: 0.0,
            helmet: 0.0,
            leggings: 0.0,
        },
        experience: 0.0,
        uuid: BinaryUUID([-796458901, -684962593, -1840418928, 923062364]),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueTest {
    one: Value,
    two: Vec<NameLessValue>,
    other: HashMap<String, NameLessValue>,
}

#[test]
pub fn value_test() {
    let mut tests = ValueTest {
        one: Value::Compound {
            name: "hey".to_string(),
            value: vec![Value::Boolean {
                name: "test".to_string(),
                value: false,
            }],
        },
        two: vec![NameLessValue::Boolean(false), NameLessValue::Boolean(true)],
        other: HashMap::new(),
    };
    tests
        .other
        .insert("test".to_string(), NameLessValue::Boolean(false));
    tests.other.insert(
        "test2".to_string(),
        NameLessValue::ByteArray(vec![1, 2, 3, 4, 5]),
    );
    tests.other.insert(
        "test3".to_string(),
        NameLessValue::IntArray(vec![1, 2, 3, 4, 5]),
    );
    tests.other.insert(
        "test4".to_string(),
        NameLessValue::LongArray(vec![1, 2, 3, 4, 5]),
    );
    let path = test_output().join("value_test.nbt");
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    let mut file = File::create(&path).expect("a file");
    serde_impl::to_writer(&mut file, &tests).unwrap();

    drop(file);
    let data: ValueTest =
        serde_impl::from_reader::<'_, Binary, File, ValueTest>(File::open(path).unwrap()).unwrap();
    println!("{:?}", data);
}
