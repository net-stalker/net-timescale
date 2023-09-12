use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GraphNodeDTO {
    id: String,
    aggregator: String,
}


impl GraphNodeDTO {
    pub fn new (id: &str, aggregator: &str) -> Self {
        GraphNodeDTO {
            id: id.into(),
            aggregator: aggregator.into(),
        }
    }

    pub fn get_id (&self) -> &str {
        &self.id
    }

    pub fn get_aggregator (&self) -> &str {
        &self.aggregator
    }
}

impl Encoder for GraphNodeDTO {
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
        
        writer.set_field_name("id");
        writer.write_string(&self.id).unwrap();

        writer.set_field_name("aggregator");
        writer.write_string(&self.aggregator).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for GraphNodeDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let id = binding.text();

        let binding = binary_user_reader.read_string().unwrap();
        let agent_id = binding.text();

        GraphNodeDTO::new(
            id,
            agent_id
        )
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

    use crate::api::network_graph::graph_node::GraphNodeDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_node() {
        const ID: &str = "0.0.0.0:0000";
        const AGGREGATOR: &str = "aggregator";

        let graph_node = GraphNodeDTO::new(ID, AGGREGATOR);
        let mut binary_user_reader = ReaderBuilder::new().build(graph_node.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("aggregator", binary_user_reader.field_name().unwrap());
        assert_eq!(AGGREGATOR,  binary_user_reader.read_string().unwrap().text());
    }

    #[test]
    #[ignore]
    fn endec_graph_node() {
        const ID: &str = "0.0.0.0:0000";
        const AGGREGATOR: &str = "aggregator";

        let graph_node = GraphNodeDTO::new(ID, AGGREGATOR);
        assert_eq!(graph_node, GraphNodeDTO::decode(&graph_node.encode()));
    }
}