use crate::capnp::query_data_capnp::query_data;

pub fn encode ( frame_time: i64, src: String, dst : String, data: Vec<u8>) -> ::capnp::Result<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut message = ::capnp::message::Builder::new_default();

    let mut struct_to_encode = message.init_root::<query_data::Builder>();
    
    struct_to_encode.set_frame_time(frame_time);

    struct_to_encode.set_src_addr(&src);
    struct_to_encode.set_dst_addr(&dst);
    
    struct_to_encode.set_data(&data);

    match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

pub fn encode_to_buffer ( buffer: &mut Vec<u8>, frame_time: i64, src: String, dst : String, data: Vec<u8>) -> ::capnp::Result<()> {
    let mut message = ::capnp::message::Builder::new_default();

    let mut struct_to_encode = message.init_root::<query_data::Builder>();
    
    struct_to_encode.set_frame_time(frame_time);

    struct_to_encode.set_src_addr(&src);
    struct_to_encode.set_dst_addr(&dst);
    
    struct_to_encode.set_data(&data);

    ::capnp::serialize_packed::write_message(buffer, &message)
}

pub fn decode (serialized_data: &[u8]) -> (i64, String, String, Vec<u8>) {
    let message_reader = ::capnp::serialize_packed::read_message(
        serialized_data, //Think about using std::io::Cursor here
        ::capnp::message::ReaderOptions::new()).unwrap();

    let decoded_struct = message_reader.get_root::<query_data::Reader>().unwrap();

    return (
        decoded_struct.get_frame_time(),

        String::from(decoded_struct.get_src_addr().unwrap()),
        String::from(decoded_struct.get_dst_addr().unwrap()),

        Vec::from(decoded_struct.get_data().unwrap())
    );
}

pub fn decode_vec (serialized_data: Vec<u8>) -> (i64, String, String, Vec<u8>) {
    let message_reader = ::capnp::serialize_packed::read_message(
        serialized_data.as_slice(), //Think about using std::io::Cursor here
        ::capnp::message::ReaderOptions::new()).unwrap();

    let decoded_struct = message_reader.get_root::<query_data::Reader>().unwrap();

    return (
        decoded_struct.get_frame_time(),

        String::from(decoded_struct.get_src_addr().unwrap()),
        String::from(decoded_struct.get_dst_addr().unwrap()),

        Vec::from(decoded_struct.get_data().unwrap())
    );
}