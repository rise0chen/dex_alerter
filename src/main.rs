mod config;
mod noticer;

use config::Config;
use core::time::Duration;

const PERIOD: u64 = 5 * 60;

#[tokio::main]
async fn main() {
    let config: Config = serde_json::from_reader(std::fs::File::open("config.json").unwrap()).unwrap();
    let mut alerters = config.alerters.clone();
    let noticer = noticer::Noticer::new(config.notifications);

    let dexscreener = dexscreener::Client::new();
    loop {
        let now = clock_source::now() / 1_000_000_000;

        for (alerter_index, alerter) in alerters.iter_mut().enumerate() {
            let mut pairs = if let Ok(pair) = dexscreener.pairs(&alerter.chain, [&alerter.pair]).await {
                pair.pairs.unwrap_or_default()
            } else {
                continue;
            };
            let Some(pair) = pairs.pop() else {
                continue;
            };
            let price_usd = pair.price_usd.as_ref().and_then(|x| x.parse::<f64>().ok()).unwrap_or_default();
            let price_native = pair.price_native.parse::<f64>().unwrap_or_default();
            let quote_token = pair.quote_token.symbol;

            for (alert_index, alert) in alerter.statistic.iter_mut().enumerate() {
                let price_change = match alert.period {
                    config::StatisticPeriod::M5 => {
                        if (0..PERIOD).contains(&(now % (5 * 60))) {
                            pair.price_change.m5
                        } else {
                            continue;
                        }
                    }
                    config::StatisticPeriod::H1 => {
                        if (0..PERIOD).contains(&(now % (60 * 60))) {
                            pair.price_change.h1
                        } else {
                            continue;
                        }
                    }
                    config::StatisticPeriod::H6 => {
                        if (0..PERIOD).contains(&(now % (6 * 60 * 60))) {
                            pair.price_change.h6
                        } else {
                            continue;
                        }
                    }
                    config::StatisticPeriod::H24 => {
                        if (0..PERIOD).contains(&(now % (24 * 60 * 60))) {
                            pair.price_change.h24
                        } else {
                            continue;
                        }
                    }
                };

                match alert.side {
                    config::StatisticSide::Up => {
                        if price_change > alert.value {
                            if alert.times <= 1 {
                                let times = config.alerters[alerter_index].statistic[alert_index].times;
                                alert.times = times;
                                let msg = format!(
                                    "{} up: {}%({}), ${}, {}{}",
                                    alerter.name, price_change, times, price_usd, price_native, quote_token
                                );
                                noticer.notice(&msg).await;
                            } else {
                                alert.times -= 1;
                            }
                        } else {
                            alert.times = config.alerters[alerter_index].statistic[alert_index].times;
                        }
                    }
                    config::StatisticSide::Down => {
                        if price_change < -alert.value {
                            if alert.times <= 1 {
                                let times = config.alerters[alerter_index].statistic[alert_index].times;
                                alert.times = times;
                                let msg = format!(
                                    "{} down: {}%({}), ${}, {}{}",
                                    alerter.name, price_change, times, price_usd, price_native, quote_token
                                );
                                noticer.notice(&msg).await;
                            } else {
                                alert.times -= 1;
                            }
                        } else {
                            alert.times = config.alerters[alerter_index].statistic[alert_index].times;
                        }
                    }
                }
            }

            for alert in alerter.price.iter_mut() {
                let price = match alert.quote {
                    config::PriceQuote::Native => price_native,
                    config::PriceQuote::Usd => price_usd,
                };
                if price == 0.0 {
                    continue;
                };
                let price_old = price * (1.0 - pair.price_change.m5 / 100.0);
                match alert.side {
                    config::PriceSide::Over => {
                        if price > alert.value && price_old < alert.value {
                            let msg = format!("{} over: ${}, {}{}", alerter.name, price_usd, price_native, quote_token);
                            noticer.notice(&msg).await;
                        }
                    }
                    config::PriceSide::Under => {
                        if price < alert.value && price_old > alert.value {
                            let msg = format!("{} under: ${}, {}{}", alerter.name, price_usd, price_native, quote_token);
                            noticer.notice(&msg).await;
                        }
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(PERIOD)).await;
    }
}
