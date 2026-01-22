use lithe::{Component, HtmlPage, browser, button, div, p, page, server};

#[server]
pub async fn get_data_step_1(val: i32) -> i32 {
    println!("Step 1 received value: {}", val);
    let memory_info = sys_info::mem_info().unwrap();
    println!("System memory info: {:?}", memory_info);
    memory_info.total as i32
}

#[server]
pub async fn get_data_final(val: i32) -> String {
    let res = get_data_step_1(val).await;
    format!("Final server result: {}", res)
}

#[page]
pub fn page() -> impl Component {
    let body = div()
        .class("container")
        .with_child(p().with_child("Testing server-to-server calls:"))
        .with_child(
            button()
                .on_click(|| async move {
                    let res = get_data_final(5).await;
                    browser::alert(&res);
                })
                .with_child("Call Multi-step Server Fn"),
        );
    HtmlPage::new("Lithe.rs - Test", body)
}
