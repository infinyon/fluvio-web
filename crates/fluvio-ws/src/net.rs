use std::io::Error as IoError;

use anyhow::Result;
use async_trait::async_trait;
use async_tungstenite::{ByteReader, ByteWriter, async_std::connect_async};
use futures_util::StreamExt;
use url::Url;

use fluvio_future::net::{
    BoxReadConnection, BoxWriteConnection, ConnectionFd, DomainConnector, TcpDomainConnector,
};

#[derive(Clone, Default)]
pub struct FluvioWebsocketConnector {
    url: String,
    domain: Option<String>,
    token: Option<String>,
}
impl FluvioWebsocketConnector {
    pub fn new(url: String, token: Option<String>, domain: Option<String>) -> Self {
        Self { url, domain, token }
    }
}

#[async_trait()]
impl TcpDomainConnector for FluvioWebsocketConnector {
    async fn connect(
        &self,
        addr: &str,
    ) -> Result<(BoxWriteConnection, BoxReadConnection, ConnectionFd), IoError> {
        let mut url = Url::parse(&self.url).map_err(|err| {
            IoError::other(format!(
                "Failed to parse URL on connect. URL: {}. Error: {}",
                self.url, err
            ))
        })?;

        match (&self.domain, &self.token) {
            (Some(domain), Some(token)) => {
                url.set_query(Some(&format!("token={token}&domain={domain}&addr={addr}")));
            }
            (Some(domain), None) => {
                url.set_query(Some(&format!("domain={domain}&addr={addr}")));
            }
            (None, Some(token)) => {
                url.set_query(Some(&format!("token={token}&addr={addr}")));
            }
            (None, None) => {
                url.set_query(Some(&format!("addr={addr}")));
            }
        }

        let (socket, _) = connect_async(&url).await.expect("Failed to connect");

        let (sink, stream) = socket.split();

        let sink = ByteWriter::new(sink);
        let stream = ByteReader::new(stream);

        Ok((Box::new(sink), Box::new(stream), -1))
    }

    fn new_domain(&self, domain: String) -> DomainConnector {
        Box::new(Self::new(
            self.url.clone(),
            self.token.clone(),
            Some(domain),
        ))
    }

    fn domain(&self) -> &str {
        if let Some(domain) = &self.domain {
            domain
        } else {
            ""
        }
    }

    fn clone_box(&self) -> DomainConnector {
        Box::new(self.clone())
    }
}
