use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::StreamItem;
use ion_rs::element::writer::TextKind;

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
    pub fn new ( graph_nodes: &[GraphNodeDTO], graph_edges: &[GraphEdgeDTO]) -> Self {
        NetworkGraphDTO {
            graph_nodes: graph_nodes.to_vec(),
            graph_edges: graph_edges.to_vec(),
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
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        #[cfg(feature = "ion-text")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("graph_nodes");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for graph_node in &self.graph_nodes {
            writer.step_in(ion_rs::IonType::Struct).expect("Error while entering an ion struct");
            
            writer.set_field_name("id");
            writer.write_string(graph_node.get_id()).unwrap();
            
            writer.set_field_name("agent_id");
            writer.write_string(graph_node.get_agent_id()).unwrap();

            writer.step_out().unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("graph_edges");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for graph_edge in &self.graph_edges {
            writer.step_in(ion_rs::IonType::Struct).expect("Error while entering an ion struct");
            
            writer.set_field_name("src_id");
            writer.write_string(graph_edge.get_src_id()).unwrap();
    
            writer.set_field_name("dst_id");
            writer.write_string(graph_edge.get_dst_id()).unwrap();
            
            writer.step_out().unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkGraphDTO {
    fn decode(data: &[u8]) -> Self {

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
            let id = binding.text();
            let binding = binary_user_reader.read_string().unwrap();
            let agent_id = binding.text();
            graph_nodes.push(GraphNodeDTO::new(id, agent_id));
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
            let src_id = binding.text();
            binary_user_reader.next().unwrap();
            let binding = binary_user_reader.read_string().unwrap();
            let dst_id = binding.text();
            graph_edges.push(GraphEdgeDTO::new(src_id, dst_id));
            binary_user_reader.step_out().unwrap();
        }
        binary_user_reader.step_out().unwrap();

        NetworkGraphDTO {
            graph_nodes,
            graph_edges,
        }
    }
}

#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;

    use crate::api::network_graph::graph_edge::GraphEdgeDTO;
    use crate::api::network_graph::graph_node::GraphNodeDTO;
    use crate::api::network_graph::network_graph::NetworkGraphDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_edge() {
        const FIRST_NODE_ID: &str = "0.0.0.0:0000";
        const FIRST_NODE_AGENT_ID: &str = "some first node agent id";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ID, FIRST_NODE_AGENT_ID);
        const SECOND_NODE_ID: &str = "0.0.0.0:5656";
        const SECOND_NODE_AGENT_ID: &str = "some second node agent id";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ID, SECOND_NODE_AGENT_ID);

        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ID, DST_ID);

        let network_graph = NetworkGraphDTO::new(
            &[first_graph_node, second_graph_node],
            &[graph_edge],
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
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(FIRST_NODE_ID,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("agent_id", binary_user_reader.field_name().unwrap());
        assert_eq!(FIRST_NODE_AGENT_ID,  binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(SECOND_NODE_ID,  binary_user_reader.read_string().unwrap().text());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("agent_id", binary_user_reader.field_name().unwrap());
        assert_eq!(SECOND_NODE_AGENT_ID,  binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("graph_edges", binary_user_reader.field_name().unwrap());

        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("src_id", binary_user_reader.field_name().unwrap());
        assert_eq!(SRC_ID,  binary_user_reader.read_string().unwrap().text());
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("dst_id", binary_user_reader.field_name().unwrap());
        assert_eq!(DST_ID,  binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();
        
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    #[ignore]
    fn endec_network_graph() {
        const FIRST_NODE_ID: &str = "0.0.0.0:0000";
        const FIRST_NODE_AGENT_ID: &str = "some first node agent id"; 
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ID, FIRST_NODE_AGENT_ID);
        const SECOND_NODE_ID: &str = "0.0.0.0:5656";
        const SECOND_NODE_AGENT_ID: &str = "some second node agent id";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ID, SECOND_NODE_AGENT_ID);

        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ID, DST_ID);

        let network_graph = NetworkGraphDTO::new(
            &[first_graph_node, second_graph_node],
            &[graph_edge],
        );

        assert_eq!(network_graph, NetworkGraphDTO::decode(&network_graph.encode()));
    }
}