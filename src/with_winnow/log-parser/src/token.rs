#[derive(Debug, Eq, PartialEq)]
pub struct Socket<'a> {
    pub ip_address: &'a str,
    pub port: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IotLog<'a> {
    pub data_time: &'a str,
    pub client_socket: Socket<'a>,
    pub server_socket: Socket<'a>,
    pub payload: &'a str,
}
