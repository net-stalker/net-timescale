pub struct PcapSplitter;

impl PcapSplitter {
    pub fn split(jsonb_pcap: &[u8]) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Sync + Send>> {
        let json_pcap: serde_json::Value = match serde_json::from_slice(jsonb_pcap) {
            Ok(json_pcap) => json_pcap,
            Err(err) => return Err(err.into())
        };
        if let serde_json::Value::Array(pcaps) = json_pcap {
            Ok(pcaps)
        } else {
            Err(String::from("wrong json pcap format, vector of values is required").into())
        }
    }  
}