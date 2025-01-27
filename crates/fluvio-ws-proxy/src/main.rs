mod fluvio_domain_connector;
mod stream;
mod ws;

const WS_PORT: u16 = 8000;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use crate::ws::start_web_services;

    ctrlc::set_handler(move || {
        println!("Received Ctrl-C, shutting down");
        std::process::exit(0);
    })?;
    fluvio_future::subscriber::init_tracer(None);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), WS_PORT);
    start_web_services(addr).await?;
    Ok(())
}
