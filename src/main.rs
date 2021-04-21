#[macro_use]
extern crate serde;

use crate::types::{SyncConfig, ZzErrors, DbConfig};
use crate::helpers::{read_file_as_sql_group, save_sql_to_dir};
use futures::executor::block_on;
use helpers::{db_to_sql_group, sql_to_db};
use crate::ui::start_tray;
use crate::db_conn::DBConn;
use crate::db_conn::mysql_conn::MysqlConn;
use std::sync::Mutex;

mod db_conn;
mod types;
mod helpers;
mod ui;

fn read_config() -> Result<SyncConfig, ZzErrors> {
    let data = r#"{
        "from":{"hostname":"127.0.0.1","username":"root","db":"zz_trans","password":"123456","port":3306},
        "to":{"hostname":"127.0.0.1","username":"root","db":"words","password":"123456","port":3306},
        "tables":"*",
        "mode":"drop-create",
        "skip_if_table_not_exist":true
    }"#;
    serde_json::from_str(data).map_err(|e| ZzErrors::ParseConfigError(e))
}

async fn run_sync(config: SyncConfig) -> Result<(), ZzErrors> {
    if let DbConfig::Path(from_sql_path) = &config.from {
        if let DbConfig::Path(_) = &config.to {
            return Err(ZzErrors::IllegalConfig("不支持sql to sql的模式".to_string()))
        } else if let DbConfig::ClientAddr(addr) = &config.to {
            let conn = MysqlConn::new(addr)?;
            read_file_as_sql_group(
                &from_sql_path,
                config.buffer_size,
                |sql_groups| Box::pin(sql_to_db(&conn, sql_groups))
            ).await?;
        }
    } else if let DbConfig::ClientAddr(addr) = &config.from {
        if let DbConfig::Path(to_dir) = &config.to {
            let sync_started_schemas: Mutex<Vec<String>> = Mutex::new(vec![]);
            db_to_sql_group(
                &MysqlConn::new(addr)?, 
                &config.tables, 
                config.buffer_size, 
                |sql_group| {
                    let res = save_sql_to_dir(
                        &mut sync_started_schemas.lock().unwrap(), 
                        &sql_group, 
                        to_dir
                    );
                    Box::pin(async {res})
                },
            ).await?;
        } else if let DbConfig::ClientAddr(to_db_addr) = &config.to {
            let to_db_conn = MysqlConn::new(to_db_addr)?;
            db_to_sql_group(
                &MysqlConn::new(addr)?, 
                &config.tables, 
                config.buffer_size, 
                |sql_group| Box::pin(sql_to_db(&to_db_conn, sql_group)),
            ).await?;
        };
    }

    Ok(())
}

fn main() {
    let config = read_config().unwrap();
    println!("{:?}", config);
    let sync_res = block_on(run_sync(config));
    println!("{:?}", sync_res);
    start_tray();
}
