pub struct DpkgOptions {
    root_dir: Option<String>,
}

impl DpkgOptions {
    pub fn build(&self) -> String {
        let mut arguments: String = "".to_string();
        arguments = match &self.root_dir {
            Some(root_dir) => format!("{} --root={}", arguments, root_dir),
            _ => arguments,
        };
        arguments
    }
}

impl DpkgOptions {
    pub fn new() -> DpkgOptions {
        DpkgOptions{ root_dir: None }
    }

    pub fn set_root_dir(mut self, root_dir: String) -> Self {
        self.root_dir = Some(root_dir);
        self
    }

    pub fn get_root_dir(&self) -> Option<String> {
        self.root_dir.clone()
    }
}