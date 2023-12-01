use optfield::optfield;
use serde::{Deserialize, Serialize};

#[optfield(pub PartialIpInfo, rewrap, attrs, merge_fn = pub merge, from )]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IpInfo {
    pub visites: u64,
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: Option<String>,
    pub postal: String,
    pub timezone: String,
}
impl From<DataFromIp> for IpInfo {
    fn from(value: DataFromIp) -> Self {
        Self {
            visites: 1,
            ip: value.ip,
            city: value.city,
            region: value.region,
            country: value.country,
            loc: value.loc,
            org: value.org,
            postal: value.postal,
            timezone: value.timezone,
        }
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
