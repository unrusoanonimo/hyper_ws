use serde::{de, ser::SerializeSeq, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, impl_new::New)]
pub struct ModpackData {
    pub version: String,
    pub mods: Vec<ModData>,
    pub config: String,
    pub datapacks: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, impl_new::New)]
pub struct ModData {
    pub side: ModSide,
    pub name: String,
    pub download: String,
}
impl ModData {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModSide(u8);
impl ModSide {
    const SERVER_BIT: u8 = 0b01;
    const CLIENT_BIT: u8 = 0b10;

    pub const SERVER: Self = Self(Self::SERVER_BIT);
    pub const CLIENT: Self = Self(Self::CLIENT_BIT);
    pub const BOTH: Self = Self(Self::SERVER_BIT | Self::CLIENT_BIT);

    fn parse_str(str: &str) -> Option<u8> {
        match str {
            "SERVER" => Some(Self::SERVER_BIT),
            "CLIENT" => Some(Self::CLIENT_BIT),
            _ => None,
        }
    }

    fn parse_arr(v: &[&str]) -> Option<u8> {
        if v.len() == 0 {
            return None;
        }
        let mut r = 0;
        for s in v {
            r |= Self::parse_str(s)?;
        }
        Some(r)
    }
}
impl Serialize for ModSide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        match *self {
            Self::SERVER => {
                seq.serialize_element("SERVER")?;
            }
            Self::CLIENT => {
                seq.serialize_element("CLIENT")?;
            }
            Self::BOTH => {
                seq.serialize_element("SERVER")?;
                seq.serialize_element("CLIENT")?;
            }
            _ => unreachable!(),
        }
        seq.end()
    }
}
impl<'de> Deserialize<'de> for ModSide {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = Vec::<&str>::deserialize(deserializer)?;

        Self::parse_arr(&v)
            .map(|v| Self(v))
            .ok_or_else(|| de::Error::custom("vec len >= 1"))
    }
}
