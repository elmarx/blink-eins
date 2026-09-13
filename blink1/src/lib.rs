mod blink1device;
mod cmd;
mod hid_device_ext;
mod hidapi_ext;

#[cfg(feature = "commands")]
pub use cmd::{QueryCmd, ReportBuf, WriteCmd};
#[cfg(feature = "commands")]
pub use hid_device_ext::HidDeviceExt;
pub use hidapi_ext::HidApiExt;

pub use blink1device::Blink1Device;

pub const BLINK1_VENDOR_ID: u16 = 0x27B8; /* = 0x27B8 = 10168 = thingm */
pub const BLINK1_DEVICE_ID: u16 = 0x01ED; /* = 0x01ED */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Led {
    All,
    Led1,
    Led2,
}
