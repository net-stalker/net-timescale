use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self},
};

use simple_websockets::{Event, Message, Responder};

use net_core::config::{ConfigManager, ConfigSpec, ConfigFile, FileReader};
use net_core::transport::connector_nng::{ConnectorNng, Proto};
use net_core::transport::context::ContextBuilder;
use net_core::transport::polling::Poller;
use net_hub::server_command::ServerCommand;

fn main() {
    //Global for the project
    let config = Arc::new(ConfigManager { application_name: "net-hub", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load());

    // //Global for the project
    // let config = hub_context.clone().config.clone();
    // if !config.dealer.enable {
    //     println!("Dealer is disabled!");
    //     return;
    // }

    let clients = Arc::new(RwLock::new(HashMap::new()));
    let clients_inner = clients.clone();

    let ws_thread_handle = thread::spawn(move || {
        let event_hub = simple_websockets::launch(9091)
            .expect("failed to listen on port 9091");

        loop {
            match event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    println!("A client connected with id #{}", client_id);
                    clients_inner
                        .write()
                        .unwrap()
                        .insert(client_id, responder.clone());

                    //TODO for every websocket conn should be created new zmq socket referenced to the websocket client connection.
                }
                Event::Disconnect(client_id) => {
                    println!("Client #{} disconnected.", client_id);
                    clients_inner.write().unwrap().remove(&client_id);
                }
                Event::Message(client_id, message) => {
                    println!(
                        "Received a message from client #{}: {:?}",
                        client_id, message
                    );
                    // let responder = clients.get(&client_id).unwrap();
                    // let responder = clients.write().unwrap().get(&client_id).unwrap();
                    // responder.send(message);
                }
            }
        }
    });

    let server_command = ServerCommand { clients };
    let server = ConnectorNng::builder()
        .with_endpoint(config.dealer.endpoint.clone())
        .with_proto(Proto::Rep)
        .with_handler(server_command)
        .build()
        .bind()
        .into_inner();

    let poller = thread::spawn(move || {
        Poller::new()
            .add(server)
            .poll();
    });

    poller.join().unwrap();

    let context = ContextBuilder::new().build(); //TODO Use From trait instead of new
    let context_translator = context.clone();

    // let translator_router = Arc::new(ConnectorBuilder::new()
    //     .with_context(context_translator)
    //     .with_xtype(zmq::DEALER)
    //     .with_endpoint("tcp://0.0.0.0:5557".to_string())
    //     .with_handler(|data| {
    //         println!("received from translator {:?}", data);
    //         let magic_num = &data[..4];
    //         if 3569595041_u32.to_be_bytes() == magic_num {
    //             println!("Global header will be skipped");
    //             return;
    //         }
    //
    //         // clients.read().unwrap().iter().for_each(|endpoint| {
    //         //     println!("Connections: {:?}", endpoint);
    //         //     let responder = endpoint.1;
    //         //     responder.send(Message::Text(format!("{:?}", &data)));
    //         // });
    //     })
    //     .build()
    //     .bind());
    // let arc = translator_router.clone();

    // let agent_router_handle = thread::spawn(move || agent_router.poll());
    // let translator_router_handle = thread::spawn(move || translator_router.poll());

    ws_thread_handle.join().unwrap();
    // agent_router_handle.join().unwrap();
    // translator_router_handle.join().unwrap();
}
