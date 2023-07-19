use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};

use net_core::transport::sockets::{Handler, Receiver, Sender};

use net_proto_api::decoder_api::Decoder;

use net_timescale_api::api::network_packet::NetworkPacketDTO;

use simple_websockets::{Event, EventHub, Message, Responder};
use chrono::{Utc, DateTime, TimeZone};

use std::sync::atomic::AtomicBool;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use net_timescale_api::api::date_cut::DateCutDTO;

pub struct WsServerCommand<S>
where S: Sender
{
    clients: Arc<RwLock<HashMap<u64, Responder>>>,
    event_hub: Option<EventHub>,
    // TODO: for now consumer is unused. Is meant to be used for sending data to net-timescale
    consumer: Arc<S>
}
impl<S> WsServerCommand<S>
where S: Sender
{
    pub fn new(consumer: Arc<S>) -> Self {
        WsServerCommand {
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_hub: None,
            consumer
        }
    }
    pub fn bind(mut self, end_point: String) -> Self {
        // TODO: changed ws server creation
        let listener = TcpListener::bind(end_point.as_str()).expect(
            format!("failed to bind web socket on address {}", end_point.as_str()).as_str()
        );
        self.event_hub = Some(
            simple_websockets::launch_from_listener(listener)
                .expect(format!("failed to listen on address {}", end_point.as_str()).as_str())
        );
        self
    }
    pub fn send(&self, msg: Vec<u8>) {
        self.clients.write().unwrap().iter().for_each(|endpoint| {
            log::debug!("connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Binary(msg.clone()));
        });
    }
    pub fn poll(&self, events_count: i32) {
        let mut counter = 0;
        while counter != events_count {
            match self.event_hub.as_ref().unwrap().poll_event() {
                Event::Connect(client_id, responder) => {
                    log::info!("a client connected with id #{}", client_id);
                    self.clients.write().unwrap().insert(client_id, responder.clone());
                }
                Event::Disconnect(client_id) => {
                    log::info!("client #{} disconnected.", client_id);
                    self.clients.write().unwrap().remove(&client_id);
                }
                Event::Message(client_id, message) => {
                    match message {
                        Message::Binary(data) => {
                            log::debug!(
                                "received a query from client #{}: {:?}",
                                client_id, data
                            );
                            let envelope = Envelope::decode(data.as_slice());
                            match envelope.get_type() {
                                "network_graph" => {
                                    // TODO: add some logs here
                                    self.send(data.to_owned());
                                },
                                _ => {
                                    log::info!("msg type {}", envelope.get_type());
                                    self.consumer.send(data.as_slice());
                                }
                            }
                        },
                        Message::Text(msg) => {
                            log::debug!(
                                "received a message from client #{}: {:?}",
                                client_id, msg
                            );
                        }
                    }
                }
            }
            counter += 1;
        }
    }
    pub fn into_inner(mut self) -> Arc<Self> {
        Arc::new(self)
    }
}