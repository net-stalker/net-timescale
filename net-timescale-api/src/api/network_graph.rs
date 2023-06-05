mod network_graph_capnp {
    include!(concat!(env!("OUT_DIR"), "/network_graph_capnp.rs"));
}
use std::ops::Deref;

use network_graph_capnp::network_graph;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;

use super::graph_link::GraphLinkDTO;


#[derive(Debug)]
pub struct NetworkGraphDTO {
    links: Vec<GraphLinkDTO>
}

impl NetworkGraphDTO {
    pub fn new ( links: Vec<GraphLinkDTO>) -> Self {
        NetworkGraphDTO {
            links,
        }
    }

    pub fn get_links (&self) -> &[GraphLinkDTO] {
        &self.links
    }
}

impl Encoder for NetworkGraphDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<network_graph::Builder>();

        let mut encoded_links = struct_to_encode.init_links(self.links.len() as u32);

        for i in 0..self.links.len() {
            encoded_links.set(i as u32, &self.links[i].encode())
        }
    
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }
}

impl Decoder for NetworkGraphDTO {
    fn decode(data: Vec<u8>) -> Self {
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(), //Think about using std::io::Cursor here
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<network_graph::Reader>().unwrap();

        let encoded_links = decoded_struct.get_links().unwrap();
        let mut decoded_links = Vec::new();

        for i in 0..encoded_links.len() {
            decoded_links.push(GraphLinkDTO::decode(encoded_links.get(i).unwrap().to_owned()))
        }

        NetworkGraphDTO {
            links: decoded_links,
        }
    }
}


#[cfg(test)]
mod tests{
    use net_proto_api::{encoder_api::Encoder, decoder_api::Decoder};

    use crate::api::graph_link::GraphLinkDTO;

    use super::NetworkGraphDTO;

    #[test]
    fn no_data_loss_encode_decode_test() {
        let graph: NetworkGraphDTO = NetworkGraphDTO::new(
            vec![
                GraphLinkDTO::new("0.0.0.0:0000".into(), "0.0.0.0:0001".into()), 
                GraphLinkDTO::new("0.0.0.0:0001".into(), "0.0.0.0:0002".into())
                ]
            );
        let encoded_graph = graph.encode();

        let graph = NetworkGraphDTO::decode(encoded_graph);
        let links = graph.get_links();

        assert_eq!(links[0], GraphLinkDTO::new("0.0.0.0:0000".into(), "0.0.0.0:0001".into()));
        assert_eq!(links[1], GraphLinkDTO::new("0.0.0.0:0001".into(), "0.0.0.0:0002".into()));
    }
}
