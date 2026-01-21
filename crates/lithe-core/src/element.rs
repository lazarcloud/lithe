use crate::Component;

struct TagHead {
    tag: String,
    attributes: Vec<(String, String)>,
}

impl TagHead {
    fn new(tag: &str) -> Self {
        TagHead {
            tag: tag.to_string(),
            attributes: Vec::new(),
        }
    }

    fn push_attribute(&mut self, key: &str, value: &str) {
        self.attributes.push((key.to_string(), value.to_string()));
    }

    fn render_open(&self, buf: &mut String) {
        buf.push('<');
        buf.push_str(&self.tag);
        for (key, value) in &self.attributes {
            buf.push(' ');
            buf.push_str(key);
            buf.push_str("=\"");
            buf.push_str(value);
            buf.push('"');
        }
        buf.push('>');
    }

    fn render_self_closing(&self, buf: &mut String) {
        buf.push('<');
        buf.push_str(&self.tag);
        for (key, value) in &self.attributes {
            buf.push(' ');
            buf.push_str(key);
            buf.push_str("=\"");
            buf.push_str(value);
            buf.push('"');
        }
        buf.push_str(" />");
    }

    fn render_close(&self, buf: &mut String) {
        buf.push_str("</");
        buf.push_str(&self.tag);
        buf.push('>');
    }
}

pub struct Element {
    head: TagHead,
    children: Vec<Box<dyn Component>>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Element {
            head: TagHead::new(tag),
            children: Vec::new(),
        }
    }

    pub fn set_attribute(mut self, key: &str, value: &str) -> Self {
        self.head.push_attribute(key, value);
        self
    }

    pub fn child<C: Component + 'static>(&mut self, child: C) {
        self.children.push(Box::new(child));
    }

    pub fn with_child<C: Component + 'static>(mut self, child: C) -> Self {
        self.child(child);
        self
    }
}

impl Component for Element {
    fn render(&self, buf: &mut String) {
        self.head.render_open(buf);
        for child in &self.children {
            child.render(buf);
        }
        self.head.render_close(buf);
    }
}

pub struct VoidElement {
    head: TagHead,
}

impl VoidElement {
    pub fn new(tag: &str) -> Self {
        VoidElement {
            head: TagHead::new(tag),
        }
    }

    pub fn set_attribute(mut self, key: &str, value: &str) -> Self {
        self.head.push_attribute(key, value);
        self
    }
}

impl Component for VoidElement {
    fn render(&self, buf: &mut String) {
        self.head.render_self_closing(buf);
    }
}

pub fn div() -> Element {
    Element::new("div")
}
pub fn span() -> Element {
    Element::new("span")
}
pub fn p() -> Element {
    Element::new("p")
}
pub fn a() -> Element {
    Element::new("a")
}

pub fn br() -> VoidElement {
    VoidElement::new("br")
}
pub fn img() -> VoidElement {
    VoidElement::new("img")
}
