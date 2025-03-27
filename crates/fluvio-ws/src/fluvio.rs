use std::ops::{Deref, DerefMut};

use anyhow::Result;
use url::Url;

use fluvio::Fluvio;

pub use fluvio::FluvioConfig;

use crate::net::FluvioWebsocketConnector;

pub struct FluvioWs {
    pub inner: Fluvio,
    cluster_name: String,
}

impl PartialEq for FluvioWs {
    fn eq(&self, other: &Self) -> bool {
        self.cluster_name == other.cluster_name
    }
}

impl Eq for FluvioWs {}

impl FluvioWs {
    pub async fn connect(addr: Url, config: &FluvioConfig) -> Result<Self> {
        let connector = FluvioWebsocketConnector::new(addr.to_string(), None, None);

        let inner = Fluvio::connect_with_connector(Box::new(connector), config).await?;

        Ok(Self {
            inner,
            cluster_name: addr.host_str().unwrap().to_string(),
        })
    }

    pub async fn connect_with_token(addr: Url, config: &FluvioConfig, token: &str) -> Result<Self> {
        let connector =
            FluvioWebsocketConnector::new(addr.to_string(), Some(token.to_owned()), None);

        let inner = Fluvio::connect_with_connector(Box::new(connector), config).await?;

        Ok(Self {
            inner,
            cluster_name: addr.host_str().unwrap().to_string(),
        })
    }
}

impl From<Fluvio> for FluvioWs {
    fn from(inner: Fluvio) -> Self {
        Self {
            inner,
            cluster_name: "unknown".to_string(),
        }
    }
}

impl std::fmt::Debug for FluvioWs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fluvio").finish()
    }
}

impl Deref for FluvioWs {
    type Target = Fluvio;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for FluvioWs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
