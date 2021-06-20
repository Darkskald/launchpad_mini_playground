use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiOutput, MidiOutputPort, Ignore, MidiOutputConnection};

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const DOUBLE_BUFFERING: u8 = 0xB0;



fn main() {
    let midi_out = MidiOutput::new("Launchpad").expect("Fail");
    let output_port = get_port(&midi_out).expect("Port creation failed");
    let mut conn_out = midi_out.connect(&output_port, "midir-test")
        .expect("Midi connection failed");

    //conn_out.send(&[NOTE_ON_MSG, 0x00, 0x3B]);
    flash(&mut conn_out, 0x00,(0x00, 0x03), (0x00, 0x00));
    return;
}

fn get_port(op: &MidiOutput) -> Result<MidiOutputPort, Box<dyn Error>> {
    let out_ports = op.ports();
    let out_port: MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!("Choosing the only available output port: {}", op.port_name(&out_ports[0]).unwrap());
            out_ports[0].clone()
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, op.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports.get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?.clone()
        }
    };
    return Ok(out_port);
}

fn get_color(color: (u8, u8)) -> u8 {
    let (red, green) = color;
    if red > 3 || green > 3 {
        panic!("Wrong!")
    }
    0x10 * green + red
}

fn flash(mp: &mut MidiOutputConnection, led_number: u8, color1: (u8, u8), color2: (u8, u8)) {

    mp.send(&[DOUBLE_BUFFERING, led_number, 0x20]);
    mp.send(&[NOTE_ON_MSG, led_number, get_color(color1)]);
    mp.send(&[DOUBLE_BUFFERING, led_number, 0x24]);
    mp.send(&[NOTE_ON_MSG, led_number, get_color(color2)]);
    mp.send(&[DOUBLE_BUFFERING, led_number, 0x28]);

}

/*
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