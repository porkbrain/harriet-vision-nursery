use conf::ServerConf;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
pub struct DirectoryToProcess {
    // Name of the directory on shared volume that contains images which should be processed.
    name: String,
}

#[post("/", format = "application/json", data = "<req>")]
pub fn find_highlights(
    conf: State<ServerConf>,
    producer: State<Mutex<Sender<Box<Path>>>>,
    req: Json<DirectoryToProcess>,
) -> Result<Status, Status> {
    let data_directory = &req.name;

    if !data_directory.chars().all(char::is_alphanumeric) {
        return Err(Status::UnprocessableEntity);
    }

    println!("hello {}", conf.input_path());

    let input_path: PathBuf = [conf.input_path(), data_directory].iter().collect();
    if !input_path.is_dir() {
        println!("Path {:?} is not a directory.", &input_path);
        return Err(Status::NotFound);
    }

    let output_path: PathBuf = [conf.output_path(), data_directory].iter().collect();
    if output_path.is_dir() {
        println!("Path {:?} already processed.", &output_path);
        return Err(Status::UnprocessableEntity);
    } else {
        fs::create_dir(output_path).map_err(|io_error| {
            println!("Cannot create output dir: {}.", io_error);
            Status::UnprocessableEntity
        })?;
    }

    // If the requested directory does not exist on file system, convert the error to
    // 404 http status.
    let items = fs::read_dir(input_path).map_err(|_| Status::NotFound)?;

    // Iterate through each result and consider files only.
    let items: Vec<Box<Path>> = items
        .into_iter()
        .filter_map(|result| result.ok().map(|item| item.path()))
        .filter(|item| item.is_file())
        .map(|file| file.into_boxed_path())
        .collect();

    // We acquire a mutex lock, clone the producer and immediately drop the lock.
    let producer = {
        producer
            .lock()
            .map(|original| original.clone())
            .map_err(|_| Status::InternalServerError)?
    };

    // We send each image path as one message. This helps the worker distribute
    // the workload into the threadpool.
    for item in items.into_iter() {
        producer
            .send(item)
            .map_err(|_| Status::ServiceUnavailable)?;
    }

    Ok(Status::Accepted)
}
