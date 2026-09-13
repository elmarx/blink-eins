use std::time::Duration;

use blink_one::Blink1Device;
use blink_one::HidApiExt;
use blink_one::Led;
use hidapi::{HidApi, HidResult};

fn main() -> HidResult<()> {
    let api = HidApi::new()?;
    let device = api.open_blink1()?;

    device.fade_to_rgb(0, 0, 255, Duration::from_secs(1), Led::All)
}
