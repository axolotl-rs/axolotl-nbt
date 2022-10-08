use serde::de::SeqAccess;
use serde::ser::SerializeSeq;
use serde::Serialize;
use uuid::Uuid;


/// Represents the UUID the way it is stored in NBT.
///
/// Via 4 i32s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinaryUUID(pub [i32; 4]);


impl From<Uuid> for BinaryUUID {
    fn from(v: Uuid) -> Self {
        let bytes: [u8; 16] = v.into_bytes();
        let mut ints = [0; 4];
        for i in (0..16).step_by(4) {
            ints[i / 4] = i32::from_be_bytes([bytes[i], bytes[1 + i], bytes[2 + i], bytes[3 + i]]);
        }
        Self(ints)
    }
}


#[allow(clippy::from_over_into)]
impl Into<[i64; 2]> for BinaryUUID {
    fn into(self) -> [i64; 2] {
        let lower = self.0[0] as i64 | (self.0[1] as i64) << 32;
        let upper = self.0[2] as i64 | (self.0[3] as i64) << 32;
        [lower, upper]
    }
}


#[allow(clippy::from_over_into)]
impl Into<Uuid> for BinaryUUID {
    fn into(self) -> Uuid {
        let mut bytes = [0; 16];
        for i in (0..16).step_by(4) {
            let int = self.0[i / 4].to_be_bytes();
            bytes[i] = int[0];
            bytes[1 + i] = int[1];
            bytes[2 + i] = int[2];
            bytes[3 + i] = int[3];
        }
        Uuid::from_bytes(bytes)
    }
}


impl Serialize for BinaryUUID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let mut result = serializer.serialize_seq(Some(4))?;
        for int in self.0 {
            result.serialize_element(&int)?;
        }
        result.end()
    }
}

pub struct BinaryUUIDVisitor;

impl<'de> serde::de::Visitor<'de> for BinaryUUIDVisitor {
    type Value = BinaryUUID;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of 4 integers")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
        let option = seq.size_hint();
        if let Some(size) = option {
            if size != 4 {
                return Err(serde::de::Error::invalid_length(size, &self));
            }
        }
        let mut ints = [0; 4];
        for x in ints.iter_mut() {
            *x = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(4, &self))?;
        }
        Ok(BinaryUUID(ints))
    }
}

impl<'de> serde::Deserialize<'de> for BinaryUUID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(BinaryUUIDVisitor)
    }
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;
    use uuid::Uuid;
    use crate::binary::binary_uuid::BinaryUUID;

    pub const MY_UUID_AS_I32: [i32; 4] = [-796458901, -684962593, -1840418928, 923062364];
    pub const MY_UUID: &str = "d087006b-d72c-4cdf-924d-6f903704d05c";


    #[test]
    pub fn test_from_uuid() {
        let uuid = Uuid::from_str(MY_UUID).unwrap();
        let binary_uuid = BinaryUUID::from(uuid);
        assert_eq!(binary_uuid.0, MY_UUID_AS_I32);
    }

    #[test]
    pub fn test_to_uuid() {
        let binary_uuid = BinaryUUID(MY_UUID_AS_I32);
        let uuid: Uuid = binary_uuid.into();
        assert_eq!(uuid.to_string(), MY_UUID);
    }
}