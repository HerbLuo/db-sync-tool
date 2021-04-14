use crate::types::{SqlGroup, SqlGroups, SyncConfigTables, ZzErrors};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};
use chrono::{Local, Datelike, Timelike};
use std::pin::Pin;
use crate::db_conn::DBConn;
use mysql::Row;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn get_base_dir() -> Result<PathBuf, ZzErrors> {
    Path::new("./").canonicalize().map_err(|e| ZzErrors::GetBaseDirError(e))
}

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

pub async fn read_file_as_sql_group<F: Future>(
    path: &String,
    buffer_size: u32,
    cb: impl Fn(Box<SqlGroups>) -> Pin<Box<F>>
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
                &promises.push(cb(Box::new(sql_groups)));
                i = 0;
                sql_groups = vec![];
                sql_group = SqlGroup { schema: schema.to_string(), sqls: vec![] };
            }
        }
        sql_groups.push(sql_group);
    }

    promises.push(cb(Box::new(sql_groups)));
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

pub async fn db_to_sql_group<F: Future>(
    conn: &impl DBConn,
    tables_config: &SyncConfigTables,
    buffer_size: u32,
    cb: fn(Box<SqlGroups>) -> Pin<Box<F>>
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
                let rows = conn.query::<_, Row>(sql)?;

                row_size = row_size + rows.len();
                if row_size > (buffer_size as usize) {
                    &sql_groups.push(sql_group);
                    promises.push(cb(Box::new(sql_groups)));
                    row_size = 0;
                    sql_groups = vec![];
                    sql_group = SqlGroup { schema: schema.to_string(), sqls: vec![] };
                }
                if rows.len() < 1000  {
                    break;
                }
                page = page + 1;
            }
            sql_groups.push(sql_group);
        }
        promises.push(cb(Box::new(sql_groups)));
        Ok(())
    };
    
    if let SyncConfigTables::Any(_) = tables_config {
        read_tables(&conn.query::<_, String>(SQL_SELECT_ALL_TABLE)?)?
    } else if let SyncConfigTables::Tables(tables) = tables_config {
        read_tables(tables)?
    } else {
        panic!("未知的config.table");
    };

    futures::future::join_all(promises).await;

    Ok(())
}
