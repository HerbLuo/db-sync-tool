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
use futures;

#[tokio::main]
async fn main() {
    helper::log::init();

    futures::executor::block_on(rocket_server::start());

    ui::start_tray();
}
