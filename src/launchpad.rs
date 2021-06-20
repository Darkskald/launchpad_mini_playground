use midir::MidiOutputConnection;
use std::arch::x86_64::_mm_extract_si64;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const DOUBLE_BUFFERING: u8 = 0xB0;

struct LedColor {
    red: u8,
    green: u8,
}

impl LedColor {
    fn new(red: u8, green: u8) -> LedColor {
        if red > 3 || green > 3 {
            panic!("Invalid value for color!")
        }
        return LedColor { red, green };
    }

    fn get_hex_data(&self) -> u8 {
        0x10 * self.green + self.red
    }
}

pub struct LaunchPad {
    conn: MidiOutputConnection,
    flash_state: bool,
}

impl LaunchPad {
    pub(crate) fn new(conn: MidiOutputConnection) -> LaunchPad {
        let mut lp = LaunchPad {
            conn: conn,
            flash_state: false,
        };
        return lp;
        // todo set flash_state to false
        // todo is it possible to disable flashing for all keys? --> 0-120
        // lp.buffer_msg()
    }

    fn keys_as_vec() -> Vec<Vec<u8>> {
        let keys: Vec<Vec<u8>> = (0..8)
            .map(|x| x * 16)
            .map(|x| (0..9).map(|y| y + x).collect())
            .collect();

        return keys;
    }

    // todo clear_flash
    // todo check that key value is valid

    fn buffer_msg(&mut self, key: u8, data: u8) {
        self.conn.send(&[DOUBLE_BUFFERING, key, data]);
    }

    fn led_on_msg(&mut self, key: u8, color: LedColor) {
        self.conn.send(&[NOTE_ON_MSG, key, color.get_hex_data()]);
    }


}