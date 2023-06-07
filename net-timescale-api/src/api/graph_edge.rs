pub mod graph_edge_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_edge_capnp.rs"));
}
use graph_edge_capnp::graph_edge;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug)]
pub struct GraphEdgeDTO {
    src_addr: String,
    dst_addr: String,
}


impl GraphEdgeDTO {
    pub fn new ( src_addr: String, dst_addr: String) -> Self {
        GraphEdgeDTO {
            src_addr, 
            dst_addr, 
        }
    }

    pub fn get_src_addr (&self) -> &str {
        &self.src_addr
    }

    pub fn get_dst_addr (&self) -> &str {
        &self.dst_addr
    }
}

impl Encoder for GraphEdgeDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<graph_edge::Builder>();
        
        struct_to_encode.set_src_addr(&self.src_addr);
        struct_to_encode.set_dst_addr(&self.dst_addr);
    
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }
}

impl Decoder for GraphEdgeDTO {
    fn decode(data: Vec<u8>) -> Self {
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(), //Think about using std::io::Cursor here
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<graph_edge::Reader>().unwrap();

        GraphEdgeDTO {  
            src_addr: String::from(decoded_struct.get_src_addr().unwrap()), 
            dst_addr: String::from(decoded_struct.get_dst_addr().unwrap()),
        }
    }
}