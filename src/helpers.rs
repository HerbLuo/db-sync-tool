use crate::types::{SqlGroups, ZzErrors};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::{Local, Datelike, Timelike};

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

pub async fn read_file_as_sql_group<T:  Future>(
    path: &String,
    _buffer_size: u32,
    _cb: fn(SqlGroups) -> T
) -> Result<(), ZzErrors> {
    let from_filepath = get_base_dir()?.join(path);
    let dir = fs::read_dir(&from_filepath).map_err(|e| {
        ZzErrors::IoError(format!("读取源文件夹失败，err: {:?}, path: {:?}", e, &from_filepath))
    })?;
    for file_enter_res in dir {
        file_enter_res.map_err(|e| ZzErrors::IoError(format!("")))
    }
    println!("{:?}", dir);

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
