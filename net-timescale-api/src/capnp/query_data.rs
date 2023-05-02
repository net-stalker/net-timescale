use crate::capnp::query_data_capnp::query_data;

pub fn encode( frame_time: i64, src: String, dst : String, data: Vec<u8>) -> ::capnp::Result<Vec<u8>> {
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

pub fn encode_to_buffer( buffer: &mut Vec<u8>, frame_time: i64, src: String, dst : String, data: Vec<u8>) -> ::capnp::Result<()> {
    let mut message = ::capnp::message::Builder::new_default();

    let mut struct_to_encode = message.init_root::<query_data::Builder>();
    
    struct_to_encode.set_frame_time(frame_time);

    struct_to_encode.set_src_addr(&src);
    struct_to_encode.set_dst_addr(&dst);
    
    struct_to_encode.set_data(&data);

    ::capnp::serialize_packed::write_message(buffer, &message)
}