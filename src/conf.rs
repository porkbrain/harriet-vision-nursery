use std::env;

#[derive(Clone)]
pub struct ServerConf {
    input_path: String,
    output_path: String,
    worker_threads_n: usize,
}

impl ServerConf {
    pub fn new() -> Self {
        let input_path = env::var("INPUT").expect("Env var INPUT is missing.");
        let output_path = env::var("OUTPUT").expect("Env var OUTPUT is missing.");
        let worker_threads_n = env::var("WORKER_THREADS")
            .expect("Env var WORKER_THREADS is missing.")
            .parse::<usize>()
            .ok()
            .filter(|n| n > &0)
            .expect("Env var WORKER_THREADS must be >= 1 and <= 255.");

        Self { input_path, output_path, worker_threads_n }
    }

    /// Absolute path of the system directory to which the artifacts are stored.
    #[allow(dead_code)]
    pub fn output_path(&self) -> &str {
        &self.output_path
    }

    /// When a new request to process images comes, the directory refereed in
    /// the request has to live in this system directory.
    pub fn input_path(&self) -> &str {
        &self.input_path
    }

    /// Maximum number of threads the worker can spawn to delegate the image processing to.
    pub fn worker_threads(&self) -> usize {
        self.worker_threads_n
    }
}
