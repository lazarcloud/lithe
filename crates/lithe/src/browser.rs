//! Browser API wrappers that work on WASM and are no-ops on the server.

use serde::{de::DeserializeOwned, Serialize};

/// Deterministic stable hashing for IDs
pub fn hash_id(path: &str) -> String {
    let mut hash: u64 = 5381;
    for c in path.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u64);
    }
    format!("f_{:x}", hash)
}

/// Shows an alert dialog with the given message.
#[inline]
pub fn alert(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.alert_with_message(message);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
    }
}

/// Logs a message to the browser console.
#[inline]
pub fn console_log(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&message.into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
    }
}

/// Calls a server function from the client.
pub async fn call_server<Args, Ret>(full_path: &str, args: Args) -> Ret
where
    Args: Serialize,
    Ret: DeserializeOwned + Default,
{
    #[cfg(target_arch = "wasm32")]
    {
        use crate::rpc::{RpcRequest, RpcResponse};
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let hashed_id = hash_id(full_path);
        web_sys::console::log_1(&format!("RPC Call: {} -> {}", full_path, hashed_id).into());

        let args_value = serde_json::to_value(args).unwrap();
        let rpc_req = RpcRequest {
            function: hashed_id,
            args: args_value,
        };
        let body = serde_json::to_string(&rpc_req).unwrap();

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);
        opts.set_body(&js_sys::JsString::from(body));

        let request = Request::new_with_str_and_init("/api/lithe-rpc", &opts).unwrap();
        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = match wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await {
            Ok(v) => v,
            Err(e) => {
                web_sys::console::error_1(&format!("Fetch error: {:?}", e).into());
                return Ret::default();
            }
        };
        
        let resp: Response = resp_value.dyn_into().unwrap();
        if !resp.ok() {
             web_sys::console::error_1(&format!("RPC failed with status: {}. Path: {}", resp.status(), full_path).into());
             return Ret::default();
        }

        let text_value = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap()).await.unwrap();
        let text = text_value.as_string().unwrap();
        
        let rpc_res: RpcResponse = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to parse RPC response: {:?}. Text: {}", e, text).into());
                return Ret::default();
            }
        };

        serde_json::from_value(rpc_res.result).unwrap_or_else(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize RPC result: {:?}", e).into());
            Ret::default()
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (full_path, args);
        panic!("call_server should only be called from WASM")
    }
}

#[inline]
pub fn location_href() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().and_then(|w| w.location().href().ok())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

#[inline]
pub fn navigate(url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(url);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = url;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
}

#[inline]
pub fn set_inner_html(id: &str, html: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(el) = get_element_by_id(id) {
            let _ = el.set_inner_html(html);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (id, html);
    }
}

#[inline]
pub fn get_inner_html(id: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        get_element_by_id(id).map(|el| el.inner_html())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = id;
        None
    }
}
