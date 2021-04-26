use crate::{ec, helper::{resp::JsonResult, run_sync}, types::{SyncConfig, ZzErrors}};
use futures::executor::block_on;
use std::future::Future;
use rocket_contrib::serve::StaticFiles;

fn to_resp<T>(res_fut: impl Future<Output=Result<T, ZzErrors>>) -> JsonResult<T> {
    block_on(res_fut).map(|o| success!(o)).map_err(|e| fail!(ec::ServerError, e))
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

#[get("/sync")]
fn sync() -> JsonResult<()> {
    let config = read_config().unwrap();
    to_resp(run_sync(config))
}

pub fn start() {
    rocket::ignite()
            .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/build")))
            .mount("/api", routes![
                sync,
            ])
            .launch();
}
