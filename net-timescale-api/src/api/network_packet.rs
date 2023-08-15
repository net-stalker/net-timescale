use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
pub struct NetworkPacketDTO {
    frame_time: i64,

    src_addr: String,
    dst_addr: String,

    network_packet_data: Vec<u8>,
}

impl NetworkPacketDTO {
    pub fn new ( frame_time: i64, src_addr: &str, dst_addr: &str, network_packet_data: &[u8]) -> Self {
        NetworkPacketDTO { 
            frame_time, 
            src_addr: src_addr.into(), 
            dst_addr: dst_addr.into(), 
            network_packet_data: network_packet_data.into()
        }
    }

    pub fn get_frame_time (&self) -> i64 {
        self.frame_time
    }

    pub fn get_src_addr (&self) -> &str {
        &self.src_addr
    }

    pub fn get_dst_addr (&self) -> &str {
        &self.dst_addr
    }

    pub fn get_network_packet_data (&self) -> &[u8] {
        &self.network_packet_data
    }
}

impl Encoder for NetworkPacketDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        #[cfg(feature = "ion-text")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("frame_time");
        writer.write_i64(self.frame_time).unwrap();

        writer.set_field_name("src_addr");
        writer.write_string(&self.src_addr).unwrap();

        writer.set_field_name("dst_addr");
        writer.write_string(&self.dst_addr).unwrap();

        writer.set_field_name("network_packet_data");
        writer.write_blob(&self.network_packet_data).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkPacketDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let frame_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let src_addr = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let dst_addr = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let network_packet_data = binding.as_slice();

        NetworkPacketDTO::new(
            frame_time, 
            src_addr, 
            dst_addr, 
            network_packet_data
        )
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

    use crate::api::network_packet::NetworkPacketDTO;


    #[test]
    fn reader_correctly_read_encoded_network_paket() {        
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_paket = NetworkPacketDTO::new(
            FRAME_TIME, 
            SRC_ADDR, 
            DST_ADDR, 
            NETWORK_PACKET_DATA
        );
        let mut binary_user_reader = ReaderBuilder::new().build(network_paket.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("frame_time", binary_user_reader.field_name().unwrap());
        assert_eq!(FRAME_TIME, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("src_addr", binary_user_reader.field_name().unwrap());
        assert_eq!(SRC_ADDR,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("dst_addr", binary_user_reader.field_name().unwrap());
        assert_eq!(DST_ADDR,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("network_packet_data", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_PACKET_DATA, binary_user_reader.read_blob().unwrap().as_slice());
    }

    #[test]
    #[ignore]
    fn endec_network_paket() {
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_paket = NetworkPacketDTO::new(
            FRAME_TIME, 
            SRC_ADDR, 
            DST_ADDR, 
            NETWORK_PACKET_DATA
        );
        assert_eq!(network_paket, NetworkPacketDTO::decode(&network_paket.encode()));
    }
}