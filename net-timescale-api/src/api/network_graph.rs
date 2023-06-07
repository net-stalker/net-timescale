pub mod graph_edge_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_edge_capnp.rs"));
}
pub mod graph_node_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_node_capnp.rs"));
}

pub mod network_graph_capnp {
    include!(concat!(env!("OUT_DIR"), "/network_graph_capnp.rs"));
}
use network_graph_capnp::network_graph;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;

use super::graph_edge::GraphEdgeDTO;
use super::graph_node::GraphNodeDTO;


#[derive(Debug)]
pub struct NetworkGraphDTO {
    graph_nodes: Vec<GraphNodeDTO>,
    graph_edges: Vec<GraphEdgeDTO>,
}

impl NetworkGraphDTO {
    pub fn new ( graph_nodes: Vec<GraphNodeDTO>, graph_edges: Vec<GraphEdgeDTO>) -> Self {
        NetworkGraphDTO {
            graph_nodes,
            graph_edges,
        }
    }

    pub fn get_graph_nodes (&self) -> &[GraphNodeDTO] {
        &self.graph_nodes
    }

    pub fn get_graph_edges (&self) -> &[GraphEdgeDTO] {
        &self.graph_edges
    }
}

impl Encoder for NetworkGraphDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<network_graph::Builder>();
        
        struct_to_encode.set_edges(self.graph_edges);
        struct_to_encode.set_nodes(self.graph_nodes);
    
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

        NetworkGraphDTO { 
            graph_nodes: decoded_struct.get_nodes(),
            graph_edges: decoded_struct.get_edges(),
        }
    }
}