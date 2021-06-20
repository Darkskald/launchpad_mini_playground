mod launchpad;
mod midi;

use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiOutput, MidiOutputPort, Ignore, MidiOutputConnection};
use crate::midi::{get_connection};
use crate::launchpad::LaunchPad;


fn main() {
    let midi_out = get_connection();
    let pad = LaunchPad::new(midi_out);
    let test: Vec<Vec<u8>> = (0..8)
        .map(|x| x * 16)
        .map(|x| (0..9).map(|y| y + x).collect())
        .collect();

    println!("{:?}", test);

    for i in 0..8 {
        for j in 0..9 {
            let key = i * 16 + j;
            println!("{},{:X?}", key, key)
        }
    }
    //conn_out.send(&[NOTE_ON_MSG, 0x00, 0x3B]);
    //flash(&mut conn_out, 0x00,(0x00, 0x03), (0x00, 0x00));
    return;
}


fn get_color(color: (u8, u8)) -> u8 {
    let (red, green) = color;
    if red > 3 || green > 3 {
        panic!("Wrong!")
    }
    0x10 * green + red
}
/*
fn flash(mp: &mut MidiOutputConnection, led_number: u8, color1: (u8, u8), color2: (u8, u8)) {

    mp.send(&[DOUBLE_BUFFERING, led_number, 0x20]);
    mp.send(&[NOTE_ON_MSG, led_number, get_color(color1)]);
    mp.send(&[DOUBLE_BUFFERING, led_number, 0x24]);
    mp.send(&[NOTE_ON_MSG, led_number, get_color(color2)]);
    mp.send(&[DOUBLE_BUFFERING, led_number, 0x28]);

}


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}



fn run() -> Result<(), Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir test input")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("midir test output")?;

    let mut input = String::new();

    loop {
        println!("Available input ports:");
        for (i, p) in midi_in.ports().iter().enumerate() {
            println!("{}: {}", i, midi_in.port_name(p)?);
        }

        println!("\nAvailable output ports:");
        for (i, p) in midi_out.ports().iter().enumerate() {
            println!("{}: {}", i, midi_out.port_name(p)?);
        }

        // run in endless loop if "--loop" parameter is specified
        match ::std::env::args().nth(1) {
            Some(ref arg) if arg == "--loop" => {}
            _ => break
        }
        print!("\nPress <enter> to retry ...");
        stdout().flush()?;
        input.clear();
        stdin().read_line(&mut input)?;
        println!("\n");
    }

    Ok(())
}
 */