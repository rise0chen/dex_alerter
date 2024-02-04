use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatisticPeriod {
    M5,
    H1,
    H6,
    H24,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatisticSide {
    Up,
    Down,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct StatisticAlerter {
    pub period: StatisticPeriod,
    pub side: StatisticSide,
    pub value: f64,
    pub times: u32,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PriceQuote {
    Native,
    Usd,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PriceSide {
    Over,
    Under,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct PriceAlerter {
    pub quote: PriceQuote,
    pub side: PriceSide,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LiquidityToken {
    /// Native,计价token
    Quote,
    /// 标的
    Base,
    Usd,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct LiquidityAlerter {
    pub token: LiquidityToken,
    pub side: PriceSide,
    pub value: f64,
    pub last_value: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Alerter {
    pub name: String,
    pub chain: String,
    pub pair: String,
    #[serde(default)]
    pub statistic: Vec<StatisticAlerter>,
    #[serde(default)]
    pub price: Vec<PriceAlerter>,
    #[serde(default)]
    pub liquidity: Vec<LiquidityAlerter>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Notifications {
    pub emails: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub alerters: Vec<Alerter>,
    pub notifications: Notifications,
}
