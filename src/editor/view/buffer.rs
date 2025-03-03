use std::fs;

use anyhow::Context;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(&mut self, file_name: &str) -> anyhow::Result<()> {
        let contents = fs::read_to_string(file_name).context("error reading file")?;

        let mut lines = Vec::new();

        for line in contents.lines() {
            lines.push(String::from(line));
        }

        self.lines = lines;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
