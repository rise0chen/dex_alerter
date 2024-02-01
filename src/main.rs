mod config;
mod noticer;

use config::Config;
use core::time::Duration;

const PERIOD: u64 = 5 * 60;

#[tokio::main]
async fn main() {
    let config: Config = serde_json::from_reader(std::fs::File::open("config.json").unwrap()).unwrap();
    let noticer = noticer::Noticer::new(config.notifications);

    let dexscreener = dexscreener::Client::new();
    loop {
        let now = clock_source::now() / 1_000_000_000;

        for alerter in &config.alerters {
            let mut pairs = if let Ok(pair) = dexscreener.pairs(&alerter.chain, [&alerter.pair]).await {
                pair.pairs.unwrap_or_default()
            } else {
                continue;
            };
            let Some(pair) = pairs.pop() else {
                continue;
            };

            if (0..PERIOD).contains(&(now % (5 * 60))) {
                for alert in &alerter.statistics.m5 {
                    let price_change = pair.price_change.m5;
                    if alert < &0.0 && &price_change < alert {
                        noticer.notice(&format!("{} m5 down: {}%", alerter.name, price_change)).await;
                    }
                    if alert > &0.0 && &price_change > alert {
                        noticer.notice(&format!("{} m5 up: {}%", alerter.name, price_change)).await;
                    }
                }
            }
            if (0..PERIOD).contains(&(now % (60 * 60))) {
                for alert in &alerter.statistics.h1 {
                    let price_change = pair.price_change.h1;
                    if alert < &0.0 && &price_change < alert {
                        noticer.notice(&format!("{} h1 down: {}%", alerter.name, price_change)).await;
                    }
                    if alert > &0.0 && &price_change > alert {
                        noticer.notice(&format!("{} h1 up: {}%", alerter.name, price_change)).await;
                    }
                }
            }
            if (0..PERIOD).contains(&(now % (6 * 60 * 60))) {
                for alert in &alerter.statistics.h6 {
                    let price_change = pair.price_change.h6;
                    if alert < &0.0 && &price_change < alert {
                        noticer.notice(&format!("{} h6 down: {}%", alerter.name, price_change)).await;
                    }
                    if alert > &0.0 && &price_change > alert {
                        noticer.notice(&format!("{} h6 up: {}%", alerter.name, price_change)).await;
                    }
                }
            }
            if (0..PERIOD).contains(&(now % (24 * 60 * 60))) {
                for alert in &alerter.statistics.h24 {
                    let price_change = pair.price_change.h24;
                    if alert < &0.0 && &price_change < alert {
                        noticer.notice(&format!("{} h24 down: {}%", alerter.name, price_change)).await;
                    }
                    if alert > &0.0 && &price_change > alert {
                        noticer.notice(&format!("{} h24 up: {}%", alerter.name, price_change)).await;
                    }
                }
            }

            if let Ok(price) = pair.price_native.parse::<f64>() {
                let price_old = price * (1.0 - pair.price_change.m5 / 100.0);
                let native = pair.quote_token.symbol;
                for alert in &alerter.price_native {
                    if alert < &0.0 && price < alert.abs() && price_old > alert.abs() {
                        noticer.notice(&format!("{} under: {}{}", alerter.name, price, native)).await;
                    }
                    if alert > &0.0 && price > *alert && price_old < *alert {
                        noticer.notice(&format!("{} over: {}{}", alerter.name, price, native)).await;
                    }
                }
            }
            if let Some(price) = pair.price_usd.and_then(|x| x.parse::<f64>().ok()) {
                let price_old = price * (1.0 - pair.price_change.m5 / 100.0);
                for alert in &alerter.price_usd {
                    if alert < &0.0 && price < alert.abs() && price_old > alert.abs() {
                        noticer.notice(&format!("{} under: ${}", alerter.name, price)).await;
                    }
                    if alert > &0.0 && price > *alert && price_old < *alert {
                        noticer.notice(&format!("{} over: ${}", alerter.name, price)).await;
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(PERIOD)).await;
    }
}
