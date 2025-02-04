use anyhow::{anyhow, Result};
use http::Uri;
use url::Url;
use web_sys::window;

pub(crate) fn absolute_url_with_path(path: Uri) -> Result<Url> {
    let location = current_location();

    let mut url =
        Url::parse(&location).map_err(|err| anyhow::anyhow!("failed to parse url: {}", err))?;

    url.set_path(path.to_string().as_str());
    Ok(url)
}

/// Retrieves the current location from `window` Global Object.
fn current_location() -> String {
    window()
        .expect("failed to get window")
        .document()
        .expect("failed to get document")
        .document_uri()
        .expect("failed to get document uri")
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
    leptos::logging::log!("location: {:?}", location);

    Url::parse(&location).unwrap()
}
