pub mod time_interval_capnp {
    include!(concat!(env!("OUT_DIR"), "/time_interval_capnp.rs"));
}
use time_interval_capnp::time_interval;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug)]
pub struct TimeIntervalDTO {
    frame_time: i64,

    interval_start: i64,
    interval_end: i64,
}

impl TimeIntervalDTO {
    pub fn new (frame_time: i64, interval_start: i64, interval_end: i64) -> Self {
        TimeIntervalDTO { 
            frame_time,
            interval_start,
            interval_end,
        }
    }

    pub fn get_frame_time (&self) -> i64 {
        self.frame_time
    }

    pub fn get_interval_start (&self) -> i64 {
        self.interval_start
    }

    pub fn get_interval_end (&self) -> i64 {
        self.interval_end
    }
}

impl Encoder for TimeIntervalDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<time_interval::Builder>();
        
        struct_to_encode.set_frame_time(self.frame_time);
        struct_to_encode.set_interval_start(self.interval_start);
        struct_to_encode.set_interval_end(self.interval_end);
    
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }
}

impl Decoder for TimeIntervalDTO {
    fn decode(data: Vec<u8>) -> Self {
//TODO: Think about using std::io::Cursor here
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(),
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<time_interval::Reader>().unwrap();

        TimeIntervalDTO { 
            frame_time: decoded_struct.get_frame_time(),
            interval_start: decoded_struct.get_interval_start(),
            interval_end: decoded_struct.get_interval_end(), 
            
        }
    }
}