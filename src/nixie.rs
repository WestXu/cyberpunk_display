use rust_decimal::prelude::*;
use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;

fn decimal_to_bytes(num: Decimal) -> [u8; 16] {
    let s = num.to_string();
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = if parts.len() > 1 { parts[1] } else { "" };

    let (digits_str, has_decimal) = if int_part == "0" {
        (format!("{:0<6}", dec_part), true)
    } else {
        let combined = format!("{}{}", int_part, dec_part);
        let mut truncated = combined;
        truncated.truncate(6);
        (
            format!("{:0<6}", truncated),
            truncated.len() > int_part.len() && !dec_part.is_empty(),
        )
    };

    let dot_pos = if int_part == "0" {
        0
    } else {
        int_part.len().min(5)
    };

    let mut dots = [b'B'; 6];
    if has_decimal && dot_pos < 6 {
        dots[dot_pos] = b'L';
    }

    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(b"TIMD");
    result[4..10].copy_from_slice(digits_str.as_bytes());
    result[10..16].copy_from_slice(&dots);
    result
}

pub struct Nixie {
    ser: Box<dyn SerialPort>,
}

impl Nixie {
    pub fn new(serialport: String) -> Self {
        Nixie {
            ser: serialport::new(serialport, 9600)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port"),
        }
    }
    pub fn send(&mut self, p: Decimal) {
        self.ser
            .write_all(&decimal_to_bytes(p))
            .unwrap_or_else(|_| panic!("failed to send {p}"));
        log::info!("Sent to Nixie {p}");
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

    assert_eq!(decimal_to_bytes(dec!(100.2)), *b"TIMD100200BBBLBB");
    assert_eq!(decimal_to_bytes(dec!(0.1513)), *b"TIMD151300LBBBBB");
    assert_eq!(decimal_to_bytes(dec!(13568.0)), *b"TIMD135680BBBBBL");
    assert_eq!(decimal_to_bytes(dec!(141.51165)), *b"TIMD141511BBBLBB");
    assert_eq!(decimal_to_bytes(dec!(94395.23)), *b"TIMD943952BBBBBL");
    assert_eq!(decimal_to_bytes(dec!(124395.52)), *b"TIMD124395BBBBBB");
}

#[test]
fn list_serial_port() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        log::info!("{}", p.port_name);
    }
}

#[test]
fn test_nixie() {
    use rust_decimal_macros::dec;
    use std::thread::sleep;

    let mut nixie = Nixie::new("/dev/ttyUSB0".to_owned());
    nixie.set_brightness(8);
    (0..=9)
        .map(|p| {
            nixie.send(Decimal::from(p) * dec!(11111.1));
            sleep(Duration::from_millis(200));
        })
        .for_each(drop);
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
