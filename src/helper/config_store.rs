use crate::types::ZzErrors;
use std::{fs, path::PathBuf};

pub trait ConfigStore {
    fn new<S: AsRef<str>>(app_name: S) -> Result<Self, ZzErrors> where Self: Sized;
    fn write<C: AsRef<[u8]>>(&self, content: C) -> Result<(), ZzErrors>;
    fn read(&self) -> Result<Vec<u8>, ZzErrors>;
}

pub struct FileConfigStore {
    filepath: PathBuf,
}

const SETTING_FILENAME: &str = "configuration";

/**
 * 读写配置文件工具
 * 默认的文件位置 ~/.${app_name}
 */
impl ConfigStore for FileConfigStore {
    fn new<S: AsRef<str>>(app_name: S) -> Result<FileConfigStore, ZzErrors> {
        let home_dir = dirs::home_dir().ok_or(ZzErrors::IoError("找不到用户文件夹".to_string()))?;
        let work_dir = home_dir.join(format!(".{}", app_name.as_ref()));
        fs::create_dir_all(&work_dir).map_err(|e| ZzErrors::IoError(format!("创建工作文件夹失败 {}", e)))?;
        let filepath = work_dir.join(SETTING_FILENAME);
        log::info!("当前的配置文件路径为 {:?}", filepath);
        Ok(FileConfigStore {
            filepath,
        })
    }

    fn write<C: AsRef<[u8]>>(&self, content: C) -> Result<(), ZzErrors> {
        fs::write(&self.filepath, content).map_err(|e| ZzErrors::IoError(format!("创建默认的配置文件失败 {}", e)))
    }

    fn read(&self) -> Result<Vec<u8>, ZzErrors> {
        fs::read(&self.filepath).map_err(|e| ZzErrors::IoError(format!("配置文件读取失败 {:?}", e)))
    }
}
