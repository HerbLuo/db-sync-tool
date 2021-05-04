use crate::types::{SqlGroup, SqlGroups, SyncConfig, SyncConfigTables, ZzErrors, DbConfig};
use crate::db_conn::{DBConn, mysql_conn::MysqlConn};
use crate::helper::dirs::get_base_dir;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};
use chrono::{Local, Datelike, Timelike};
use std::pin::Pin;
use mysql::{Row, Value, from_value, consts::ColumnType};
use std::sync::Mutex;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

// 备份并清空（或创建）文件
pub fn backup_file_and_clear_it(filepath: &Path) -> Result<(), ZzErrors> {
    if filepath.exists() {
        let time = Local::now();
        let bak_filepath = PathBuf::from(filepath).with_extension(
            format!(
                "{}-{}-{} {}:{}:{}.bak",
                time.year(), time.month(), time.day(),
                time.hour(), time.minute(), time.second()
            )
        );
        fs::copy(filepath, &bak_filepath).map_err(|e| {
            ZzErrors::IoError(format!("备份文件失败，err: {:?} from_path: {:?} to_path: {:?}", e, filepath, &bak_filepath))
        })?;

        OpenOptions::new().write(true).truncate(true).open(filepath).map_err(|e| {
            ZzErrors::IoError(format!("清空文件失败, err: {:?} path: {:?}", e, filepath))
        })?;
    }
    Ok(())
}

pub async fn read_file_as_sql_group<F: Future<Output = Result<(), ZzErrors>>>(
    path: &String,
    buffer_size: u32,
    cb: impl Fn(SqlGroups) -> Pin<Box<F>>
) -> Result<(), ZzErrors>{
    let from_filepath = get_base_dir()?.join(path);
    let dir = fs::read_dir(&from_filepath).map_err(|e| {
        ZzErrors::IoError(format!("读取源文件夹失败，err: {:?}, path: {:?}", e, &from_filepath))
    })?;
    let mut i = 0u32;
    let mut sql_groups = vec![];
    let mut promises = vec![];
    for file_enter_res in dir {
        let dir_entry = &file_enter_res.map_err(|e| {
            ZzErrors::IoError(format!("读取文件夹失败, err: {:?}, path: {:?}", e, &from_filepath))
        })?;
        if !dir_entry.file_type().map_err(|e| {
            ZzErrors::IoError(format!("获取文件类型失败, err: {:?}, path: {:?}", e, &from_filepath))
        })?.is_file() {
            continue;
        }
        let sql_path = dir_entry.path();
        if !&sql_path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_lowercase() == "sql").unwrap_or(false) {
            continue;
        }
        let sql_file = File::open(&sql_path).map_err(|e| {
            ZzErrors::IoError(format!("读取方式打开文件失败, err: {:?}, path: {:?}", e, &sql_path))
        })?;
        println!("sql文件读取中 {:?}", sql_path);
        let schema = sql_path.file_stem().and_then(|os| os.to_str()).ok_or_else(|| ZzErrors::IoError(format!("无法读取文件名{:?}", sql_path)))?;
        let mut sql_group = SqlGroup { schema: schema.to_string(), sqls: vec![] };
        for line_res in BufReader::new(sql_file).lines() {
            let line = line_res.map_err(|e| {
                ZzErrors::IoError(format!("按行读取文件失败, err: {:?}, path: {:?}", e, &sql_path))
            })?;
            &sql_group.sqls.push(line);
            i = i + 1;

            if i > buffer_size {
                &sql_groups.push(sql_group);
                &promises.push(cb(sql_groups));
                i = 0;
                sql_groups = vec![];
                sql_group = SqlGroup { schema: schema.to_string(), sqls: vec![] };
            }
        }
        sql_groups.push(sql_group);
    }

    promises.push(cb(sql_groups));
    futures::future::join_all(promises).await;
    Ok(())
}

// 保存SqlGroup到文件夹中
pub fn save_sql_to_dir(
    sync_started_schemas: &mut Vec<String>,
    sql_groups: &SqlGroups,
    to_dir: &String
) -> Result<(), ZzErrors> {
    // 目标文件夹
    let base_path = get_base_dir()?.join(to_dir);
    // 确保目标文件夹存在
    fs::create_dir_all(&base_path)
        .map_err(|e| ZzErrors::IoError(format!("创建目标文件夹失败, err: {:?}", e)))?;
    // 开始写文件
    for sql_group in sql_groups {
        let schema = &sql_group.schema;
        let to_sql_filepath = &base_path.join(format!("{}.sql", schema));
        // 第一次写入该schema时，备份并清空文件
        if !sync_started_schemas.contains(schema) {
            backup_file_and_clear_it(to_sql_filepath)?;
            sync_started_schemas.push(schema.clone());
        }
        // 追加模式写入sql文件
        OpenOptions::new().create(true).append(true).open(to_sql_filepath)
            .and_then(|mut file| {
                for sql in &sql_group.sqls {
                    file.write_all(sql.as_ref())?;
                    file.write_all(LINE_ENDING.as_ref())?;
                }
                Ok(())
            })
            .map_err(|e| {
                ZzErrors::IoError(format!("追加模式打开文件失败，err: {:?}, path: {:?}", e, to_sql_filepath))
            })?;
    }
    Ok(())
}

