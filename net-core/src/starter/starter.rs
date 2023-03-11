use shaku::Interface;

pub trait Starter: Interface {
    fn start(&self);
}