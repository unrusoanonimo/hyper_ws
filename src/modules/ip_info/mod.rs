use std::collections::HashMap;

use sqlite::{Connection, Statement};

use crate::model::{self, ip_info::DataFromIp};

use super::{Error, Result};

pub trait Api {
    fn register_visit(&mut self, data: DataFromIp) -> Result<HashMap<String, u64>>;
}
impl<'a> Api for IpInfoModule<'a> {
    fn register_visit(&mut self, data: DataFromIp) -> Result<HashMap<String, u64>> {
        match self.get_by_ip(&data.ip) {
            Ok(mut v) => {
                v.visites += 1;
                self.update(&v)?;
            }
            Err(e) => match e {
                Error::InvalidOperation => {
                    self.insert(&model::IpInfo { data, visites: 1 })?;
                }
                e => Err(e)?,
            },
        }
        
        todo!()
    }
}

pub struct IpInfoModule<'a> {
    connection: Connection,
    stmts: Option<Statements<'a>>,
}
impl<'a> IpInfoModule<'a> {
    pub fn new() -> Self {
        let connection: sqlite::Connection = sqlite::open("data/ip_info.sqlite").unwrap();

        connection
            .iterate("SELECT * FROM IP_INFO", |p| {
                dbg!(p);
                true
            })
            .unwrap();

        let module: IpInfoModule = Self {
            connection,
            stmts: None,
        };
        module.build();

        module
    }
    fn build(&'a self) {
        let uncheked = unsafe { (self as *const _ as *mut Self).as_mut().unwrap() };
        let st = Statements::new(&self.connection);
        uncheked.stmts = Some(st);
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
            data: model::ip_info::DataFromIp {
                city,
                country,
                ip,
                loc,
                postal,
                region,
                timezone,
                org,
            },
            visites: visites as u64,
        };
        Ok(info)
    }

    fn exists_ip(&mut self, ip: &str) -> Result<bool> {
        let statement = &mut self.stmts.as_mut().unwrap().exists_ip;
        statement.reset()?;
        statement.bind((":ip", ip))?;

        statement.next().or(Err(Error::InvalidOperation))?;

        let n: i64 = statement.read("exists")?;
        Ok(n > 0)
    }

    fn get_by_ip(&mut self, ip: &str) -> Result<model::IpInfo> {
        let statement = &mut self.stmts.as_mut().unwrap().get_by_ip;
        statement.reset()?;
        statement.bind((":ip", ip))?;

        statement.next().or(Err(Error::InvalidOperation))?;

        Self::parse_row(statement)
    }

    fn insert(&mut self, info: &model::IpInfo) -> Result<()> {
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
        statement.bind((":visites", info.visites as i64))?;

        statement.next().or(Err(Error::InvalidOperation))?;

        Ok(())
    }

    fn len(&mut self) -> Result<u64> {
        let statement = &mut self.stmts.as_mut().unwrap().len;
        statement.reset()?;

        statement.next().or(Err(Error::InvalidOperation))?;
        let len: i64 = statement.read("len")?;

        Ok(len as u64)
    }

    fn update(&mut self, info: &model::IpInfo) -> Result<()> {
        let statement = &mut self.stmts.as_mut().unwrap().insert;
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

        statement.next().or(Err(Error::InvalidOperation))?;

        Ok(())
    }
}
unsafe impl<'a> Send for IpInfoModule<'a> {}

struct Statements<'a> {
    pub insert: Statement<'a>,
    pub get_by_ip: Statement<'a>,
    pub len: Statement<'a>,
    pub exists_ip: Statement<'a>,
    pub update: Statement<'a>,
}

impl<'a> Statements<'a> {
    pub fn new(con: &'a Connection) -> Statements {
        con.execute(include_str!("init.sql")).unwrap();

        let insert: Statement<'a> = con.prepare(include_str!("insert.sql")).unwrap();
        let get_by_ip = con.prepare(include_str!("get_by_ip.sql")).unwrap();
        let len = con.prepare(include_str!("len.sql")).unwrap();
        let exists_ip = con.prepare(include_str!("exists_ip.sql")).unwrap();
        let update = con.prepare(include_str!("update.sql")).unwrap();

        return Self {
            insert,
            get_by_ip,
            len,
            exists_ip,
            update,
        };
    }
}
