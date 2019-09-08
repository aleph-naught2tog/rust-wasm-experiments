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

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("no document on the window");
    let body = document.body().expect("no body on the document");

    let paragraph = document
        .create_element("p")
        .expect("error creating paragraph");

    paragraph.set_inner_html("<b>hello world</b>");

    body.append_child(&paragraph)
        .expect("error appending child");

    Ok(())
}
