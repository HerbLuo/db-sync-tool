use std::sync::MutexGuard;
use mysql::{Conn, prelude::FromRow};
use crate::types::{ZzErrors, ClientAddr};

pub mod mysql_conn;

pub trait DBConn {
    fn new(addr: &ClientAddr) -> Result<Self, ZzErrors> where Self: Sized;
    fn exec<S: AsRef<str>>(&self, sql: S) -> Result<(), ZzErrors>;
    fn get_conn(&self) -> MutexGuard<Conn>;
    fn query<S: AsRef<str>, R: FromRow>(&self, sql: S) -> Result<Vec<R>, ZzErrors>;
}
