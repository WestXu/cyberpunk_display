use reqwest::blocking::Client;
use std::time::SystemTime;

pub struct Awtrix {
    ip: String,
    port: u16,
    ssn: Client,
    min_interval: u128, // in milliseconds
    last_sent_time: SystemTime,
}

impl Awtrix {
    pub fn new(ip: String, port: u16) -> Awtrix {
        Awtrix {
            ip,
            port,
            ssn: Client::new(),
            min_interval: 100,
            last_sent_time: SystemTime::now(),
        }
    }

    fn push(&self, data: serde_json::Value, endpoint: &str) {
        let resp = self
            .ssn
            .post(format!(
                "http://{}:{}/api/v3/{}",
                self.ip, self.port, endpoint
            ))
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .unwrap();
        if !resp.status().is_success() {
            println!("Error posting to awtrix server: {:?}", resp.text());
        }
    }

    fn exit(&self) {
        self.push(
            serde_json::json!({
                "draw": [{"type": "exit"}],
            }),
            "draw",
        )
    }

    pub fn plot(&mut self, rgb565: &[u16]) {
        if self.last_sent_time.elapsed().unwrap().as_millis() < self.min_interval {
            // 小于0.1秒的间隔没有必要发送，人眼无法分辨
            println!("Skipped sending because of too little interval.");
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
        );
        self.last_sent_time = SystemTime::now();
    }
}
