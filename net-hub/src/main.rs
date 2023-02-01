use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self},
};

use simple_websockets::{Event, Message, Responder};

use net_core::config::{ConfigManager, ConfigSpec, ConfigFile, FileReader};
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::context::ContextBuilder;
use net_core::transport::polling::Poller;
use net_hub::command::agent::AgentCommand;
use net_hub::command::translator::TranslatorCommand;

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

    //TODO use instead ws from ConnectoNNG
    thread::spawn(move || {
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

    let translator = ConnectorNNG::builder()
        .with_endpoint("tcp://0.0.0.0:5557".to_string())
        .with_proto(Proto::Req)
        .with_handler(TranslatorCommand)
        .build()
        .connect()
        .into_inner();
    let translator_clone = translator.clone();

    let server_command = AgentCommand { clients, translator: translator_clone };
    let server = ConnectorNNG::builder()
        .with_endpoint(config.dealer.endpoint.clone())
        .with_proto(Proto::Rep)
        .with_handler(server_command)
        .build()
        .bind()
        .into_inner();

    thread::spawn(move || {
        Poller::new()
            .add(server)
            .add(translator)
            .poll();
    }).join().unwrap();
}
