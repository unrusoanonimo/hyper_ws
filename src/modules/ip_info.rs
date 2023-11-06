use sqlite::Connection;

pub struct IpInfoModule<'a> {
    connection: Connection,
    stmts: Option<Statements<'a>>,
}
impl<'a> IpInfoModule<'a> {
    pub fn new() -> Self {
        let connection: sqlite::Connection = sqlite::open("e.sqlite").unwrap();

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
}
unsafe impl<'a> Send for IpInfoModule<'a> {}

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
