pub mod time_interval_capnp {
    include!(concat!(env!("OUT_DIR"), "/time_interval_capnp.rs"));
}
use time_interval_capnp::time_interval;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug)]
pub struct TimeIntervalDTO {
    start_date_time: i64,
    end_date_time: i64,
}

impl TimeIntervalDTO {
    pub fn new (start_date_time: i64, end_date_time: i64) -> Self {
        TimeIntervalDTO {
            start_date_time,
            end_date_time,
        }
    }

    pub fn get_start_date_time (&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time (&self) -> i64 {
        self.end_date_time
    }
}

impl Encoder for TimeIntervalDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<time_interval::Builder>();
        
        struct_to_encode.set_start_date_time(self.start_date_time);
        struct_to_encode.set_end_date_time(self.end_date_time);
    
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
            start_date_time: decoded_struct.get_start_date_time(),
            end_date_time: decoded_struct.get_end_date_time(), 
            
        }
    }
}