#[cfg(feature = "capnp-endec")]
mod envelope_capnp {
    include!(concat!(env!("OUT_DIR"), "/envelope_capnp.rs"));
}
#[cfg(feature = "capnp-endec")] 
use envelope_capnp::envelope;


#[cfg(feature = "ion-endec")]
use ion_rs;
#[cfg(feature = "ion-endec")]
use ion_rs::IonWriter;
#[cfg(feature = "ion-endec")]
use ion_rs::IonReader;

use crate::ion_validator::IonSchemaValidator;


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

#[cfg(feature = "capnp-endec")] 
impl crate::encoder_api::Encoder for Envelope {
    fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        
        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<envelope::Builder>();
        
        struct_to_encode.set_type(&self.envelope_type);
        struct_to_encode.set_data(&self.data);
        
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }  
}

#[cfg(feature = "ion-endec")] 
impl crate::encoder_api::Encoder for Envelope {
    fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut binary_writer = binary_writer_builder.build(&mut buffer).unwrap();

        binary_writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");
        
        binary_writer.set_field_name("type");
        binary_writer.write_string(&self.envelope_type).unwrap();

        binary_writer.set_field_name("data");
        binary_writer.write_blob(&self.data).unwrap();

        binary_writer.step_out().unwrap();
        binary_writer.flush().unwrap();

        buffer
    }
}

#[cfg(feature = "capnp-endec")] 
impl crate::decoder_api::Decoder for Envelope {
    fn decode(data: Vec<u8>) -> Self {    
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(), //Think about using std::io::Cursor here
            ::capnp::message::ReaderOptions::new()).unwrap();

        let decoded_struct = message_reader.get_root::<envelope::Reader>().unwrap();

        Envelope { 
            envelope_type: String::from(decoded_struct.get_type().unwrap()),
            data: Vec::from(decoded_struct.get_data().unwrap()),
        }
    }
}

#[cfg(feature = "ion-endec")] 
impl crate::decoder_api::Decoder for Envelope {
    fn decode(data: Vec<u8>) -> Self {
        if !IonSchemaValidator::validate(&data, "envelope.isl") {
            todo!();
        }

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


#[cfg(feature = "ion-endec")]
#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    #[cfg(feature = "ion-schema-validation")]
    use std::path::Path;
    #[cfg(feature = "ion-schema-validation")]
    use ion_schema::authority::MapDocumentAuthority;
    #[cfg(feature = "ion-schema-validation")]
    use ion_schema::system::SchemaSystem;
    #[cfg(feature = "ion-schema-validation")]
    use ion_schema::authority::DocumentAuthority;
    #[cfg(feature = "ion-schema-validation")]
    use ion_schema::authority::FileSystemDocumentAuthority;
    #[cfg(feature = "ion-schema-validation")]
    use crate::ion_validator::IonSchemaValidator;


    use crate::decoder_api::Decoder;
    use crate::encoder_api::Encoder;

    use super::Envelope;

    #[test]
    fn reader_correctly_read_encoded_envelope() {
        let envelope = Envelope::new("ENVELOPE_TYPE".into(), "ENVELOP_DATA".into());
        
        let mut binary_user_reader = ReaderBuilder::new().build(envelope.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("type", binary_user_reader.field_name().unwrap());
        assert_eq!("ENVELOPE_TYPE", binary_user_reader.read_string().unwrap().text());
        
        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("data", binary_user_reader.field_name().unwrap());
        assert_eq!("ENVELOP_DATA".as_bytes(), binary_user_reader.read_blob().unwrap().as_slice());
    }

    #[test]
    fn endec_envelope() {
        let envelope = Envelope::new("ENVELOPE_TYPE".into(), "ENVELOP_DATA".into());
        assert_eq!(envelope, Envelope::decode(envelope.encode()));
    }

    #[cfg(feature = "ion-schema-validation")]
    #[test]
    fn ion_schema_validation() {
        let envelope = Envelope::new("ENVELOPE_TYPE".into(), "ENVELOP_DATA".into());

        let owned_elements = ion_schema::external::ion_rs::element::Element::read_all(envelope.encode()).expect("parsing failed unexpectedly");

        let document_authorities = [(
            "schema", 
            r#"
                schema_header::{}

                type::{
                    name: envelope,
                    type: struct,
                    fields: {
                        type: string,
                        data: blob
                    },
                }

                schema_footer::{}
            "#
        )];
        let mut schema_system = SchemaSystem::new(vec![Box::new(MapDocumentAuthority::new(document_authorities))]);

        let schema = schema_system.load_schema("envelope.isl").unwrap();
        let mut type_ref = schema.get_types();
        
        for owned_element in owned_elements {
            let type_definition = type_ref.next().unwrap();
            let validation_result = type_definition.validate(&owned_element);

            assert!(validation_result.is_ok())
        }
    }

    #[cfg(feature = "ion-schema-validation")]
    #[test]
    fn ion_schema_load() {
        let document_authorities: Vec<Box<dyn DocumentAuthority>> = vec![Box::new(
            FileSystemDocumentAuthority::new(Path::new(".isl")),
        )];
        let mut schema_system = SchemaSystem::new(document_authorities);
        assert!(schema_system.load_schema("envelope.isl").is_ok());
    }

    #[cfg(feature = "ion-schema-validation")]
    #[test]
    fn validator_test() {
        let envelope = Envelope::new("ENVELOPE_TYPE".into(), "ENVELOP_DATA".into());
        assert!(IonSchemaValidator::validate(&envelope.encode(), "envelope.isl"));
    }
}