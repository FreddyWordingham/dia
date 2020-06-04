//! Player binary.

use dia::*;
use std::{collections::HashMap, path::PathBuf};

/// Main function.
pub fn main() {
    banner::title("Player");
    let (in_dir, _out_dir) = init();

    let bytes =
        std::fs::read(&in_dir.join("JennyWasaFriendofMine.mid")).expect("Could not load file.");
    let smf = midly::Smf::parse(&bytes).unwrap();

    let mut total_time = 0;
    let mut music: HashMap<u8, HashMap<u8, u32>> = HashMap::new();
    for track in smf.tracks.iter() {
        for (_i, event) in track.iter().enumerate() {
            let t = event.delta.as_int();
            total_time += t;
            if total_time > 45000 {
                pause!((t / 30) as u64);
            }
            print!("{:>8} : ", format!("{:?}", total_time as f64 / 1000.0));
            // print!("{:<16} : ", format!("{:?}", t));
            // print!("{:<16} : ", i);

            if let midly::EventKind::Midi { channel, message } = event.kind {
                let ch = channel.as_int();
                print!("{:^8} : ", ch);

                music.entry(ch).or_insert_with(HashMap::new);
                let instrument = music.get_mut(&ch).unwrap();

                match message {
                    midly::MidiMessage::NoteOff { key, vel } => {
                        let key = key.as_int();
                        let _vel = vel.as_int();

                        instrument.entry(key).or_insert(0);
                    }
                    midly::MidiMessage::NoteOn { key, vel } => {
                        let key = key.as_int();
                        let _vel = vel.as_int();

                        instrument.entry(key).or_insert(0);
                        *instrument.get_mut(&key).unwrap() += 1;
                        print!("{}", key);
                    }
                    _ => {}
                }
            }

            println!();
        }
    }

    let mut total_hits = 0;
    for (instrument, notes) in music {
        for (note, vel) in notes {
            print!("{:>8} : ", instrument);
            print!("{:^8} : ", note);
            println!("{}", vel);
            total_hits += vel;
        }
        println!("_____");
    }
    println!("Total hits: {}", total_hits);

    banner::section("Finished");
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Command line args");
    args!(bin_path: PathBuf);
    report!("binary path", bin_path.display());

    banner::sub_section("Directories");
    let (in_dir, out_dir) = dir::io_dirs(None, None).expect("Could not initialise directories");
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());

    (in_dir, out_dir)
}
