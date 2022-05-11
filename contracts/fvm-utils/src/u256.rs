use anyhow::Error;
use fvm_shared::bigint::BigUint;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;
use std::marker::Copy;
use std::ops::{Add, Sub};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct U256 {
    pub big_uint: BigUint,
}

impl U256 {
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        Self {
            big_uint: BigUint::from_bytes_be(bytes),
        }
    }

    pub fn from_dec_str(value: &str) -> Self {
        Self {
            big_uint: BigUint::parse_bytes(("b".to_string() + value).as_bytes(), 10).unwrap()
        }
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.big_uint.to_bytes_be()
    }

    pub fn to_string(&self) -> String {
        self.big_uint.to_string()
    }
}

impl Add for U256 {
    type Output = U256;

    fn add(self, rhs: Self) -> Self::Output {
        return U256 {
            big_uint: self.big_uint + rhs.big_uint,
        };
    }
}

impl Sub for U256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            big_uint: self.big_uint - other.big_uint,
        }
    }
}

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut ts = serializer.serialize_struct("U256", 1)?;
        ts.serialize_field("big_uint", &self.big_uint.to_bytes_le())?;
        ts.end()
    }
}

impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field {
            Big_Uint,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("big_uint")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "big_uint" => Ok(Field::Big_Uint),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct U256Visitor;

        impl<'de> Visitor<'de> for U256Visitor {
            type Value = U256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct U256")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<U256, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let big_uint = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let big_uint = BigUint::from_bytes_le(big_uint);
                Ok(U256 { big_uint: big_uint })
            }

            fn visit_map<V>(self, mut map: V) -> Result<U256, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut big_uint = None;
                if let Some(key) = map.next_key()? {
                    match key {
                        Field::Big_Uint => {
                            if big_uint.is_some() {
                                return Err(de::Error::duplicate_field("big_uint"));
                            }
                            let value = BigUint::from_bytes_le(&map.next_value::<Vec<u8>>()?);
                            big_uint = Some(value);
                        }
                    }
                }

                let big_uint = big_uint.ok_or_else(|| de::Error::missing_field("big_uint"))?;

                Ok(U256 { big_uint: big_uint })
            }
        }

        const FIELDS: &'static [&'static str] = &["big_uint"];
        deserializer.deserialize_struct("U256", FIELDS, U256Visitor)
    }
}