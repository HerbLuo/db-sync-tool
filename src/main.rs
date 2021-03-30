#[macro_use]
extern crate serde;

use crate::types::{SyncConfig, ZzErrors, DbConfig, SqlGroups, SqlGroup};
use crate::helpers::{read_file_as_sql_group, save_sql_to_dir};
use futures::executor::block_on;
use mysql::Conn;
use mysql::prelude::Queryable;

mod types;
mod helpers;

fn read_config() -> Result<SyncConfig, ZzErrors> {
    let data = r#"{"from":{"hostname":"127.0.0.1","username":"root","db":"words","password":"123456"},"to":"sql","tables":"*","mode":"drop-create"}"#;
    // let data = r#"{"from":"sql","to":"sql","tables":"*","mode":"drop-create"}"#;
    serde_json::from_str(data).map_err(|e| ZzErrors::ParseConfigError(e))
}

async fn sql_to_db(sql: Box<SqlGroups>) {
    println!("{:?}", sql);
}

fn db_to_sql() {

}

async fn run_sync(config: SyncConfig) -> Result<(), ZzErrors> {
    if let DbConfig::Path(from_sql_path) = config.from {
        if let DbConfig::Path(_) = config.to {
            return Err(ZzErrors::IllegalConfig("不支持sql to sql的模式".to_string()))
        }
        read_file_as_sql_group(
            &from_sql_path,
            config.buffer_size,
            |sql|
               Box::pin(sql_to_db(sql))
        ).await?;
    } else if let DbConfig::ClientAddr(_addr) = config.from {
        let mut conn = Conn::new("mysql://root:123456@127.0.0.1:3306/zz_trans")
            .map_err(|e| {
                ZzErrors::ConnectError(format!("{:?}", e))
            })?;
        let a = conn.query::<u32, _>("select 1");
        println!("{:?}", a);
        db_to_sql();
        if let DbConfig::Path(to_dir) = config.to {
            let mut sync_started_schemas: Vec<String> = vec![];

            let sql_groups = vec![SqlGroup{
                schema: "test".to_string(),
                sqls: vec!["select 1;".to_string()]
            }];
            save_sql_to_dir(&mut sync_started_schemas, &sql_groups, &to_dir)?;
            save_sql_to_dir(&mut sync_started_schemas, &sql_groups, &to_dir)?;
        }
    }

    Ok(())
}

fn main() {
    let config = read_config().unwrap();
    println!("{:?}", config);
    let sync_res = block_on(run_sync(config));
    println!("{:?}", sync_res);
}
