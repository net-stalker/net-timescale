
use std::sync::{Arc};



use log::debug;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct AgentCommand<S> {
    pub translator: Arc<S>,
}

impl<S> AgentCommand<S> {
    fn compare_first_n_bytes(a: &[u8], b: &[u8], n: usize) -> bool {
        if a.len() < n || b.len() < n {
            return false;
        }
        a[0..n] == b[0..n]
    }
}

impl<S: Sender> Handler for AgentCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let mut data = receiver.recv();

        // let magic_num = &data[..4];
        // if 3569595041_u32.to_be_bytes() == magic_num {
        // debug!("Global header will be skipped");
        // return;
        // }
        // improvized dispatcher
        if AgentCommand::<S>::compare_first_n_bytes(&data, "db".as_bytes(), "db".as_bytes().len()) {
            debug!("received from decoder {:?}", data);
        } else {
            debug!("received from agent {:?}", data);
            let temp_topic = "decode".as_bytes().to_owned();
            data.splice(0..0, temp_topic);
        }

        self.translator.send(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bytes_comparing() {
        let lhs: &[u8] = &[1,2,3];
        let rhs: &[u8] = &[1,2,4];

        assert_eq!(AgentCommand::<u8>::compare_first_n_bytes(lhs, rhs, 2), true);

        assert_eq!(AgentCommand::<u8>::compare_first_n_bytes(lhs, rhs, 3), false);

        assert_eq!(AgentCommand::<u8>::compare_first_n_bytes(lhs, rhs, 4), false);
    }
}