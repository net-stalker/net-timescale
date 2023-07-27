use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};

use net_core::transport::sockets::{Handler, Receiver, Sender};

use net_proto_api::decoder_api::Decoder;

use net_timescale_api::api::network_packet::NetworkPacketDTO;

use simple_websockets::{Event, EventHub, Message, Responder};
use chrono::{Utc, DateTime, TimeZone};

use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use crate::command::ws_context::WsContext;
use net_timescale_api::api::network_graph_request::NetworkGraphRequest;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;

#[derive(Debug)]
pub struct WsServerCommand<S>
where S: Sender
{
    context: WsContext,
    event_hub: Option<EventHub>,
    consumer: Option<Arc<S>>,
    graphs_history: Arc<RwLock<HashMap<u64, (DateTime<Utc>, NetworkGraphDTO)>>>,
}

impl<S> Default for WsServerCommand<S>
where S: Sender
{
    fn default() -> Self {
        Self {
            context: WsContext::default(),
            event_hub: None,
            consumer: None,
            graphs_history: Arc::new(RwLock::new(HashMap::default())),
        }
    }
}

impl<S> WsServerCommand<S>
where S: Sender
{
    pub fn new(consumer: Arc<S>) -> Self {
        WsServerCommand {
            context: WsContext::default(),
            event_hub: None,
            consumer: Some(consumer),
            graphs_history: Arc::new(RwLock::new(HashMap::default())),
        }
    }
    pub fn bind(mut self, end_point: String) -> Self {
        let listener = TcpListener::bind(end_point.as_str()).expect(
            format!("failed to bind web socket on address {}", end_point.as_str()).as_str()
        );
        self.event_hub = Some(
            simple_websockets::launch_from_listener(listener)
                .expect(format!("failed to listen on address {}", end_point.as_str()).as_str())
        );
        self
    }
    pub fn set_consumer(mut self, consumer: Arc<S>) -> Self {
        self.consumer = Some(consumer);
        self
    }
    pub fn get_context(&self) -> WsContext {
        self.context.clone()
    }
    pub fn poll(&self, events_count: i32) {
        let mut counter = 0;
        while counter != events_count {
            match self.event_hub.as_ref().unwrap().poll_event() {
                Event::Connect(connection_id, responder) => {
                    log::debug!("a new connection with id #{}", connection_id);
                    self.context.add_connection(connection_id, responder);
                    log::debug!("amount of connections {}", self.context.get_size());
                }
                Event::Disconnect(connection_id) => {
                    log::debug!("a connection with id #{} is disconnected.", connection_id);
                    self.context.remove_connection(connection_id);
                    log::debug!("amount of connections {}", self.context.get_size());
                }
                Event::Message(connection_id, message) => {
                    match message {
                        Message::Binary(data) => {
                            log::debug!(
                                "received a query from #{}: {:?}",
                                connection_id, data
                            );
                            let envelope = Envelope::decode(data.as_slice());
                            match envelope.get_type() {
                                "NG_request" => {
                                    let graph_request = NetworkGraphRequest::decode(envelope.get_data());
                                    match graph_request.get_end_date_time() == 0 {
                                        true => {
                                            // TODO: change realtime requesting in net-explorer to make it work
                                            log::debug!("adding {} to graph_history", connection_id);
                                            self.graphs_history
                                                .write()
                                                .unwrap()
                                                .insert(
                                                    connection_id,
                                                    (
                                                        Utc.timestamp_millis_opt(graph_request.get_start_date_time()).unwrap(),
                                                        NetworkGraphDTO::new(&[], &[])
                                                    )
                                                );
                                        },
                                        false => {
                                            log::debug!("removing {} from connection_id", connection_id);
                                            self.graphs_history
                                                .write()
                                                .unwrap()
                                                .remove(
                                                    &connection_id
                                                );
                                        }
                                    }
                                    self.consumer.as_ref().unwrap().send(data.as_slice());
                                },
                                _ => {
                                    log::error!("unknown msg type {}", envelope.get_type());
                                }
                            }
                        },
                        Message::Text(msg) => {
                            log::debug!(
                                "received a message from client #{}: {:?}",
                                connection_id, msg
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