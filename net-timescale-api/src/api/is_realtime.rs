use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
pub struct IsRealtimeDTO {
    is_realtime: bool,
}

impl IsRealtimeDTO {
    pub fn new (is_realtime: bool) -> Self {
        IsRealtimeDTO {
            is_realtime,
        }
    }

    pub fn is_realtime (&self) -> bool {
        self.is_realtime
    }
}

impl Encoder for IsRealtimeDTO {
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
        
        writer.set_field_name("is_realtime");
        writer.write_bool(self.is_realtime).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for IsRealtimeDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        
        binary_user_reader.next().unwrap();
        let is_realtime = binary_user_reader.read_bool().unwrap();

        IsRealtimeDTO::new(
            is_realtime,
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

    use crate::api::is_realtime::IsRealtimeDTO;

    #[test]
    fn reader_correctly_read_encoded_date_cut() {
        const IS_REALTIME: bool = true;
        let is_realtime = IsRealtimeDTO::new(IS_REALTIME);
        
        let mut binary_user_reader = ReaderBuilder::new().build(is_realtime.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("is_realtime", binary_user_reader.field_name().unwrap());
        assert_eq!(IS_REALTIME, binary_user_reader.read_bool().unwrap());
    }

    #[test]
    fn endec_date_cut() {
        const IS_REALTIME: bool = true;
        let is_realtime = IsRealtimeDTO::new(IS_REALTIME);
        assert_eq!(is_realtime, IsRealtimeDTO::decode(&is_realtime.encode()));
    }
}