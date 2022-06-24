#[derive(PartialEq)]
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
        value: Vec<u8>,
    },
    String {
        name: String,
        value: String,
    },
    List {
        name: String,
        value: Vec<Value>,
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

}
