use std::os::unix::io::RawFd;
use std::num::TryFromIntError;
use std::sync::Arc;

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
    fn fd(&self) -> RawFd;

    fn fd_as_usize(&self) -> Result<usize, TryFromIntError>;

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender);

    fn get_receiver(&self) -> &dyn Receiver;

    fn get_sender(&self) -> &dyn Sender;
}
