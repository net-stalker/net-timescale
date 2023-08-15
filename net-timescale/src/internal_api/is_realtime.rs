use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
pub struct RealtimeRequestDTO {
    connection_id: i64,
}

impl RealtimeRequestDTO {
    pub fn new (connection_id: i64) -> Self {
        RealtimeRequestDTO {
            connection_id,
        }
    }

    pub fn get_connection_id (&self) -> i64 {
        self.connection_id
    }
}

impl Encoder for RealtimeRequestDTO {
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
        
        writer.set_field_name("connection_id");
        writer.write_i64(self.get_connection_id()).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for RealtimeRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        
        binary_user_reader.next().unwrap();
        let connection_id = binary_user_reader.read_i64().unwrap();

        RealtimeRequestDTO::new(
            connection_id,
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

    use crate::internal_api::is_realtime::RealtimeRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_date_cut() {
        const CONNECTION_ID: i64 = 228;
        let real_req = RealtimeRequestDTO::new(CONNECTION_ID);
        
        let mut binary_user_reader = ReaderBuilder::new().build(real_req.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("connection_id", binary_user_reader.field_name().unwrap());
        assert_eq!(CONNECTION_ID, binary_user_reader.read_i64().unwrap());
    }

    #[test]
    fn endec_date_cut() {
        const CONNECTION_ID: i64 = 228;
        let real_req = RealtimeRequestDTO::new(CONNECTION_ID);
        assert_eq!(real_req, RealtimeRequestDTO::decode(&real_req.encode()));
    }
}