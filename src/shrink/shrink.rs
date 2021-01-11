#[derive(Debug)]
pub struct Shrink<'a> {
    data: &'a[u8],
}

impl <'a> Shrink <'a> {
    pub fn new(data: &'a[u8]) -> Self {
        Self {
            data
        }
    }
}

impl <'a> From<String> for Shrink <'a> {
    fn from(_json: String) -> Self {
       // Convert json string to Shrink.
       Self {
           data: &[]
       }
    }
}
