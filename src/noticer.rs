use crate::config::Notifications;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct SendEmail<'a> {
    email: &'a str,
    title: &'a str,
    content: &'a str,
}

pub struct Noticer {
    cfg: Notifications,
    client: Client,
}
impl Noticer {
    pub fn new(cfg: Notifications) -> Self {
        Self { cfg, client: Client::new() }
    }
    pub async fn notice(&self, context: &str) {
        for email in &self.cfg.emails {
            let send_email = SendEmail {
                email,
                title: context,
                content: "",
            };
            let _ = self.client.post("https://tool.crise.cn/api/tool/email").json(&send_email).send().await;
        }
    }
}
