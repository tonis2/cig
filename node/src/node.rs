use std::collections::HashMap;
#[derive(Debug)]
pub struct Node {
    pub tag: String,
    pub children: Vec<Node>,
    attributes: HashMap<String, String>,
    dirty: bool,
}

impl Node {
    pub fn new(tag: &str, children: Vec<Node>, attributes: HashMap<String, String>) -> Self {
        Self {
            tag: tag.into(),
            children,
            attributes,
            dirty: false,
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn set_attribute<T: Into<String>>(&mut self, key: T, value: T) {
        self.attributes.entry(key.into()).or_insert(value.into());
        self.dirty = true;
    }
}
