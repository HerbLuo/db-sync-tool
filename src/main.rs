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

use helper::{arguments, resp_error_code as ec};

#[tokio::main]
async fn main() {
    helper::log::init();
    arguments::init();

    ctrlc::set_handler(move || {
        std::process::exit(0); 
    }).unwrap();

    match ui::start_tray() {
        Ok(open_tray) => {
            tokio::spawn(rocket_server::start());
            open_tray();
        },
        Err(e) => {
            warn!("gui init failed, {:?}", e);
            rocket_server::start().await;
        },
    }
}
