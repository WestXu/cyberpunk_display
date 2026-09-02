use anyhow::{Context as _, Result};
use rust_decimal::prelude::*;
use serialport::{SerialPort, SerialPortType};
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

const TUBES: u32 = 6;
const MAX_SHOWN: u64 = 999_999;

impl From<Decimal> for NixieMsg {
    fn from(num: Decimal) -> Self {
        // Spend the tubes on decimals first, giving one back to the integer part
        // whenever the rounded value doesn't fit, so a carry can't shift the dot.
        let mut decimals = TUBES;
        let digits = loop {
            let scaled = num
                .checked_mul(Decimal::from(10u64.pow(decimals)))
                .and_then(|n| n.round().to_u64());
            match scaled {
                Some(d) if d <= MAX_SHOWN => break d,
                _ if decimals == 0 => break MAX_SHOWN,
                _ => decimals -= 1,
            }
        };

        let mut bytes = *b"TIMD000000BBBBBB";
        bytes[4..10].copy_from_slice(format!("{digits:06}").as_bytes());
        if decimals > 0 {
            bytes[10 + (TUBES - decimals) as usize] = b'L';
        }
        NixieMsg { num, bytes }
    }
}

/// The CH340 USB-to-serial bridge the Nixie tube is wired behind.
const USB_VID: u16 = 0x1a86;
const USB_PID: u16 = 0x7523;

fn find_port() -> Result<String> {
    let ports = serialport::available_ports().context("failed to enumerate serial ports")?;
    ports
        .into_iter()
        .find(|p| {
            // macOS exposes both a callout (/dev/cu.*) and a dialin (/dev/tty.*) node per
            // device; opening the dialin one blocks until DCD is asserted.
            !p.port_name.starts_with("/dev/tty.")
                && matches!(&p.port_type, SerialPortType::UsbPort(usb)
                    if usb.vid == USB_VID && usb.pid == USB_PID)
        })
        .map(|p| p.port_name)
        .with_context(|| format!("no Nixie found at USB {USB_VID:04x}:{USB_PID:04x}"))
}

pub struct Nixie {
    ser: Box<dyn SerialPort>,
}

/// Poll until the Nixie is plugged in (and not already claimed by another instance).
pub async fn wait_for_device() -> Nixie {
    let mut warned = false;
    loop {
        match Nixie::new() {
            Ok(nixie) => return nixie,
            Err(e) => {
                if warned {
                    log::debug!("Waiting for Nixie: {e:#}");
                } else {
                    log::warn!("Waiting for Nixie: {e:#}");
                    warned = true;
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}

impl Nixie {
    pub fn new() -> Result<Self> {
        let port = find_port()?;
        log::info!("Found Nixie at {port}");
        Ok(Nixie {
            ser: serialport::new(&port, 9600)
                .timeout(Duration::from_millis(100))
                .open()
                .with_context(|| format!("failed to open {port}"))?,
        })
    }
    pub async fn send(&mut self, bytes: NixieMsg) -> std::io::Result<()> {
        self.ser.write_all(&bytes.bytes)?;
        tokio::time::sleep(Duration::from_millis(50)).await;
        log::trace!("Sent to Nixie: {}", bytes.num);
        Ok(())
    }
    pub fn set_brightness(&mut self, b: u8) -> std::io::Result<()> {
        assert!(b <= 8, "brightness should be between (0, 8)");
        self.ser.write_all(format!("TIMB{b}").as_bytes())?;
        log::info!("Set Nixie brightness to {b}");
        Ok(())
    }
    fn close(&mut self) {
        if let Err(e) = self.ser.write_all("TIMDBBBBBBBBBBBB".as_bytes()) {
            log::error!("Failed to close Nixie: {e}");
            return;
        }
        log::info!("Closed Nixie");
    }
}

impl Drop for Nixie {
    fn drop(&mut self) {
        self.close();
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
    assert_eq!(NixieMsg::from(dec!(999999.5)).bytes, *b"TIMD999999BBBBBB");
    assert_eq!(NixieMsg::from(dec!(99999.96)).bytes, *b"TIMD100000BBBBBB");
    assert_eq!(NixieMsg::from(dec!(9.999999)).bytes, *b"TIMD100000BBLBBB");
    assert_eq!(NixieMsg::from(dec!(0.0001513)).bytes, *b"TIMD000151LBBBBB");
    assert_eq!(NixieMsg::from(dec!(12345678)).bytes, *b"TIMD999999BBBBBB");
}

#[tokio::test]
async fn test_nixie() {
    use rust_decimal_macros::dec;
    use std::thread::sleep;

    let mut nixie = Nixie::new().unwrap();
    nixie.set_brightness(8).unwrap();
    for p in 0..=9 {
        nixie
            .send((Decimal::from(p) * dec!(11111.1)).into())
            .await
            .unwrap();
        sleep(Duration::from_millis(200));
    }

    (0..=8)
        .rev()
        .map(|b| {
            nixie.set_brightness(b).unwrap();
            sleep(Duration::from_millis(200));
        })
        .for_each(drop);

    drop(nixie);
    sleep(Duration::from_millis(200));
}
