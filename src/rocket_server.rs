use crate::{
    helper::{
        resp::ZzJsonResult, 
        run_sync, 
        arguments::MATCHES, 
        config_store::{ConfigStore, FileConfigStore}
    }, 
    types::{SyncConfig, ZzErrors}
};
use rocket::{fs::FileServer, serde::json::Json};

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
async fn do_synchronization(sync_config: Json<SyncConfig>) -> ZzJsonResult<()> {
    return run_sync(sync_config.0).await.map(|o| success!(o))
}

pub async fn start() {
    let (port, address) = MATCHES.with(|matches| {
        (
            matches.value_of("PORT").unwrap().to_string().parse::<usize>().unwrap(),
            matches.value_of("ADDRESS").unwrap().to_string(),
        )
    });
    
    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", address));

    rocket::custom(figment)
        .mount("/", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/build")))
        .mount("/api", routes![
            do_synchronization,
            get_settings,
            save_settings,
        ])
        .launch().await
        .unwrap()
}
