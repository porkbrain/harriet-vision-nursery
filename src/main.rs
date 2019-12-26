#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;
extern crate rayon;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate rocket;

mod conf;
mod highlights;
mod routes;

use dotenv::dotenv;
use rayon::ThreadPoolBuilder;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread;

fn main() {
    dotenv().ok();
    let conf = conf::ServerConf::new();

    // Creates a channel between the worker and the web server.
    let (producer, consumer) = channel::<Box<Path>>();

    // Prepares new thread pool for the worker to delegate jobs to.
    let pool = ThreadPoolBuilder::new()
        .num_threads(conf.worker_threads())
        .build()
        .expect("Couldn't build worker threadpool");

    thread::spawn(move || highlights::listen(consumer, pool));

    rocket::ignite()
        .mount("/highlights", routes![routes::find_highlights])
        .manage(conf)
        .manage(Mutex::new(producer))
        .launch();
}
