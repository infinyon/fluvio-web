mod components;

use components::app::App;
use custom_element::CustomElement;
use js_sys::Array;
use leptos::view;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

struct Counter;

// all custom element lifecycle hooks have a default
// no-op implementation
impl CustomElement for Counter {}

impl Counter {
    // called from the JavaScript custom element's `constructor`
    fn new(instance: JsValue, _args: Array) -> Self {
        let instance: HtmlElement = instance.into();

        let fluvio_url = match instance.get_attribute("fluvio-url") {
            Some(url) => url,
            None => {
                leptos::logging::log!("Failed to get fluvio-url attribute from web component");
                return Counter;
            }
        };

        let topic = match instance.get_attribute("topic") {
            Some(topic) => topic,
            None => {
                leptos::logging::log!("Failed to get topic attribute from web component");
                return Counter;
            }
        };

        leptos::mount_to(instance.clone(), || {
            view! {
                <App fluvio_websocket_url=fluvio_url topic />
            }
        });

        Counter
    }
}

fn main() {
    // create custom element constructor
    let component_name = "fluvio-counter";
    let (closure, constructor) = custom_element::create_custom_element(Counter::new, vec![]);

    // we want our custom element to live forever
    closure.forget();

    // define the element
    let window = web_sys::window().unwrap();

    window
        .custom_elements()
        .define(component_name, &constructor)
        .unwrap();
}
