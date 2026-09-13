#[cfg(feature = "commands")]
use crate::cmd::{QueryCmd, ReportBuf};

use crate::cmd::WriteCmd;
use crate::{BLINK1_DEVICE_ID, BLINK1_VENDOR_ID};
use hidapi::{HidApi, HidDevice, HidResult};

pub trait HidDeviceExt {
    /// # Errors
    ///
    /// `HidResult` if the underlying HID operation fails
    fn send_cmd(&self, cmd: &WriteCmd) -> HidResult<()>;

    #[cfg(feature = "commands")]
    /// # Errors
    ///
    /// `HidResult` if the underlying HID operation fails
    fn query_cmd(&self, cmd: &QueryCmd) -> HidResult<ReportBuf>;
}

pub trait HidApiExt {
    /// # Errors
    ///
    /// `HidResult` if the underlying HID operation fails
    fn open_blink1(&self) -> HidResult<HidDevice>;
}

impl HidDeviceExt for HidDevice {
    fn send_cmd(&self, cmd: &WriteCmd) -> HidResult<()> {
        let buf = cmd.to_buffer();
        self.send_feature_report(buf.as_slice())
    }

    #[cfg(feature = "commands")]
    fn query_cmd(&self, cmd: &QueryCmd) -> HidResult<ReportBuf> {
        let request_buf = cmd.to_buffer();
        self.send_feature_report(request_buf.as_slice())?;

        let mut response_buf = cmd.response_buffer();
        self.get_feature_report(response_buf.as_mut_slice())?;
        Ok(response_buf)
    }
}

impl HidApiExt for HidApi {
    fn open_blink1(&self) -> HidResult<HidDevice> {
        self.open(BLINK1_VENDOR_ID, BLINK1_DEVICE_ID)
    }
}
