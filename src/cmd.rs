//! implementation of [HID Commands](https://github.com/todbot/blink1/blob/main/docs/blink1-hid-commands.md)

use crate::Led;
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
    /// build a "report id 1" buffer: `{ 1, cmd, args[0], ..., args[5] }`
    fn new_report1(cmd: u8, args: [u8; REPORT1_SIZE - 2]) -> Self {
        let mut buf = [0u8; REPORT1_BUF_SIZE];
        buf[0] = 1;
        buf[1] = cmd;
        buf[2..2 + args.len()].copy_from_slice(&args);
        ReportBuf::Report1(buf)
    }

    /// build a "report id 2" buffer: `{ 2, cmd, payload... }`
    #[cfg(feature = "commands")]
    fn new_report2(cmd: u8, payload: &[u8]) -> Self {
        let mut buf = [0u8; REPORT2_BUF_SIZE];
        buf[0] = 2;
        buf[1] = cmd;
        buf[2..2 + payload.len()].copy_from_slice(payload);
        ReportBuf::Report2(buf)
    }

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

fn into_th(dms: u16) -> u8 {
    (dms >> 8) as u8
}

fn into_tl(dms: u16) -> u8 {
    (dms & 0xff) as u8
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
        duration: u16,
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
        timeout: u16,
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
        duration: u16,
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
    #[must_use]
    pub fn to_buffer(&self) -> ReportBuf {
        match self {
            WriteCmd::FadeToRgb {
                r,
                g,
                b,
                duration,
                led,
            } => ReportBuf::new_report1(
                b'c',
                [
                    *r,
                    *g,
                    *b,
                    into_th(*duration),
                    into_tl(*duration),
                    u8::from(*led),
                ],
            ),
            WriteCmd::SetRgbNow { r, g, b, led } => {
                ReportBuf::new_report1(b'n', [*r, *g, *b, 0, 0, u8::from(*led)])
            }
            #[cfg(feature = "commands")]
            WriteCmd::ServerDownTickle {
                enable,
                timeout,
                stay_on,
                start_pos,
                end_pos,
            } => ReportBuf::new_report1(
                b'D',
                [
                    u8::from(*enable),
                    into_th(*timeout),
                    into_tl(*timeout),
                    u8::from(*stay_on),
                    *start_pos,
                    *end_pos,
                ],
            ),
            #[cfg(feature = "commands")]
            WriteCmd::PlayLoop {
                play,
                start_pos,
                end_pos,
                count,
            } => {
                ReportBuf::new_report1(b'p', [u8::from(*play), *start_pos, *end_pos, *count, 0, 0])
            }
            #[cfg(feature = "commands")]
            WriteCmd::SetColorPatternLine {
                r,
                g,
                b,
                duration,
                pos,
            } => ReportBuf::new_report1(
                b'P',
                [*r, *g, *b, into_th(*duration), into_tl(*duration), *pos],
            ),
            #[cfg(feature = "commands")]
            WriteCmd::SaveColorPatterns => ReportBuf::new_report1(b'W', [0xBE, 0xEF, 0, 0, 0, 0]),
            #[cfg(feature = "commands")]
            WriteCmd::SetLed { led } => {
                ReportBuf::new_report1(b'l', [u8::from(*led), 0, 0, 0, 0, 0])
            }
            #[cfg(feature = "commands")]
            WriteCmd::WriteEeprom { addr, val } => {
                ReportBuf::new_report1(b'E', [*addr, *val, 0, 0, 0, 0])
            }
            #[cfg(feature = "commands")]
            WriteCmd::Test => ReportBuf::new_report1(b'!', [0; REPORT1_SIZE - 2]),
            #[cfg(feature = "commands")]
            WriteCmd::WriteNote { note_id, data } => {
                let mut payload = [0u8; 51];
                payload[0] = *note_id;
                payload[1..].copy_from_slice(data);
                ReportBuf::new_report2(b'F', &payload)
            }
            #[cfg(feature = "commands")]
            WriteCmd::GoToBootloader => ReportBuf::new_report1(b'G', *b"oBoot\0"),
            #[cfg(feature = "commands")]
            WriteCmd::LockBootloader => ReportBuf::new_report2(b'L', b"ockBootload"),
            #[cfg(feature = "commands")]
            WriteCmd::SetStartupParams {
                boot_mode,
                play_start,
                play_end,
                play_count,
            } => ReportBuf::new_report1(
                b'B',
                [*boot_mode, *play_start, *play_end, *play_count, 0, 0],
            ),
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
                ReportBuf::new_report1(b'r', [0, 0, 0, 0, 0, u8::from(*led)])
            }
            QueryCmd::PlaystateReadback => ReportBuf::new_report1(b'S', [0; REPORT1_SIZE - 2]),
            QueryCmd::ReadColorPatternLine { pos } => {
                ReportBuf::new_report1(b'R', [0, 0, 0, 0, 0, *pos])
            }
            QueryCmd::ReadEeprom { addr } => ReportBuf::new_report1(b'e', [*addr, 0, 0, 0, 0, 0]),
            QueryCmd::GetVersion => ReportBuf::new_report1(b'v', [0; REPORT1_SIZE - 2]),
            QueryCmd::ReadNote { note_id } => ReportBuf::new_report2(b'f', &[*note_id]),
            QueryCmd::GetStartupParams => ReportBuf::new_report1(b'b', [0; REPORT1_SIZE - 2]),
            QueryCmd::GetChipUniqueId => ReportBuf::new_report2(b'U', &[]),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_to_rgb() {
        let cmd = WriteCmd::FadeToRgb {
            r: 0x10,
            g: 0x20,
            b: 0x30,
            duration: 0x0102,
            led: Led::Led1,
        };
        let actual = cmd.to_buffer();
        assert_eq!(
            actual.as_slice(),
            &[1, b'c', 0x10, 0x20, 0x30, 0x01, 0x02, 1, 0]
        );
    }

    #[test]
    fn set_rgb_now() {
        let cmd = WriteCmd::SetRgbNow {
            r: 0x10,
            g: 0x20,
            b: 0x30,
            led: Led::All,
        };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'n', 0x10, 0x20, 0x30, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn server_down_tickle() {
        let cmd = WriteCmd::ServerDownTickle {
            enable: true,
            timeout: 0x0102,
            stay_on: true,
            start_pos: 3,
            end_pos: 4,
        };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'D', 1, 0x01, 0x02, 1, 3, 4, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn play_loop() {
        let cmd = WriteCmd::PlayLoop {
            play: true,
            start_pos: 1,
            end_pos: 2,
            count: 3,
        };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'p', 1, 1, 2, 3, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn set_color_pattern_line() {
        let cmd = WriteCmd::SetColorPatternLine {
            r: 0x10,
            g: 0x20,
            b: 0x30,
            duration: 0x0102,
            pos: 5,
        };
        let actual = cmd.to_buffer();
        assert_eq!(
            actual.as_slice(),
            &[1, b'P', 0x10, 0x20, 0x30, 0x01, 0x02, 5, 0]
        );
    }

    #[test]
    #[cfg(feature = "commands")]
    fn save_color_patterns() {
        let cmd = WriteCmd::SaveColorPatterns;
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'W', 0xBE, 0xEF, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn set_led() {
        let cmd = WriteCmd::SetLed { led: Led::Led2 };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'l', 2, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn write_eeprom() {
        let cmd = WriteCmd::WriteEeprom {
            addr: 0x10,
            val: 0x20,
        };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'E', 0x10, 0x20, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn test_cmd() {
        let cmd = WriteCmd::Test;
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'!', 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn write_note() {
        let mut data = [0u8; 50];
        data[0] = 0xAA;
        data[49] = 0xBB;
        let cmd = WriteCmd::WriteNote { note_id: 7, data };

        let actual = cmd.to_buffer();
        let actual = actual.as_slice();
        assert_eq!(actual.len(), REPORT2_BUF_SIZE);
        assert_eq!(actual[0], 2);
        assert_eq!(actual[1], b'F');
        assert_eq!(actual[2], 7);
        assert_eq!(actual[3], 0xAA);
        assert_eq!(actual[52], 0xBB);
        // remaining bytes are untouched padding
        assert!(actual[53..].iter().all(|&b| b == 0));
    }

    #[test]
    #[cfg(feature = "commands")]
    fn go_to_bootloader() {
        let cmd = WriteCmd::GoToBootloader;
        let actual = cmd.to_buffer();
        assert_eq!(
            actual.as_slice(),
            &[1, b'G', b'o', b'B', b'o', b'o', b't', 0, 0]
        );
    }

    #[test]
    #[cfg(feature = "commands")]
    fn lock_bootloader() {
        let cmd = WriteCmd::LockBootloader;
        let buf = cmd.to_buffer();
        let actual = buf.as_slice();
        assert_eq!(actual.len(), REPORT2_BUF_SIZE);
        assert_eq!(&actual[0..13], b"\x02LockBootload");
        assert!(actual[13..].iter().all(|&b| b == 0));
    }

    #[test]
    #[cfg(feature = "commands")]
    fn set_startup_params() {
        let cmd = WriteCmd::SetStartupParams {
            boot_mode: 1,
            play_start: 2,
            play_end: 3,
            play_count: 4,
        };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'B', 1, 2, 3, 4, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn read_current_rgb() {
        let cmd = QueryCmd::ReadCurrentRgb { led: Led::Led1 };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'r', 0, 0, 0, 0, 0, 1, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn playstate_readback() {
        let cmd = QueryCmd::PlaystateReadback;
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'S', 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn read_color_pattern_line() {
        let cmd = QueryCmd::ReadColorPatternLine { pos: 9 };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'R', 0, 0, 0, 0, 0, 9, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn read_eeprom() {
        let cmd = QueryCmd::ReadEeprom { addr: 0x42 };
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'e', 0x42, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn get_version() {
        let cmd = QueryCmd::GetVersion;
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'v', 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn read_note() {
        let cmd = QueryCmd::ReadNote { note_id: 3 };
        let buf = cmd.to_buffer();
        let actual = buf.as_slice();
        assert_eq!(actual.len(), REPORT2_BUF_SIZE);
        assert_eq!(&actual[0..3], &[2, b'f', 3]);
        assert!(actual[3..].iter().all(|&b| b == 0));
    }

    #[test]
    #[cfg(feature = "commands")]
    fn get_startup_params() {
        let cmd = QueryCmd::GetStartupParams;
        let actual = cmd.to_buffer();
        assert_eq!(actual.as_slice(), &[1, b'b', 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "commands")]
    fn get_chip_unique_id() {
        let cmd = QueryCmd::GetChipUniqueId;
        let buf = cmd.to_buffer();
        let actual = buf.as_slice();
        assert_eq!(actual.len(), REPORT2_BUF_SIZE);
        assert_eq!(&actual[0..2], &[2, b'U']);
        assert!(actual[2..].iter().all(|&b| b == 0));
    }

    #[test]
    #[cfg(feature = "commands")]
    fn response_buffer_sizes() {
        let actual = QueryCmd::GetVersion.response_buffer();
        assert_eq!(actual.as_slice().len(), REPORT1_BUF_SIZE);

        let actual = QueryCmd::ReadNote { note_id: 0 }.response_buffer();
        assert_eq!(actual.as_slice().len(), REPORT2_BUF_SIZE);

        let actual = QueryCmd::GetChipUniqueId.response_buffer();
        assert_eq!(actual.as_slice().len(), REPORT2_BUF_SIZE);
    }
}
