mod list_serialize;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

/// Minecraft uses lists to represent a lot of things that I want to represent in memory as a Structure.
/// For example Armor within a Mob is an Array of 4 Items. I want it represented as a Structure
/// ```rust ignore
///#[derive(Debug, Clone, ListSerialize)]
///pub struct ArmorDropChances {
///    pub boots: f32,
///    pub chestplate: f32,
///    pub helmet: f32,
///    pub leggings: f32,
///}
///```
/// Is represented in NBT as a List of 4 floats
///
/// When using this macro please make sure you order the fields in the same order as the list.
///
/// This macro will generate a `Serialize` and `Deserialize` implementation for a structure that in NBT will be represented as a list.
#[proc_macro_derive(ListSerialize)]
pub fn list_serialize(stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(stream as DeriveInput);
    if let Data::Struct(v) = input.data {
        list_serialize::parse_struct(input.ident, v)
            .unwrap_or_else(|e| e.to_compile_error())
            .into()
    } else {
        panic!("ListSerialize can only be used on structs");
    }
}
