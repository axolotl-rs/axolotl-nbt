use std::fmt::Debug;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    End,
    Byte {
        name: String,
        value: i8,
    },
    Short {
        name: String,
        value: i16,
    },
    Int {
        name: String,
        value: i32,
    },
    Long {
        name: String,
        value: i64,
    },
    Float {
        name: String,
        value: f32,
    },
    Double {
        name: String,
        value: f64,
    },
    ByteArray {
        name: String,
        value: Vec<i8>,
    },
    String {
        name: String,
        value: String,
    },
    List {
        name: String,
        value: Vec<NameLessValue>,
    },
    Compound {
        name: String,
        value: Vec<Value>,
    },
    IntArray {
        name: String,
        value: Vec<i32>,
    },
    LongArray {
        name: String,
        value: Vec<i64>,
    },
    Boolean {
        name: String,
        value: bool,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum NameLessValue {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NameLessValue>),
    Compound(Vec<Value>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    Bool(bool),
}
