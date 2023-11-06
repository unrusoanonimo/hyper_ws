use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpInfo {
    ip: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: Option<String>,
    postal: String,
    timezone: String,
}
