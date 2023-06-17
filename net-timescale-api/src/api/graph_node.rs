#[cfg(feature = "capnp-endec")] 
pub mod graph_node_capnp {
    include!(concat!(env!("OUT_DIR"), "/graph_node_capnp.rs"));
}
#[cfg(feature = "capnp-endec")] 
use graph_node_capnp::graph_node;


#[cfg(feature = "ion-endec")]
use ion_rs;
#[cfg(feature = "ion-endec")]
use ion_rs::IonWriter;
#[cfg(feature = "ion-endec")]
use ion_rs::IonReader;


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

#[cfg(feature = "capnp-endec")] 
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

#[cfg(feature = "ion-endec")] 
impl Encoder for GraphNodeDTO {
    fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut binary_writer = binary_writer_builder.build(&mut buffer).unwrap();

        binary_writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");
        
        binary_writer.set_field_name("address");
        binary_writer.write_string(&self.address).unwrap();

        binary_writer.step_out().unwrap();
        binary_writer.flush().unwrap();

        buffer
    }
}

#[cfg(feature = "capnp-endec")] 
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

#[cfg(feature = "ion-endec")] 
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


#[cfg(feature = "ion-endec")]
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