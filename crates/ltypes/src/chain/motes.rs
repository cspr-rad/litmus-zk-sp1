use core::ops::Add;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Base unit of system economic security mechanism.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Motes(u64);

/// Constants.
impl Motes {
    /// Maximum possible value.
    pub const MIN: Motes = Motes(u64::MIN);
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Motes {
    /// Factory: new [`Motes`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Motes {
    pub fn inner(&self) -> u64 {
        self.0
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl Add for Motes {
    type Output = Motes;

    fn add(self, rhs: Self) -> Self::Output {
        Motes::new(self.inner() + rhs.inner())
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

// impl<'de> Deserialize<'de> for Motes {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let raw: &str = Deserialize::deserialize(deserializer).unwrap();
//         println!("123 {:?}", raw);

//         let parsed: u64 = raw.parse().unwrap();
//         Ok(Motes::new(parsed))
//     }
// }

impl<'de> Deserialize<'de> for Motes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MotesVistor;

        impl<'de> Visitor<'de> for MotesVistor {
            type Value = Motes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("supported formats: 64 char hex encoded string | 32 byte array")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Motes(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u64(v.parse().unwrap())
            }
        }

        deserializer.deserialize_any(MotesVistor)
    }
}

impl Serialize for Motes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_u64(self.inner()).unwrap())
    }
}
