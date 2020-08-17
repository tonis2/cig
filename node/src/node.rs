use std::collections::HashMap;

pub type EventType = Box<dyn Fn()>;

pub enum Events {
    OnClick(EventType),
    OnHover(EventType),
}

pub struct Node {
    pub tag: String,
    pub children: Vec<Node>,
    attributes: HashMap<String, String>,
    actions: Vec<Events>,
    dirty: bool,
}

impl Node {
    pub fn new<T: Into<String>>(tag: T, children: Vec<Node>) -> Self {
        Self {
            tag: tag.into(),
            children,
            attributes: HashMap::new(),
            actions: Vec::new(),
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

    pub fn set_action(&mut self, action: Events) {
        self.actions.push(action);
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Node {{ tag: {:?}, children: {:?}, attributes: {:?} }}",
            self.tag, self.children, self.attributes
        )
    }
}
