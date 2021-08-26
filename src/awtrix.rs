use reqwest::blocking::Client;
use reqwest::header;
use std::time::SystemTime;

pub struct Awtrix {
    host: String,
    port: u16,
    ssn: Client,
    last_sent_time: SystemTime,
}

impl Awtrix {
    pub fn new(host: String, port: u16) -> Awtrix {
        Awtrix {
            host,
            port,
            ssn: Client::new(),
            last_sent_time: SystemTime::now(),
        }
    }

    fn push(&self, data: serde_json::Value, endpoint: &str) {
        let resp = self
            .ssn
            .post(format!(
                "http://{}:{}/api/v3/{}",
                self.host, self.port, endpoint
            ))
            .body(serde_json::to_string(&data).unwrap())
            .header(header::CONTENT_TYPE, "application/json")
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
