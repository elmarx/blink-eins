use crate::Led;
use crate::cmd::WriteCmd;
use crate::hid_device_ext::HidDeviceExt;
use hidapi::{HidDevice, HidResult};
use std::time::Duration;

pub trait Blink1Device {
    /// maximum duration supported is 10 minutes, 55 seconds (`u16::Max` * 10 milliseconds)
    ///
    /// # Errors
    ///
    /// `HidResult` if the underlying HID operation fails
    fn fade_to_rgb(&self, r: u8, g: u8, b: u8, duration: Duration, led: Led) -> HidResult<()>;

    /// # Errors
    ///
    /// `HidResult` if the underlying HID operation fails
    fn set_rgb_now(&self, r: u8, g: u8, b: u8, led: Led) -> HidResult<()>;
}

impl Blink1Device for HidDevice {
    fn fade_to_rgb(&self, r: u8, g: u8, b: u8, duration: Duration, led: Led) -> HidResult<()> {
        self.send_cmd(&WriteCmd::FadeToRgb {
            r,
            g,
            b,
            duration,
            led,
        })
    }

    fn set_rgb_now(&self, r: u8, g: u8, b: u8, led: Led) -> HidResult<()> {
        self.send_cmd(&WriteCmd::SetRgbNow { r, g, b, led })
    }
}
