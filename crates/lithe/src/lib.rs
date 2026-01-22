pub mod core {
    pub use lithe_core::*;
}
pub mod browser;

pub use lithe_core::*;
pub use lithe_macros::{client, page, server};

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_futures;

pub fn render_page<C: Component + 'static>(comp: C, app_name: &str, id_map_json: &str) -> String {
    let mut s = render_to_string(&comp);

    if s.contains("</head>") {
        let script = format!(
            r#"    <script type="module">
                window.Lithe = {{
                    id_map: {id_map_json},
                    dispatch: (name) => {{
                        if (!window.wasm_module) {{
                            console.warn('WASM module not initialized');
                            return;
                        }}
                        const mapped = window.Lithe.id_map[name] || name;
                        if (window.wasm_module[mapped]) {{
                            window.wasm_module[mapped]();
                            return;
                        }}
                        // Fallback to suffix matching for backward compatibility or direct calls
                        const suffix = '_' + name;
                        for (const key of Object.keys(window.wasm_module)) {{
                            if (key.endsWith(suffix) || key === mapped) {{
                                window.wasm_module[key]();
                                return;
                            }}
                        }}
                        console.warn('WASM function not found:', name, ' (mapped to ' + mapped + ')');
                    }}
                }};
                import init, * as exports from '/public/pkg/{app_name}.js';
                init().then(() => {{
                    window.wasm_module = exports;
                }});
            </script>"#,
            id_map_json = id_map_json,
            app_name = app_name
        );
        s = s.replace("</head>", &format!("{}\n</head>", script));
    }
    s
}
