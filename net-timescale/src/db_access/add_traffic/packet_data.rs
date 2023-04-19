#[derive(serde::Serialize, serde::Deserialize)]
pub struct PacketData {
    pub frame_time: i64,
    pub src_addr: String,
    pub dst_addr: String,
    pub binary_json: Vec<u8>,
}