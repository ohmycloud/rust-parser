#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, PartialEq)]
pub struct Destination {
    pub name: String,
    pub coordinate: Coordinate,
    pub tickets: u32,
}
