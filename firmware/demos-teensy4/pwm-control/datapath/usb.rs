//! IMU datapath over USB
//!
//! # Future work
//!
//! teensy4-bsp 0.2 requires that the user drive the USB polling interval. That will
//! either need to happen here, or in a USB_OTG1 ISR. Prefer the ISR so we can still
//! support USB logging.

use crate::bsp::usb::Writer;

pub struct Datapath {
    usb: Writer,
}

#[derive(Debug)]
pub enum Error {
    IncompleteWrite { expected: usize, actual: usize },
}

impl Datapath {
    pub fn new(usb: Writer) -> Result<Self, Error> {
        Ok(Datapath { usb })
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<(), Error> {
        self.poll()?;

        self.usb.write(buffer);
        Ok(())
    }

    pub fn poll(&mut self) -> Result<(), Error> {
        // See "Future work" notes. This empty implementation assumes that
        // something else is polling the USB driver.
        Ok(())
    }
}
