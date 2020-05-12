#![feature(
proc_macro_hygiene,
decl_macro,
rustc_private,
type_ascription
)]
#[macro_use]
extern crate rocket;

extern crate ws;

use std::thread;

mod route;
use route::{get, static_files};

mod chat;
use chat::ws_rs;


fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        get::index,
        get::chat,
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}

fn main() {

    thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        .spawn(|| {
            ws_rs::websocket();
        })
        .unwrap();

    rocket().launch();
}