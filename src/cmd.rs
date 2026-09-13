use crate::Led;
/// implementation of [HID Commands](https://github.com/todbot/blink1/blob/main/docs/blink1-hid-commands.md)
use std::time::Duration;

// sizes copied from https://github.com/todbot/blink1-tool/blob/main/blink1-lib.h#L36-L41
const REPORT1_SIZE: usize = 8;

#[cfg(feature = "commands")]
const REPORT2_SIZE: usize = 60;
const REPORT1_BUF_SIZE: usize = REPORT1_SIZE + 1;
#[cfg(feature = "commands")]
const REPORT2_BUF_SIZE: usize = REPORT2_SIZE + 1;

impl From<Led> for u8 {
    fn from(led: Led) -> Self {
        match led {
            Led::All => 0,
            Led::Led1 => 1,
            Led::Led2 => 2,
        }
    }
}

/// some commands are "report id 1" and "report id 2"
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportBuf {
    Report1([u8; REPORT1_BUF_SIZE]),

    #[cfg(feature = "commands")]
    Report2([u8; REPORT2_BUF_SIZE]),
}

impl ReportBuf {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            ReportBuf::Report1(buf) => buf,
            #[cfg(feature = "commands")]
            ReportBuf::Report2(buf) => buf,
        }
    }

    #[cfg(feature = "commands")]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            ReportBuf::Report1(buf) => buf,
            ReportBuf::Report2(buf) => buf,
        }
    }
}

