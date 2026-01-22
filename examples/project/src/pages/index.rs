use lithe::{Component, HtmlPage, a, button, client, div, h1, p, page};

#[client]
pub fn handle_click() {
    #[cfg(target_arch = "wasm32")]
    web_sys::window()
        .unwrap()
        .alert_with_message("Hello from Rust WASM!")
        .unwrap();
}

#[page]
pub fn page() -> impl Component {
    let body = div()
        .class("container")
        .with_child(h1().with_child("Welcome to Lithe.rs"))
        .with_child(
            p().with_child("This button is mapped directly to a Rust function in this file."),
        )
        .with_child(
            button()
                .on_click(handle_click)
                .with_child("Click Me (WASM)"),
        )
        .with_child(
            div().with_child(
                a().set_attribute("href", "/about")
                    .with_child("Go to About"),
            ),
        );
    HtmlPage::new("Lithe.rs - Home", body)
}
