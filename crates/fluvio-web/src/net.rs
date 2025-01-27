use std::io::Error as IoError;

use anyhow::Result;
use async_trait::async_trait;
use futures_util::AsyncReadExt;
use url::Url;
use ws_stream_wasm::WsMeta;

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

#[async_trait(?Send)]
impl TcpDomainConnector for FluvioWebsocketConnector {
    async fn connect(
        &self,
        addr: &str,
    ) -> Result<(BoxWriteConnection, BoxReadConnection, ConnectionFd), IoError> {
        let mut url = Url::parse(&self.url).map_err(|err| {
            IoError::other(format!(
                "Failed to parse URL on connect. URL: {}. Error: {}",
                self.url,
                err.to_string()
            ))
        })?;

        if let Some(ref domain) = self.domain {
            let token = self.token.clone().unwrap_or_default();
            url.set_query(Some(&format!(
                "token={}&domain={}&addr={}",
                &token, domain, addr
            )));
        }

        let (mut _ws, wsstream) = WsMeta::connect(url.clone(), None)
            .await
            .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?;

        let wsstream_io = wsstream.into_io();
        let (stream, sink) = wsstream_io.split();

        Ok((Box::new(sink), Box::new(stream), url.to_string()))
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
}
