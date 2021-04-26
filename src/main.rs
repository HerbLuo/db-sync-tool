#![feature(decl_macro)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

#[macro_use]
mod helper;
mod db_conn;
mod types;
mod ui;
mod rocket_server;

use helper::resp_error_code as ec;

fn main() {
    helper::log::init();

    std::thread::spawn(move || {
        rocket_server::start();
    });
    ui::start_tray();
}
