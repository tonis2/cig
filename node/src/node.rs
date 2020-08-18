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
    events: Vec<Events>,
    dirty: bool,
}

impl Node {
    pub fn new<T: Into<String>>(tag: T) -> Self {
        Self {
            tag: tag.into(),
            children: Vec::new(),
            attributes: HashMap::new(),
            events: Vec::new(),
            dirty: false,
        }
    }

    pub fn append(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn set_attribute<T: Into<String>>(&mut self, key: T, value: T) {
        self.attributes.entry(key.into()).or_insert(value.into());
        self.dirty = true;
    }

    pub fn set_event(&mut self, action: Events) {
        self.events.push(action);
    }

    pub fn events(&self) -> &Vec<Events> {
        &self.events
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
