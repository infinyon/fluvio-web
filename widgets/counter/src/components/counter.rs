use std::time::Duration;

use leptos::prelude::*;

use fluvio::{consumer::ConsumerConfigExtBuilder, Offset, TopicProducerConfigBuilder};
use fluvio_web::fluvio::FluvioBrowser;
use fluvio_web::leptos_fluvio::{topic_consumer, topic_producer};

use crate::components::count::Count;
use crate::components::increment_button::IncrementButton;

#[component]
pub fn Counter(client: FluvioBrowser, topic: String) -> impl IntoView {
    let fluvio = client.inner_clone();
    let (state, set_state) = signal(0);
    let producer = match TopicProducerConfigBuilder::default()
        .linger(Duration::ZERO)
        .build()
    {
        Ok(producer_config) => topic_producer(fluvio.clone(), &topic, producer_config),
        Err(e) => {
            return view! {
                <div>
                    <h1>{format!("Failed to create producer: {e:?}")}</h1>
                </div>
            }
            .into_any();
        }
    };

    let Ok(consumer_config) = ConsumerConfigExtBuilder::default()
        .topic(&topic)
        .offset_start(Offset::end())
        .build()
    else {
        return view! {
            <div>
                <h1>{"Failed to create consumer config.".to_string()}</h1>
            </div>
        }
        .into_any();
    };

    let consumer = topic_consumer(fluvio, consumer_config);

    view! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            <h1>"Counter for topic: "{topic}</h1>
            <Count consumer set_state />
            <IncrementButton producer state />
        </div>
    }
    .into_any()
}
