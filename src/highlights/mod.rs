extern crate image;

mod cellular_automaton;
mod cut_highlights_from_image;
mod extract_highlights;
mod find_edges;
mod heat_map;
mod helpers;
mod point;
mod visual_object;

use rayon::ThreadPool;
use std::path::Path;
use std::sync::mpsc::Receiver;

use self::cellular_automaton::cellular_automaton;
use self::cut_highlights_from_image::cut_highlights_from_image;
use self::extract_highlights::extract_highlights;
use self::find_edges::find_edges;
use self::heat_map::heat_map;
use self::point::Point;
use self::visual_object::VisualObject;

/// Starts the worker by opening a channel mailbox. Messages from the web server
/// are going to be distributed to the thread pool to be processed.
pub fn listen(consumer: Receiver<Box<Path>>, pool: ThreadPool) {
    loop {
        match consumer.recv() {
            Ok(image) => pool.spawn(move || identify_objects(image)),
            Err(error) => println!("[Worker] {:?}", error),
        }
    }
}

pub fn identify_objects(path: Box<Path>) {
    println!("Identifying image at {:?}.", path);
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Provided image has no file stem.");
    let dir = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .expect("Malformed image path.");
    let image = image::open(Box::clone(&path)).expect("Could not open image");

    // Converts the image to grayscale and finds edges within the picture. Works
    // only with bright images. Resulting image has white background with dark
    // edges highlighted.
    let edge_detector = find_edges(&image);

    // From the bricked heat map creates more detailed one where each cell is half
    // of the size of those in the bricked heat map. This multi-dimensional vector
    // represents density of edges in the original image.
    // Also returns maximum heat observed in the map and an average heat. This is
    // used for calculating the rules of the cellular automaton.
    let (heat_map, heat_max, heat_mean) = heat_map(&edge_detector);

    // Stabilizes each cell into one of two states.
    let point_map = cellular_automaton(heat_map, heat_max, heat_mean);

    // Finds objects using a recursive flood fill method.
    let mut highlights: Vec<VisualObject> = Vec::new();
    extract_highlights(point_map, Point::new(0, 0), &mut highlights);

    println!(
        "Found {} highlights for image {}/{}.",
        highlights.len(),
        dir,
        file_stem
    );

    for (i, highlight) in cut_highlights_from_image(highlights, image)
        .iter()
        .enumerate()
    {
        let output_path = format!("/etc/harriet/output/{}/{}_{}.png", dir, file_stem, &i);
        highlight
            .save(output_path)
            .expect("Cannot persist highlight");
    }
}
