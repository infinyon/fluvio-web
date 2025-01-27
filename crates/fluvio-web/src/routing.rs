use anyhow::{anyhow, Result};
use http::Uri;
use url::Url;
use web_sys::{window, Location};

pub(crate) fn absolute_url_with_path(path: Uri) -> Result<Url> {
    let location = current_location();
    let href = location
        .href()
        .map_err(|_| anyhow::anyhow!("failed to get href"))?;

    let mut url =
        Url::parse(&href).map_err(|err| anyhow::anyhow!("failed to parse url: {}", err))?;

    url.set_path(path.to_string().as_str());
    Ok(url)
}

/// Retrieves the current location from `window` Global Object.
fn current_location() -> Location {
    window().expect("failed to get window").location()
}

pub fn local_websocket_url() -> Result<Url> {
    let uri = "/ws/".parse::<Uri>()?;
    let mut url = absolute_url_with_path(uri)?;
    url.set_scheme("ws")
        .map_err(|e| anyhow!("could not set url scheme: {:?}", e))?;

    Ok(url)
}

pub fn origin() -> Url {
    let location = current_location();
    let href = location.origin().unwrap();

    Url::parse(&href).unwrap()
}
