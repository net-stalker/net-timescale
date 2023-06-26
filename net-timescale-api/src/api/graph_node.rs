use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
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

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        let mut writer = binary_writer_builder.build(buffer).unwrap();
        #[cfg(feature = "ion-text")]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("address");
        writer.write_string(&self.address).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for GraphNodeDTO {
    fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let address = String::from(binding.text());

        GraphNodeDTO {
            address,
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

    use crate::api::graph_node::GraphNodeDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_node() {
        const ADDRESS: &str = "0.0.0.0:0000";
        let graph_node = GraphNodeDTO::new(ADDRESS.to_owned());
        let mut binary_user_reader = ReaderBuilder::new().build(graph_node.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("address", binary_user_reader.field_name().unwrap());
        assert_eq!(ADDRESS,  binary_user_reader.read_string().unwrap().text());
    }

    #[test]
    fn endec_graph_node() {
        const ADDRESS: &str = "0.0.0.0:0000";
        let graph_node = GraphNodeDTO::new(ADDRESS.to_owned());
        assert_eq!(graph_node, GraphNodeDTO::decode(graph_node.encode()));
    }

    #[cfg(feature = "ion-schema-validation")]
    #[test]
    fn ion_schema_validation() {
        //TODO: Write schema validation tests (should be done in #85zta68kj task)
    }
}