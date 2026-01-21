use crate::{Component, render_to_string};

struct DummyComponent;

impl Component for DummyComponent {
    fn render(&self, buf: &mut String) {
        buf.push_str("<div>Hello, World!</div>");
    }
}

#[test]
fn test_render_to_string() {
    let component = DummyComponent;
    let rendered = render_to_string(&component);
    assert_eq!(rendered, "<div>Hello, World!</div>");
}