const SQL_SELECT_ALL_TABLE: &'static str = 
    "SELECT TABLE_NAME FROM information_schema.TABLES WHERE table_type = 'BASE TABLE' AND table_schema = DATABASE()";

pub async fn sql_to_db(conn: &impl DBConn, sql_groups: SqlGroups) -> Result<(), ZzErrors> {
    for sql_group in sql_groups {
        conn.exec(format!("TRUNCATE table `{}`", sql_group.schema))?;
        for sql in sql_group.sqls {
            conn.exec(sql)?;
        }
    }
    Ok(())
}

pub async fn db_to_sql_group<F: Future<Output = Result<(), ZzErrors>>>(
    source_db_conn: &impl DBConn,
    target_db_conn_opt: Option<&impl DBConn>,
    config: &SyncConfig,
    buffer_size: u32,
    cb: impl Fn(SqlGroups) -> Pin<Box<F>>
) -> Result<(), ZzErrors> {
    let mut promises = vec![];

    let mut read_tables = |tables: &Vec<String>| -> Result<(), ZzErrors> {
        let mut sql_groups = vec![];
        let mut row_size = 0usize;
        for schema in tables {
            let mut page = 0;
            let mut sql_group = SqlGroup { schema: schema.to_string(), sqls: vec![] };
            loop {
                let sql = format!("SELECT * FROM {} LIMIT {}, {}", schema, page * buffer_size, buffer_size);
                let rows = source_db_conn.query::<_, Row>(sql)?;
                let cur_rows_len = rows.len();

                for row in rows {
                    let mut sql = String::new();
                    sql.push_str("INSERT INTO ");
                    sql.push_str(schema);
                    sql.push('(');
                    let cols = row.columns_ref();
                    let last_index_of_cols = cols.len() - 1;
                    let mut column_types = vec![];
                    for (i, col) in cols.iter().enumerate() {
                        column_types.push(col.column_type());
                        sql.push('`');
                        sql.push_str(&col.name_str());
                        sql.push('`');
                        if i != last_index_of_cols {
                            sql.push_str(", ");
                        } 
                    }
                    sql.push_str(") VALUES (");
                    for (i, val) in row.unwrap().iter().enumerate() {
                        let column_type = column_types[i];
                        let value_str = if val == &Value::NULL {
                            String::from("NULL")
                        } else {
                            let val_owned = val.to_owned();
                            match column_type {
                                ColumnType::MYSQL_TYPE_LONGLONG => from_value::<i128>(val_owned).to_string(),
                                ColumnType::MYSQL_TYPE_TINY => from_value::<i16>(val_owned).to_string(),
                                ColumnType::MYSQL_TYPE_NEWDECIMAL => from_value::<String>(val_owned),
                                _ => val_owned.as_sql(false),
                            }
                        };
                        sql.push_str(&value_str);
                        if i != last_index_of_cols {
                            sql.push_str(", ");
                        } 
                    }
                    sql.push_str(");");
                    &sql_group.sqls.push(sql);
                }

                row_size = row_size + cur_rows_len;
                if row_size >= (buffer_size as usize) {
                    &sql_groups.push(sql_group);
                    promises.push(cb(sql_groups));
                    row_size = 0;
                    sql_groups = vec![];
                    sql_group = SqlGroup { schema: schema.to_string(), sqls: vec![] };
                }
                if cur_rows_len < 1000  {
                    break;
                }
                page = page + 1;
            }
            sql_groups.push(sql_group);
        }
        promises.push(cb(sql_groups));
        Ok(())
    };
    
    if let SyncConfigTables::Any(_) = &config.tables {
        let source_db_tables = source_db_conn.query::<_, String>(SQL_SELECT_ALL_TABLE)?;
        let tables = if !config.skip_sync_if_table_not_exist {
            source_db_tables
        } else if let Some(target_db_conn) = target_db_conn_opt {
            let target_db_tables = target_db_conn.query::<_, String>(SQL_SELECT_ALL_TABLE)?;
            let common_tables = source_db_tables.into_iter()
                .filter(|t| target_db_tables.contains(t))
                .collect::<Vec<_>>();
            common_tables    
        } else {
            source_db_tables
        };
        read_tables(&tables)?;
    } else if let SyncConfigTables::Tables(tables) = &config.tables {
        read_tables(tables)?
    } else {
        panic!("未知的config.table");
    };

    let res_vec = futures::future::join_all(promises).await;
    for res in res_vec {
        res?;
    }

    Ok(())
}

pub async fn run_sync(config: SyncConfig) -> Result<(), ZzErrors> {
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
    } else if let DbConfig::ClientAddr(from_db_addr) = &config.from {
        if let DbConfig::Path(to_dir) = &config.to {
            let sync_started_schemas: Mutex<Vec<String>> = Mutex::new(vec![]);
            db_to_sql_group(
                &MysqlConn::new(from_db_addr)?, 
                Option::<&MysqlConn>::None,
                &config, 
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
                &MysqlConn::new(from_db_addr)?, 
                Some(&to_db_conn),
                &config, 
                config.buffer_size, 
                |sql_group| Box::pin(sql_to_db(&to_db_conn, sql_group)),
            ).await?;
        };
    }

    Ok(())
}

