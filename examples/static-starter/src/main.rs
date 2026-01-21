use lithe::{body, br, div, doctype, head, html, img, meta, p, render_to_string, style, title};
use std::fs;

fn main() {
    let page = (
        doctype(),
        html().lang("en").with_child((
            head()
                .with_child(meta().charset("UTF-8"))
                .with_child(meta().set_attribute("name", "viewport").set_attribute("content", "width=device-width, initial-scale=1.0"))
                .with_child(title().with_child("Lithe.rs Ergonomic Starter"))
                .with_child(style().with_child(r#"
                    body { font-family: sans-serif; display: flex; justify-content: center; padding: 50px; background: #fafafa; }
                    .container { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
                    .header { font-size: 24px; font-weight: bold; color: #f74c00; margin-bottom: 10px; }
                "#)),
            body().with_child(
                div().class("container")
                    .with_child(div().class("header").with_child("Lithe.rs Ergonomic Starter"))
                    .with_child(br())
                    .with_child(
                        div().class("content")
                            .with_child(p().with_child("This page was generated using the new ergonomic API!"))
                            .with_child(img().src("https://via.placeholder.com/150").alt("Placeholder"))
                    )
            )
        ))
    );

    let full_html = render_to_string(&page);

    fs::write("examples/static-starter/index.html", full_html).expect("Unable to write file");

    println!("Successfully generated index.html using the ergonomic API!");
}
