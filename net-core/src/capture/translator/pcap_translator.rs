use subprocess::{Exec, Redirection};

use crate::capture::translator::translator::Translator;
use crate::file::files::Files;

pub struct PcapTranslator;

const TSHARK_APP_NAME: &str = "tshark";
const TSHARK_CMD: &'static str = "TZ=UTC tshark -V --no-duplicate-keys -Tjson -n -r -";

impl Translator for PcapTranslator {
    type Input = Vec<u8>;
    type Output = Vec<u8>;

    /// https://tshark.dev/capture/tshark/
    /// Translate pcap file to json format
    ///
    /// The behavior experiencing with tshark may be due to differences in the default system time zone between Linux and macOS.
    //
    // On Linux, tshark (and the underlying libpcap library) will use UTC as the default time zone unless otherwise specified. On macOS, the default time zone is typically set to the system time zone. This means that if your system time zone is not set to UTC, tshark will display timestamps in your local time zone by default.
    //
    // To ensure that tshark behaves consistently across different platforms, will explicitly specify a specific time zone using the TZ=UTC environment variable.
    // This sets the TZ environment variable to UTC before running tshark, which will force tshark to use UTC as the time zone.
    ///
    /// # Arguments
    ///
    /// * `buf`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn translate(buf: Vec<u8>) -> Vec<u8> {
        if !Files::which(TSHARK_APP_NAME).success() {
            panic!("An application {} is not installed", TSHARK_APP_NAME)
        }

        Exec::cmd("sh")
            .args(&["-c"])
            .arg(&TSHARK_CMD.to_string())
            .stdin(buf)
            .stdout(Redirection::Pipe)
            .capture().unwrap()
            .stdout
    }
}

#[cfg(test)]
mod tests {
    use crate::file::files::Files;
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_translate_arp_packet() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.pcap"));
        let json_result = PcapTranslator::translate(pcap_buffer);

        let json_buffer = Files::read_vector(test_resources!("captures/arp.json"));

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_translate_dhcp_packet() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/dhcp.pcap"));
        let json_buffer = Files::read_vector(test_resources!("captures/dhcp.json"));

        let json_result = PcapTranslator::translate(pcap_buffer);

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }
}
