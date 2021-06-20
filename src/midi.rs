use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};
use std::error::Error;
use std::io::{stdout, Write, stdin};

// todo add documentation

pub fn get_connection() -> MidiOutputConnection {
    let midi_out = MidiOutput::new("Launchpad").expect("Fail");
    let output_port = get_port(&midi_out).expect("Port creation failed");
    return midi_out.connect(&output_port, "midir-test")
        .expect("Midi connection failed");
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