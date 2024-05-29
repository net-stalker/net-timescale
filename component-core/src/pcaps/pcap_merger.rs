use subprocess::Exec;
use subprocess::Redirection;

pub struct PcapMerger;

const MERGECAP_COMMAND: &str = "mergecap -a -w -";

impl PcapMerger {
    pub fn merge(path_to_files: &[&str]) -> Vec<u8> {
        // it must just merge the files
        let mut cmd = MERGECAP_COMMAND.to_string();
        path_to_files.iter().for_each(|path_to_file| cmd.push_str(&format!(" {path_to_file}")));
        Exec::cmd("sh")
            .args(&["-c"])
            .arg(&cmd)
            .stdout(Redirection::Pipe)
            .capture()
            .unwrap()
            .stdout
    }
}
