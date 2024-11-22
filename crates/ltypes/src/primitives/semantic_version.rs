use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    /// Major version.
    pub major: u32,

    /// Minor version.
    pub minor: u32,

    /// Patch version.
    pub patch: u32,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl SemanticVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        // TODO: validate inputs
        Self {
            major,
            minor,
            patch,
        }
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: &str = Deserialize::deserialize(deserializer).unwrap();
        let tokens: Vec<&str> = raw.split('.').collect();
        if tokens.len() != 3 {
            panic!("SemanticVersion deserialization error.")
        }

        Ok(SemanticVersion {
            major: tokens[0].parse().unwrap(),
            minor: tokens[1].parse().unwrap(),
            patch: tokens[2].parse().unwrap(),
        })
    }
}

impl Serialize for SemanticVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}.{}.{}", self.major, self.minor, self.patch);

        Ok(serializer.serialize_str(&s).unwrap())
    }
}
