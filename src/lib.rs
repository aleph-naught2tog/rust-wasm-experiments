use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement};

mod dom;
use dom::Dom;

macro_rules! console_log {
    ( $($token:tt)* ) => {
        {
            let js_string = JsValue::from_str(&format_args!($($token)*).to_string());

            web_sys::console::log_1(&js_string)
        }
    }
}

#[wasm_bindgen]
pub fn click(event: web_sys::MouseEvent) {
    web_sys::console::table_1(&event)
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

    let doc = Dom::find();
    let body = doc.get_body();

    let button_element = doc.button();
    button_element.set_id("click_button");
    button_element.set_inner_text("click!");
    body.append_child(&button_element)
        .expect("error appending child");

    let button_target: web_sys::EventTarget = button_element.into();

    let mut count = 0i32;

    let paragraph = doc.p();
    paragraph.set_id("click_display_element");
    body.append_child(&paragraph)
        .expect("error appending child");
    paragraph.set_inner_html(format!("count: {}", count).as_str());

    let callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        count += 1i32;
        paragraph.set_inner_html(format!("count: {}", count).as_str());
    }) as Box<dyn FnMut(_)>);

    button_target
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .expect("failed to add click handler");

    callback.forget();

    Ok(())
}
