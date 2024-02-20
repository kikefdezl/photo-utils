use clap::Command;

use crate::remove_unpaired_raws::remove_unpaired_raws;

mod remove_unpaired_raws;

fn main() {
    let matches = Command::new("photo_utils")
        .version("0.1")
        .author("Kike Fernández-Laguilhoat Sánchez-Biezma")
        .about("Utilities for Photographies")
        .subcommand(
            Command::new("remove-unpaired-raws")
                .about("Remove RAW/RAF files from a folder if there is no mirror JPG/JPEG")
                .arg(
                    clap::arg!([path])
                        .required(true)
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("remove-raws", submatches)) => {
            let path = submatches.get_one::<std::path::PathBuf>("path").unwrap();
            let _ = remove_unpaired_raws(path);
            return;
        }
        _ => println!("Invalid subcommand."),
    }
}
