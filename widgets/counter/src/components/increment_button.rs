use std::rc::Rc;

use leptos::*;

use fluvio::{RecordKey, TopicProducerPool};

use crate::components::app::Count;

#[component]
pub fn IncrementButton(
    producer: RwSignal<Option<Rc<TopicProducerPool>>>,
    state: ReadSignal<i32>,
) -> impl IntoView {
    view! {
        <button
            style="padding-left: 20px; padding-right: 20px; font-size: 1.5em;
            padding-top: 5px; padding-bottom: 5px; border-radius: 6px; cursor: pointer;"
            type="button"
            on:click=move |_| {
                let new_record = Count {
                    value: state.get() + 1,
                    label: String::new(),
                    description: String::new(),
                };
                let json_string: String = match serde_json::to_string(&new_record) {
                    Ok(json_string) => json_string,
                    Err(e) => {
                        leptos::logging::error!("Failed to serialize record: {:?}", e);
                        return;
                    }
                };
                spawn_local(async move {
                    match producer.get_untracked() {
                        Some(producer) => {
                            if let Err(e) = producer.send(RecordKey::NULL, json_string).await {
                                leptos::logging::error!("Failed to send record: {:?}", e);
                            }
                        }
                        None => {
                            leptos::logging::log!("Producer not yet established");
                        }
                    }
                });
            }
        >
            "increment"
        </button>
    }
}
