use crate::{BLINK1_DEVICE_ID, BLINK1_VENDOR_ID};
use hidapi::{HidApi, HidDevice, HidResult};

pub trait HidApiExt {
    /// # Errors
    ///
    /// `HidResult` if the underlying HID operation fails
    fn open_blink1(&self) -> HidResult<HidDevice>;
}

impl HidApiExt for HidApi {
    fn open_blink1(&self) -> HidResult<HidDevice> {
        self.open(BLINK1_VENDOR_ID, BLINK1_DEVICE_ID)
    }
}
