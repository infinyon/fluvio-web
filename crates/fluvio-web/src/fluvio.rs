use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

use anyhow::{anyhow, Result};
use url::Url;

pub use fluvio::metadata::objects::ListRequest;
pub use fluvio::metadata::topic::TopicSpec;
pub use fluvio::{Fluvio as NativeFluvio, FluvioAdmin, FluvioConfig};

#[cfg(target_arch = "wasm32")]
use crate::net::FluvioWebsocketConnector;

#[cfg(target_arch = "wasm32")]
thread_local!(pub static APP_SERVICES: RefCell<AppServices> = RefCell::new(AppServices::default()));

#[derive(Serialize, Deserialize)]
struct Query {
    token: Option<String>,
}

#[derive(Debug, Default)]
#[cfg(target_arch = "wasm32")]
pub(crate) struct AppServices {
    pub(crate) fluvio: Option<FluvioBrowser>,
}

#[cfg(target_arch = "wasm32")]
impl AppServices {
    /// Creates a [`Fluvio`] (Fluvio Client) instance and stores it in the [`AppServices`].
    pub(crate) async fn reconnect_fluvio_client(addr: Url) -> Result<FluvioBrowser> {
        let config = FluvioConfig::new(addr.to_owned());

        let token = if let Some(query) = addr.query() {
            let parsed_query = match serde_qs::from_str::<Query>(query) {
                Ok(q) => q,
                Err(e) => return Err(anyhow::anyhow!("Failed to parse query: {}", e)),
            };

            parsed_query.token
        } else {
            #[cfg(feature = "leptos")]
            leptos::logging::log!("No query in fluvio conn string");
            None
        };

        let fluvio = match token {
            Some(token) => FluvioBrowser::connect_with_token(addr, &config, token).await?,
            None => FluvioBrowser::connect(addr, &config).await?,
        };

        APP_SERVICES.with(|services| {
            services.borrow_mut().fluvio = Some(fluvio.clone());
        });

        Ok(fluvio)
    }
}

#[derive(Clone)]
pub struct FluvioBrowser {
    pub inner: Arc<NativeFluvio>,
    cluster_name: String,
}

impl PartialEq for FluvioBrowser {
    fn eq(&self, other: &Self) -> bool {
        self.cluster_name == other.cluster_name
    }
}

impl Eq for FluvioBrowser {}

impl FluvioBrowser {
    #[cfg(target_arch = "wasm32")]
    pub async fn connect(addr: Url, config: &FluvioConfig) -> Result<Self> {
        let connector = FluvioWebsocketConnector::new(addr.to_string(), None, None);

        let inner =
            Arc::new(NativeFluvio::connect_with_connector(Box::new(connector), config).await?);

        Ok(Self {
            inner,
            cluster_name: addr.host_str().unwrap().to_string(),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect(_addr: Url, _config: &FluvioConfig) -> Result<Self> {
        panic!(
            "Browser fluvio client only supported for wasm targets. Uso fluvio-ws crate instead."
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn connect_with_token(
        addr: Url,
        config: &FluvioConfig,
        token: String,
    ) -> Result<Self> {
        let connector = FluvioWebsocketConnector::new(addr.to_string(), Some(token), None);

        let inner =
            Arc::new(NativeFluvio::connect_with_connector(Box::new(connector), config).await?);

        Ok(Self {
            inner,
            cluster_name: addr.host_str().unwrap().to_string(),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect_with_token(
        _addr: Url,
        _config: &FluvioConfig,
        _token: String,
    ) -> Result<Self> {
        panic!(
            "Browser fluvio client only supported for wasm targets. Uso fluvio-ws crate instead."
        )
    }

    pub async fn topics(&self) -> Result<Vec<String>> {
        let admin = self.inner.admin().await;

        let topics = admin
            .list_with_config::<TopicSpec, String>(ListRequest::default())
            .await;

        match topics {
            Ok(topics) => Ok(topics
                .iter()
                .map(|meta| meta.name.clone())
                .collect::<Vec<String>>()),
            Err(e) => Err(anyhow!("failed to get topics: {:?}", e)),
        }
    }

    pub fn inner_clone(&self) -> Arc<NativeFluvio> {
        Arc::clone(&self.inner)
    }
}

impl From<Arc<NativeFluvio>> for FluvioBrowser {
    fn from(inner: Arc<NativeFluvio>) -> Self {
        Self {
            inner,
            cluster_name: "unknown".to_string(),
        }
    }
}

impl Debug for FluvioBrowser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fluvio").finish()
    }
}

impl Deref for FluvioBrowser {
    type Target = NativeFluvio;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