impl AsRef<[u8]> for ReportBuf {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

fn duration_to_fade_time(duration: &Duration) -> (u8, u8) {
    let dms = u16::try_from(duration.as_millis() / 10).unwrap_or(u16::MAX);
    let th = (dms >> 8) as u8;
    let tl = (dms & 0xff) as u8;

    (th, tl)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteCmd {
    /// Fade to RGB color: format `{ 1, 'c', r, g, b, th, tl, n }`
    ///
    /// Lights up the blink(1) with the specified RGB color,
    /// fading to that color over a specified duration.
    FadeToRgb {
        r: u8,
        g: u8,
        b: u8,
        duration: Duration,
        led: Led,
    },
    /// Set RGB color now: format `{ 1, 'n', r, g, b, 0, 0, n }` (*)
    ///
    /// Sets RGB color immediately with no fade time.
    SetRgbNow { r: u8, g: u8, b: u8, led: Led },

    #[cfg(feature = "commands")]
    /// Serverdown tickle/off: format `{ 1, 'D', on, th, tl, st, sp, ep }` (*)
    ///
    /// Enables/disables the server watchdog tickle mode with timeout,
    /// stay on flag, and start/end loop positions.
    ServerDownTickle {
        enable: bool,
        timeout: Duration,
        stay_on: bool,
        start_pos: u8,
        end_pos: u8,
    },

    #[cfg(feature = "commands")]
    /// `PlayLoop`: format `{ 1, 'p', on, sp, ep, c, 0, 0 }` (2)
    ///
    /// Starts or pauses playing a color pattern loop between `start_pos`
    /// and `end_pos` for `count` repetitions.
    PlayLoop {
        play: bool,
        start_pos: u8,
        end_pos: u8,
        count: u8,
    },
    #[cfg(feature = "commands")]
    /// Set color pattern line: format `{ 1, 'P', r, g, b, th, tl, p }`
    ///
    /// Stores an RGB color and fade duration at pattern line position `pos`.
    SetColorPatternLine {
        r: u8,
        g: u8,
        b: u8,
        duration: Duration,
        pos: u8,
    },
    #[cfg(feature = "commands")]
    /// Save color patterns: format `{ 1, 'W', 0, 0, 0, 0, 0, 0 }` (2)
    ///
    /// Saves current RAM color patterns to non-volatile flash/EEPROM storage.
    SaveColorPatterns,

    #[cfg(feature = "commands")]
    /// Set ledn: format `{ 1, 'l', n, 0, 0, 0, 0, 0 }` (2+)
    ///
    /// Sets which LED subsequent commands address.
    SetLed { led: Led },

    #[cfg(feature = "commands")]
    /// Write EEPROM location: format `{ 1, 'E', ad, v, 0, 0, 0, 0 }` (1)
    ///
    /// Writes arbitrary value `val` to EEPROM address `addr`.
    WriteEeprom { addr: u8, val: u8 },

    #[cfg(feature = "commands")]
    /// Test command: format `{ 1, '!', 0, 0, 0, 0, 0, 0 }`
    Test,
    #[cfg(feature = "commands")]
    /// Write 50-byte note: format `{ 2, 'F', noteid, data0 ... data49 }` (3)
    ///
    /// Writes a 50-byte text note to note slot `note_id`.
    WriteNote { note_id: u8, data: [u8; 50] },
    #[cfg(feature = "commands")]
    /// Go to bootloader: format `{ 1, 'G', 'o', 'B', 'o', 'o', 't', 0 }` (3)
    ///
    /// Reboots the device into USB bootloader mode.
    GoToBootloader,

    #[cfg(feature = "commands")]
    /// Lock go to bootload: format `{ 2, 'L', 'o', 'c', 'k', 'B', 'o', 'o', 't', 'l', 'o', 'a', 'd' }` (3)
    ///
    /// Locks or confirms bootloader entry lock.
    LockBootloader,

    #[cfg(feature = "commands")]
    /// Set startup params: format `{ 1, 'B', bootmode, playstart, playend, playcnt, 0, 0 }` (3)
    ///
    /// Sets default behavior when blink(1) powers up.
    SetStartupParams {
        boot_mode: u8,
        play_start: u8,
        play_end: u8,
        play_count: u8,
    },
}

impl WriteCmd {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn to_buffer(&self) -> ReportBuf {
        match self {
            WriteCmd::FadeToRgb {
                r,
                g,
                b,
                duration,
                led,
            } => {
                let (th, tl) = duration_to_fade_time(duration);
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'c';
                buf[2] = *r;
                buf[3] = *g;
                buf[4] = *b;
                buf[5] = th;
                buf[6] = tl;
                buf[7] = u8::from(*led);
                ReportBuf::Report1(buf)
            }
            WriteCmd::SetRgbNow { r, g, b, led } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'n';
                buf[2] = *r;
                buf[3] = *g;
                buf[4] = *b;
                buf[5] = 0;
                buf[6] = 0;
                buf[7] = u8::from(*led);
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::ServerDownTickle {
                enable,
                timeout,
                stay_on,
                start_pos,
                end_pos,
            } => {
                let (th, tl) = duration_to_fade_time(timeout);
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'D';
                buf[2] = u8::from(*enable);
                buf[3] = th;
                buf[4] = tl;
                buf[5] = u8::from(*stay_on);
                buf[6] = *start_pos;
                buf[7] = *end_pos;
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::PlayLoop {
                play,
                start_pos,
                end_pos,
                count,
            } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'p';
                buf[2] = u8::from(*play);
                buf[3] = *start_pos;
                buf[4] = *end_pos;
                buf[5] = *count;
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::SetColorPatternLine {
                r,
                g,
                b,
                duration,
                pos,
            } => {
                let (th, tl) = duration_to_fade_time(duration);
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'P';
                buf[2] = *r;
                buf[3] = *g;
                buf[4] = *b;
                buf[5] = th;
                buf[6] = tl;
                buf[7] = *pos;
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::SaveColorPatterns => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'W';
                buf[2] = 0xBE;
                buf[3] = 0xEF;
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::SetLed { led } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'l';
                buf[2] = u8::from(*led);
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::WriteEeprom { addr, val } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'E';
                buf[2] = *addr;
                buf[3] = *val;
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::Test => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'!';
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::WriteNote { note_id, data } => {
                let mut buf = [0u8; REPORT2_BUF_SIZE];
                buf[0] = 2;
                buf[1] = b'F';
                buf[2] = *note_id;
                buf[3..53].copy_from_slice(data);
                ReportBuf::Report2(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::GoToBootloader => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'G';
                buf[2..8].copy_from_slice(b"oBoot\0");
                ReportBuf::Report1(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::LockBootloader => {
                let mut buf = [0u8; REPORT2_BUF_SIZE];
                buf[0] = 2;
                buf[1] = b'L';
                let lock_magic = b"ockBootload";
                buf[2..2 + lock_magic.len()].copy_from_slice(lock_magic);
                ReportBuf::Report2(buf)
            }
            #[cfg(feature = "commands")]
            WriteCmd::SetStartupParams {
                boot_mode,
                play_start,
                play_end,
                play_count,
            } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'B';
                buf[2] = *boot_mode;
                buf[3] = *play_start;
                buf[4] = *play_end;
                buf[5] = *play_count;
                ReportBuf::Report1(buf)
            }
        }
    }
}

#[cfg(feature = "commands")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryCmd {
    /// Read current RGB color: format `{ 1, 'r', n, 0, 0, 0, 0, n }` (2)
    ///
    /// Returns the current RGB color of the specified LED (`{ 1, 'r', r, g, b, 0, 0, ledn }`).
    ReadCurrentRgb { led: Led },
    /// Playstate readback: format `{ 1, 'S', 0, 0, 0, 0, 0, 0 }` (2)
    ///
    /// Reads back current playing status: playing state, start/end loop, loop count, and position.
    PlaystateReadback,
    /// Read color pattern line: format `{ 1, 'R', 0, 0, 0, 0, 0, p }`
    ///
    /// Reads the RGB color, fade time, and position stored at pattern line `pos`.
    ReadColorPatternLine { pos: u8 },
    /// Read EEPROM location: format `{ 1, 'e', ad, 0, 0, 0, 0, 0 }` (1)
    ///
    /// Reads raw value stored in EEPROM address `addr`.
    ReadEeprom { addr: u8 },
    /// Get version: format `{ 1, 'v', 0, 0, 0, 0, 0, 0 }`
    ///
    /// Retrieves blink(1) firmware version.
    GetVersion,
    /// Read 50-byte note: format `{ 2, 'f', noteid, data0 ... data49 }` (3)
    ///
    /// Reads a 50-byte note from note slot `note_id`.
    ReadNote { note_id: u8 },
    /// Get startup params: format `{ 1, 'b', 0, 0, 0, 0, 0, 0 }` (3)
    ///
    /// Reads configured startup playback parameters.
    GetStartupParams,
    /// Get chip unique id: format `{ 2, 'U', 0 }` (3)
    ///
    /// Returns unique hardware chip identifier.
    GetChipUniqueId,
}

#[cfg(feature = "commands")]
impl QueryCmd {
    #[must_use]
    pub fn to_buffer(&self) -> ReportBuf {
        match self {
            QueryCmd::ReadCurrentRgb { led } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'r';
                buf[7] = u8::from(*led);
                ReportBuf::Report1(buf)
            }
            QueryCmd::PlaystateReadback => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'S';
                ReportBuf::Report1(buf)
            }
            QueryCmd::ReadColorPatternLine { pos } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'R';
                buf[7] = *pos;
                ReportBuf::Report1(buf)
            }
            QueryCmd::ReadEeprom { addr } => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'e';
                buf[2] = *addr;
                ReportBuf::Report1(buf)
            }
            QueryCmd::GetVersion => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'v';
                ReportBuf::Report1(buf)
            }
            QueryCmd::ReadNote { note_id } => {
                let mut buf = [0u8; REPORT2_BUF_SIZE];
                buf[0] = 2;
                buf[1] = b'f';
                buf[2] = *note_id;
                ReportBuf::Report2(buf)
            }
            QueryCmd::GetStartupParams => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                buf[1] = b'b';
                ReportBuf::Report1(buf)
            }
            QueryCmd::GetChipUniqueId => {
                let mut buf = [0u8; REPORT2_BUF_SIZE];
                buf[0] = 2;
                buf[1] = b'U';
                ReportBuf::Report2(buf)
            }
        }
    }

    #[must_use]
    pub fn response_buffer(&self) -> ReportBuf {
        match self {
            QueryCmd::ReadNote { .. } | QueryCmd::GetChipUniqueId => {
                let mut buf = [0u8; REPORT2_BUF_SIZE];
                buf[0] = 2;
                ReportBuf::Report2(buf)
            }
            _ => {
                let mut buf = [0u8; REPORT1_BUF_SIZE];
                buf[0] = 1;
                ReportBuf::Report1(buf)
            }
        }
    }
}
