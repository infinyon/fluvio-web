use leptos::prelude::*;

use fluvio_web::leptos_fluvio::ConsumerStreamSignal;

use crate::components::app::Count;

#[component]
pub fn Count(
    consumer: RwSignal<Option<ConsumerStreamSignal>>,
    set_state: WriteSignal<i32>,
) -> impl IntoView {
    move || {
        match consumer.get() {
        Some(count) => count.with(|count| match count {
            Some(Ok(consumer_record)) => {
                match serde_json::from_slice::<Count>(&consumer_record.record.value) {
                    Ok(count) => {
                        set_state.set(count.value);

                        view! { <CountOutput text=format!("Count: {}", count.value) /> }
                        .into_view()
                    }
                    Err(_e) => view! { <CountOutput text="Failed to parse count".to_string() /> }
                    .into_view(),
                }
            }
            _ => view! { <CountOutput text="No count returned from topic, try incrementing!".to_string() /> }
            .into_view(),
        }),
        None => view! { <CountOutput text="Fluvio consumer not yet established. Does the topic exist?".to_string() /> }
        .into_view(),
    }
    }
}

#[component]
fn CountOutput(text: String) -> impl IntoView {
    view! { <div style="margin-bottom: 15px; font-size: 24px;">{text}</div> }
}
