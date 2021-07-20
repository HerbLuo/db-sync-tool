#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod helper;
mod db_conn;
mod types;
mod ui;
mod rocket_server;

use helper::resp_error_code as ec;

#[tokio::main]
async fn main() {
    helper::log::init();

    match ui::start_tray() {
        Ok(call) => {
            tokio::spawn(rocket_server::start());
            call();
        },
        Err(e) => {
            warn!("gui init failed, {:?}", e);
            rocket_server::start().await;
        },
    }
}
