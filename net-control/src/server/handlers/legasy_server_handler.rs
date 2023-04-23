use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, str::from_utf8};

use crate::server::aggregator::{AddClient, ReadBufferForClient, Ended, IdentifyStatus, self};

use super::super::aggregator::Aggregator;

//TODO: Change trait to the ServerHandler

#[derive(Clone)]
pub struct LegasyServerHandler {
    aggregator: std::sync::Arc<std::sync::Mutex<Aggregator>>
}

impl LegasyServerHandler {
    fn aggregate_msg(&mut self, client: u64, buf: &[u8]) -> aggregator::Result<()> {
        self.aggregator.lock().unwrap().read(client, buf)
    }

    fn identify_client_msg_status(&self, client: u64) -> aggregator::Result<Ended> {
        self.aggregator.lock().unwrap().identify_status(client).clone()
    }

    fn erase_client_data (&self, client: u64) -> aggregator::Result<()> {
        self.aggregator.lock().unwrap().erase_data(client)
    }

    fn get_client_msg (&self, client: u64) -> aggregator::Result<String> {
        let aggregator = self.aggregator.lock().unwrap();
        let client_data = aggregator.data(client);
        match client_data {
            Ok(data) => Ok(from_utf8(data).unwrap().to_owned()),
            Err(e) => Err(e)
        }
    }
}

impl Default for LegasyServerHandler {
    fn default() -> Self {
        Self {
            aggregator: std::sync::Arc::new(std::sync::Mutex::new(Aggregator::default()))
        }
    }
}

impl AddClient<russh::ChannelId> for LegasyServerHandler {
    fn add_client (&mut self, client: russh::ChannelId) -> aggregator::Result<()> {
        let mut aggregator = self.aggregator.lock().unwrap();
        let hasher = &mut DefaultHasher::new();
        client.hash(hasher);
        aggregator.add_client(hasher.finish())
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

        if self.add_client(channel.id()).is_ok() {
            return Ok((self, true, session))
        } else {
            todo!();
            Ok((self, false, session))
        }
    }

    async fn channel_close(self, channel: russh::ChannelId, mut session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        session.data(channel, russh::CryptoVec::from("Goodbye, user!".to_string()));
        Ok((self, session))
    }

    async fn data(mut self, channel: russh::ChannelId, data: &[u8], mut session: russh::server::Session) -> Result<(Self, russh::server::Session), Self::Error> {
        //Cook data into a string
        //This data is used for echo only
        let mut cooked_data = std::str::from_utf8(data).unwrap().to_string();
        
        //Generate ChannelId hash
        let hasher = &mut DefaultHasher::new();
        channel.hash(hasher);
        let client_hash = hasher.finish();
        
        //Aggregate data
        let client_msg_read_result = self.aggregate_msg(client_hash, data);
        match client_msg_read_result {
            Ok(_) => (),
            Err(_) => todo!()
        }

        let client_msg_status = self.identify_client_msg_status(client_hash).unwrap();
        match client_msg_status {
            Ended::Ended => {
                cooked_data.push_str(format!("\n{}@cli:", "user").as_str());

                let client_msg = self.get_client_msg(client_hash);
                match client_msg {
                    Ok(msg) => println!("Client [{}] msg is: {}", client_hash, msg),
                    Err(_) => todo!()
                } 

                match self.erase_client_data(client_hash) {
                    Ok(_) => (),
                    Err(_) => todo!()
                }
            }
            Ended::NotEnded => (),
        }
        
//TODO: Move to the own function/struct
        //Echo every symbol (user CLI)
        session.data(channel, russh::CryptoVec::from(cooked_data));


        Ok((self, session))
    }
}