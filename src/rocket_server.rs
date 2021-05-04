use crate::{helper::{dirs::config_file_path, resp::{ZzJsonResult}, run_sync}, types::{ProjectConfig, SyncConfig, ZzErrors}};
use futures::executor::block_on;
use std::future::Future;
use rocket_contrib::{json::Json, serve::StaticFiles};
use std::fs;

fn to_resp<T>(res_fut: impl Future<Output=Result<T, ZzErrors>>) -> ZzJsonResult<T> {
    block_on(res_fut).map(|o| success!(o))
}

fn read_config() -> Result<SyncConfig, ZzErrors> {
    let data = r#"{
        "from":{"hostname":"127.0.0.1","username":"root","db":"zz_trans","password":"123456","port":3306},
        "to":{"hostname":"127.0.0.1","username":"root","db":"words","password":"123456","port":3306},
        "tables":"*",
        "mode":"drop-create"
    }"#;
    serde_json::from_str(data).map_err(|e| ZzErrors::ParseConfigError(e))
}

#[get("/settings")]
fn get_settings() -> ZzJsonResult<Vec<ProjectConfig>> {
    let config_file = config_file_path()?;
    let config_file_vec = fs::read(config_file)
        .map_err(|e| ZzErrors::IoError(format!("配置文件读取失败 {:?}", e)))?;
    let proj_conf  = serde_json::from_slice::<Vec<ProjectConfig>>(&config_file_vec)
        .map_err(|e| ZzErrors::IoError(format!("配置文件格式不对 {:?}", e)))?;
    return Ok(success!(proj_conf));
}

#[post("/settings", data = "<setting>")]
fn save_settings(setting: Json<Vec<ProjectConfig>>) -> ZzJsonResult<()> {
    let config_file = config_file_path()?;
    let json_str = serde_json::to_vec(&setting.into_inner())
        .map_err(|e| ZzErrors::IoError(format!("保存配置，序列化失败 {}", e)))?;
    fs::write(config_file, json_str)
        .map_err(|e| ZzErrors::IoError(format!("写入配置文件失败 {}", e)))?;
    return Ok(success!(()));
}

#[get("/sync")]
fn sync() -> ZzJsonResult<()> {
    let config = read_config().unwrap();
    to_resp(run_sync(config))
}

pub fn start() {
    rocket::ignite()
            .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/build")))
            .mount("/api", routes![
                sync,
                get_settings,
                save_settings,
            ])
            .launch();
}
