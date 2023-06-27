use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::ion_validator::IonSchemaValidator;
use net_proto_api::load_schema;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
pub struct TimeIntervalDTO {
    start_date_time: i64,
    end_date_time: i64,
    is_realtime: bool,
}

impl TimeIntervalDTO {
    pub fn new (start_date_time: i64, end_date_time: i64, is_realtime: bool,) -> Self {
        TimeIntervalDTO {
            start_date_time,
            end_date_time,
            is_realtime
        }
    }

    pub fn get_start_date_time (&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time (&self) -> i64 {
        self.end_date_time
    }

    pub fn is_realtime (&self) -> bool {
        self.is_realtime
    }
}

impl Encoder for TimeIntervalDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        let mut writer = binary_writer_builder.build(buffer).unwrap();
        #[cfg(feature = "ion-text")]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("start_date_time");
        writer.write_i64(self.start_date_time).unwrap();

        writer.set_field_name("end_date_time");
        writer.write_i64(self.end_date_time).unwrap();

        writer.set_field_name("is_realtime");
        writer.write_bool(self.is_realtime).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for TimeIntervalDTO {
    fn decode(data: Vec<u8>) -> Self {
        if IonSchemaValidator::validate(&data, load_schema!(".isl", "time_interval.isl").unwrap()).is_err() {
            todo!();
        }

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let is_realtime = binary_user_reader.read_bool().unwrap();

        TimeIntervalDTO {
            start_date_time,
            end_date_time,
            is_realtime
        }
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;
    use net_proto_api::ion_validator::IonSchemaValidator;
    use net_proto_api::generate_schema;
    use net_proto_api::load_schema;

    use crate::api::time_interval::TimeIntervalDTO;

    #[test]
    fn reader_correctly_read_encoded_time_interval() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const IS_REALTIME: bool = true;
        let time_interval = TimeIntervalDTO::new(START_DATE_TIME, END_DATE_TIME, IS_REALTIME);
        
        let mut binary_user_reader = ReaderBuilder::new().build(time_interval.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("start_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(START_DATE_TIME, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("end_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(END_DATE_TIME,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("is_realtime", binary_user_reader.field_name().unwrap());
        assert_eq!(IS_REALTIME,  binary_user_reader.read_bool().unwrap());
    }

    #[test]
    fn endec_time_interval() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const IS_REALTIME: bool = true;
        let time_interval = TimeIntervalDTO::new(START_DATE_TIME, END_DATE_TIME, IS_REALTIME);
        assert_eq!(time_interval, TimeIntervalDTO::decode(time_interval.encode()));
    }

    #[test]
    fn ion_schema_validation() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const IS_REALTIME: bool = true;
        let time_interval = TimeIntervalDTO::new(START_DATE_TIME, END_DATE_TIME, IS_REALTIME);

        let schema = generate_schema!(
            r#"
                schema_header::{}

                type::{
                    name: time_interval,
                    type: struct,
                    fields: {
                        start_date_time: int,
                        end_date_time: int,
                        is_realtime: bool,
                    },
                }

                schema_footer::{}
            "#
        );
        assert!(schema.is_ok());

        assert!(IonSchemaValidator::validate(&time_interval.encode(), schema.unwrap()).is_ok());
    }

    #[test]
    fn schema_load_test() {
        assert!(load_schema!(".isl", "time_interval.isl").is_ok())
    }

    #[test]
    fn validator_test() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const IS_REALTIME: bool = true;
        let time_interval = TimeIntervalDTO::new(START_DATE_TIME, END_DATE_TIME, IS_REALTIME);

        let schema = load_schema!(".isl", "time_interval.isl");
        assert!(schema.is_ok());

        assert!(IonSchemaValidator::validate(&time_interval.encode(), schema.unwrap()).is_ok());
    }
}