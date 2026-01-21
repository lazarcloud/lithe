use crate::{Component, br, div, img, p, render_to_string, span};

#[test]
fn test_basic_element_rendering() {
    let el = div();
    assert_eq!(render_to_string(&el), "<div></div>");
}

#[test]
fn test_void_element_rendering() {
    let el = br();
    assert_eq!(render_to_string(&el), "<br />");
}

#[test]
fn test_attributes_rendering() {
    let el = img()
        .set_attribute("src", "logo.png")
        .set_attribute("alt", "Lithe Logo");

    assert_eq!(
        render_to_string(&el),
        "<img src=\"logo.png\" alt=\"Lithe Logo\" />"
    );
}

#[test]
fn test_nested_elements_fluent() {
    let root = div().set_attribute("class", "container").with_child(
        p().with_child("Hello, ").with_child(
            span()
                .set_attribute("style", "color: red")
                .with_child("Lithe"),
        ),
    );

    let expected =
        "<div class=\"container\"><p>Hello, <span style=\"color: red\">Lithe</span></p></div>";
    assert_eq!(render_to_string(&root), expected);
}

#[test]
fn test_append_pattern_in_loop() {
    let mut list = div().set_attribute("id", "list");

    for i in 1..=3 {
        let mut item = span();
        item.child(format!("Item {}", i));
        list.child(item);
    }

    let expected =
        "<div id=\"list\"><span>Item 1</span><span>Item 2</span><span>Item 3</span></div>";
    assert_eq!(render_to_string(&list), expected);
}

#[test]
fn test_complex_tree() {
    let mut root = div().set_attribute("id", "app");

    let mut header = div().set_attribute("class", "header");
    header.child(span().with_child("Logo"));

    let mut body = div().set_attribute("class", "body");
    body.child(p().with_child("Welcome to the monorepo."));
    body.child(br());
    body.child(img().set_attribute("src", "avatar.jpg"));

    root.child(header);
    root.child(body);

    let expected = "<div id=\"app\"><div class=\"header\"><span>Logo</span></div><div class=\"body\"><p>Welcome to the monorepo.</p><br /><img src=\"avatar.jpg\" /></div></div>";
    assert_eq!(render_to_string(&root), expected);
}
