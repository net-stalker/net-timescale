#[cfg(feature = "capnp-endec")] 
pub mod graph_edge_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_edge_capnp.rs"));
}
#[cfg(feature = "capnp-endec")] 
pub mod graph_node_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_node_capnp.rs"));
}

#[cfg(feature = "capnp-endec")] 
pub mod network_graph_capnp {
    include!(concat!(env!("OUT_DIR"), "/network_graph_capnp.rs"));
}
#[cfg(feature = "capnp-endec")] 
use network_graph_capnp::network_graph;


#[cfg(feature = "ion-endec")]
use ion_rs;
#[cfg(feature = "ion-endec")]
use ion_rs::IonWriter;
#[cfg(feature = "ion-endec")]
use ion_rs::IonReader;
#[cfg(feature = "ion-endec")]
use ion_rs::StreamItem;
#[cfg(feature = "ion-endec")]
use ion_rs::element::writer::TextKind;

#[cfg(feature = "ion-endec")]
use net_proto_api::ion_validator::IonSchemaValidator;
#[cfg(feature = "ion-endec")]
use net_proto_api::load_schema;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;

use super::graph_edge::GraphEdgeDTO;
use super::graph_node::GraphNodeDTO;


#[derive(Debug, PartialEq, Eq)]
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

#[cfg(feature = "capnp-endec")] 
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

#[cfg(feature = "ion-endec")] 
impl Encoder for NetworkGraphDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        let mut writer = binary_writer_builder.build(buffer).unwrap();
        #[cfg(feature = "ion-text")]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("graph_nodes");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for graph_node in &self.graph_nodes {
            writer.step_in(ion_rs::IonType::Struct).expect("Error while entering an ion struct");
            
            writer.set_field_name("address");
            writer.write_string(graph_node.get_address()).unwrap();
            
            writer.step_out().unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("graph_edges");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for graph_edge in &self.graph_edges {
            writer.step_in(ion_rs::IonType::Struct).expect("Error while entering an ion struct");
            
            writer.set_field_name("src_addr");
            writer.write_string(graph_edge.get_src_addr()).unwrap();
    
            writer.set_field_name("dst_addr");
            writer.write_string(graph_edge.get_dst_addr()).unwrap();
            
            writer.step_out().unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}


#[cfg(feature = "capnp-endec")] 
impl Decoder for NetworkGraphDTO {
    fn decode(data: Vec<u8>) -> Self {
//TODO: Think about using std::io::Cursor here
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(),
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<network_graph::Reader>().unwrap();
        
        let mut graph_nodes = Vec::new();
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

#[cfg(feature = "ion-endec")] 
impl Decoder for NetworkGraphDTO {
    fn decode(data: Vec<u8>) -> Self {
        if IonSchemaValidator::validate(&data, load_schema!(".isl", "network_graph.isl").unwrap()).is_err() {
            todo!();
        }

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let mut graph_nodes = Vec::new();
        binary_user_reader.step_in().unwrap();
        while binary_user_reader.next().unwrap() != StreamItem::Nothing {
            binary_user_reader.step_in().unwrap();
            binary_user_reader.next().unwrap();
            let binding = binary_user_reader.read_string().unwrap();
            let address = String::from(binding.text());
            graph_nodes.push(GraphNodeDTO::new(address));
            binary_user_reader.step_out().unwrap();
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        let mut graph_edges = Vec::new();
        binary_user_reader.step_in().unwrap();
        while binary_user_reader.next().unwrap() != StreamItem::Nothing {
            binary_user_reader.step_in().unwrap();
            binary_user_reader.next().unwrap();
            let binding = binary_user_reader.read_string().unwrap();
            let src_addr = String::from(binding.text());
            binary_user_reader.next().unwrap();
            let binding = binary_user_reader.read_string().unwrap();
            let dst_addr = String::from(binding.text());
            graph_edges.push(GraphEdgeDTO::new(src_addr, dst_addr));
            binary_user_reader.step_out().unwrap();
        }
        binary_user_reader.step_out().unwrap();

        NetworkGraphDTO {
            graph_nodes,
            graph_edges,
        }
    }
}

#[cfg(feature = "ion-endec")]
#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;
    use net_proto_api::ion_validator::IonSchemaValidator;
    use net_proto_api::generate_schema;
    use net_proto_api::load_schema;

    use crate::api::graph_edge::GraphEdgeDTO;
    use crate::api::graph_node::GraphNodeDTO;
    use crate::api::network_graph::NetworkGraphDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_edge() {
        const FIRST_NODE_ADDRESS: &str = "0.0.0.0:0000";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ADDRESS.to_owned());
        const SECOND_NODE_ADDRESS: &str = "0.0.0.0:5656";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ADDRESS.to_owned());

        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ADDR.to_owned(), DST_ADDR.to_owned());

