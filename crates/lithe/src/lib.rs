pub mod core {
    pub use lithe_core::*;
}
pub use lithe_core::*;
pub use lithe_macros::{client, page};

pub fn render_page<C: Component + 'static>(comp: C, app_name: &str) -> String {
    let mut s = render_to_string(&comp);

    if s.contains("</head>") {
        let script = format!(
            r#"    <script type="module">
                window.Lithe = {{
                    dispatch: (name) => {{
                        if (!window.wasm_module) {{
                            console.warn('WASM module not initialized');
                            return;
                        }}
                        // Try exact match first
                        if (window.wasm_module[name]) {{
                            window.wasm_module[name]();
                            return;
                        }}
                        // Try to find a function ending with the name (for local functions)
                        const suffix = '_' + name;
                        for (const key of Object.keys(window.wasm_module)) {{
                            if (key.endsWith(suffix) || key === name) {{
                                window.wasm_module[key]();
                                return;
                            }}
                        }}
                        console.warn('WASM function not found:', name);
                    }}
                }};
                import init, * as exports from '/public/pkg/{app_name}.js';
                init().then(() => {{
                    window.wasm_module = exports;
                }});
            </script>"#,
            app_name = app_name
        );
        s = s.replace("</head>", &format!("{}\n</head>", script));
    }
    s
}
