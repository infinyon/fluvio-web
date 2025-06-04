use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};
use url::Url;

use fluvio_web::leptos_fluvio::connect_fluvio_client;

use crate::components::counter::Counter as CounterComponent;

// TODO get this from generated pkg code
#[derive(Serialize, Deserialize)]
pub(crate) struct Count {
    pub value: i32,
    pub label: String,
    pub description: String,
}

#[component]
pub fn App(fluvio_websocket_url: String, topic: String) -> impl IntoView {
    provide_meta_context();

    let fluvio_client = match Url::parse(&fluvio_websocket_url) {
        Ok(url) => connect_fluvio_client(url),
        Err(_) => {
            return view! { <h1>"Failed to get websocket url from dashboard"</h1> }.into_any()
        }
    };

    view! {
        <div style="width: 100%; margin-top: 30px; display: flex; align-items: center; justify-content: center;">
            {move || {
                match fluvio_client.get() {
                    Some(client) => {
                        view! { <CounterComponent client topic=topic.clone() /> }.into_any()
                    }
                    None => {
                        view! { <h1>"Fluvio connection not yet established :("</h1> }.into_any()
                    }
                }
            }}
        </div>
    }.into_any()
}
