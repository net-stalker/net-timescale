use std::os::unix::io::RawFd;
use std::num::TryFromIntError;

pub trait Handler {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender);
}

pub trait Receiver {
    fn recv(&self) -> Vec<u8>;
}

pub trait Sender {
    fn send(&self, data: Vec<u8>);
}

pub trait Socket {
    fn as_raw_fd(&self) -> RawFd;

    fn fd_as_usize(&self) -> Result<usize, TryFromIntError> {
        usize::try_from(self.as_raw_fd())
    }

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender);

    fn get_receiver(&self) -> &dyn Receiver;

    fn get_sender(&self) -> &dyn Sender;
}
