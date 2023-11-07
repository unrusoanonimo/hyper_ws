use sqlite::Connection;

use crate::model;

use super::{Error, Result};

pub struct IpInfoModule<'a> {
    connection: Connection,
    stmts: Option<Statements<'a>>,
}
impl<'a> IpInfoModule<'a> {
    pub fn new() -> Self {
        let connection: sqlite::Connection = sqlite::open("data/ip_info.sqlite").unwrap();

        let modules: IpInfoModule = Self {
            connection,
            stmts: None,
        };
        modules.build();
        modules
    }
    fn build(&'a self) {
        let uncheked = unsafe { (self as *const _ as *mut Self).as_mut().unwrap() };
        let st = Statements::new(&self.connection);
        uncheked.stmts = Some(st);
    }

    pub fn get_by_ip(&mut self, ip: &str) -> Result<model::IpInfo> {
        let statement = &mut self.stmts.as_mut().unwrap().get_by_ip;
        statement.reset()?;
        statement.bind((":ip", ip))?;

        statement.next().or(Err(Error::InvalidOperation))?;

        let ip: String = statement.read("ip")?;
        let city: String = statement.read("city")?;
        let region: String = statement.read("region")?;
        let country: String = statement.read("country")?;
        let loc: String = statement.read("loc")?;
        let org: Option<String> = statement.read("org")?;
        let postal: String = statement.read("postal")?;
        let timezone: String = statement.read("timezone")?;

        let info = model::IpInfo {
            city,
            country,
            ip,
            loc,
            postal,
            region,
            timezone,
            org,
        };
        Ok(info)
    }

    pub fn insert(&mut self, info: &model::IpInfo) -> Result<()> {
        let statement = &mut self.stmts.as_mut().unwrap().insert;
        statement.reset()?;

        statement.bind((":ip", info.ip.as_str()))?;
        statement.bind((":city", info.city.as_str()))?;
        statement.bind((":region", info.region.as_str()))?;
        statement.bind((":country", info.country.as_str()))?;
        statement.bind((":loc", info.loc.as_str()))?;
        statement.bind((":org", info.org.as_ref().map(|v| v.as_str())))?;
        statement.bind((":postal", info.postal.as_str()))?;
        statement.bind((":timezone", info.timezone.as_str()))?;

        statement.next().or(Err(Error::InvalidOperation))?;

        Ok(())
    }
}
unsafe impl<'a> Send for IpInfoModule<'a> {}

struct Statements<'a> {
    pub insert: sqlite::Statement<'a>,
    pub get_by_ip: sqlite::Statement<'a>,
}

impl<'a> Statements<'a> {
    pub fn new(con: &'a sqlite::Connection) -> Statements {
        let insert: sqlite::Statement<'a> = con
            .prepare(
                "INSERT INTO table_name (ip, city, region, country, loc, org, postal, timezone) VALUES (:ip, :city, :region, :country, :loc, :org, :postal, :timezone)",
            )
            .unwrap();
        let get_by_ip = con
            .prepare(
                "SELECT ip, city, region, country, loc, org, postal, timezone FROM table_name WHERE ip == :ip",
            )
            .unwrap();

        return Self { insert, get_by_ip };
    }
}
