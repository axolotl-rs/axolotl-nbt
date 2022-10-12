use crate::Tag;
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

impl From<Value> for (NameLessValue, String) {
    fn from(v: Value) -> Self {
        match v {
            Value::End => (NameLessValue::End, String::new()),
            Value::Byte { value, name } => (NameLessValue::Byte(value), name),
            Value::Short { value, name } => (NameLessValue::Short(value), name),
            Value::Int { value, name } => (NameLessValue::Int(value), name),
            Value::Long { value, name } => (NameLessValue::Long(value), name),
            Value::Float { value, name } => (NameLessValue::Float(value), name),
            Value::Double { value, name } => (NameLessValue::Double(value), name),
            Value::ByteArray { value, name } => (NameLessValue::ByteArray(value), name),
            Value::String { value, name } => (NameLessValue::String(value), name),
            Value::List { value, name } => (NameLessValue::List(value), name),
            Value::Compound { value, name } => (NameLessValue::Compound(value), name),
            Value::IntArray { value, name } => (NameLessValue::IntArray(value), name),
            Value::LongArray { value, name } => (NameLessValue::LongArray(value), name),
            Value::Boolean { value, name } => (NameLessValue::Boolean(value), name),
        }
    }
}

impl Value {
    pub fn tag(&self) -> Tag {
        match self {
            Value::End => Tag::End,
            Value::Byte { .. } => Tag::Byte,
            Value::Short { .. } => Tag::Short,
            Value::Int { .. } => Tag::Int,
            Value::Long { .. } => Tag::Long,
            Value::Float { .. } => Tag::Float,
            Value::Double { .. } => Tag::Double,
            Value::ByteArray { .. } => Tag::ByteArray,
            Value::String { .. } => Tag::String,
            Value::List { .. } => Tag::List,
            Value::Compound { .. } => Tag::Compound,
            Value::IntArray { .. } => Tag::IntArray,
            Value::LongArray { .. } => Tag::LongArray,
            Value::Boolean { .. } => Tag::Byte,
        }
    }
    pub fn get_name(&self) -> &str {
        match self {
            Value::End => "",
            Value::Byte { name, .. } => name.as_str(),
            Value::Short { name, .. } => name.as_str(),
            Value::Int { name, .. } => name.as_str(),
            Value::Long { name, .. } => name.as_str(),

            Value::Float { name, .. } => name.as_str(),
            Value::Double { name, .. } => name.as_str(),
            Value::ByteArray { name, .. } => name.as_str(),
            Value::String { name, .. } => name.as_str(),
            Value::List { name, .. } => name.as_str(),
            Value::Compound { name, .. } => name.as_str(),
            Value::IntArray { name, .. } => name.as_str(),
            Value::LongArray { name, .. } => name.as_str(),
            Value::Boolean { name, .. } => name.as_str(),
        }
    }
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
    Boolean(bool),
}

impl NameLessValue {
    pub fn tag(&self) -> Tag {
        match self {
            NameLessValue::End => Tag::End,
            NameLessValue::Byte(_) => Tag::Byte,
            NameLessValue::Short(_) => Tag::Short,
            NameLessValue::Int(_) => Tag::Int,
            NameLessValue::Long(_) => Tag::Long,
            NameLessValue::Float(_) => Tag::Float,
            NameLessValue::Double(_) => Tag::Double,
            NameLessValue::ByteArray(_) => Tag::ByteArray,
            NameLessValue::String(_) => Tag::String,
            NameLessValue::List(_) => Tag::List,
            NameLessValue::Compound(_) => Tag::Compound,
            NameLessValue::IntArray(_) => Tag::IntArray,
            NameLessValue::LongArray(_) => Tag::LongArray,
            NameLessValue::Boolean(_) => Tag::Byte,
        }
    }
}
