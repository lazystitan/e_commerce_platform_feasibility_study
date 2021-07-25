#[derive(Debug, Hash)]
pub struct Attribute(u32, String);

impl Attribute {
    fn name(&self) -> &str {
        &self.1
    }
}

pub type Attr = Attribute;