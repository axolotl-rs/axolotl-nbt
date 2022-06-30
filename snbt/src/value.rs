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

#[cfg(feature = "axolotl-nbt")]
impl From<axolotl_nbt::value::NameLessValue> for NameLessValue {
    fn from(value: axolotl_nbt::value::NameLessValue) -> Self {
        match value {
            axolotl_nbt::value::NameLessValue::End => NameLessValue::End,
            axolotl_nbt::value::NameLessValue::Byte(value) => NameLessValue::Byte(value),
            axolotl_nbt::value::NameLessValue::Short(value) => NameLessValue::Short(value),
            axolotl_nbt::value::NameLessValue::Int(value) => NameLessValue::Int(value),
            axolotl_nbt::value::NameLessValue::Long(value) => NameLessValue::Long(value),
            axolotl_nbt::value::NameLessValue::Float(value) => NameLessValue::Float(value),
            axolotl_nbt::value::NameLessValue::Double(value) => NameLessValue::Double(value),
            axolotl_nbt::value::NameLessValue::ByteArray(value) => NameLessValue::ByteArray(value),
            axolotl_nbt::value::NameLessValue::String(value) => NameLessValue::String(value),
            axolotl_nbt::value::NameLessValue::List(value) => {
                NameLessValue::List(value.into_iter().map(|v| v.into()).collect())
            }
            axolotl_nbt::value::NameLessValue::Compound(value) => {
                NameLessValue::Compound(value.into_iter().map(|v| v.into()).collect())
            }
            axolotl_nbt::value::NameLessValue::IntArray(value) => NameLessValue::IntArray(value),
            axolotl_nbt::value::NameLessValue::LongArray(value) => NameLessValue::LongArray(value),
        }
    }
}

#[cfg(feature = "axolotl-nbt")]
impl From<axolotl_nbt::value::Value> for Value {
    fn from(value: axolotl_nbt::value::Value) -> Self {
        match value {
            axolotl_nbt::value::Value::End => Value::End,
            axolotl_nbt::value::Value::Byte { value, name } => Value::Byte {
                name: name,
                value: value,
            },
            axolotl_nbt::value::Value::Short { value, name } => Value::Short {
                name: name,
                value: value,
            },
            axolotl_nbt::value::Value::Int { value, name } => Value::Int {
                name: name,
                value: value,
            },

            axolotl_nbt::value::Value::Long { value, name } => Value::Long { name, value },
            axolotl_nbt::value::Value::Float { value, name } => Value::Float {
                name: name,
                value: value,
            },
            axolotl_nbt::value::Value::Double { value, name } => Value::Double {
                name: name,
                value: value,
            },
            axolotl_nbt::value::Value::ByteArray { value, name } => Value::ByteArray {
                name: name,
                value: value,
            },
            axolotl_nbt::value::Value::String { value, name } => Value::String {
                name: name,
                value: value,
            },
            axolotl_nbt::value::Value::List { value, name } => Value::List {
                name: name,
                value: value.into_iter().map(|v| v.into()).collect(),
            },
            axolotl_nbt::value::Value::Compound { value, name } => Value::Compound {
                name: name,
                value: value.into_iter().map(|v| v.into()).collect(),
            },
            axolotl_nbt::value::Value::IntArray { value, name } => Value::IntArray {
                name: name,
                value: value,
            },
            axolotl_nbt::value::Value::LongArray { value, name } => Value::LongArray {
                name: name,
                value: value,
            },
        }
    }
}
