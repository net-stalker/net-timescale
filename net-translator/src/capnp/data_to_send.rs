use crate::capnp::data_to_send_capnp::data_to_send;

pub fn form_data( buffer: &mut Vec<u8>, frame_time: i64, src: String, dst : String, json: Vec<u8>) -> ::capnp::Result<()> {

    let mut message = ::capnp::message::Builder::new_default();

    let mut packed_data = message.init_root::<data_to_send::Builder>();

    
    packed_data.set_requirement("add_packet".as_bytes());
    
    packed_data.set_frame_time(frame_time);
    packed_data.set_src_addr(&src);
    packed_data.set_dst_addr(&dst);
    
    packed_data.set_json(&json);

    //message.get_segments_for_output();

    ::capnp::serialize_packed::write_message(buffer, &message)
}