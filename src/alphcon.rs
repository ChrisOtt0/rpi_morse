use std::{process, thread, time::Duration};
use rppal::gpio::{Gpio, OutputPin};

pub struct AlphCon {
    pin: OutputPin,
    unit: u64,
}

impl AlphCon {
    pub fn new(port: u8, unit: u64) -> AlphCon {
        let gpio = match Gpio::new() {
            Ok(v) => v,
            Err(_) => {
                println!("Error creating GPIO struct.");
                process::exit(1);
            },
        };

        let pin = match gpio.get(port) {
            Ok(v) => v,
            Err(_) => {
                println!("Error creating Pin struct.");
                process::exit(1);
            },
        };
        let pin = pin.into_output();

        AlphCon { pin: pin, unit: unit }
    }

    fn dot(&mut self) {
        self.pin.set_high();
        thread::sleep(Duration::from_millis(self.unit));
        self.pin.set_low();
        thread::sleep(Duration::from_millis(self.unit));
    }

    fn dash(&mut self) {
        self.pin.set_high();
        thread::sleep(Duration::from_millis(3 * self.unit));
        self.pin.set_low();
        thread::sleep(Duration::from_millis(self.unit));
    }

    pub fn letter_space(&mut self) {
        thread::sleep(Duration::from_millis(2 * self.unit));
    }
    
    pub fn word_space(&mut self) {
        thread::sleep(Duration::from_millis(6 * self.unit));
    }

    pub fn a(&mut self) {
        self.dot();
        self.dash();
    }

    pub fn b(&mut self) {
        self.dash();
        for _ in 0..3 {
            self.dot();
        }
    }

    pub fn c(&mut self) {
        for _ in 0..2 {
            self.dash();
            self.dot();
        }
    }
}
