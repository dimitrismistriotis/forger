#[derive(Debug)]
pub struct ApplicationContext {
    target: String,
    verbose: bool,
}

impl ApplicationContext {
    pub fn new(target: String, verbose: bool) -> Self {
        Self { target, verbose }
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
