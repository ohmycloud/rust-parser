use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Kv<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct WeatherStation<'a> {
    pub kvs: Vec<Kv<'a>>,
    pub observations: Vec<HashMap<&'a str, Vec<f64>>>,
}
