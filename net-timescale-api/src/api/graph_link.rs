mod graph_link_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_link_capnp.rs"));
}
use graph_link_capnp::graph_link;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphLinkDTO {
    left_node: String,
    right_node: String
}

impl GraphLinkDTO {
    pub fn new ( left_node: String, right_node: String) -> Self {
        GraphLinkDTO {
            left_node,
            right_node,
        }
    }

    pub fn get_left_node (&self) -> &str {
        &self.left_node
    }

    pub fn get_right_node (&self) -> &str {
        &self.right_node
    }
}

impl Encoder for GraphLinkDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<graph_link::Builder>();
        
        struct_to_encode.set_left_node(&self.left_node);
        struct_to_encode.set_right_node(&self.right_node);
    
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }
}

impl Decoder for GraphLinkDTO {
    fn decode(data: Vec<u8>) -> Self {
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(), //Think about using std::io::Cursor here
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<graph_link::Reader>().unwrap();

        GraphLinkDTO {
            left_node: String::from(decoded_struct.get_left_node().unwrap()),
            right_node: String::from(decoded_struct.get_right_node().unwrap()),
        }
    }
}