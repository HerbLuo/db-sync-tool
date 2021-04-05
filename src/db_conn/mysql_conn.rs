use std::sync::{Mutex, MutexGuard};

use crate::db_conn::DBConn;
use crate::types::{ZzErrors, ClientAddr};
use mysql::{Conn, OptsBuilder, prelude::{FromRow, Queryable}};

pub struct MysqlConn {
    conn: Mutex<Conn>,
}

fn establish_connection(addr: &ClientAddr) -> Result<Conn, ZzErrors> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(&addr.hostname))
        .user(Some(&addr.username))
        .pass(Some(&addr.password))
        .tcp_port(addr.port)
        .db_name(Some(&addr.db));
    return Conn::new(opts)
        .map_err(|e| {
            ZzErrors::ConnectError(format!("无法链接数据库: err: {:?}, addr: {:?}", e, addr))
        });
}

impl DBConn for MysqlConn {
    fn new(addr: &ClientAddr) -> Result<MysqlConn, ZzErrors> {
        Ok(MysqlConn{
            conn: Mutex::new(establish_connection(addr)?),
        })
    }

    fn exec<S: AsRef<str>>(&self, sql: S) -> Result<(), ZzErrors> {
        let mut conn = self.conn.lock().unwrap();
        println!("{}", sql.as_ref());
        (*conn).exec_drop(sql, ()).map_err(|e| {
            ZzErrors::ExecSqlError(format!("执行sql遇到了一个错误 {:?}", e))
        })?;
        return Ok(());
    }

    fn get_conn(&self) -> MutexGuard<Conn> {
        return self.conn.lock().unwrap();
    }

    fn query<S: AsRef<str>, R: FromRow>(&self, sql: S) -> Result<Vec<R>, ZzErrors> {
        let mut conn = self.conn.lock().unwrap();
        println!("{}", sql.as_ref());
        (*conn).query(sql).map_err(|e| {
            ZzErrors::ExecSqlError(format!("执行sql遇到了一个错误 {:?}", e))
        })
    }
}
