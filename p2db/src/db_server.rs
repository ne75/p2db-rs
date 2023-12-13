use std::thread;
use std::time::Duration;
use std::io::{BufReader, BufRead};
use serialport::SerialPort;

use crate::tools;

pub struct DBServer {
    inner: Option<Inner>,
}

impl DBServer {
    pub fn new(port: String, app: String) -> Self {
        let ser_port = serialport::new(&port, 2000000)
            .timeout(Duration::MAX)
            .open()
            .expect("Failed to open port");

        let port_reader = BufReader::new(ser_port);
        let inn = Inner{serial: port_reader, app: app, port: port};
        return Self{inner: Some(inn)};
    }

    pub fn start(&mut self) {
        if let Some(inn) = self.inner.take() {
            tools::load_app(&inn.app, &inn.port).unwrap();
            thread::spawn(move||inn.run());
        } else {
            println!("Don't call this twice, dummy");
        }
    }
}

struct Inner {
    // serial: BufReader<Box<dyn SerialPort>>,
    serial: Box<dyn SerialPort>,
    app: String,
    port: String,
}

impl Inner {
    fn run(&self) {
        let i = 0;
        loop {
            self.serial.read(buf)
            let i = i+1;
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    }
}  