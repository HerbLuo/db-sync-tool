use crate::{helper::{resp::ZzJsonResult, run_sync}, types::{SyncConfig, ZzErrors}};
use futures::executor::block_on;
use std::future::Future;
use rocket::{fs::FileServer, serde::json::Json};
use crate::helper::config_store::{ConfigStore, FileConfigStore};

fn to_resp<T>(res_fut: impl Future<Output=Result<T, ZzErrors>>) -> ZzJsonResult<T> {
    block_on(res_fut).map(|o| success!(o))
}

lazy_static! {
    static ref CONFIG_STORE: FileConfigStore = FileConfigStore::new("zz-db-sync").unwrap();
}


#[get("/setting")]
fn get_settings() -> Result<Vec<u8>, ZzErrors> {
    return Ok(CONFIG_STORE.read()?);
}

#[post("/setting", data = "<setting>")]
fn save_settings(setting: Vec<u8>) -> ZzJsonResult<()> {
    return Ok(success!(CONFIG_STORE.write(setting)?));
}

#[post("/do-synchronization", data = "<sync_config>")]
fn do_synchronization(sync_config: Json<SyncConfig>) -> ZzJsonResult<()> {
    to_resp(run_sync(sync_config.0))
}

pub async fn start() {
    rocket::build() 
        .mount("/", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/build")))
        .mount("/api", routes![
            do_synchronization,
            get_settings,
            save_settings,
        ])
        .launch().await
        .unwrap()
}
