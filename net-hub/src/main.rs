use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self},
};

use simple_websockets::{Event, Message, Responder};
use zmq::Socket;

use net_core::config::{ConfigManager, ConfigSpec, ConfigFile, FileReader};
use net_core::transport::connector::{ConnectorBuilder, Poller};
use net_core::transport::context::ContextBuilder;

fn main() {
    //Global for the project
    let config = Arc::new(ConfigManager { application_name: "net-hub", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load());

    // //Global for the project
    // let config = hub_context.clone().config.clone();
    // if !config.dealer.enable {
    //     println!("Dealer is disabled!");
    //     return;
    // }

    let event_hub = simple_websockets::launch(9091)
        .expect("failed to listen on port 9001");
    let clients: Arc<RwLock<HashMap<u64, Responder>>> = Arc::new(RwLock::new(HashMap::new()));
    let clients_inner = clients.clone();

    let ws_thread_handle = thread::spawn(move || {
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

    let context = ContextBuilder::new().build(); //TODO Use From trait instead of new
    let connector = thread::spawn(move || {
        ConnectorBuilder::new()
            .with_context(context)
            .with_xtype(zmq::DEALER)
            .with_endpoint(config.dealer.endpoint.clone())
            .with_handler(|data| {
                println!("received from monitor {:?}", data);
                let magic_num = &data[..4];
                if 3569595041_u32.to_be_bytes() == magic_num {
                    println!("Global header will be skipped");
                    return;
                }

                // clients.read().unwrap().iter().for_each(|endpoint| {
                    //     println!("Connections: {:?}", endpoint);
                    //     let responder = endpoint.1;
                    //     responder.send(Message::Text(format!("{:?}", &data)));
                // });
            })
            .build()
            .bind()
            .poll();
    });

    ws_thread_handle.join().unwrap();
    connector.join().unwrap();
}
