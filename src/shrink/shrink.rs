use super::json;

#[derive(Debug)]
pub struct Shrink {
    pub(crate) data: Vec<u8>,
}

impl Shrink {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn from_json(json: String) -> Self {
        let json_bytes = json.into_bytes();
        Self {
            data: json::Writer::from_json(&json_bytes),
        }
    }
}
