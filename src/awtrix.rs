use reqwest::{header, Client};
use std::time::SystemTime;

pub struct Awtrix {
    host: String,
    port: u16,
    ssn: Client,
    min_interval: u128, // in milliseconds
    last_sent_time: SystemTime,
}

impl Awtrix {
    pub fn new(host: String, port: u16) -> Awtrix {
        Awtrix {
            host,
            port,
            ssn: Client::new(),
            min_interval: 100,
            last_sent_time: SystemTime::now(),
        }
    }

    async fn push(&self, data: serde_json::Value, endpoint: &str) {
        let _ = self
            .ssn
            .post(format!(
                "http://{}:{}/api/v3/{}",
                self.host, self.port, endpoint
            ))
            .body(serde_json::to_string(&data).unwrap())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await;
    }

    pub async fn exit(&self) {
        self.push(
            serde_json::json!({
                "draw": [{"type": "exit"}],
            }),
            "draw",
        )
        .await
    }

    pub async fn plot(&mut self, rgb565: &[u16]) {
        if self.last_sent_time.elapsed().unwrap().as_millis() < self.min_interval {
            // 小于0.1秒的间隔没有必要发送，人眼无法分辨
            return;
        }
        self.push(
            serde_json::json!({
                "draw": [
                    {
                        "type": "bmp",
                        "position": [0, 0],
                        "size": [32, 8],
                        "data": rgb565,
                    },
                    {"type": "show"},
                ],
            }),
            "draw",
        )
        .await;
        self.last_sent_time = SystemTime::now();
    }
}
