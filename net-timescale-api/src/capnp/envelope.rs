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