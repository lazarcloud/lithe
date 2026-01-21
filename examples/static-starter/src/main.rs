use lithe::core::{br, div, h1, img, p, render_to_string};
use std::fs;

fn main() {
    let page = div()
        .set_attribute("class", "container")
        .with_child(
            div()
                .set_attribute("class", "header")
                .with_child("Lithe.rs Static Starter"),
        )
        .with_child(br())
        .with_child(
            div()
                .set_attribute("class", "content")
                .with_child(p().with_child("This page was generated using Lithe.rs"))
                .with_child(
                    img()
                        .set_attribute("src", "https://via.placeholder.com/150")
                        .set_attribute("alt", "Placeholder"),
                ),
        );

    let html = render_to_string(&page);

    let full_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lithe.rs App</title>
    <style>
        body {{ font-family: sans-serif; display: flex; justify-content: center; padding: 50px; background: #fafafa; }}
        .container {{ background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .header {{ font-size: 24px; font-weight: bold; color: #f74c00; margin-bottom: 10px; }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
        html
    );

    fs::write("index.html", full_html).expect("Unable to write file");

    println!("Successfully generated index.html!");
}
