#[derive(Debug, Default, Clone)]
pub struct Markdown {
    string: String,
}

impl Markdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, s: impl AsRef<str>) {
        self.string.push('\n');
        self.string.push('\n');
        self.string.push_str(s.as_ref());
    }

    pub fn finish(self) -> String {
        let mut string = self.string.trim().to_owned();
        string.push('\n');
        string
    }
}
