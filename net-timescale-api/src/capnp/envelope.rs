use crate::capnp::envelope_capnp::envelope;

pub fn encode( envelope_type: String, data: Vec<u8>) -> ::capnp::Result<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut message = ::capnp::message::Builder::new_default();

    let mut struct_to_encode = message.init_root::<envelope::Builder>();
    
    struct_to_encode.set_type(&envelope_type);

    struct_to_encode.set_data(&data);

    match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

pub fn encode_to_buffer( buffer: &mut Vec<u8>, envelope_type: String, data: Vec<u8>) -> ::capnp::Result<()> {
    let mut message = ::capnp::message::Builder::new_default();

    let mut struct_to_encode = message.init_root::<envelope::Builder>();
    
    struct_to_encode.set_type(&envelope_type);

    struct_to_encode.set_data(&data);

    ::capnp::serialize_packed::write_message(buffer, &message)
}

pub fn encode_query_data( frame_time: i64, src: String, dst : String, data: Vec<u8> ) -> ::capnp::Result<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut message = ::capnp::message::Builder::new_default();

    let mut struct_to_encode = message.init_root::<envelope::Builder>();
    
    struct_to_encode.set_type("add_packet");

    struct_to_encode.set_data(&(crate::capnp::query_data::encode(frame_time, src, dst, data).unwrap()));

    match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

pub fn encode_query_data_to_buffer( buffer: &mut Vec<u8>, frame_time: i64, src: String, dst : String, data: Vec<u8> ) -> ::capnp::Result<()> {
    let mut message = ::capnp::message::Builder::new_default();

    let mut struct_to_encode = message.init_root::<envelope::Builder>();
    
    struct_to_encode.set_type("add_packet");

    struct_to_encode.set_data(&(crate::capnp::query_data::encode(frame_time, src, dst, data).unwrap()));

    ::capnp::serialize_packed::write_message(buffer, &message)
}

pub fn decode (serialized_data: &[u8]) -> (String, Vec<u8>) {
    let message_reader = ::capnp::serialize_packed::read_message(
        serialized_data, //Think about using std::io::Cursor here
        ::capnp::message::ReaderOptions::new()).unwrap();

    let decoded_struct = message_reader.get_root::<envelope::Reader>().unwrap();

    return (
        String::from(decoded_struct.get_type().unwrap()),

        Vec::from(decoded_struct.get_data().unwrap())
    );
}

pub fn decode_vec (serialized_data: Vec<u8>) -> (String, Vec<u8>) {
    let message_reader = ::capnp::serialize_packed::read_message(
        serialized_data.as_slice(), //Think about using std::io::Cursor here
        ::capnp::message::ReaderOptions::new()).unwrap();

    let decoded_struct = message_reader.get_root::<envelope::Reader>().unwrap();

    return (
        String::from(decoded_struct.get_type().unwrap()),

        Vec::from(decoded_struct.get_data().unwrap())
    );
}

pub fn decode_query_data (serialized_data: &[u8]) -> (i64, String, String, Vec<u8>) {
    let message_reader = ::capnp::serialize_packed::read_message(
        serialized_data, //Think about using std::io::Cursor here
        ::capnp::message::ReaderOptions::new()).unwrap();

    let decoded_struct = message_reader.get_root::<envelope::Reader>().unwrap();

    let query_data = decoded_struct.get_data().unwrap();

    crate::capnp::query_data::decode(query_data)
}

pub fn decode_query_data_vec (serialized_data: Vec<u8>) -> (i64, String, String, Vec<u8>) {
    let message_reader = ::capnp::serialize_packed::read_message(
        serialized_data.as_slice(), //Think about using std::io::Cursor here
        ::capnp::message::ReaderOptions::new()).unwrap();

    let decoded_struct = message_reader.get_root::<envelope::Reader>().unwrap();

    let query_data = decoded_struct.get_data().unwrap();

    crate::capnp::query_data::decode(query_data)
}