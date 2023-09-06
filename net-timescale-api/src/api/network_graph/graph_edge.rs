use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GraphEdgeDTO {
    src_id: String,
    dst_id: String,
}

impl GraphEdgeDTO {
    pub fn new ( src_id: &str, dst_id: &str) -> Self {
        GraphEdgeDTO {
            src_id: src_id.into(), 
            dst_id: dst_id.into(), 
        }
    }

    pub fn get_src_id (&self) -> &str {
        &self.src_id
    }

    pub fn get_dst_id (&self) -> &str {
        &self.dst_id
    }
}

impl Encoder for GraphEdgeDTO {
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
        
        writer.set_field_name("src_id");
        writer.write_string(&self.src_id).unwrap();

        writer.set_field_name("dst_id");
        writer.write_string(&self.dst_id).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for GraphEdgeDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let src_id = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let dst_id = binding.text();

        GraphEdgeDTO::new(
            src_id,
            dst_id,
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

    use crate::api::network_graph::graph_edge::GraphEdgeDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_edge() {
        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ID, DST_ID);
        let mut binary_user_reader = ReaderBuilder::new().build(graph_edge.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("src_id", binary_user_reader.field_name().unwrap());
        assert_eq!(SRC_ID,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("dst_id", binary_user_reader.field_name().unwrap());
        assert_eq!(DST_ID,  binary_user_reader.read_string().unwrap().text());
    }

    #[test]
    #[ignore]
    fn endec_graph_edge() {
        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let graph_edge = GraphEdgeDTO::new(SRC_ID, DST_ID);
        assert_eq!(graph_edge, GraphEdgeDTO::decode(&graph_edge.encode()));
    }
}