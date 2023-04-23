pub trait ServerHandler : Send + Clone + russh::server::Handler {}
impl <T> ServerHandler for T where T : Send + Clone + russh::server::Handler {}