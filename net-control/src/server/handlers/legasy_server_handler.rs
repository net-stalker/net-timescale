use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use crate::server::aggregator::AddClient;

use super::super::aggregator::Aggregator;

//TODO: Change trait to the ServerHandler

#[derive(Clone)]
pub struct LegasyServerHandler {
    aggregator: std::sync::Arc<std::sync::Mutex<Aggregator>>
}

impl LegasyServerHandler {
}

impl Default for LegasyServerHandler {
    fn default() -> Self {
        Self {
            aggregator: std::sync::Arc::new(std::sync::Mutex::new(Aggregator::default()))
        }
    }
}

impl AddClient<russh::ChannelId> for LegasyServerHandler {
    fn add_client (&mut self, client: russh::ChannelId) {
        let mut aggregator = self.aggregator.lock().unwrap();
        let hasher = &mut DefaultHasher::new();
        client.hash(hasher);
        aggregator.add_client(hasher.finish());
    }
}

#[async_trait::async_trait]
impl russh::server::Handler for LegasyServerHandler {
    type Error = anyhow::Error;

    async fn disconnected(self, session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        Ok((self, session))
    }

    async fn auth_none(self, user: &str) -> Result<(Self, russh::server::Auth), Self::Error> {
        Ok((self, russh::server::Auth::Accept))
    }

    async fn auth_password(self, user: &str, password: &str) -> Result<(Self, russh::server::Auth), Self::Error> {
        Ok((self, russh::server::Auth::Accept))
    }

    async fn auth_publickey(self, user: &str, public_key: &russh_keys::key::PublicKey) -> Result<(Self, russh::server::Auth), Self::Error> {
        Ok((self, russh::server::Auth::Accept))
    }

    async fn auth_succeeded(self, session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        Ok((self, session))
    }

    async fn channel_open_session(mut self, mut channel: russh::Channel<russh::server::Msg> , session: russh::server::Session) -> Result<(Self, bool, russh::server::Session), Self::Error> {
        match channel.data("\nHello from the CLI!\r\n\r\nuser@cli:".as_bytes()).await {
            Ok(_) => (),
            Err(_) => todo!(),
        }

        self.add_client(channel.id());

        Ok((self, true, session))
    }

    async fn channel_close(self, channel: russh::ChannelId, mut session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        session.data(channel, russh::CryptoVec::from("Goodbye, user!".to_string()));
        Ok((self, session))
    }

    async fn data(self, channel: russh::ChannelId, data: &[u8], mut session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        // let mut data_cooked = std::str::from_utf8(data).unwrap().to_string();
        
        // if !data_cooked.ends_with("\r") {
        //     println!("Got data from the user! Here it is: \"{}\"", data_cooked);

        //     //Echo every non "\r" symbol to the user
        //     session.data(channel, russh::CryptoVec::from(data_cooked));
        // } else {
        //     data_cooked.pop();
        //     data_cooked.push_str("\\r");

        //     println!("Got data from the user! Here it is: \"{}\"", data_cooked);
        // }

        // let push_result = 
        //     match self.push_symbol_to_aggregator_for(channel, data) {
        //         Some(Ok(s)) => s,
        //         Some(Err(_)) => todo!(),
        //         None => todo!()
        //     };

        // match push_result {
        //     Full::Ended => {
        //         session.data(channel, russh::CryptoVec::from("\r\nuser@cli:".to_string()));

        //         let mut user_command = self.pull_buffer_for(channel).unwrap().unwrap();
        //         user_command.pop();
        //         println!("Got command from the user! Here it is: \"{}\"", user_command);
                
        //         self.push_buffer_reset_for(channel); 
        //     }
        //     Full::NotEnded => {
        //     }
        // }


        Ok((self, session))
    }
}