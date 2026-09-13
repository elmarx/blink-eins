#[cfg(feature = "commands")]
use crate::ReportBuf;
#[cfg(feature = "commands")]
use crate::cmd::QueryCmd;
use crate::cmd::WriteCmd;
use hidapi::{HidDevice, HidResult};

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
