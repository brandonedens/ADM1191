//! Driver for Digital Power Monitor with Convert Pin and ALERTB Output
//!
//! https://www.analog.com/en/products/adm1191.html#product-overview

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

use embedded_hal as hal;

use hal::blocking::i2c::{Read, Write, WriteRead};

/// Driver for the ADM1191 Power Monitor IC.
pub struct Adm1191<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> Adm1191<I2C>
where
    I2C: Read<Error = E> + WriteRead<Error = E> + Write<Error = E>,
{
    /// Create a new instance of the ADM1191 driver for the given i2c bus and address.
    /// Note that the address for the ADM1191 is configurable via pins on the IC.
    pub fn new(i2c: I2C, addr: u8) -> Result<Self, E> {
        Ok(Adm1191 { i2c, addr })
    }

    /// Destroy the ADM1191 driver yielding the I2C bus.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write the command code register.
    fn write_command_code(&mut self, cmd: u8) -> Result<(), E> {
        self.i2c.write(self.addr, &[cmd])?;
        Ok(())
    }

    /// Read the ADM1191 status.
    pub fn read_status(&mut self) -> Result<u8, E> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.addr, &[1 << 6], &mut buf)?;
        Ok(buf[0])
    }

    /// Continuously measure current and voltage.
    pub fn continuous_volt_current(&mut self) -> Result<(), E> {
        self.write_command_code(1 << 0 | 1 << 2 | 1 << 4)
    }

    /// Read voltage and current from the device.
    pub fn read_volt_current(&mut self) -> Result<(u16, u16), E> {
        let mut buf = [0u8; 3];
        self.i2c.read(self.addr, &mut buf)?;
        let voltage = ((buf[0] as u16) << 4) | ((buf[2] >> 4) & 0xF) as u16;
        let current = ((buf[1] as u16) << 4) | (buf[2] & 0xF) as u16;
        Ok((voltage, current))
    }
}
