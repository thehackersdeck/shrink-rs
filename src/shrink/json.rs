#[derive(Debug)]
pub struct Writer<'a> {
    json: &'a [u8],
    cursor: usize,
}

impl<'a> Writer<'a> {
    fn new(json: &'a [u8]) -> Self {
        Self { json, cursor: 0 }
    }

    pub fn from_json(json_bytes: &[u8]) -> Vec<u8> {
        let writer = Writer::new(json_bytes);
        loop {
            break;
        }

        Vec::new()
    }

    pub fn eat_byte(&mut self) -> Option<u8> {
        let index = self.cursor;
        // Check if range is within bounds
        if index < self.json.len() {
            // Advance the cursor
            self.cursor += 1;
            return Some(self.json[index]);
        }
        None
    }

    pub fn eat_bytes(&mut self, range: usize) -> Option<&[u8]> {
        let start = self.cursor;
        let end = start + range;
        // Check if range is within bounds
        if end > self.json.len() {
            return None;
        }
        // Advance the cursor
        self.cursor = end;
        Some(&self.json[start..end])
    }
}