        let network_graph = NetworkGraphDTO::new(
            vec![first_graph_node, second_graph_node],
            vec![graph_edge],
        );

        let mut binary_user_reader = ReaderBuilder::new().build(network_graph.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("graph_nodes", binary_user_reader.field_name().unwrap());
        
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("address", binary_user_reader.field_name().unwrap());
        assert_eq!(FIRST_NODE_ADDRESS,  binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("address", binary_user_reader.field_name().unwrap());
        assert_eq!(SECOND_NODE_ADDRESS,  binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("graph_edges", binary_user_reader.field_name().unwrap());

        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("src_addr", binary_user_reader.field_name().unwrap());
        assert_eq!(SRC_ADDR,  binary_user_reader.read_string().unwrap().text());
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("dst_addr", binary_user_reader.field_name().unwrap());
        assert_eq!(DST_ADDR,  binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();
        
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_graph() {
        const FIRST_NODE_ADDRESS: &str = "0.0.0.0:0000";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ADDRESS.to_owned());
        const SECOND_NODE_ADDRESS: &str = "0.0.0.0:5656";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ADDRESS.to_owned());

        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ADDR.to_owned(), DST_ADDR.to_owned());

        let network_graph = NetworkGraphDTO::new(
            vec![first_graph_node, second_graph_node],
            vec![graph_edge],
        );

        assert_eq!(network_graph, NetworkGraphDTO::decode(network_graph.encode()));
    }

    #[test]
    fn ion_schema_validation() {
        const FIRST_NODE_ADDRESS: &str = "0.0.0.0:0000";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ADDRESS.to_owned());
        const SECOND_NODE_ADDRESS: &str = "0.0.0.0:5656";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ADDRESS.to_owned());

        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ADDR.to_owned(), DST_ADDR.to_owned());

        let network_graph = NetworkGraphDTO::new(
            vec![first_graph_node, second_graph_node],
            vec![graph_edge],
        );

        let schema = generate_schema!(
            r#"
                schema_header::{}

                type::{
                    name: network_graph,
                    type: struct,
                    fields: {
                        graph_nodes: {
                            type: list,
                            element: {
                                type: struct,
                                fields: {
                                    address: string,
                                },
                            },
                        },
                        graph_edges: {
                            type: list,
                            element: {
                                type: struct,
                                fields: {
                                    src_addr: string,
                                    dst_addr: string
                                },
                            },
                        },
                    },
                }

                schema_footer::{}
            "#
        );
        assert!(schema.is_ok());

        assert!(IonSchemaValidator::validate(&network_graph.encode(), schema.unwrap()).is_ok());
    }

    #[test]
    fn schema_load_test() {
        assert!(load_schema!(".isl", "network_graph.isl").is_ok())
    }

    #[test]
    fn validator_test() {
        const FIRST_NODE_ADDRESS: &str = "0.0.0.0:0000";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ADDRESS.to_owned());
        const SECOND_NODE_ADDRESS: &str = "0.0.0.0:5656";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ADDRESS.to_owned());

        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ADDR.to_owned(), DST_ADDR.to_owned());

        let network_graph = NetworkGraphDTO::new(
            vec![first_graph_node, second_graph_node],
            vec![graph_edge],
        );

        let schema = load_schema!(".isl", "network_graph.isl");
        assert!(schema.is_ok());

        assert!(IonSchemaValidator::validate(&network_graph.encode(), schema.unwrap()).is_ok());
    }
}