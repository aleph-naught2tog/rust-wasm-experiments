use wasm_bindgen::JsCast;

pub struct Dom {
    document: web_sys::Document,
}

impl Dom {
    pub fn find() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("no document on the window");

        Dom { document }
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<web_sys::HtmlElement> {
        let maybe_element: Option<web_sys::Element> = self.document.get_element_by_id(id);
        maybe_element.map_or(None, |element| {
            Some(element.dyn_into::<web_sys::HtmlElement>().unwrap_or_else(|_| {
                panic!("error converting to HtmlElement")
            }))
        })
    }

    pub fn p(&self) -> web_sys::HtmlParagraphElement {
        self.create_element_wrap("p")
            .dyn_into::<web_sys::HtmlParagraphElement>()
            .unwrap_or_else(|_| {
                panic!("error converting Element to HtmlParagraphElement'. This is probably a bug.")
            })
    }

    pub fn button(&self) -> web_sys::HtmlButtonElement {
        self.create_element_wrap("button")
            .dyn_into::<web_sys::HtmlButtonElement>()
            .unwrap_or_else(|_| {
                panic!("error converting Element to HtmlButtonElement'. This is probably a bug.")
            })
    }

    pub fn get_body(&self) -> web_sys::HtmlElement {
        self.document.body().expect("no body on the document")
    }

    fn create_element_wrap(&self, element_type: &str) -> web_sys::Element {
        self.document
            .create_element(element_type)
            .unwrap_or_else(|_| {
                let message = format!("error creating {}", element_type);
                panic!(message)
            })
    }
}
