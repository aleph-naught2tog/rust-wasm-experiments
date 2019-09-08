use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ( $($token:tt)* ) => {
        {
            let js_string = JsValue::from_str(&format_args!($($token)*).to_string());

            web_sys::console::log_1(&js_string)
        }
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log!("Hello, world!");
    console_log!("the magic number is: {}", 42);

    // error[E0425]: cannot find function `window` in module `web_sys`
    let window = web_sys::window().expect("no global `window` exists");

    Ok(())
}
