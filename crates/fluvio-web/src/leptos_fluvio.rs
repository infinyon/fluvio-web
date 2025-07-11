use std::sync::Arc;

use leptos::prelude::*;
use leptos::task::spawn_local;
use url::Url;

use crate::fluvio::FluvioBrowser;

use fluvio::{
    consumer::ConsumerConfigExt,
    dataplane::{link::ErrorCode, record::ConsumerRecord},
    Fluvio, TopicProducerConfig, TopicProducerPool,
};

pub type ConsumerStreamSignal = ReadSignal<Option<Result<ConsumerRecord, ErrorCode>>>;

pub fn connect_fluvio_client(url: Url) -> RwSignal<Option<FluvioBrowser>> {
    let client_signal: RwSignal<Option<FluvioBrowser>> = RwSignal::new(None);

    spawn_local(async move {
        let fluvio = super::remote::connect(url).await;

        match fluvio {
            Ok(fluvio) => {
                leptos::logging::log!("successfully established fluvio client");

                client_signal.set(Some(fluvio.into()));
            }
            Err(e) => {
                leptos::logging::error!("Failed to establish Fluvio client: {:?}", e);
            }
        }
    });

    client_signal
}

pub fn topic_producer(
    fluvio: Arc<Fluvio>,
    topic: &str,
    producer_config: TopicProducerConfig,
) -> RwSignal<Option<Arc<TopicProducerPool>>> {
    let producer_signal: RwSignal<Option<Arc<TopicProducerPool>>> = RwSignal::new(None);
    let topic = topic.to_owned();

    spawn_local(async move {
        match fluvio
            .topic_producer_with_config(topic, producer_config)
            .await
        {
            Ok(producer) => producer_signal.set(Some(Arc::new(producer))),
            Err(e) => {
                leptos::logging::error!("Failed to create producer: {:?}", e);
            }
        }
    });

    producer_signal
}

pub fn topic_consumer(
    fluvio: Arc<Fluvio>,
    config: ConsumerConfigExt,
) -> RwSignal<Option<ConsumerStreamSignal>> {
    let consumer_signal: RwSignal<Option<ConsumerStreamSignal>> = RwSignal::new(None);

    spawn_local(async move {
        let stream = fluvio.consumer_with_config(config).await;

        match stream {
            Ok(consumer) => {
                let stream_signal = ReadSignal::from_stream_unsync(consumer);
                consumer_signal.set(Some(stream_signal));
            }
            Err(e) => {
                leptos::logging::error!("Failed to create consumer: {:?}", e);
            }
        }
    });

    consumer_signal
}
