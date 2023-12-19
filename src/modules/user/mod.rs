use std::{
    ops::DerefMut,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex, OnceLock,
    },
    thread, time::Duration,
};

use sqlite::{Connection, Statement};

use super::AppModules;

pub struct UserModule {
    // connection: Connection,
    // statements: Statements<'a>,
    sender: Sender<Querry>,
}
pub enum Querry {
    GetById(Arc<Mutex<Option<String>>>, String),
}
impl UserModule {
    pub fn new() -> Self {
        let (sender, reciver) = channel::<Querry>();
        thread::spawn(move || {
            let con = AppModules::atenda_conection();
            let _statements = Statements::new(&con);

            while let Ok(q) = reciver.recv() {
                match q {
                    Querry::GetById(r, mut s) => {
                        thread::sleep(Duration::from_secs(s.parse().unwrap()));
                        s += "manolo";
                        r.lock().unwrap().insert(s);
                    }
                }
            }
        });

        // Statements::init(&connection).next().unwrap();

        Self { sender }
    }

    pub fn get_by_id(&self, id: String) -> String {
        let r = Arc::new(Mutex::new(None));
        self.sender
            .send(Querry::GetById(Arc::clone(&r), id))
            .unwrap();
        let b = loop {
            if let Some(v) = r.lock().unwrap().take() {
                break v;
            }
        };
        b
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
