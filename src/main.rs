#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
extern crate dotenv;
extern crate rocket_contrib;
#[macro_use] extern crate rocket;

mod conf;
mod routes;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let conf = conf::ServerConf::new();

    rocket::ignite()
        .mount("/highlights", routes![routes::find_highlights])
        .manage(conf)
        .launch();
}
