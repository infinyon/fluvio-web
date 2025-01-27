use std::time::Duration;

use fluvio::{
    consumer::{ConsumerConfigBuilder, ConsumerConfigExtBuilder},
    Offset, TopicProducerConfig, TopicProducerConfigBuilder,
};
use leptos::*;

use fluvio_web::{
    fluvio::FluvioBrowser,
    leptos_fluvio::{topic_consumer, topic_producer},
};

use crate::components::count::Count;
use crate::components::increment_button::IncrementButton;

#[component]
pub fn Counter(client: FluvioBrowser, topic: String) -> impl IntoView {
    let fluvio = client.inner_clone();

    let (state, set_state) = create_signal(0);

    let producer = match TopicProducerConfigBuilder::default()
        .linger(Duration::ZERO)
        .build()
    {
        Ok(producer_config) => topic_producer(fluvio.clone(), &topic, producer_config),
        Err(e) => {
            return view! { <div>
                <h1>{format!("Failed to create producer: {:?}", e)}</h1>
            </div> }
        }
    };

    let consumer = match ConsumerConfigExtBuilder::default()
        .topic(&topic)
        .offset_start(Offset::end())
        .build()
    {
        Ok(consumer_config) => topic_consumer(fluvio.clone(), consumer_config),
        Err(e) => {
            return view! { <div>
                <h1>{format!("Failed to create consumer: {:?}", e)}</h1>
            </div> }
        }
    };

    view! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            <h1>{format!("Counter for topic: {}", topic)}</h1>
            <Count consumer set_state />
            <IncrementButton producer state />
        </div>
    }
}
