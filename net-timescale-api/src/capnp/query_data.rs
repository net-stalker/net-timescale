use capnp::data;

use crate::capnp::query_data_capnp::query_data;

pub fn encode_to_buffer( buffer: &mut Vec<u8>, frame_time: i64, src: String, dst : String, data: Vec<u8>) -> ::capnp::Result<()> {
    let mut message = ::capnp::message::Builder::new_default();

    let mut packed_data = message.init_root::<query_data::Builder>();
    
    packed_data.set_frame_time(frame_time);

    packed_data.set_src_addr(&src);
    packed_data.set_dst_addr(&dst);
    
    packed_data.set_data(&data);

    ::capnp::serialize_packed::write_message(buffer, &message)
}