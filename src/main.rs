#[macro_use]
extern crate serde;

use crate::types::{SyncConfig, ZzErrors, DbConfig, SqlGroups, SqlGroup};
use crate::helpers::{read_file_as_sql_group, save_sql_to_dir};
use futures::executor::block_on;
use helpers::db_to_sql_group;
use crate::ui::start_tray;
use crate::db_conn::DBConn;
use crate::db_conn::mysql_conn::MysqlConn;

mod db_conn;
mod types;
mod helpers;
mod ui;

fn read_config() -> Result<SyncConfig, ZzErrors> {
    let data = r#"{"from":{"hostname":"127.0.0.1","username":"root","db":"zz_trans","password":"123456","port":3306},"to":"sql","tables":"*","mode":"drop-create"}"#;
    // let data = r#"{"from":"sql","to":"sql","tables":"*","mode":"drop-create"}"#;
    serde_json::from_str(data).map_err(|e| ZzErrors::ParseConfigError(e))
}

async fn sql_to_db(conn: &impl DBConn, boxed_sql_groups: Box<SqlGroups>) -> Result<(), ZzErrors> {
    let sql_groups = *boxed_sql_groups;
    for sql_group in sql_groups {
        conn.exec(format!("TRUNCATE table `{}`", sql_group.schema))?;
        for sql in sql_group.sqls {
            conn.exec(sql)?;
        }
    }
    Ok(())
}

fn db_to_sql() {

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
                |sql_groups|
                    Box::pin(sql_to_db(&conn, sql_groups))
            ).await?;
        }
    } else if let DbConfig::ClientAddr(addr) = &config.from {
        db_to_sql_group(
            &MysqlConn::new(addr)?, 
            &config.tables, 
            config.buffer_size, 
            |sql_group| {
                println!("{:?}", *sql_group);
                Box::pin(async {})
            }
        ).await?;
        if let DbConfig::Path(to_dir) = config.to {
            let mut sync_started_schemas: Vec<String> = vec![];

            let sql_groups = vec![SqlGroup{
                schema: "test".to_string(),
                sqls: vec!["select 1;".to_string()]
            }];
            save_sql_to_dir(&mut sync_started_schemas, &sql_groups, &to_dir)?;
            save_sql_to_dir(&mut sync_started_schemas, &sql_groups, &to_dir)?;
        } else if let DbConfig::ClientAddr(_to_db_conn) = config.to {
            // sql_to_db()
        }
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
