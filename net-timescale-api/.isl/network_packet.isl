schema_header::{}

type::{
    name: network_packet,
    type: struct,
    fields: {
        frame_time: int,
        src_addr: string,
        dst_addr: string,
        network_packet_data: blob,
    },
}

schema_footer::{}