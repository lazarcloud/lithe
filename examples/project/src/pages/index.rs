use lithe::{Component, Element, div, h1, p};

pub fn page() -> impl Component {
    div()
        .class("container")
        .with_child(h1().with_child("Welcome to Lithe.rs"))
        .with_child(p().with_child("This page was generated via FS routing."))
}
