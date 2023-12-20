use std::sync::{Arc, Mutex, RwLock};

use mysql::{prelude::Queryable, PooledConn, Statement};

use super::AppModules;

pub struct UserModule {
    statements: Statements,
    con: RwLock<PooledConn>,
}

pub enum Querry {
    GetById(Arc<Mutex<Option<String>>>, String),
}
impl UserModule {
    pub fn new() -> Self {
        let mut con: PooledConn = AppModules::atenda_conection();
        let statements = Statements::new(&mut con);
        Self {
            statements,
            con: RwLock::new(con),
        }
    }
    pub fn test(&self) {
        let r: (u32, String, String, String, String, String, bool) = self
            .con
            .write()
            .unwrap()
            .exec_first(&self.statements.test, ())
            .unwrap()
            .unwrap();
        dbg!(r);
    }
}

struct Statements {
    test: Statement,
}
impl Statements {
    fn new(con: &mut PooledConn) -> Self {
        // "SELECT `id`,`username`,`password`,`nome`,`rol`,`avatar`,`baixa` FROM `usuario` WHERE 1"
        let test = con
            .prep("SELECT `id`,`username`,`password`,`nome`,`rol`,`avatar`,`baixa` FROM `usuario` WHERE 1")
            .unwrap();
        Self { test }
    }
}
