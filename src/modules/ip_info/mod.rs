use std::collections::HashMap;

use ip_info::IpInfo;
use sqlite::{Connection, State, Statement};

use crate::{
    model::{
        self,
        ip_info::{self, DataFromIp, PartialIpInfo},
    },
    util::count_map,
};

use super::{Error, Result};

pub trait Api {
    fn register_visit(&mut self, data: DataFromIp) -> Result<()>;
    fn get_flags(&mut self) -> Result<HashMap<String, usize>>;
    fn update(&mut self, info: ip_info::PartialIpInfo) -> Result<model::IpInfo>;
}
impl Api for IpInfoModule {
    fn register_visit(&mut self, data: DataFromIp) -> Result<()> {
        match self.get_by_ip(&data.ip) {
            Ok(mut v) => {
                v.visites += 1;
                self.update(&v)?;
            }
            Err(e) => match e {
                Error::InvalidOperation => {
                    self.insert(&data.into())?;
                }
                e => Err(e)?,
            },
        }
        Ok(())
    }
    fn get_flags(&mut self) -> Result<HashMap<String, usize>> {
        Ok(count_map(self.get_all()?.into_iter().map(|v| v.country)))
    }
    fn update(&mut self, info: PartialIpInfo) -> Result<IpInfo> {
        let ip = info.ip.as_ref().ok_or(Error::InvalidInput)?;
        let mut base = self.get_by_ip(ip)?;
        base.merge(info);
        self.update(&base)?;
        Ok(base)
    }
}

pub struct IpInfoModule {
    connection: Connection,
}
impl IpInfoModule {
    pub fn new() -> Self {
        let connection: sqlite::Connection = sqlite::open("data/ip_info.sqlite").unwrap();

        Self { connection }
    }

    fn parse_row(statement: &mut Statement) -> Result<model::IpInfo> {
        let ip: String = statement.read("ip")?;
        let city: String = statement.read("city")?;
        let region: String = statement.read("region")?;
        let country: String = statement.read("country")?;
        let loc: String = statement.read("loc")?;
        let org: Option<String> = statement.read("org")?;
        let postal: String = statement.read("postal")?;
        let timezone: String = statement.read("timezone")?;
        let visites: i64 = statement.read("visites")?;

        let info = model::IpInfo {
            city,
            country,
            ip,
            loc,
            postal,
            region,
            timezone,
            org,
            visites: visites as u64,
        };
        Ok(info)
    }

    fn exists_ip(&mut self, ip: &str) -> Result<bool> {
        let mut statement = Statements::exists_ip(&self.connection);
        statement.reset()?;
        statement.bind((":ip", ip))?;

        if statement.next()? == State::Done {
            return Err(Error::InvalidOperation);
        }

        let n: i64 = statement.read("exists")?;
        Ok(n > 0)
    }

    pub fn get_by_ip(&mut self, ip: &str) -> Result<model::IpInfo> {
        let mut statement = Statements::get_by_ip(&self.connection);
        statement.reset()?;
        statement.bind((":ip", ip))?;

        if statement.next()? == State::Done {
            return Err(Error::InvalidOperation);
        }
        Self::parse_row(&mut statement)
    }

    fn insert(&mut self, info: &model::IpInfo) -> Result<()> {
        let mut statement = Statements::insert(&self.connection);
        statement.reset()?;

        statement.bind((":ip", info.ip.as_str()))?;
        statement.bind((":city", info.city.as_str()))?;
        statement.bind((":region", info.region.as_str()))?;
        statement.bind((":country", info.country.as_str()))?;
        statement.bind((":loc", info.loc.as_str()))?;
        statement.bind((":org", info.org.as_ref().map(|v| v.as_str())))?;
        statement.bind((":postal", info.postal.as_str()))?;
        statement.bind((":timezone", info.timezone.as_str()))?;
        statement.bind((":visites", info.visites as i64))?;

        if statement.next()? == State::Done {
            return Err(Error::InvalidOperation);
        }

        Ok(())
    }

    fn len(&mut self) -> Result<u64> {
        let mut statement = Statements::len(&self.connection);
        statement.reset()?;

        if statement.next()? == State::Done {
            return Err(Error::InvalidOperation);
        }
        let len: i64 = statement.read("len")?;

        Ok(len as u64)
    }

    fn update(&mut self, info: &model::IpInfo) -> Result<()> {
        let mut statement = Statements::update(&self.connection);
        statement.reset()?;

        statement.bind((":city", info.city.as_str()))?;
        statement.bind((":region", info.region.as_str()))?;
        statement.bind((":country", info.country.as_str()))?;
        statement.bind((":loc", info.loc.as_str()))?;
        statement.bind((":org", info.org.as_ref().map(|v| v.as_str())))?;
        statement.bind((":postal", info.postal.as_str()))?;
        statement.bind((":timezone", info.timezone.as_str()))?;
        statement.bind((":visites", info.visites as i64))?;
        statement.bind((":ip", info.ip.as_str()))?;

        statement.next()?;

        Ok(())
    }

    pub fn get_all(&mut self) -> Result<Vec<model::IpInfo>> {
        let mut statement = Statements::get_all(&self.connection);
        statement.reset()?;

        let mut r = vec![];
        while let Ok(State::Row) = statement.next() {
            r.push(Self::parse_row(&mut statement)?);
        }

        Ok(r)
    }
}
unsafe impl Send for IpInfoModule {}

struct Statements;

impl Statements {
    pub fn insert(con: &Connection) -> Statement<'_> {
        con.prepare(include_str!("insert.sql")).unwrap()
    }

    pub fn get_all(con: &Connection) -> Statement<'_> {
        con.prepare(include_str!("get_all.sql")).unwrap()
    }

    pub fn get_by_ip(con: &Connection) -> Statement<'_> {
        con.prepare(include_str!("get_by_ip.sql")).unwrap()
    }

    pub fn len(con: &Connection) -> Statement<'_> {
        con.prepare(include_str!("len.sql")).unwrap()
    }

    pub fn exists_ip(con: &Connection) -> Statement<'_> {
        con.prepare(include_str!("exists_ip.sql")).unwrap()
    }

    pub fn update(con: &Connection) -> Statement<'_> {
        con.prepare(include_str!("update.sql")).unwrap()
    }
}
