use sqlite::Connection;
use tokio::sync::Mutex;

mod ip_info;
pub use ip_info::IpInfoModule;

pub struct AppModules<'a> {
    pub db: Mutex<DbModule<'a>>,
    pub c: Mutex<u32>,
    pub ip_info: Mutex<IpInfoModule<'a>>,
}
unsafe impl<'a> Sync for AppModules<'a> {}

impl<'a> AppModules<'a> {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(DbModule::new()),
            c: Mutex::new(0),
            ip_info: Mutex::new(IpInfoModule::new()),
        }
    }
}

pub struct DbModule<'a> {
    connection: Connection,
    stmts: Option<Statements<'a>>,
}
unsafe impl<'a> Send for DbModule<'a> {}

impl<'a> DbModule<'a> {
    pub fn new() -> Self {
        let connection: sqlite::Connection = sqlite::open("e.sqlite").unwrap();

        let modules: DbModule = Self {
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
    pub fn get_by_name(&mut self, name: &str) -> Option<String> {
        let statement = &mut self.stmts.as_mut().unwrap().statement1;
        statement.reset().ok()?;
        statement.bind((":name", name)).ok()?;
        statement.next().ok()?;
        let out = statement.read("age").ok()?;
        Some(out)
    }
}

struct Statements<'a> {
    pub statement1: sqlite::Statement<'a>,
}

impl<'a> Statements<'a> {
    pub fn new(con: &'a sqlite::Connection) -> Statements {
        let statement1: sqlite::Statement<'a> = con
            .prepare("SELECT * FROM users WHERE name == :name")
            .unwrap();
        return Self { statement1 };
    }
}

const DB_INIT: &str = "
CREATE TABLE users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('Alice', 42);
INSERT INTO users VALUES ('Bob', 69);
";
