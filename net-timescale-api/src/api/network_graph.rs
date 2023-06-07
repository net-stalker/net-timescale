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
        
        let mut edges_builder = struct_to_encode.reborrow().init_edges(self.graph_edges.len() as u32);
        for i in 0..self.graph_edges.len() {
            let mut edge_builder = edges_builder.reborrow().get(i as u32);
            edge_builder.set_src_addr(self.graph_edges[i].get_src_addr());
            edge_builder.set_dst_addr(self.graph_edges[i].get_dst_addr());
        }

        let mut nodes_builder = struct_to_encode.reborrow().init_nodes(self.graph_nodes.len() as u32);
        for i in 0..self.graph_nodes.len() {
            let mut node_builder = nodes_builder.reborrow().get(i as u32);
            node_builder.set_address(self.graph_nodes[i].get_address());
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
        
        let mut graph_nodes: Vec<GraphNodeDTO> = Vec::new();
        let graph_nodes_reader = decoded_struct.reborrow().get_nodes().unwrap();
        for graph_node_reader in graph_nodes_reader {
            graph_nodes.push(
                GraphNodeDTO::new(
                    String::from(graph_node_reader.get_address().unwrap())
                )
            );
        }
        
        let mut graph_edges = Vec::new();
        let graph_edges_reader = decoded_struct.reborrow().get_edges().unwrap();
        for graph_edge_reader in graph_edges_reader {
            graph_edges.push(
                GraphEdgeDTO::new(
                    String::from(graph_edge_reader.get_src_addr().unwrap()),
                    String::from(graph_edge_reader.get_dst_addr().unwrap())
                )
            )
        }

        NetworkGraphDTO {
            graph_nodes,
            graph_edges,
        }
    }
}