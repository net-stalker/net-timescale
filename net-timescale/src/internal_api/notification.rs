use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
pub struct NotificationDTO {
    payload: String,
    channel: String,
}

impl NotificationDTO {
    pub fn new (payload: &str, channel: &str) -> Self {
        NotificationDTO {
            payload: payload.to_string(),
            channel: channel.to_string(),
        }
    }

    pub fn get_payload (&self) -> &str {
        self.payload.as_str()
    }

    pub fn get_channel (&self) -> &str {
        self.channel.as_str()
    }
}

impl Encoder for NotificationDTO {
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

        writer.set_field_name("payload");
        writer.write_string(self.payload.as_str()).unwrap();

        writer.set_field_name("channel");
        writer.write_string(self.channel.as_str()).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NotificationDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let payload = binary_user_reader.read_string().unwrap();
        let payload = payload.as_ref();

        binary_user_reader.next().unwrap();
        let channel = binary_user_reader.read_str().unwrap();

        NotificationDTO::new(payload, channel)
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

    use crate::internal_api::notification::NotificationDTO;

    #[test]
    fn reader_correctly_read_encoded_date_cut() {
        const PAYLOAD: &str = "some important info";
        const CHANNEL: &str = "from a very important channel";

        let notification = NotificationDTO::new(PAYLOAD, CHANNEL);

        let mut binary_user_reader = ReaderBuilder::new().build(notification.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("payload", binary_user_reader.field_name().unwrap());
        assert_eq!(PAYLOAD, binary_user_reader.read_str().unwrap());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("channel", binary_user_reader.field_name().unwrap());
        assert_eq!(CHANNEL, binary_user_reader.read_str().unwrap());
    }

    #[test]
    fn endec_date_cut() {
        const PAYLOAD: &str = "some important info";
        const CHANNEL: &str = "from a very important channel";

        let notification = NotificationDTO::new(PAYLOAD, CHANNEL);
        assert_eq!(notification, NotificationDTO::decode(&notification.encode()));
    }
}