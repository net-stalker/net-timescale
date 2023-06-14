pub mod graph_node_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_node_capnp.rs"));
}
use graph_node_capnp::graph_node;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug)]
pub struct GraphNodeDTO {
    address: String,
}


impl GraphNodeDTO {
    pub fn new ( address: String) -> Self {
        GraphNodeDTO {
            address,
        }
    }

    pub fn get_address (&self) -> &str {
        &self.address
    }
}

impl Encoder for GraphNodeDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<graph_node::Builder>();
        
        struct_to_encode.set_address(&self.address);
    
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }
}

impl Decoder for GraphNodeDTO {
    fn decode(data: Vec<u8>) -> Self {
//TODO: Think about using std::io::Cursor here
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(),
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<graph_node::Reader>().unwrap();

        GraphNodeDTO {  
            address: String::from(decoded_struct.get_address().unwrap()),
        }
    }
}