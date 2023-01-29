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

    pub fn d(&mut self) {
        self.dash();
        for _ in 0..2 {
            self.dot();
        }
    }

    pub fn e(&mut self) {
        self.dot();
    }

    pub fn f(&mut self) {
        for _ in 0..2 {
            self.dot();
        }
        self.dash();
        self.dot();
    }

    pub fn g(&mut self) {
        for _ in 0..2 {
            self.dash();
        }
        self.dot();
    }

    pub fn h(&mut self) {
        for _ in 0..4 {
            self.dot();
        }
    }

    pub fn i(&mut self) {
        for _ in 0..2 {
            self.dot();
        }
    }

    pub fn j(&mut self) {
        self.dot();
        for _ in 0..3 {
            self.dash();
        }
    }

    pub fn k(&mut self) {
        self.dash();
        self.dot();
        self.dash();
    }

    pub fn l(&mut self) {
        self.dot();
        self.dash();
        for _ in 0..2 {
            self.dot();
        }
    }

    pub fn m(&mut self) {
        for _ in 0..2 {
            self.dash();
        }
    }

    pub fn n(&mut self) {
        self.dash();
        self.dot();
    }

    pub fn o(&mut self) {
        for _ in 0..3 {
            self.dash();
        }
    }

    pub fn p(&mut self) {
        self.dot();
        for _ in 0..2 {
            self.dash();
        }
        self.dot();
    }
    
    pub fn q(&mut self) {
        for _ in 0..2 {
            self.dash();
        }
        self.dot();
        self.dash();
    }

    pub fn r(&mut self) {
        self.dot();
        self.dash();
        self.dot();
    }
    
    pub fn s(&mut self) {
        for _ in 0..3 {
            self.dot();
        }
    }

    pub fn t(&mut self) {
        self.dash();
    }

    pub fn u(&mut self) {
        for _ in 0..2 {
            self.dot();
        }
        self.dash();
    }

    pub fn v(&mut self) {
        for _ in 0..3 {
            self.dot();
        }
        self.dash();
    }

    pub fn w(&mut self) {
        self.dot();
        for _ in 0..2 {
            self.dash();
        }
    }

    pub fn x(&mut self) {
        self.dash();
        for _ in 0..2 {
            self.dot();
        }
        self.dash();
    }

    pub fn y(&mut self) {
        self.dash();
        self.dot();
        for _ in 0..2 {
            self.dash();
        }
    }

    pub fn z(&mut self) {
        for _ in 0..2 {
            self.dash();
        }
        for _ in 0..2 {
            self.dot();
        }
    }

    pub fn one(&mut self) {
        self.dot();
        for _ in 0..4 {
            self.dash();
        }
    }

    pub fn two(&mut self) {
        for _ in 0..2 {
            self.dot();
        }
        for _ in 0..3 {
            self.dash();
        }
    }

    pub fn three(&mut self) {
        for _ in 0..3 {
            self.dot();
        }
        for _ in 0..2 {
            self.dash();
        }
    }

    pub fn four(&mut self) {
        for _ in 0..4 {
            self.dot();
        }
        self.dash();
    }

    pub fn five(&mut self) {
        for _ in 0..5 {
            self.dot();
        }
    }

    pub fn six(&mut self) {
        self.dash();
        for _ in 0..4 {
            self.dot();
        }
    }

    pub fn seven(&mut self) {
        for _ in 0..2 {
            self.dash();
        }
        for _ in 0..3 {
            self.dot();
        }
    }

    pub fn eight(&mut self) {
        for _ in 0..3 {
            self.dash();
        }
        for _ in 0..2 {
            self.dot();
        }
    }

    pub fn nine(&mut self) {
        for _ in 0..4 {
            self.dash();
        }
        self.dot();
    }

    pub fn zero(&mut self) {
        for _ in 0..5 {
            self.dash();
        }
    }

}

