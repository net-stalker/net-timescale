use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};

use net_core::transport::sockets::{Handler, Receiver, Sender};

use net_proto_api::decoder_api::Decoder;

use net_timescale_api::api::network_packet::NetworkPacketDTO;

use simple_websockets::{Event, EventHub, Message, Responder};

use std::sync::atomic::AtomicBool;

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
    pub fn send(&self, msg: String) {
        self.clients.write().unwrap().iter().for_each(|endpoint| {
            log::debug!("connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Text(format!("{:?}", msg)));
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
                    log::debug!(
                            "received a message from client #{}: {:?}",
                            client_id, message
                        );
                    // TODO: finish with sending data to consumer
                    // self.consumer.send(message)
                }
            }
            counter += 1;
        }
    }
    pub fn into_inner(mut self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl<S> Handler for WsServerCommand<S>
where S: Sender
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let formated_string = match String::from_utf8(data) {
            Ok(msg) => msg,
            Err(_) => "error while parsing the msg".to_string()
        };
        log::debug!("received from timescale {}", formated_string);
        self.send(formated_string);
    }
}