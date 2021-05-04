use std::path::{Path, PathBuf};
use std::fs;

use crate::types::{ZzErrors, ProjectConfig};

pub fn work_dir() -> Result<PathBuf, ZzErrors> {
    let home_dir = dirs::home_dir()
        .ok_or(ZzErrors::IoError("找不到用户文件夹".to_string()))?;
    let work_dir = home_dir.join(".zz-db-sync");
    fs::create_dir_all(&work_dir)
        .map_err(|e| ZzErrors::IoError(format!("创建工作文件夹失败 {}", e)))?;
    return work_dir.canonicalize()
        .map_err(|e| ZzErrors::IoError(format!("找不到工作文件夹, {}", e)));
}

pub fn get_base_dir() -> Result<PathBuf, ZzErrors> {
    Path::new("./").canonicalize().map_err(|e| ZzErrors::GetBaseDirError(e))
}

pub fn config_file_path() -> Result<PathBuf, ZzErrors> {
    let setting_file = work_dir().map(|dir| dir.join("setting.json"))?;
    if !setting_file.exists() {
        let def_proj_conf = ProjectConfig {
            name: "默认".to_string(),
            def: Some(true),
            syncs: vec![],
        };
        let def_config_file_content = serde_json::to_vec(&vec![def_proj_conf])
            .map_err(|e| ZzErrors::IoError(format!("创建默认的配置文件, 序列化失败 {}", e)))?;
        fs::write(&setting_file, def_config_file_content)
            .map_err(|e| ZzErrors::IoError(format!("创建默认的配置文件失败 {}", e)))?;
        return Ok(setting_file);
    }
    return Ok(setting_file);
}
