use actix_web::{error::InternalError, http::StatusCode, Error};

use fluvio::config::ConfigFile;
use fluvio_future::net::{
    DefaultDomainConnector, ReadConnection, TcpDomainConnector, WriteConnection,
};

pub(crate) async fn fluvio_domain_connector(
    addr: Option<String>,
) -> Result<(Box<dyn WriteConnection>, Box<dyn ReadConnection>), Error> {
    let endpoint = addr.unwrap_or(fluvio_sc_endpoint()?);
    let connector = DefaultDomainConnector::new();

    let (fluvio_writer, fluvio_reader, _fd) =
        connector.connect(&endpoint).await.map_err(Error::from)?;

    Ok((fluvio_writer, fluvio_reader))
}

fn fluvio_sc_endpoint() -> Result<String, Error> {
    let config_file = ConfigFile::load_default_or_new().map_err(Error::from)?;
    let config = match config_file.config().current_cluster() {
        Ok(config) => config,
        Err(_) => {
            return Err(InternalError::new(
                "cluster config not found for current profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into())
        }
    };

    Ok(config.endpoint.clone())
}
