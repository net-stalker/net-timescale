use std::thread::JoinHandle;

use shaku::Interface;

pub trait Starter: Interface {
    //FIXME I'm not sure that is good idea to return JoinHandle
    fn start(&self) -> JoinHandle<()>;
}