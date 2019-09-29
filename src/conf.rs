use std::env;

#[derive(Clone)]
pub struct ServerConf {
    input_path: String,
    output_path: String,
}

impl ServerConf {
    pub fn new() -> Self {
        let input_path = env::var("INPUT").expect("Env var INPUT is missing.");
        let output_path = env::var("OUTPUT").expect("Env var OUTPUT is missing.");

        Self { input_path, output_path }
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
}
