use std::os::unix::io::RawFd;
use nng::options::{Options, RecvFd};
use nng::Socket;

/// Return File Descriptor
///
/// # Arguments
///
/// * `socket`:
///
/// returns: i32
///
pub fn get_fd(socket: &Socket) -> RawFd {
    socket.get_opt::<RecvFd>().unwrap()
}

pub fn as_unsize(fd: RawFd) -> usize {
    usize::try_from(fd).unwrap()
}