use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;


#[derive(Debug, PartialEq, Eq)]
pub struct Envelope {
    envelope_type: String,
    data: Vec<u8>,
}

impl Envelope{
    pub fn new( envelope_type: String, data: Vec<u8>) -> Self {
        Envelope { 
            envelope_type, 
            data
        }
    }

    pub fn get_type (&self) -> &str {
        &self.envelope_type
    }

    pub fn get_data (&self) -> &[u8] {
        &self.data
    }
}

impl crate::encoder_api::Encoder for Envelope {
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
        
        writer.set_field_name("type");
        writer.write_string(&self.envelope_type).unwrap();

        writer.set_field_name("data");
        writer.write_blob(&self.data).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl crate::decoder_api::Decoder for Envelope {
    fn decode(data: Vec<u8>) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let envelope_type = String::from(binding.text());
        
        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let data = binding.as_slice().to_owned();

        Envelope {
            envelope_type,
            data,
        }
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;
    
    use crate::decoder_api::Decoder;
    use crate::encoder_api::Encoder;

    use super::Envelope;

    #[test]
    fn reader_correctly_read_encoded_envelope() {
        let envelope = Envelope::new("ENVELOPE_TYPE".into(), "ENVELOPE_DATA".into());
        
        let mut binary_user_reader = ReaderBuilder::new().build(envelope.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("type", binary_user_reader.field_name().unwrap());
        assert_eq!("ENVELOPE_TYPE", binary_user_reader.read_string().unwrap().text());
        
        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("data", binary_user_reader.field_name().unwrap());
        assert_eq!("ENVELOPE_DATA".as_bytes(), binary_user_reader.read_blob().unwrap().as_slice());
    }

    #[test]
    #[ignore]
    fn endec_envelope() {
        let envelope = Envelope::new("ENVELOPE_TYPE".into(), "ENVELOP_DATA".into());
        assert_eq!(envelope, Envelope::decode(envelope.encode()));
    }
}