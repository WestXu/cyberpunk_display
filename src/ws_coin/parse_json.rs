use ordered_float::NotNan;
use serde_json::Value;

#[derive(Debug)]
pub enum Msg {
    Ping(u64),
    Subscribed(String),
    Price(NotNan<f64>),
}

pub fn parse_json(data: &str) -> Msg {
    let fail_msg = format!("Failed parsing data: {}", data);
    let v: Value = serde_json::from_str(data).expect(&fail_msg);

    assert!(v.is_object());
    let o = v.as_object().expect(&fail_msg);

    if o.contains_key("status") {
        assert_eq!(v["status"], "ok", "{}", fail_msg);
        return Msg::Subscribed(
            o.get("subbed")
                .expect(&fail_msg)
                .as_str()
                .expect(&fail_msg)
                .to_string(),
        );
    }

    if o.contains_key("ping") {
        return Msg::Ping(v["ping"].as_u64().expect(&fail_msg));
    }

    Msg::Price(
        NotNan::new(v["tick"]["data"][0]["price"].as_f64().expect(&fail_msg)).expect(&fail_msg),
    )
}
