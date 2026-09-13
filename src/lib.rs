#[cfg(not(feature = "commands"))]
use crate::hid_ext::HidDeviceExt;
mod cmd;
mod hid_ext;

use std::time::Duration;

#[cfg(not(feature = "commands"))]
use crate::cmd::WriteCmd;
#[cfg(feature = "commands")]
pub use cmd::{QueryCmd, ReportBuf, WriteCmd};
pub use hid_ext::HidApiExt;
#[cfg(feature = "commands")]
pub use hid_ext::HidDeviceExt;
use hidapi::{HidDevice, HidResult};

pub const BLINK1_VENDOR_ID: u16 = 0x27B8; /* = 0x27B8 = 10168 = thingm */
pub const BLINK1_DEVICE_ID: u16 = 0x01ED; /* = 0x01ED */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Led {
    All,
    Led1,
    Led2,
}

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
