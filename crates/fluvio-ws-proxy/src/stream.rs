use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::web::Bytes;
use actix_web_actors::ws::{self, CloseReason};
use async_channel::Sender;
use tracing::{debug, error};

/// Define HTTP actor
pub(crate) struct CmdStream {
    pub write_queue_tx: Sender<Bytes>,
}

impl Actor for CmdStream {
    type Context = ws::WebsocketContext<CmdStream>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for CmdStream {
    // forward records sent from client to fluvio
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Binary(bin)) => {
                if let Err(err) = self.write_queue_tx.send_blocking(bin) {
                    error!("Failed to send message to fluvio: {}", err);
                }
            }
            Ok(ws::Message::Pong(_)) => {
                debug!("Received pong from websocket client.")
            }
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => ctx.close(Some(CloseReason {
                description: Some(
                    "Fluvio websocket proxy only supports binary messages".to_string(),
                ),
                code: ws::CloseCode::Invalid,
            })),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct FluvioReply {
    pub event: Vec<u8>,
}

impl Handler<FluvioReply> for CmdStream {
    type Result = ();

    // send fluvio records back to client
    fn handle(&mut self, msg: FluvioReply, ctx: &mut Self::Context) {
        ctx.binary(msg.event);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Ping {}

impl Handler<Ping> for CmdStream {
    type Result = ();

    fn handle(&mut self, _msg: Ping, ctx: &mut Self::Context) {
        debug!("Sending ping to websocket client.");

        ctx.ping(&[]);
    }
}
