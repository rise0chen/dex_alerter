use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Alerter {
    pub name: String,
    pub chain: String,
    pub pair: String,
    pub statistics: Statistics,
    pub price_native: Vec<f64>,
    pub price_usd: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Notifications {
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Statistics {
    pub m5: Vec<f64>,
    pub h1: Vec<f64>,
    pub h6: Vec<f64>,
    pub h24: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub alerters: Vec<Alerter>,
    pub notifications: Notifications,
}
