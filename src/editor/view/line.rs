use std::ops::Range;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let left = range.start;
        let end = std::cmp::min(range.end, self.string.len());

        self.string.get(left..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}
