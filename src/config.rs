use std::path::PathBuf;

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Config {
    config: String,
    path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reload(&mut self) -> Result<(), String> {
        unimplemented!();
    }

    pub fn path<P: Into<PathBuf>>(&mut self, path: P) {
        self.path = path.into();
    }
}
