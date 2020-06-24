use linux_embedded_hal::I2cdev;

use adm1191::Adm1191;

/// Address of I2C device.
pub const ADDRESS: u8 = 0b011_0101;

fn main() -> Result<(), linux_embedded_hal::i2cdev::linux::LinuxI2CError> {
    let dev = I2cdev::new("/dev/i2c-1")?;
    let mut adm1191 = Adm1191::new(dev, ADDRESS)?;

    let status = adm1191.read_status()?;
    println!("status: {:x}", status);

    adm1191.continuous_volt_current()?;

    let status = adm1191.read_status()?;
    println!("status: {:x}", status);

    let (voltage, current) = adm1191.read_volt_current()?;
    println!("voltage: {} current: {}", voltage, current);

    let _dev = adm1191.destroy();

    Ok(())
}
