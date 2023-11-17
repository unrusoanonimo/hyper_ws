use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpInfo {
    pub visites: u64,
    pub data: DataFromIp,
}
impl Deref for IpInfo {
    type Target = DataFromIp;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for IpInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataFromIp {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: Option<String>,
    pub postal: String,
    pub timezone: String,
}
