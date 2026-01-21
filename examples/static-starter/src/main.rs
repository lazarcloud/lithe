use lithe::{br, div, img, p, HtmlPage};
use std::fs;

fn main() {
    let content = div()
        .set_attribute("class", "container")
        .with_child(
            div()
                .set_attribute("class", "header")
                .with_child("Lithe.rs Utility Starter"),
        )
        .with_child(br())
        .with_child(
            div()
                .set_attribute("class", "content")
                .with_child(
                    p().with_child("This page was generated using the new HtmlPage utility!"),
                )
                .with_child(
                    img()
                        .set_attribute("src", "https://via.placeholder.com/150")
                        .set_attribute("alt", "Placeholder"),
                ),
        );

    let page = HtmlPage::new("Lithe.rs App", content)
        .add_style(r#"
            body { font-family: sans-serif; display: flex; justify-content: center; padding: 50px; background: #fafafa; }
            .container { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
            .header { font-size: 24px; font-weight: bold; color: #f74c00; margin-bottom: 10px; }
        "#);

    let full_html = page.render_to_string();

    fs::write("examples/static-starter/index.html", full_html).expect("Unable to write file");

    println!("Successfully generated index.html using HtmlPage utility!");
}
