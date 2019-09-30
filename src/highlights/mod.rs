extern crate image;

mod point;
mod helpers;
mod heat_map;
mod find_edges;
mod visual_object;
mod cellular_automaton;
mod extract_highlights;
mod cut_highlights_from_image;

use std::path::Path;
use rayon::ThreadPool;
use std::sync::mpsc::Receiver;

use self::point::Point;
use self::heat_map::heat_map;
use self::find_edges::find_edges;
use self::visual_object::VisualObject;
use self::cellular_automaton::cellular_automaton;
use self::extract_highlights::extract_highlights;
use self::cut_highlights_from_image::cut_highlights_from_image;


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
  let image = image::open(path).expect("Could not open image.");

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
  extract_highlights(
    point_map,
    Point::new(0, 0),
    &mut highlights,
  );

  for (i, highlight) in cut_highlights_from_image(highlights, image).iter().enumerate() {
    highlight.save("/etc/harriet/output/".to_owned() + &i.to_string() + ".png").unwrap();
  }
}
