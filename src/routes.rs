use rocket::State;
use conf::ServerConf;
use serde::Deserialize;
use rocket::http::Status;
use std::{fs, path::Path};
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct DirectoryToProcess {
    // Name of the directory on shared volume that contains images which should be processed.
    name: String
}

#[post("/", format = "application/json", data = "<req>")]
pub fn find_highlights(conf: State<ServerConf>, req: Json<DirectoryToProcess>) -> Result<Status, Status> {
    let data_directory = &req.name;

    if !data_directory.chars().all(char::is_alphanumeric) {
        return Err(Status::UnprocessableEntity);
    }

    let base_path = conf.input_path();

    // Creates a new string and joins the absolute base path with the requested directory.
    let mut path = String::with_capacity(base_path.len() + 1 + data_directory.len());
    path.push_str(base_path);
    path.push('/');
    path.push_str(data_directory);
    let path = Path::new(&path);

    // If the requested directory does not exist on file system, convert the error to
    // 404 http status.
    let items = fs::read_dir(path).map_err(|_| Status::NotFound)?;

    // Iterate through each result and consider all files.
    let items: Vec<Box<Path>> = items.into_iter()
        .filter_map(|result| result.ok().map(|item| item.path()))
        .filter(|item| item.is_file())
        .map(|file| file.into_boxed_path())
        .collect();

    Ok(Status::Ok)
}
