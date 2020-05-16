//! Player binary.

use dia::*;
use std::path::PathBuf;

/// Main function.
pub fn main() {
    banner::title("Player");
    let (in_dir, _out_dir) = init();

    let bytes = std::fs::read(&in_dir.join("72460.mid")).expect("Could not load file.");
    let smf = midly::Smf::parse(&bytes).unwrap();

    for track in smf.tracks.iter() {
        for (i, event) in track.iter().enumerate() {
            let t = event.delta;
            print!("{:<16} : ", i);
            print!("{:<16} : ", format!("{:?}", t));

            match event.kind {
                midly::EventKind::Midi { channel, message } => {
                    print!("{:<16} : ", format!("{:?}", message));
                }
                _ => {}
            }

            println!();
        }
    }

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
