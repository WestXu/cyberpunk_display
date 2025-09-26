use serialport::SerialPort;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn float_to_bytes(num: f32) -> [u8; 16] {
    let num_str = format!("{num:07.1}");
    let out_str = format!("{}{}{}", "TIMD", num_str.replace(".", ""), "BBBBBL");
    out_str
        .as_bytes()
        .try_into()
        .expect(&format!("Failed to format num {num} to {out_str}"))
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
    pub fn send(&mut self, p: f32) {
        self.ser
            .write(&float_to_bytes(p))
            .expect(&format!("failed to send {p}"));
        log::info!("Sent to Nixie {p}");
    }
    pub fn set_brightness(&mut self, b: u8) {
        assert!(b <= 8, "brightness should be between (0, 8)");
        self.ser
            .write(format!("TIMB{b}").as_bytes())
            .expect(&format!("failed to set brightness to {b}"));
        log::info!("Set Nixie brightness to {b}");
    }
    pub fn close(&mut self) {
        self.ser
            .write("TIMDBBBBBBBBBBBB".as_bytes())
            .expect("Failed to close");
        log::info!("Closed Nixie");
    }
}

#[test]
fn test_float_to_bytes() {
    let fs = [100.2, 0.1513, 13568.0, 141.5116548165, 0.0000005186];
    assert_eq!(
        fs.map(float_to_bytes),
        [
            "TIMD001002BBBBBL".as_bytes(),
            "TIMD000002BBBBBL".as_bytes(),
            "TIMD135680BBBBBL".as_bytes(),
            "TIMD001415BBBBBL".as_bytes(),
            "TIMD000000BBBBBL".as_bytes(),
        ]
    );
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
    let mut nixie = Nixie::new("COM3".to_owned());
    nixie.set_brightness(8);
    (0..=9)
        .map(|p| {
            nixie.send(p as f32 * 11111.1);
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
