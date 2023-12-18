use sqlite::{Connection, Statement};

pub struct UserModule {
    // connection: Connection,
    // statements: Statements<'a>,
}

impl UserModule {
    pub fn new(con: Connection) -> Self {
        let statements = Statements::new(&con);

        // Statements::init(&connection).next().unwrap();

        Self { }
    }
}

struct Statements<'a> {
    init: Statement<'a>,
}
impl<'a> Statements<'a> {
    fn new(con: &'a Connection) -> Self {
        Self {
            init: con.prepare(include_str!("sql/init.sql")).unwrap(),
        }
    }
}
