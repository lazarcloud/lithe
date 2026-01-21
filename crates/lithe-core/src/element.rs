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

    pub fn class(self, name: &str) -> Self {
        self.set_attribute("class", name)
    }

    pub fn id(self, name: &str) -> Self {
        self.set_attribute("id", name)
    }

    pub fn lang(self, lang: &str) -> Self {
        self.set_attribute("lang", lang)
    }

    pub fn href(self, href: &str) -> Self {
        self.set_attribute("href", href)
    }

    pub fn src(self, src: &str) -> Self {
        self.set_attribute("src", src)
    }

    pub fn alt(self, alt: &str) -> Self {
        self.set_attribute("alt", alt)
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

    pub fn class(self, name: &str) -> Self {
        self.set_attribute("class", name)
    }

    pub fn id(self, name: &str) -> Self {
        self.set_attribute("id", name)
    }

    pub fn src(self, src: &str) -> Self {
        self.set_attribute("src", src)
    }

    pub fn alt(self, alt: &str) -> Self {
        self.set_attribute("alt", alt)
    }

    pub fn charset(self, charset: &str) -> Self {
        self.set_attribute("charset", charset)
    }

    pub fn href(self, href: &str) -> Self {
        self.set_attribute("href", href)
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
pub fn h1() -> Element {
    Element::new("h1")
}
pub fn h2() -> Element {
    Element::new("h2")
}
pub fn h3() -> Element {
    Element::new("h3")
}
pub fn h4() -> Element {
    Element::new("h4")
}
pub fn h5() -> Element {
    Element::new("h5")
}
pub fn h6() -> Element {
    Element::new("h6")
}
pub fn main_tag() -> Element {
    Element::new("main")
}
pub fn section() -> Element {
    Element::new("section")
}
pub fn header() -> Element {
    Element::new("header")
}
pub fn footer() -> Element {
    Element::new("footer")
}
pub fn nav() -> Element {
    Element::new("nav")
}
pub fn ul() -> Element {
    Element::new("ul")
}
pub fn li() -> Element {
    Element::new("li")
}
pub fn a() -> Element {
    Element::new("a")
}
pub fn html() -> Element {
    Element::new("html")
}
pub fn head() -> Element {
    Element::new("head")
}
pub fn body() -> Element {
    Element::new("body")
}
pub fn title() -> Element {
    Element::new("title")
}
pub fn style() -> Element {
    Element::new("style")
}
pub fn script() -> Element {
    Element::new("script")
}

pub fn br() -> VoidElement {
    VoidElement::new("br")
}
pub fn img() -> VoidElement {
    VoidElement::new("img")
}
pub fn meta() -> VoidElement {
    VoidElement::new("meta")
}
pub fn link() -> VoidElement {
    VoidElement::new("link")
}
