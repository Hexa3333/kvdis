#[derive(Debug)]
pub enum CommandType {
    Set,
    Get,
    Del,
    Exists,
    Expire,
    Incr, Decr
}
