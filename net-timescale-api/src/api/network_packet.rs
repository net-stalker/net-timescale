#[cfg(feature = "capnp-endec")] 
pub mod network_packet_capnp {
    include!(concat!(env!("OUT_DIR"), "/network_packet_capnp.rs"));
}
#[cfg(feature = "capnp-endec")] 
use network_packet_capnp::network_packet;


#[cfg(feature = "ion-endec")]
use ion_rs;
#[cfg(feature = "ion-endec")]
use ion_rs::IonWriter;
#[cfg(feature = "ion-endec")]
use ion_rs::IonReader;


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
    pub fn new ( frame_time: i64, src_addr: String, dst_addr: String, network_packet_data: Vec<u8>) -> Self {
        NetworkPacketDTO { 
            frame_time, 
            src_addr, 
            dst_addr, 
            network_packet_data
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

#[cfg(feature = "capnp-endec")] 
impl Encoder for NetworkPacketDTO {
    fn encode(&self) -> Vec<u8> {    
        let mut buffer: Vec<u8> = Vec::new();

        let mut message = ::capnp::message::Builder::new_default();
        let mut struct_to_encode = message.init_root::<network_packet::Builder>();
        
        struct_to_encode.set_frame_time(self.frame_time);
        struct_to_encode.set_src_addr(&self.src_addr);
        struct_to_encode.set_dst_addr(&self.dst_addr);        
        struct_to_encode.set_data(&self.network_packet_data);
    
        match ::capnp::serialize_packed::write_message(&mut buffer, &message) {
            Ok(_) => buffer,
            Err(_) => todo!(),
        }
    }
}

#[cfg(feature = "ion-endec")] 
impl Encoder for NetworkPacketDTO {
    fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut binary_writer = binary_writer_builder.build(&mut buffer).unwrap();

        binary_writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");
        
        binary_writer.set_field_name("frame_time");
        binary_writer.write_i64(self.frame_time).unwrap();

        binary_writer.set_field_name("src_addr");
        binary_writer.write_string(&self.src_addr).unwrap();

        binary_writer.set_field_name("dst_addr");
        binary_writer.write_string(&self.dst_addr).unwrap();

        binary_writer.set_field_name("network_packet_data");
        binary_writer.write_blob(&self.network_packet_data).unwrap();

        binary_writer.step_out().unwrap();
        binary_writer.flush().unwrap();

        buffer
    }
}

#[cfg(feature = "capnp-endec")] 
impl Decoder for NetworkPacketDTO {
    fn decode(data: Vec<u8>) -> Self {
//TODO: Think about using std::io::Cursor here
        let message_reader = ::capnp::serialize_packed::read_message(
            data.as_slice(),
            ::capnp::message::ReaderOptions::new()).unwrap();
    
        let decoded_struct = message_reader.get_root::<network_packet::Reader>().unwrap();

        NetworkPacketDTO { 
            frame_time: decoded_struct.get_frame_time(), 
            src_addr: String::from(decoded_struct.get_src_addr().unwrap()), 
            dst_addr: String::from(decoded_struct.get_dst_addr().unwrap()), 
            network_packet_data: Vec::from(decoded_struct.get_data().unwrap())
        }
    }
}

#[cfg(feature = "ion-endec")] 
impl Decoder for NetworkPacketDTO {
    fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let frame_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let src_addr = String::from(binding.text());

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let dst_addr = String::from(binding.text());

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let network_packet_data = binding.as_slice().to_owned();

        NetworkPacketDTO {
            frame_time,
            src_addr,
            dst_addr,
            network_packet_data,
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

    use crate::api::network_packet::NetworkPacketDTO;


    #[test]
    fn reader_correctly_read_encoded_network_paket() {        
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_paket = NetworkPacketDTO::new(
            FRAME_TIME, 
            SRC_ADDR.to_owned(), 
            DST_ADDR.to_owned(), 
            NETWORK_PACKET_DATA.to_owned()
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
    fn endec_network_paket() {
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_paket = NetworkPacketDTO::new(
            FRAME_TIME, 
            SRC_ADDR.to_owned(), 
            DST_ADDR.to_owned(), 
            NETWORK_PACKET_DATA.to_owned()
        );
        assert_eq!(network_paket, NetworkPacketDTO::decode(network_paket.encode()));
    }

    #[cfg(feature = "ion-schema-validation")]
    #[test]
    fn ion_schema_validation() {
        //TODO: Write schema validation tests (should be done in #85zta68kj task)
    }
}