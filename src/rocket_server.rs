use crate::{helper::{resp::{ZzJsonResult}, run_sync}, types::{SyncConfig, ZzErrors}};
use futures::{executor::block_on};
use std::{future::Future, lazy::SyncOnceCell};
use rocket_contrib::{json::Json, serve::StaticFiles};
use crate::helper::config_store::{ConfigStore, FileConfigStore};

fn to_resp<T>(res_fut: impl Future<Output=Result<T, ZzErrors>>) -> ZzJsonResult<T> {
    block_on(res_fut).map(|o| success!(o))
}

static CONFIG_STORE: SyncOnceCell<FileConfigStore> = SyncOnceCell::new();

#[get("/setting")]
fn get_settings() -> Result<Vec<u8>, ZzErrors> {
    return Ok(CONFIG_STORE.get().unwrap().read()?);
}

#[post("/setting", data = "<setting>")]
fn save_settings(setting: Vec<u8>) -> ZzJsonResult<()> {
    return Ok(success!(CONFIG_STORE.get().unwrap().write(setting)?));
}

#[post("/do-synchronization", data = "<sync_config>")]
fn do_synchronization(sync_config: Json<SyncConfig>) -> ZzJsonResult<()> {
    to_resp(run_sync(sync_config.0))
}

pub fn start() {
    CONFIG_STORE.get_or_init(|| {
        FileConfigStore::new("zz-db-sync").unwrap()
    });

    rocket::ignite()
            .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/build")))
            .mount("/api", routes![
                do_synchronization,
                get_settings,
                save_settings,
            ])
            .launch();
}
