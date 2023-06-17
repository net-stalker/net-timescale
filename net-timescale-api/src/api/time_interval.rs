#[cfg(feature = "capnp-endec")] 
pub mod time_interval_capnp {
    include!(concat!(env!("OUT_DIR"), "/time_interval_capnp.rs"));
}
#[cfg(feature = "capnp-endec")] 
use time_interval_capnp::time_interval;


#[cfg(feature = "ion-endec")]
use ion_rs;
#[cfg(feature = "ion-endec")]
use ion_rs::IonWriter;
#[cfg(feature = "ion-endec")]
use ion_rs::IonReader;


use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
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

#[cfg(feature = "capnp-endec")] 
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

#[cfg(feature = "ion-endec")] 
impl Encoder for TimeIntervalDTO {
    fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut binary_writer = binary_writer_builder.build(&mut buffer).unwrap();

        binary_writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");
        
        binary_writer.set_field_name("start_date_time");
        binary_writer.write_i64(self.start_date_time).unwrap();

        binary_writer.set_field_name("end_date_time");
        binary_writer.write_i64(self.end_date_time).unwrap();

        binary_writer.step_out().unwrap();
        binary_writer.flush().unwrap();

        buffer
    }
}

#[cfg(feature = "capnp-endec")] 
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

#[cfg(feature = "ion-endec")] 
impl Decoder for TimeIntervalDTO {
    fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        TimeIntervalDTO {
            start_date_time,
            end_date_time,
        }
    }
}


#[cfg(feature = "ion-endec")]
#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;

    use crate::api::time_interval::TimeIntervalDTO;

    #[test]
    fn reader_correctly_read_encoded_time_interval() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        let time_interval = TimeIntervalDTO::new(START_DATE_TIME, END_DATE_TIME);
        
        let mut binary_user_reader = ReaderBuilder::new().build(time_interval.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("start_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(START_DATE_TIME, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("end_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(END_DATE_TIME,  binary_user_reader.read_i64().unwrap());
    }

    #[test]
    fn endec_time_interval() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        let time_interval = TimeIntervalDTO::new(START_DATE_TIME, END_DATE_TIME);
        assert_eq!(time_interval, TimeIntervalDTO::decode(time_interval.encode()));
    }

    #[cfg(feature = "ion-schema-validation")]
    #[test]
    fn ion_schema_validation() {
        //TODO: Write schema validation tests (should be done in #85zta68kj task)
    }
}