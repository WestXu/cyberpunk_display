use rust_decimal::prelude::*;
use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct NixieMsg {
    num: Decimal,
    pub bytes: [u8; 16],
}
impl NixieMsg {
    pub fn flip_first_decimal_point(&mut self) {
        self.bytes[10] = if self.bytes[10] == b'B' { b'L' } else { b'B' }
    }
}

impl From<Decimal> for NixieMsg {
    fn from(num: Decimal) -> Self {
        let s = num.to_string();
        let parts: Vec<&str> = s.split('.').collect();
        let int_part = parts[0];
        let dec_part = if parts.len() > 1 { parts[1] } else { "" };

        let (digits_str, has_decimal, dot_pos) = if int_part == "0" {
            let dec_str = format!("{:0<6}", dec_part);
            (dec_str, true, 0)
        } else {
            let int_len = int_part.len();
            if int_len >= 6 {
                let rounded = num.round_dp(0);
                let rounded_str = rounded.to_string().split('.').next().unwrap().to_string();
                let mut final_str = rounded_str;
                final_str.truncate(6);
                (format!("{:0<6}", final_str), false, int_len.min(5))
            } else {
                let needed_dec = 6 - int_len;
                let rounded = num.round_dp(needed_dec as u32);
                let rounded_str = rounded.to_string();
                let rounded_parts: Vec<&str> = rounded_str.split('.').collect();
                let rounded_dec = if rounded_parts.len() > 1 {
                    rounded_parts[1]
                } else {
                    ""
                };
                let mut combined = format!("{}{}", rounded_parts[0], rounded_dec);
                combined.truncate(6);
                (
                    format!("{:0<6}", combined),
                    !rounded_dec.is_empty(),
                    int_len,
                )
            }
        };

        let mut dots = [b'B'; 6];
        if has_decimal && dot_pos < 6 {
            dots[dot_pos] = b'L';
        }

        let mut result = [0u8; 16];
        result[0..4].copy_from_slice(b"TIMD");
        result[4..10].copy_from_slice(digits_str.as_bytes());
        result[10..16].copy_from_slice(&dots);
        NixieMsg { num, bytes: result }
    }
}

pub fn decimal_to_bytes(num: Decimal) -> NixieMsg {
    num.into()
}

pub struct Nixie {
    ser: Box<dyn SerialPort>,
}

impl Nixie {
    pub fn new(serialport: String) -> Self {
        Nixie {
            ser: serialport::new(serialport, 9600)
                .timeout(Duration::from_millis(100))
                .open()
                .expect("Failed to open port"),
        }
    }
    pub async fn send(&mut self, bytes: NixieMsg) {
        if let Err(e) = self.ser.write_all(&bytes.bytes) {
            log::error!("Failed to send to Nixie: {}", e);
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
        log::info!("Sent to Nixie: {}", bytes.num);
    }
    pub fn set_brightness(&mut self, b: u8) {
        assert!(b <= 8, "brightness should be between (0, 8)");
        self.ser
            .write_all(format!("TIMB{b}").as_bytes())
            .unwrap_or_else(|_| panic!("failed to set brightness to {b}"));
        log::info!("Set Nixie brightness to {b}");
    }
    pub fn close(&mut self) {
        self.ser
            .write_all("TIMDBBBBBBBBBBBB".as_bytes())
            .expect("Failed to close");
        log::info!("Closed Nixie");
    }
}

#[test]
fn test_float_to_bytes() {
    use rust_decimal_macros::dec;

    assert_eq!(NixieMsg::from(dec!(100.2)).bytes, *b"TIMD100200BBBLBB");
    assert_eq!(NixieMsg::from(dec!(0.1513)).bytes, *b"TIMD151300LBBBBB");
    assert_eq!(NixieMsg::from(dec!(13568.0)).bytes, *b"TIMD135680BBBBBL");
    assert_eq!(NixieMsg::from(dec!(141.51165)).bytes, *b"TIMD141512BBBLBB");
    assert_eq!(NixieMsg::from(dec!(94395.23)).bytes, *b"TIMD943952BBBBBL");
    assert_eq!(NixieMsg::from(dec!(124395.52)).bytes, *b"TIMD124396BBBBBB");
    assert_eq!(NixieMsg::from(dec!(99999.73)).bytes, *b"TIMD999997BBBBBL");
    assert_eq!(NixieMsg::from(dec!(100000)).bytes, *b"TIMD100000BBBBBB");
    assert_eq!(NixieMsg::from(dec!(999999.5)).bytes, *b"TIMD100000BBBBBB");
}

#[test]
fn list_serial_port() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        log::info!("{}", p.port_name);
    }
}

#[tokio::test]
async fn test_nixie() {
    use rust_decimal_macros::dec;
    use std::thread::sleep;

    let mut nixie = Nixie::new("/dev/ttyUSB0".to_owned());
    nixie.set_brightness(8);
    for p in 0..=9 {
        nixie
            .send(decimal_to_bytes(Decimal::from(p) * dec!(11111.1)))
            .await;
        sleep(Duration::from_millis(200));
    }

    (0..=8)
        .rev()
        .map(|b| {
            nixie.set_brightness(b);
            sleep(Duration::from_millis(200));
        })
        .for_each(drop);

    nixie.close();
    sleep(Duration::from_millis(200));
}
