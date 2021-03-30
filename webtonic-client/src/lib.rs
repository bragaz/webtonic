//! Client crate of the [`WebTonic`](https://github.com/Sawchord/webtonic) project.
//!
//! This crate only contains the [`Client`](Client), which requires a browser runtime
//! to function.

use web_sys::console;
use wasm_bindgen::JsValue;

mod websocket;
pub mod client;

pub(crate) fn console_log(s: &str) {
    console::log_1(&JsValue::from_str(s));
}