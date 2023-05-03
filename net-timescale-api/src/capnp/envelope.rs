use crate::capnp::envelope_capnp::envelope;

pub struct Envelope {
    envelope_type: String,
    data: Vec<u8>,
}

impl crate::Encoder for Envelope {
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

impl crate::Decoder for Envelope {
    fn decode(data: Vec<u8>) -> Self {    let message_reader = ::capnp::serialize_packed::read_message(
        data.as_slice(), //Think about using std::io::Cursor here
        ::capnp::message::ReaderOptions::new()).unwrap();

        let decoded_struct = message_reader.get_root::<envelope::Reader>().unwrap();

        Envelope { 
            envelope_type: String::from(decoded_struct.get_type().unwrap()),
            data: Vec::from(decoded_struct.get_data().unwrap()),
        }
    }
}