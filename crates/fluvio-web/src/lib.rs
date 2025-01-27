pub mod fluvio;
pub mod routing;

#[cfg(feature = "leptos")]
pub mod leptos_fluvio;

#[cfg(target_arch = "wasm32")]
mod net;

#[cfg(target_arch = "wasm32")]
pub mod local {
    use std::rc::Rc;

    use anyhow::Result;
    use fluvio::Fluvio;

    use crate::fluvio::AppServices;
    use crate::routing::local_websocket_url;

    pub async fn connect() -> Result<Rc<Fluvio>> {
        let url = local_websocket_url()?;

        let fluvio_client = AppServices::reconnect_fluvio_client(url)
            .await?
            .inner_clone();
        Ok(fluvio_client)
    }
}

/// This module is only available when the target is not wasm32 which is never in this case
/// This is to satisfy the compiler and vscode to make it easier to discover
#[cfg(not(target_arch = "wasm32"))]
pub mod local {
    use std::rc::Rc;

    use anyhow::Result;
    use fluvio::Fluvio;

    pub async fn connect() -> Result<Rc<Fluvio>> {
        todo!("not implemented")
    }
}

#[cfg(target_arch = "wasm32")]
pub mod remote {
    use std::rc::Rc;

    use anyhow::Result;
    use fluvio::Fluvio;
    use url::Url;

    use crate::fluvio::AppServices;

    pub async fn connect(url: Url) -> Result<Rc<Fluvio>> {
        let fluvio_client = AppServices::reconnect_fluvio_client(url)
            .await?
            .inner_clone();
        Ok(fluvio_client)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod remote {
    use std::rc::Rc;

    use anyhow::Result;
    use fluvio::Fluvio;
    use url::Url;

    pub async fn connect(_url: Url) -> Result<Rc<Fluvio>> {
        todo!("not implemented")
    }
}
