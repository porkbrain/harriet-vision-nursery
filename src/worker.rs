use std::path::Path;
use rayon::ThreadPool;
use std::sync::mpsc::Receiver;

/// Starts the worker by opening a channel mailbox. Messages from the web server
/// are going to be distributed to the thread pool to be processed.
pub fn listen(consumer: Receiver<Box<Path>>, pool: ThreadPool) {
    loop {
        match consumer.recv() {
            Ok(image) => pool.spawn(move || find_highlights(image)),
            Err(error) => println!("[Worker] {:?}", error),
        }
    }
}

fn find_highlights(path: Box<Path>) {
    println!("{:?}", path);
}
