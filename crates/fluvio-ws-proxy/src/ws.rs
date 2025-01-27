use std::net::SocketAddr;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use tracing::info;

pub(crate) async fn start_web_services(addr: SocketAddr) -> Result<()> {
    info!(ip = %addr.ip(),port = addr.port(),"starting web server at");
    HttpServer::new(move || App::new().route("/ws/", web::get().to(ws::register)))
        .bind(addr)?
        .run()
        .await
        .map_err(|err| err.into())
}

mod ws {
    use actix_web::{
        web::{Payload, Query},
        Error, HttpRequest, HttpResponse,
    };
    use actix_web_actors::ws;
    use async_channel::bounded;
    use futures_util::{AsyncReadExt, AsyncWriteExt};
    use serde::Deserialize;
    use tracing::{debug, info};

    #[derive(Deserialize, Debug)]
    pub struct RegisterQuery {
        pub addr: Option<String>,
    }

    use crate::{
        fluvio_domain_connector::fluvio_domain_connector,
        stream::{CmdStream, FluvioReply, Ping},
    };

    pub async fn register(
        req: HttpRequest,
        stream: Payload,
        my_query: Query<RegisterQuery>,
    ) -> Result<HttpResponse, Error> {
        let (mut fluvio_write_connection, mut fluvio_read_connection) =
            fluvio_domain_connector(my_query.addr.clone()).await?;
        let (write_queue_tx, write_queue_rx) = bounded(100);

        let cmd_stream = CmdStream { write_queue_tx };

        let response_builder = ws::WsResponseBuilder::new(cmd_stream, &req, stream);
        let (addr, res) = response_builder.start_with_addr()?;

        let mut buf = vec![0; 10000];

        actix::spawn(async move {
            loop {
                tokio::select! {
                    bin = write_queue_rx.recv() => {

                        match bin {
                            Ok(bin) => {
                                if let Err(e) = fluvio_write_connection.write_all(&bin).await {
                                    debug!("Failed to write to fluvio: {}", e);
                                };
                            }
                            Err(e) => {
                                debug!("Failed to receive message from client: {}", e);
                            }
                        }
                    }
                    bytes_read = fluvio_read_connection.read(&mut buf) => {
                        if let Ok(bytes_read) = bytes_read {
                            addr.do_send(FluvioReply { event: buf[..bytes_read].to_vec(),
                            });
                        } else {
                            break
                        }
                    }
                    _ = fluvio_future::timer::sleep(std::time::Duration::from_secs(50)) => {
                        addr.do_send(Ping {});
                    }
                }
            }
        });

        info!("Websocket connection established.");

        Ok(res)
    }
}
